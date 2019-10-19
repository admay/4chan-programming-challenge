use structopt::StructOpt;
use std::net::{IpAddr};
use std::str::FromStr;

const MAX: u16 = 65535;

#[derive(StructOpt, Debug)]
#[structopt(name = "port_scanner")]
struct Opt {
    /// IP of host to scan
    #[structopt(short = "H", long = "host")]
    host: Option<IpAddr>, // how to handle localhost

    /// Number of threads
    #[structopt(short = "t", long = "threads")]
    num_threads: Option<u8>,

    /// Starting port
    #[structopt(short = "s", long = "start")]
    start: u8,

    /// Ending port
    #[structopt(short = "e", long = "end")]
    end: u8,
}

fn scan() {
}

fn main() {
    let opt = Opt::from_args();

    let host = opt.host.unwrap_or_else(|| IpAddr::from_str("127.0.0.1").unwrap());
    let num_threads = opt.num_threads.unwrap_or(4);

    let start = opt.start;
    let end = opt.end;
}
