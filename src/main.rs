use std::path::PathBuf;

use clap::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    in_file: PathBuf,

    #[arg(value_name = "FILE")]
    out_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let mut reader = hound::WavReader::open(cli.in_file).unwrap();
    let len = reader.len();
    let spec = reader.spec();
    let mut writer = hound::WavWriter::create(cli.out_file, spec).unwrap();

    println!("WAV channels: {}", spec.channels);
    println!("WAV sample rate: {}", spec.sample_rate);
    println!("WAV bits per sample: {}", spec.bits_per_sample);

    let mut min: f64 = 0.0;
    let mut max: f64 = 0.0;
    let mut avg: f64 = 0.0;
    for s in reader.samples::<i16>() {
	let sample = s.unwrap() as f64;
	if sample > max {
	    max = sample;
	}
	if sample < min {
	    min = sample;
	}
	avg += sample / len as f64
    }

    println!("Minimum is {}", min);
    println!("Maximum is {}", max);
    println!("Average is {}", avg);

    reader.seek(0).expect("Unable to seek the beginning of the input file");

    for s in reader.samples::<i16>() {
	let sample = s.unwrap() as f64 - avg;
	writer.write_sample(sample as i16).unwrap();
    }
    writer.finalize().unwrap()
}
