use std::path::PathBuf;

use clap::{Parser};
mod moving_average_correction;

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

    match spec.sample_format {
	hound::SampleFormat::Float => {
	    let samples = reader.samples::<f32>().map(|x| x.unwrap());
	    let correction = moving_average_correction::MovingAverageDCOffsetCorrection::new(
		samples,
		cli.samples_to_consider,
	    );
	    for s in correction {
		writer.write_sample(s).ok();
	    }
	},
	hound::SampleFormat::Int => match spec.bits_per_sample {
	    16 => {
		let samples = reader.samples::<i16>().map(|x| x.unwrap());
		let correction = moving_average_correction::MovingAverageDCOffsetCorrection::new(
		    samples,
		    cli.samples_to_consider,
		);
		for s in correction {
		    writer.write_sample(s).ok();
		}
	    },
	    _ => {
		let samples = reader.samples::<i32>().map(|x| x.unwrap());
		let correction = moving_average_correction::MovingAverageDCOffsetCorrection::new(
		    samples,
		    cli.samples_to_consider,
		);
		for s in correction {
		    writer.write_sample(s).ok();
		}
	    },
	},
    };

    writer.finalize().unwrap();
}
