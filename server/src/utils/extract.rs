use std::collections::HashMap;
use log::trace;

use crate::utils::crypto::aes_decrypt;
use crate::utils::files::{make_file,check_file};

/// Function to get id/data decode AES and make file
pub fn get_data(
   querie_name: String,
   id_hexadata: &mut HashMap<String, String>,
   key: &String,
) {
   // Get file id and hexa data
   let splited = querie_name.split("-");
   let id = splited.to_owned().collect::<Vec<&str>>()[0];
   let data = splited.collect::<Vec<&str>>()[1];

   trace!("id: {}",id);
   trace!("data: {}",data);

   // If data dosen't contains End Of File add data to Hashmap
   if !data.contains("crdeeofcrde")
   {
       if !id_hexadata.contains_key(id) 
       {
           id_hexadata.insert(id.to_string(), data.to_string());
       }
       else
       {
           if !id_hexadata.get(id).unwrap().contains(data)
           {
               let datas = format!("{}{}",id_hexadata.get(id).unwrap().to_string(),data.to_string());
               id_hexadata.insert(id.to_string(), datas.to_string());
           }
       }
   }
   else
   {
       // File not already in output folder?
       if !check_file(id.to_string())
       {
            let hexa = id_hexadata.get(id).unwrap().to_string();
            trace!("id_hexadata:{}: {}",id,hexa);
            // Decode data with the master key
            let result = aes_decrypt(&hex::decode(hexa).unwrap()[..], key.as_bytes());
            if result.len() != 0
            {
                trace!("result decoded: {:?}",result);
                // Write decoded data to output/<ID> folder
                make_file(&id.to_string(),result);
            }
       }
   }
}
