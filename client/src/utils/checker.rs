use regex::Regex;
use std::process;
use log::error;

/// Using regex to make sure user send https url
pub fn is_https_url(
   url: &str,
){
   let re = Regex::new(r"^https://").unwrap();
   if !re.is_match(url){
      error!("Please use https url...");
      process::exit(0x0100);
   }
}

/// Using regex to make sure user send IPV4
pub fn is_ipv4(
   ipv4: &str,
){
   let re = Regex::new(r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$").unwrap();
   if !re.is_match(ipv4){
      error!("Please use IPV4 address...");
      process::exit(0x0100);
   }}