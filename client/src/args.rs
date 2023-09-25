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
    pub filename: String,
    pub key: String,
    pub domain_name: String,
    pub name_server: String,
    pub dns_tcp: bool,
    pub url: String,
    pub verbose: log::LevelFilter,
}

fn cli() -> Command {
    Command::new("crde")
        .about("CRDE (Client Rusty Data Exfiltrator) is a tool allowing auditor to extract files from different protocol written in rust.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("dns")
                .about("Exfiltrate datas from DNS queries")
                .arg(Arg::new("filename")
                    .short('f')
                    .long("filename")
                    .help("File to exfiltrate")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("key to encode datas")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("domain-name")
                    .short('d')
                    .long("domain-name")
                    .help("Domain name like google.com where exfiltrate datas")
                    .required(false)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("name-server")
                    .short('n')
                    .long("name-server")
                    .help("Alternative IP address name server to use for queries")
                    .required(false)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("dns-tcp")
                    .long("dns-tcp")
                    .help("Use TCP instead of UDP for DNS queries")
                    .required(false)
                    .action(ArgAction::SetTrue)
                    .global(false)
                )
                .arg(Arg::new("v")
                    .short('v')
                    .help("Set the level of verbosity")
                    .action(ArgAction::Count),
                )
        )
        .subcommand(
            Command::new("https")
                .about("Exfiltrate datas from HTTPS queries")
                .arg(Arg::new("filename")
                    .short('f')
                    .long("filename")
                    .help("File to exfiltrate")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("url")
                    .short('u')
                    .long("url")
                    .help("URL site to extract datas like: https://www.htwmcl.fr/ or https://127.0.0.1:443/")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("key to encode datas")
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
                .arg(Arg::new("filename")
                    .short('f')
                    .long("filename")
                    .help("File to exfiltrate")
                    .required(true)
                    .value_parser(value_parser!(String))
                )
                .arg(Arg::new("key")
                    .short('k')
                    .long("key")
                    .help("key to encode datas")
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
    let mut filename = "not set";
    let mut key = "no key";
    let mut domain_name = "google.com";
    let mut name_server = "no set";
    let mut dns_tcp = false;
    let mut url = "https://127.0.0.1/";
    let mut v =  log::LevelFilter::Info;

    match matches.subcommand() {
        Some(("dns", sub_matches)) => {
            exfiltration_type = Type::Dns;
            filename = sub_matches.get_one::<String>("filename").map(|s| s.as_str()).unwrap();
            key = sub_matches.get_one::<String>("key").map(|s| s.as_str()).unwrap();
            domain_name = sub_matches.get_one::<String>("domain-name").map(|s| s.as_str()).unwrap_or("google.com");
            name_server = sub_matches.get_one::<String>("name-server").map(|s| s.as_str()).unwrap_or("127.0.0.1");
            dns_tcp = sub_matches.get_one::<bool>("dns-tcp").map(|s| s.to_owned()).unwrap_or(false);
            v = match sub_matches.get_count("v") {
                0 => log::LevelFilter::Info,
                1 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            };
        }
        Some(("https", sub_matches)) => {
            exfiltration_type = Type::Https;
            filename = sub_matches.get_one::<String>("filename").map(|s| s.as_str()).unwrap();
            url = sub_matches.get_one::<String>("url").map(|s| s.as_str()).unwrap();
            key = sub_matches.get_one::<String>("key").map(|s| s.as_str()).unwrap();
            v = match sub_matches.get_count("v") {
                0 => log::LevelFilter::Info,
                1 => log::LevelFilter::Debug,
                _ => log::LevelFilter::Trace,
            };
        }
        Some(("icmp", sub_matches)) => {
            exfiltration_type = Type::Icmp;
            filename = sub_matches.get_one::<String>("filename").map(|s| s.as_str()).unwrap();
            key = sub_matches.get_one::<String>("key").map(|s| s.as_str()).unwrap();
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
        filename: filename.to_string(),
        key: key.to_string(),
        domain_name: domain_name.to_string(),
        name_server: name_server.to_string(),
        dns_tcp: dns_tcp,
        url: url.to_string(),
        verbose: v,
    }
}