mod crypto;
mod error;
mod example;
mod file;
mod logger;
mod protobuf_msg;
mod sandbox;
mod server;

//extern crate log;
pub use log::*;

pub fn main() {
    logger::init(Default::default());
    warn!("Setup inited!");
    info!("Hello main!");

    //sandbox::sandbox();
    //crate::example::create_server_client();
    //crate::example::run_examples();
    //let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
    //loop {
    //    server.update();
    //}
    println!("Program finished!");
}
