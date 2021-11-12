use prost::Message;

mod crypto;
mod error;
mod example;
mod file;
mod protobuf_msg;
mod server;


pub fn sandbox() {
    // Example on splitting messages
    let mut msg1 = protobuf_msg::SomeMessage {
        action: protobuf_msg::Action::Login as i32,
        filename: "1".into(),
        data: "2".into(),
    };
    for i in 0..100 {
        msg1.filename += "1234567890";
    }
    let mut buf1: Vec<u8> = vec!();
    let data1 = msg1.encode_length_delimited(&mut buf1);
    println!("buf1: {}", buf1.len());

    let msg2 = protobuf_msg::SomeMessage {
        action: protobuf_msg::Action::Login as i32,
        filename: "3".into(),
        data: "4".into(),
    };
    let mut buf2: Vec<u8> = vec!();
    let data2 = msg2.encode_length_delimited(&mut buf2);
    println!("buf2: {}", buf2.len());


    buf1.append(&mut buf2);
    println!("buf1 and 2: {}", buf1.len());
    let dec_msg = protobuf_msg::SomeMessage::decode_length_delimited(&*buf1);
    if dec_msg.is_err() {
        println!("err");
    }
    else {
        let dec_msg = dec_msg.unwrap();
        println!("ok");
        println!("{}", dec_msg.to_string());
        let mut dec_msg_len = dec_msg.encoded_len();
        dec_msg_len += prost::length_delimiter_len(dec_msg_len);
        println!("{}", dec_msg_len);
        let second_msg = buf1.split_at(dec_msg_len).1.to_vec();
        let dec_msg2 = protobuf_msg::SomeMessage::decode_length_delimited(&*second_msg);
        if dec_msg2.is_err() {
            println!("err");
        }
        else {
            let dec_msg2 = dec_msg2.unwrap();
            println!("ok");
            println!("{}", dec_msg2.to_string());
        }
    }
}

pub fn main() {
    sandbox();
    //crate::example::create_server_client();
    //crate::example::run_examples();
    //let mut server = crate::server::Server::new(3000, true).expect("Create server failed: ");
    //loop {
    //    server.update();
    //}
    println!("Program finished!");
}
