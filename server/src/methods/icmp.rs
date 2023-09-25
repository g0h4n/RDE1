use log::info;
use colored::Colorize;
//use std::collections::HashMap;
//use pcap::{Device,Capture};

/// ICMP listener
/// TODO
/// <http://academy.delmar.edu/courses/itsy2430/Handouts/PingPacketDecoded.html>
pub async fn icmp_exfiltrator(
   interface: &str,
   _key: &String,
) {
   info!("Starting ICMP sniffing..");
   info!("Using device {}", interface.bold().green());
    
   // let mut cap = pcap::Capture::from_device(interface)
   //    .unwrap()
   //    .immediate_mode(true)
   //    .open()
   //    .unwrap();

   // cap.filter("icmp[0] == 8", true).unwrap();
   // while let Ok(packet) = cap.next_packet() {
   //     info!("received packet! {:?}", packet);
   // }
}