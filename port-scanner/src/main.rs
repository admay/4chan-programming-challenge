use structopt::StructOpt;

use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::io::{self, Write};
use std::sync::mpsc::{Sender, channel};
use std::thread;

#[derive(StructOpt, Debug)]
#[structopt(name = "port_scanner")]
struct Opt {
    /// IP of host to scan
    #[structopt(short = "H", long = "host")]
    host: Option<IpAddr>, // how to handle localhost

    /// Number of threads
    #[structopt(short = "t", long = "threads")]
    num_threads: Option<u16>,

    /// Starting port
    #[structopt(short = "s", long = "start")]
    start: u16,

    /// Ending port
    #[structopt(short = "e", long = "end")]
    end: u16,
}

fn scan(tx: Sender<u16>, start_port: u16, end_port: u16, host: IpAddr, num_threads: u16) {
    let mut current_port = start_port + 1;

    loop {
        println!("Scanning port {}", current_port);
        match TcpStream::connect((host, current_port)) {
            Ok(_) => {
                io::stdout().flush().unwrap();
                tx.send(current_port).unwrap();
            }
            Err(_) => {}
        }

        if (end_port - current_port) <= num_threads {
            break;
        }

        current_port += num_threads;
    }
}

fn main() {
    let opt = Opt::from_args();

    let host = opt.host.unwrap_or_else(|| IpAddr::from_str("127.0.0.1").unwrap());
    let num_threads = opt.num_threads.unwrap_or(4);

    let start = opt.start;
    let end = opt.end;

    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, end, host, num_threads);
        });
    }

    let mut ports = vec![];
    drop(tx);
    for port in rx {
        ports.push(port);
    }

    println!("");
    ports.sort();
    for port in ports {
        println!("Port {} is open!", port);
    }
}
