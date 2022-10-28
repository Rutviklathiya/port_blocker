use clap::Parser;
use std::num::ParseIntError;

#[derive(Parser)]
struct Args {
    // value to ONLY unload the firewall
    #[clap(short, long, value_parser, action)]
    unload_firewall: bool,

    // eg -a "9000,22,9090"
    #[clap(short, long, value_parser, default_value = "22")]
    allowed_ports: String,
}

fn parse_ports(args: Args) -> Result<Vec<u16>, ParseIntError> {
    let ports: Result<Vec<u16>, _> = args.allowed_ports
        .split(',')
        .map(|p| p.parse::<u16>())
        .collect();

    ports
}

fn main() {
    let args = Args::parse();

    let ports = if let Ok(ports) = parse_ports(args) {
        ports
    } else {
        eprintln!("error parsing ports");
        std::process::exit(1);
    };

    lib::unload_firewall();

    if !args.unload_firewall {
        if let Err(e) = lib::load_firewall(ports) {
            eprintln!("Error loading firewall: {e}");
        } else {
            println!("Firewall loaded");
        }
    }
}
