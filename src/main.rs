use hound;
use std::i16;
use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input = Path::new(&args.input);
    if !input.exists() {
        anyhow::bail!("input wave file does not exist");
    }

    let mut reader = hound::WavReader::open(input).unwrap();
    let sqr_sum = reader.samples::<i16>().fold(0.0, |sqr_sum, s| {
        let sample = s.unwrap() as f64;
        sqr_sum + sample * sample
    });
    println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
    Ok(())
}
