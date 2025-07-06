use anyhow::Context;
use std::path::Path;

use hound::{WavReader, WavWriter};

pub(crate) fn encode_and_save(input: &Path, output: &Path) -> anyhow::Result<()> {
    let mut reader = WavReader::open(input).context("failed to read wave file")?;
    let samples: Vec<i32> = reader
        .samples::<i32>()
        .map(|s| {
            if let Ok(s) = s {
                return s;
            } else {
                // TODO Not sure if this is the right thing to do.
                return 0;
            }
        })
        .collect();

    // TODO Do the actual encoding

    let mut writer =
        WavWriter::create(output, reader.spec()).context("failed to create wave writer")?;
    for s in samples {
        writer.write_sample(s).context("failed to write sample")?;
    }
    writer.finalize().context("failed to close writer")?;

    Ok(())
}
