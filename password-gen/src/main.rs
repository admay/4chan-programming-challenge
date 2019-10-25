use structopt::StructOpt;
use std::process;
use rand::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

// Constants used for random character selection when generating a password
const LOWERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

const UPPERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z',
];

const SPECIALS: [char; 32] = [
    '!', '@', '#', '$', '%',
    '^', '&', '*', '(', ')',
    '-', '_', '=', '+', '{',
    '}', '[', ']', '|', '\\',
    ':', ';', '"', '\'', '<',
    ',', '>', '.', '?', '/',
    '~', '`',
];

const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

#[derive(Debug)]
enum CharType {
    Lower,
    Upper,
    Special,
    Number,
}

impl Distribution<CharType> for Standard {
    fn sample<R: Rng + ?Sized> (&self, rng: &mut R) -> CharType {
        match rng.gen_range(0, 6) {
            0 | 1 => CharType::Lower,
            2 | 3 => CharType::Upper,
            4 => CharType::Special,
            _ => CharType::Number
        }
    }
}


#[derive(Debug, StructOpt)]
#[structopt(name = "password_gen")]
struct Opt {

    /// Number of characters
    #[structopt(short = "L", long = "length")]
    length: Option<u8>,

    /// Use uppercase letters
    #[structopt(short = "u", long = "upper-case")]
    upper: bool,

    /// Use lowercase letters
    #[structopt(short = "l", long = "lower-case")]
    lower: bool,

    /// Use numbers
    #[structopt(short = "n", long = "numbers")]
    numbers: bool,

    /// Use special characters
    #[structopt(short = "s", long = "special")]
    special: bool,

    /// Min numbers
    #[structopt(long = "min-numbers")]
    min_numbers: Option<u8>,

    /// Min special characters
    #[structopt(long = "min-special")]
    min_special: Option<u8>,
}

fn comp_length(l: u8, min_num: u8, min_spec: u8, up: bool, low: bool) -> bool {
    let min_up: u8 = up.into();
    let min_low: u8 = low.into();
    l >= min_num + min_spec + min_up + min_low
}

fn random_char(t: CharType) -> char {
    let mut rng = thread_rng();
    match t {
        CharType::Lower => LOWERS[rng.gen_range(0, LOWERS.len()) as usize],
        CharType::Upper => UPPERS[rng.gen_range(0, UPPERS.len()) as usize],
        CharType::Special => SPECIALS[rng.gen_range(0, SPECIALS.len()) as usize],
        CharType::Number => NUMBERS[rng.gen_range(0, NUMBERS.len()) as usize],
    }
}

fn gen_password(l: u8, min_num: u8, min_spec: u8, _up: bool, _low: bool) -> String {
    // first get the nums and specials
    // then get the uppers and lowers
    // for the rest of the password
    // we can choose the char type at random
    // and then choose the index at random
    // bang...

    let mut pass_chars: Vec<char> = vec![];

    for _ in 0..min_num {
        pass_chars.push(random_char(CharType::Number));
    }

    for _ in 0..min_spec {
        pass_chars.push(random_char(CharType::Special));
    }

    for _ in pass_chars.len()..l as usize {
        let ct: CharType = rand::random();
        pass_chars.push(random_char(ct));
    }

    pass_chars.into_iter().collect()
}

fn main() {
    let opt = Opt::from_args();

    // add a bit here for a 'flush passwords' routine

    let len = opt.length.unwrap_or(12);

    let up = opt.upper;
    let low = opt.lower;
    let num = opt.numbers;
    let spec = opt.special;

    let mut min_num: u8 = 0;
    if num && opt.min_numbers.is_some() {
        min_num = opt.min_numbers.unwrap_or(1);
    }

    let mut min_spec: u8 = 0;
    if spec && opt.min_special.is_some() {
        min_spec = opt.min_special.unwrap_or(1);
    }

    if !comp_length(len, min_num, min_spec, up, low) {
        println!("The requested parameters do not fit into the requested length.");
        println!("Length: {}, Min Special: {}, Min Numbers: {}", len, min_spec, min_num);
        println!("Increase the maximum length to support you're requested parameters.");
        process::exit(1);
    }

    let pass = gen_password(len, min_num, min_spec, up, low);

    println!("Password generated: {}", pass);
    println!("If you forget it, it will be saved in ~/.pass for the next 5 password generations.");


}
