use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "fizzbuzz")]
struct Opt {
    /// The number to FizzBuzz to
    #[structopt(short = "n", long = "number")]
    number: u16,
}

fn main() {
    let opt = Opt::from_args();

    let n = opt.number;

    for i in 1..n + 1 {
        if (i % 5 == 0) && (i % 3 == 0) {
            println!("FizzBuzz");
        } else if (i % 5 == 0) && !(i % 3 == 0) {
            println!("Buzz");
        } else if !(i % 5 == 0) && (i % 3 == 0) {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
}
