use std::env; // get cli args
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
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments: {flag: String::from(""), ipaddr, threads: 4})
        } else {
            let flag = 
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    for i in &args {
        println!("{}", i);
    }
    println!("{:?}", args);
}
