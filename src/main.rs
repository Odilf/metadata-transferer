use std::{process::Command, fs::{self, metadata}, path::PathBuf};
use clap::Parser;


#[derive(Parser)]
#[clap(name = clap::crate_name!())]
#[clap(version = clap::crate_version!())]
#[clap(about = clap::crate_description!())]
struct Cli {
	/// Input path (reference, doesn't get affected)
	#[clap(parse(from_os_str))]
	input: PathBuf,
	
	/// Output path (DOES GET AFFECTED)
	#[clap(parse(from_os_str))]
	output: PathBuf,
	
	/// Try to match the extension of files [default: false]
	#[clap(short, long)]
	match_extension: bool,

	/// Print extra info
	#[clap(short, long, default_value="true", takes_value=true)]
	verbose: bool,

	/// Don't actually write the metadata
	#[clap(short, long)]
	dry_run: bool,
}

fn main() {
	let args = Cli::parse();

	let inputs = get_input(&args.input).expect("Invalid input");
	let outputs = get_input(&args.output).expect("Invalid output");

	for input in inputs {
		let output = outputs.iter().find(|output| do_match(&input, output, args.match_extension));

		if let Some(output) = output {
			set_metadata(&input, output, args.verbose, args.dry_run)
		} else if args.verbose {
			eprintln!("Didn't find match for {:?}", input);
		}
	}

	println!("{:?}", args.match_extension)
}

fn do_match(a: &PathBuf, b: &PathBuf, match_extension: bool) -> bool {
	if match_extension {
		a.with_extension("").file_name() == b.with_extension("").file_name()
	} else {
		a.file_name() == b.file_name()
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



fn set_metadata(input: &PathBuf, output: &PathBuf, verbose: bool, dry: bool) {
	let creation_date = Command::new("mdls")
		.args(["--name", "kMDItemContentCreationDate", input.to_str().unwrap()])
		.output();

	let creation_date = String::from_utf8(
		creation_date
		.expect(format!("File {:?} doesn't have creation date", input).as_str())
		.stdout
	).unwrap();
	
	if verbose {
		println!("Setting metadata of {:?} to {}", output, creation_date);
	}

	if dry {
		return
	}

	let res = Command::new("touch")
		.args(["-r", input.to_str().unwrap(), output.to_str().unwrap()])
		.output();

	if let Err(err) = res {
		eprintln!("Error. {}", err)
	}
}