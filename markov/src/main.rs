use rand::{seq::SliceRandom, thread_rng, Rng};
use regex::Regex;
use std::{
    collections::HashMap, error::Error, fs::OpenOptions, io::Read, path::PathBuf, str::FromStr,
};
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

    // grab our random starting word seqence
    let mut rng = thread_rng();
    let i = rng.gen_range(0, words.len() - 3);

    let mut w0 = words[i];
    let mut w1 = words[i + 1];
    let mut w2 = words[i + 2];

    // create the words table
    let lookup = build_table(words);

    for _ in 0..length {
        // append to output
        print!("{} ", w2);

        w2 = &lookup[&(w0, w1)].choose(&mut rng).unwrap();
        w0 = w1;
        w1 = w2;
    }

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
        .unwrap_or_else(|| PathBuf::from_str("poetry.txt").unwrap());
    let length = opt.length.unwrap_or(150);

    if let Err(e) = run(filename, length) {
        // if error, exit 1
        eprintln!("ERROR: {}", e);
        ::std::process::exit(1);
    }
}
