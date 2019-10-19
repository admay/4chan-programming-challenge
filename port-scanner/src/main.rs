use structopt::StructOpt;
use std::net::{IpAddr};
use std::str::FromStr;

#[derive(StructOpt, Debug)]
#[structopt(name = "port_scanner")]
struct Opt {
    /// IP of host to scan
    #[structopt(short = "H", long = "host")]
    host: Option<IpAddr>,

    /// Number of threads
    #[structopt(short = "t", long = "threads")]
    threads: Option<u8>,

    /// Starting port
    #[structopt(short = "s", long = "start")]
    start: u8,

    /// Ending port
    #[structopt(short = "e", long = "end")]
    end: u8,
}

fn main() {
    let opt = Opt::from_args();

    let host = opt.host.unwrap_or_else(|| IpAddr::from_str("127.0.0.1").unwrap());
    let threads = opt.threads.unwrap_or(4);

    let start = opt.start;
    let end = opt.end;
}
