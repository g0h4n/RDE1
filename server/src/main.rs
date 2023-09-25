pub mod args;
pub mod methods;
pub mod utils;

use args::*;
use env_logger::Builder;
use log::{info,trace,error};
use colored::Colorize;

use crate::methods::dns::dns_exfiltrator;
use crate::methods::https::https_exfiltrator;
use crate::methods::icmp::icmp_exfiltrator;

/// Main of SRDE (Server Rusty Data Exfiltrator)
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // Get args
    let common_args = extract_args();

    // Build logger
    Builder::new()
        .filter(Some("srde"), common_args.verbose)
        .filter_level(log::LevelFilter::Error)
        .init();

    // Exfiltrate datas
    match common_args.exfiltration_type {
        Type::Dns => {
            // DNS server exfiltrator
            info!("Exfiltration from DNS queries.");
            info!("key: {}",common_args.key.bold().green());
            let res = dns_exfiltrator(
                &common_args.ip,
                &common_args.port,
                &common_args.key,
            );
            match res {
                Ok(_res) => trace!("Exfiltration from DNS queries finished!"),
                Err(err) => error!("Exfiltration from DNS queries error. Reason: {err}")
            }
        }
        Type::Https => {
            // HTTPS server exfiltrator
            info!("Exfiltration from HTTPS queries.");
            info!("key: {}",common_args.key.bold().green());
            https_exfiltrator(
                &common_args.ip,
                &common_args.port,
                &common_args.private_cert,
                &common_args.public_cert,
            ).await?;
        }
        Type::Icmp => {
            // ICMP server exfiltrator
            info!("Exfiltration from ICMP queries. (not yet implemented)");
            info!("key: {}",common_args.key.bold().green());
            let _res = icmp_exfiltrator(
                &common_args.interface,
                &common_args.key,
            ).await;
            error!("ICMP: {:?}",_res);
        }
        _ => { 
            // Unknown Type
            error!("Unknown type, please check usage --help");
        }
    }
    Ok(())
}