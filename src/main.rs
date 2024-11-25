use std::path::PathBuf;

use clap::{Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "IN_FILE")]
    in_file: PathBuf,

    #[arg(value_name = "OUT_FILE")]
    out_file: PathBuf,

    #[arg(short, long, value_name = "N", default_value_t = 1000)]
    samples_to_consider: usize,
}

fn main() {
    let cli = Cli::parse();
    let mut reader = hound::WavReader::open(cli.in_file).unwrap();
    let spec = reader.spec();
    let mut writer = hound::WavWriter::create(cli.out_file, spec).unwrap();

    let mut rolling_avg = 0.0;
    let center = cli.samples_to_consider / 2;
    let mut samples_for_avg = vec![0.0; cli.samples_to_consider];

    for (i, s) in reader.samples::<i16>().enumerate() {
	let sample = s.unwrap() as f64;
	let write_index = i % cli.samples_to_consider;
	let read_index = (i + center) % cli.samples_to_consider;
	let sample_to_remove = samples_for_avg[write_index];
	rolling_avg -= sample_to_remove / cli.samples_to_consider as f64;
	samples_for_avg[write_index] = sample;
	rolling_avg += sample / cli.samples_to_consider as f64;
	if i < center {
	    continue;
	} else if i == cli.samples_to_consider {
	    for j in 0..center {
		let new_sample = (samples_for_avg[j] - rolling_avg) as i16;
		writer.write_sample(new_sample).unwrap();
	    }
	}
	let new_sample = (samples_for_avg[read_index] - rolling_avg) as i16;
	writer.write_sample(new_sample).unwrap();
    }
    for i in center + 1..cli.samples_to_consider {
	writer.write_sample((samples_for_avg[i] - rolling_avg) as i16).unwrap();
    }
    writer.finalize().unwrap()
}
