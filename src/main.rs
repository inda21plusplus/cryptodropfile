mod crypto;
mod error;
mod example;
mod file;
mod logger;
mod protobuf_msg;
mod sandbox;
mod server;

pub use log::*;

pub fn main() {
    logger::init(Default::default());
    info!("Setup inited!");

    //sandbox::sandbox();
    crate::example::example::spawn_server();
    //crate::example::run_examples();
    //let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
    //loop {
    //    server.update();
    //}
    println!("Program finished!");
}
