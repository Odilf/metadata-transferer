use std::{process::Command, fs::{self, metadata}, path::PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
	#[clap(parse(from_os_str))]
	input: PathBuf,

	#[clap(parse(from_os_str))]
	output: PathBuf,
}

fn main() {
	let args = Cli::parse();

	let inputs = get_input(&args.input).expect("Invalid input");
	let outputs = get_input(&args.output).expect("Invalid output");

	// for (input, output) in input.iter().zip(output) {
	// 	println!("in: {:?}. out: {:?}", input, output);
	// }

	for input in inputs {
		let output = outputs.iter().find(|output| output.with_extension("").file_name() == input.with_extension("").file_name());

		if let Some(output) = output {
			println!("Input: {:?}, (matching) output: {:?}", input, output);

			set_metadata(&input, output)
		} else {
			println!("Didn't find match for {:?}", input);
		}
	}
}

fn get_input(path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
	let metadata = metadata(path)?;
	if metadata.is_file() {
		return Ok(vec![path.to_path_buf()])
	} else {
		let files = fs::read_dir(path).expect("Caca");
		let paths = files
				.map(|file| {
					file.expect("caca")
					.path()
				})
				.collect();

		Ok(paths)
	}
}

fn set_metadata(input: &PathBuf, output: &PathBuf) {
	let res = Command::new("touch")
		.args(["-r", input.to_str().unwrap(), output.to_str().unwrap()])
		.output();

	if let Err(err) = res {
		eprintln!("Error. {}", err)
	}
}