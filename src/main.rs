use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use std::io::{self, Write};

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

const MAX_PORT_SNIFF: u16 = 65535;

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        //we succeed converting my argument into an ip address
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments { flag: String::from(""), ipaddr, threads: 4 });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many aguments")
            } else if flag.contains("-j") {
                let ipaddr= match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse")
                };

                return Ok(Arguments { flag, ipaddr, threads})
            } else {
                return Err("Invalid syntax")
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut num_port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, num_port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(num_port).unwrap();
            }
            Err(_) => {}
        };

        if (MAX_PORT_SNIFF - num_port) <= num_threads {
            break;
        }

        num_port += num_threads;
    }

}

fn main() {
    //we take all the arguments passed and we create a collection with ip address, the flag and the number of threads
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprint!("{} problem parsing the arguments: {}", program, err);
                process::exit(0)
            }
        }
    );

    let num_threads = arguments.threads;
    let(tx, rx) = channel();
    let addr = arguments.ipaddr;

    for i in 0..num_threads {
        let tx = tx.clone(); // each thread has its own transmitor

        thread::spawn(move || {
            scan(tx, i, addr, num_threads );
        });
    }


    let mut out = vec![];

    drop(tx);

    for p in rx {
        out.push(p);
    }

    print!("");

    out.sort();

    for v in out {
        println!("{} is open", v);
    }



}
