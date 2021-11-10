use prost;
use prost::{Message, Enumeration};

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

#[allow(dead_code)]
pub fn example() {
    let msg = SomeMessage{
        action: Action::AddFile.into(),
        filename: "somefile".into(),
        data: vec!()
    };
    println!("before: {}", msg.to_string());
    let mut buf: Vec<u8> = vec!();
    let _ = msg.encode(&mut buf);
    let read = SomeMessage::decode(&*buf).unwrap();
    println!("after: {}", read.to_string());
}