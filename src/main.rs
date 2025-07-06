use anyhow::Context;
use std::path::Path;

use clap::Parser;

mod decoder;
mod encoder;

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
    let output = Path::new(&args.output);
    encoder::encode_and_save(input, output).context("failed to encode input wave file")?;

    Ok(())
}
