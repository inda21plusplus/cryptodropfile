

#[allow(dead_code)]
pub mod example {
    use crate::crypto;
    use crate::protobuf_msg;
    //use crate::protobuf_msg::SomeMessage;
    use crate::server;
    //use prost::Message;
    use std::net::TcpStream;
    pub use log::*;
    pub fn register_user_example() {
        let mut user = server::UserConnection::new(None);
        let username = "someuser";
        let user_hash = crypto::hash_str(username).to_vec();
        let msg = protobuf_msg::SomeMessage {
            action: protobuf_msg::Action::Register as i32,
            filename: "".into(),
            data: user_hash,
        };
        info!("handle message");
        let result = user.handle_message(&msg);
        if result.is_err() {
            info!("print error");
            let _err = result.as_ref().err().unwrap();
            //print!("{}", err.to_string());
        }
    }
    
    pub fn login_add_file_example() {
        // Login
        let mut user = server::UserConnection::new(None);
        let username = "someuser";
        let user_hash = crypto::hash_str(username).to_vec();
        let msg = protobuf_msg::SomeMessage {
            action: protobuf_msg::Action::Login as i32,
            filename: "".into(),
            data: user_hash,
        };
        let result = user.handle_message(&msg);
        if result.is_err() {
            info!("print error");
            let _err = result.as_ref().err().unwrap();
            //print!("{}", err.to_string());
        }
    
        // Add file
        let file_content = "this is some text".as_bytes();
        let msg = protobuf_msg::SomeMessage {
            action: protobuf_msg::Action::AddFile as i32,
            filename: "otherfile.txt".into(),
            data: file_content.to_vec(),
        };
        let result = user.handle_message(&msg);
        if result.is_err() {
            info!("print error");
            let _err = result.as_ref().err().unwrap();
        }
    }
    
    pub fn login_get_file_list_example() {
        // Login
        let mut user = server::UserConnection::new(None);
        let username = "someuser";
        let user_hash = crypto::hash_str(username).to_vec();
        let msg = protobuf_msg::SomeMessage {
            action: protobuf_msg::Action::Login as i32,
            filename: "".into(),
            data: user_hash,
        };
        let result = user.handle_message(&msg);
        if result.is_err() {
            info!("print error");
            let _err = result.as_ref().err().unwrap();
            //print!("{}", err.to_string());
        }
    
        // Get file list
        let _msg = protobuf_msg::SomeMessage {
            action: protobuf_msg::Action::GetFileList as i32,
            filename: "myfile.txt".into(),
            data: vec![],
        };
        let result = user.get_file_list();
        if result.is_err() {
            info!("print error");
            let _err = result.as_ref().err().unwrap();
        } else {
            info!("File_list: ");
            for i in result.unwrap().data {
                print!("{}", i as char);
            }
            info!("");
        }
    }
    
    pub fn run_client() {
        info!("Setting up client!");
        let mut clientstream = TcpStream::connect("127.0.0.1:3000");
        if clientstream.is_ok() {
            let mut clientstream = clientstream.as_mut().unwrap();
            info!("Client setup successfull!");
    
            // Login client
            let username = "someuser";
            let user_hash = crypto::hash_str(username).to_vec();
            let msg = protobuf_msg::SomeMessage {
                action: protobuf_msg::Action::Login as i32,
                filename: "".into(),
                data: user_hash,
            };
            let buf = protobuf_msg::encode(vec![&msg]);
            info!("Sending message");
            let _ = server::write_to_tcp_stream_bytes(&mut clientstream, &buf);
    
            let msg = protobuf_msg::SomeMessage {
                action: protobuf_msg::Action::AddFile as i32,
                filename: "new file".into(),
                data: "this text is inside file".to_string().as_bytes().to_vec(),
            };
            let buf = protobuf_msg::encode(vec![&msg]);
            info!("Sending message");
            let _ = server::write_to_tcp_stream_bytes(&mut clientstream, &buf);
            info!("Waiting for respons");
    
            std::thread::sleep(std::time::Duration::from_secs(10));
            /*loop {
                let result = server::read_tcp_stream_bytes(clientstream, 100000000);
                if result.is_ok() {
                    info!("recieved bytes: {}", result.unwrap().len())
                }
            }*/
        } else {
            info!("Client setup failed");
        }
        println!("Client exited");
    }
    
    pub fn spawn_server() {
        let handle = std::thread::Builder::new().name("server".to_string()).spawn(create_server_client).unwrap();
        handle.join().unwrap();
    }
    
    pub fn create_server_client() {
        let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
        //let dur: std::time::Duration = std::time::Duration::from_secs_f64(1.0);
        //std::thread::sleep(dur);
        let _ = std::thread::Builder::new().name("client".to_string()).spawn(run_client);
    
        loop {
            server.update();
        }
    }
    
    pub fn run_examples() {
        register_user_example();
        login_add_file_example();
        login_get_file_list_example();
    }
}