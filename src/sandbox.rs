// A file to test new ideas, but that are not yet ready to implement in the code

use crate::protobuf_msg;
use prost::Message;

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
    let msg2 = protobuf_msg::SomeMessage {
        action: protobuf_msg::Action::Login as i32,
        filename: "3".into(),
        data: "4".into(),
    };
    let buf = protobuf_msg::messages_to_bytes(vec!(msg1, msg2));
    println!("buf1 and 2: {}", buf.len());

    let result = protobuf_msg::parse_bytes_to_msg(&buf);
    for msg in result.msg {
        println!("{}", msg.to_string());
    }

    /*let dec_msg = protobuf_msg::SomeMessage::decode_length_delimited(&*buf1);
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
    }*/
}