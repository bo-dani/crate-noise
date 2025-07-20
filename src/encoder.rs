use anyhow::Context;
use std::path::Path;

use hound::{WavReader, WavWriter};

static INDEX_TABLE: [i8; 8] = [-1, -1, -1, -1, 2, 4, 6, 8];

static STEP_SIZE_TABLE: [i16; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];

pub(crate) fn encode_and_save(input: &Path, output: &Path) -> anyhow::Result<()> {
    let mut reader = WavReader::open(input).context("failed to read wave file")?;
    let samples: Vec<i16> = reader
        .samples::<i16>()
        .map(|s| {
            if let Ok(s) = s {
                return s;
            } else {
                // TODO Not sure if this is the right thing to do.
                return 0;
            }
        })
        .collect();

    let mut predicted: i32 = samples.as_slice()[0] as i32;
    let mut step_index: isize = 0;
    let mut compressed = Vec::new();
    compressed.push(predicted);

    for &s in samples[1..].iter() {
        let mut delta = s - predicted as i16;
        let step_size = STEP_SIZE_TABLE[step_index as usize];
        let mut code = 0;
        if delta >= step_size {
            code |= 0b100;
            delta -= step_size;
        }
        if delta >= (step_size / 2) {
            code |= 0b010;
            delta -= step_size / 2;
        }
        if delta >= (step_size / 4) {
            code |= 0b001;
        }

        if delta < 0 {
            code |= 0b1000;
        }

        delta = step_size / 8;
        if code | 0b0001 != 0 {
            delta += step_size / 4;
        }
        if code | 0b0010 != 0 {
            delta += step_size / 2;
        }
        if code | 0b0100 != 0 {
            delta += step_size;
        }

        if code | 0b1000 != 0 {
            delta = -delta;
        }

        predicted += delta as i32;
        predicted = predicted.clamp(-32768, 32768);

        step_index += INDEX_TABLE[code & 0b0111] as isize;
        step_index = step_index.clamp(0, 88);

        compressed.push(predicted);
    }

    let mut writer =
        WavWriter::create(output, reader.spec()).context("failed to create wave writer")?;
    for s in compressed.chunks(2) {
        writer.write_sample(s).context("failed to write sample")?;
    }
    writer.finalize().context("failed to close writer")?;

    Ok(())
}
