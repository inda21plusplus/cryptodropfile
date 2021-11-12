use prost;
use prost::{Enumeration, Message};

#[derive(Clone, PartialEq, Message)]
pub struct SomeMessage {
    #[prost(int32, tag = "1")]
    pub action: i32,

    #[prost(string, tag = "2")]
    pub filename: String,

    #[prost(tag = "3", bytes)]
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
#[repr(i32)]
pub enum Action {
    DeleteFile = 0,
    AddFile = 1,
    GetFile = 2,
    GetFileList = 3,
    Login = 4,
    Logout = 5,
    Error = 6,
    Register = 7,
}

impl SomeMessage {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut out_str = String::new();
        out_str += "action: '";
        out_str += self.action.to_string().as_str();
        out_str += "', filename: '";
        out_str += self.filename.as_str();
        out_str += "', data_len: '";
        out_str += self.data.len().to_string().as_str();
        out_str += "'";
        return out_str;
    }
}

pub fn message_from_bytes(buf: &Vec<u8>) -> Result<SomeMessage, prost::DecodeError> {
    return SomeMessage::decode(&**buf);
}

pub struct ParseResult {
    pub msg: Vec<SomeMessage>,
    pub len: usize
}

impl ParseResult {
    pub fn new() -> Self {
        Self {
            msg: vec!(),
            len: 0
        }
    }
}

pub fn parse_bytes_to_msg(buf: &Vec<u8>) -> ParseResult {
    let mut parse_result = ParseResult::new();
    let mut read_buf = buf.to_vec();
    loop {
        let msg = SomeMessage::decode_length_delimited(&*read_buf);
        if msg.is_err() {
            return parse_result;
        }
        let msg = msg.unwrap();
        let msg_len = msg.encoded_len();
        parse_result.len += msg_len;
        parse_result.len += prost::length_delimiter_len(msg_len);
        parse_result.msg.push(msg);
        read_buf = buf.split_at(parse_result.len as usize).1.to_vec();
    }
}

pub fn messages_to_bytes(msg_list: Vec<SomeMessage>) -> Vec<u8> {
    let mut return_buf: Vec<u8> = vec![];
    for msg in msg_list {
        let mut buf: Vec<u8> = vec![];
        let _ = msg.encode_length_delimited(&mut buf);
        return_buf.append(&mut buf);
    }
    return return_buf;
}

/*#[allow(dead_code)]
pub fn example() {
    let msg = SomeMessage {
        action: Action::AddFile.into(),
        filename: "somefile".into(),
        data: vec![],
    };
    println!("before: {}", msg.to_string());
    let mut buf: Vec<u8> = vec![];
    let _ = msg.encode(&mut buf);
    let read = SomeMessage::decode(&*buf).unwrap();
    println!("after: {}", read.to_string());
}*/
