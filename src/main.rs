mod server;
mod error;
mod file;
mod crypto;
mod protobuf_msg;
mod example;




pub fn main() {
    //crate::example::run_examples();
    let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
    loop {
        server.update();
    }
    println!("Program finished!");
}
