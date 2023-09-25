use log::{info,error};
use std::fs;
use std::fs::File;
use std::io::Read;
use colored::Colorize;
use std::process;

/// Open file and get buffer
pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let f = File::open(&filename);
    match f {
        Ok(mut file) => {
            info!("Can open/read file {}", filename.bold().green());
            let metadata = fs::metadata(&filename).expect("unable to read metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            file.read(&mut buffer).expect("buffer overflow");
            buffer
        }
        Err(err) => {
            error!("Can't open/read file {}. Reason: {err}", filename.bold().red());
            process::exit(0x0100);
        }
    }
}