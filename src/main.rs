mod crypto;
mod error;
mod example;
mod file;
mod protobuf_msg;
mod server;

pub fn main() {
    crate::example::create_server_client();
    //crate::example::run_examples();
    //let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
    //loop {
    //    server.update();
    //}
    println!("Program finished!");
}
