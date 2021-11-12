use crate::crypto;
use crate::protobuf_msg;
use crate::server;
use std::net::TcpStream;
use prost::Message;
use crate::protobuf_msg::SomeMessage;

pub fn register_user_example() {
    let mut user = server::UserConnection::new(None);
    let username = "someuser";
    let user_hash = crypto::hash_str(username).to_vec();
    let msg = protobuf_msg::SomeMessage {
        action: protobuf_msg::Action::Register as i32,
        filename: "".into(),
        data: user_hash,
    };
    println!("handle message");
    let result = user.handle_message(&msg);
    if result.is_err() {
        println!("print error");
        let err = result.as_ref().err().unwrap();
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
        println!("print error");
        let err = result.as_ref().err().unwrap();
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
        println!("print error");
        let err = result.as_ref().err().unwrap();
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
        println!("print error");
        let err = result.as_ref().err().unwrap();
        //print!("{}", err.to_string());
    }

    // Get file list
    let msg = protobuf_msg::SomeMessage {
        action: protobuf_msg::Action::GetFileList as i32,
        filename: "myfile.txt".into(),
        data: vec![],
    };
    let result = user.get_file_list();
    if result.is_err() {
        println!("print error");
        let err = result.as_ref().err().unwrap();
    } else {
        println!("File_list: ");
        for i in result.unwrap().data {
            print!("{}", i as char);
        }
        println!();
    }
}



pub fn run_client() {
    println!("Setting up client!");
    let mut clientstream =
        TcpStream::connect("127.0.0.1:3000");
    if clientstream.is_ok() {
        let mut clientstream = clientstream.as_mut().unwrap();
        println!("Client setup successfull!");

        // Login client
        let username = "someuser";
        let user_hash = crypto::hash_str(username).to_vec();
        let msg = protobuf_msg::SomeMessage {
            action: protobuf_msg::Action::Login as i32,
            filename: "".into(),
            data: user_hash,
        };
        let mut buf: Vec<u8> = vec!();
        let data = msg.encode(&mut buf);
        println!("Client: Sending message");
        server::write_to_tcp_stream_bytes(&mut clientstream, &buf);

        // Wait for server to parse message
        let dur: std::time::Duration = std::time::Duration::from_secs_f64(10.0);
        std::thread::sleep(dur);

        let msg = protobuf_msg::SomeMessage {
            action: protobuf_msg::Action::AddFile as i32,
            filename: "new file".into(),
            data: "this text is inside file".to_string().as_bytes().to_vec(),
        };
        let mut buf: Vec<u8> = vec!();
        let data = msg.encode(&mut buf);
        println!("Client: Sending message");
        server::write_to_tcp_stream_bytes(&mut clientstream, &buf);
        println!("Client: Waiting for respons");

        loop {
            let result = server::read_to_end_tcp_stream_bytes(clientstream, 100000000);
            if result.is_ok() {
                println!("Client: recieved bytes: {}", result.unwrap().len())
            }
        }

    }
    else {
        println!("Client setup failed");
    }
}

pub fn create_server_client() {
    let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
    //let dur: std::time::Duration = std::time::Duration::from_secs_f64(1.0);
    //std::thread::sleep(dur);
    std::thread::spawn(run_client);
    
    loop {
        server.update();
    }
}

pub fn run_examples() {
    register_user_example();
    login_add_file_example();
    login_get_file_list_example();
}
