use std::{
    collections::HashMap, error::Error, fs::OpenOptions, io::Read, path::PathBuf, str::FromStr,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)] // StructOpt for cli args, debug for toString()
#[structopt(name = "markov")]
struct Opt {
    /// Input Text File
    #[structopt(short = "i", long = "input")]
    input: Option<PathBuf>,
    /// Output Length
    #[structopt(short = "l", long = "length")]
    length: Option<u32>,
}

fn read_file(filename: PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    let file_str = read_file(input)?;
    println!("File String:\n{}", file_str);
    Ok(())
}

// Example Usage:
//
// With defined output length
// ./markov -i input.txt -l 500
//
// With no defined output length
// it will default to 350
// ./markov -i input.txt
//
fn main() {
    let opt = Opt::from_args();
    let filename = opt
        .input
        .unwrap_or_else(|| PathBuf::from_str("example.txt").unwrap());
    let length = opt.length.unwrap_or(350);

    if let Err(e) = run(filename, length) { // if error, exit 1
        eprintln!("ERROR: {}", e);
        ::std::process::exit(1);
    }
}
