use std::env;
use std::net::IpAddr;
use std::str::FromStr;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

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

fn main() {
    //we take all the arguments passed and we create a collection
    let args: Vec<String> = env::args().collect();



}
