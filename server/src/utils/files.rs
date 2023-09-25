use log::{info,debug,error};
use std::fs;
use colored::Colorize;
use std::path::Path;

/// Create file
pub fn make_file(
    filename: &String,
    content: Vec<u8>
){
    let path = "output/";
    let f = fs::create_dir_all(path);
    match f {
        Ok(mut _file) => {
            debug!("Can create path {}", path.bold().green());
        }
        Err(err) => {
            error!("Can't create path {}. Reason: {err}", path.bold().red());
        }
    }
    let final_path = format!("{}{}",path,filename);
    let f = fs::write(&final_path,&content);
    match f {
        Ok(mut _file) => {
            info!("Can write file {}", final_path.bold().green());
        }
        Err(err) => {
            error!("Can't write file {}. Reason: {err}", final_path.bold().red());
        }
    }
}

/// Check if file already exist
pub fn check_file(
    filename: String,
) -> bool {
    let path = format!("output/{}",filename);
    Path::new(&path).exists()
}