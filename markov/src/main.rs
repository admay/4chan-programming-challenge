use std::{
    collections::HashMap, error::Error, fs::OpenOptions, io::Read, path::PathBuf, str::FromStr,
};
use regex::Regex;
use structopt::StructOpt;

#[macro_use]
extern crate itertools;

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

fn build_table(words: Vec<&str>) -> HashMap<(&str, &str), Vec<&str>> {
    let mut ret = HashMap::new();
    for (w0, w1, w2) in izip!(&words, &words[1..], &words[2..]) {
        // add w2 to the key (w0, w1)
        let current = ret.entry((*w0, *w1)).or_insert_with(Vec::new);
        current.push(*w2);
    }
    ret
}

fn split_words(w: &str) -> Vec<&str> {
    let spaces_re = Regex::new(r" +").unwrap();
    spaces_re.split(w).collect::<Vec<&str>>()
}

fn read_file(filename: PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn run(input: PathBuf, length: u32) -> Result<(), Box<dyn Error>> {
    // read file and build lookup table
    let file_str = read_file(input)?;
    let words = split_words(&file_str);
    let words_table = build_table(words);

    println!("File String:\n{}\n\n", file_str);
    println!("Words Table:\n{:#?}\n\n", words_table);
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
