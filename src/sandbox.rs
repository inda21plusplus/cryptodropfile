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
    let buf = protobuf_msg::encode(vec![&msg1, &msg2]);
    println!("buf1 and 2: {}", buf.len());

    let result = protobuf_msg::decode(&buf);
    for msg in result.msg {
        println!("{}", msg.to_string());
    }
}
