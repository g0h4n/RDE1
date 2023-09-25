//! Parsing arguments
use clap::{Arg, ArgAction, value_parser, Command};

#[derive(Debug)]
pub enum Type {
    Dns,
    Https,
    Icmp,
    Unknown
}

#[derive(Debug)]
pub struct Options {
    pub exfiltration_type: Type,
    pub ip: String,
    pub port: String,
    pub key: String,
    pub private_cert: String,
    pub public_cert: String,
    pub interface: String,
    pub verbose: log::LevelFilter,
}

fn cli() -> Command {
    Command::new("srde")
        .about("SRDE (Server Rusty Data Exfiltrator) is a tool allowing auditor to extract files from different protocol written in rust.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("dns")
                .about("Exfiltrate datas from DNS server")
                .arg(Arg::new("ip")
                    .short('i')
                    .long("ip")
                    .help("IP for dns server")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("port")
                    .short('P')
                    .long("port")
                    .help("Port for dns server")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("key to decode datas")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("v")
                    .short('v')
                    .help("Set the level of verbosity")
                    .action(ArgAction::Count),
                )
        )
        .subcommand(
            Command::new("https")
                .about("Exfiltrate datas from HTTPS server")
                .arg(Arg::new("ip")
                    .short('i')
                    .long("ip")
                    .help("IP for https server.")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("port")
                    .short('P')
                    .long("port")
                    .help("Port for https server.")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("private-cert")
                    .long("private-cert")
                    .help("Private certificate for SSL (key.pem)")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("public-cert")
                    .long("public-cert")
                    .help("Public certificate for SSL (cert.pem)")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("key to decode datas")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("v")
                    .short('v')
                    .help("Set the level of verbosity")
                    .action(ArgAction::Count),
                )
        )
        .subcommand(
            Command::new("icmp")
                .about("Exfiltrate datas from ICMP queries (not yet implemented)")
                .arg(Arg::new("interface")
                    .short('i')
                    .long("interface")
                    .help("Interface where capture ICMP packet")
                    .required(false)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("key to decode datas")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("v")
                    .short('v')
                    .help("Set the level of verbosity")
                    .action(ArgAction::Count),
                )
        )
}

pub fn extract_args() -> Options {
    
    let matches = cli().get_matches();
    let mut exfiltration_type= Type::Unknown;
    let mut ip = "not set";
    let mut port = "not set";
    let mut key = "no key";
    let mut private_cert =  "no cert";
    let mut public_cert = "no cert";
    let mut interface = "any";
    let mut v =  log::LevelFilter::Info;

    match matches.subcommand() {
        Some(("dns", sub_matches)) => {
            exfiltration_type = Type::Dns;
            ip = sub_matches.get_one::<String>("ip").map(|s| s.as_str()).unwrap();
            port = sub_matches.get_one::<String>("port").map(|s| s.as_str()).unwrap();
            key = sub_matches.get_one::<String>("key").map(|s| s.as_str()).unwrap();
            v = match sub_matches.get_count("v") {
                0 => log::LevelFilter::Info,
                1 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            };
        }
        Some(("https", sub_matches)) => {
            exfiltration_type = Type::Https;
            ip = sub_matches.get_one::<String>("ip").map(|s| s.as_str()).unwrap();
            port = sub_matches.get_one::<String>("port").map(|s| s.as_str()).unwrap();
            key = sub_matches.get_one::<String>("key").map(|s| s.as_str()).unwrap();
            private_cert = sub_matches.get_one::<String>("private-cert").map(|s| s.as_str()).unwrap();
            public_cert = sub_matches.get_one::<String>("public-cert").map(|s| s.as_str()).unwrap();
            v = match sub_matches.get_count("v") {
                0 => log::LevelFilter::Info,
                1 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            };
        }
        Some(("icmp", sub_matches)) => {
            exfiltration_type = Type::Icmp;
            key = sub_matches.get_one::<String>("key").map(|s| s.as_str()).unwrap();
            interface = sub_matches.get_one::<String>("interface").map(|s| s.as_str()).unwrap_or("any");
            v = match sub_matches.get_count("v") {
                0 => log::LevelFilter::Info,
                1 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            };
        }
        _ => {},
    }

    Options {
        exfiltration_type: exfiltration_type,
        ip: ip.to_string(),
        port: port.to_string(),
        key: key.to_string(),
        private_cert: private_cert.to_string(),
        public_cert: public_cert.to_string(),
        interface: interface.to_string(),
        verbose: v,
    }
}