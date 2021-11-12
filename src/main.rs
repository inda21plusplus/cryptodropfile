use std::net::TcpStream;

mod server;
mod error;
mod file;
mod crypto;
mod protobuf_msg;


pub fn register_user_example() {
    let mut user = server::UserConnection::new(None);
    let username = "someuser";
    let user_hash = crypto::hash_str(username).to_vec();
    let msg = protobuf_msg::SomeMessage {
        action: protobuf_msg::Action::Register as i32,
        filename: "".into(),
        data: user_hash
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
        data: user_hash
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
        data: file_content.to_vec()
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
        data: user_hash
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
        data: vec!()
    };
    let result = user.get_file_list();
    if result.is_err() {
        println!("print error");
        let err = result.as_ref().err().unwrap();
    }
    else {
        println!("File_list: ");
        for i in result.unwrap().data {
            print!("{}", i as char);
        }
        println!();
    }
}

pub fn main() {
    register_user_example();
    login_add_file_example();
    login_get_file_list_example();
    //let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
    /*loop {
        server.update();
    }*/
    println!("Program finished!");
}
