use prost::Message;
use std::collections::LinkedList;
use std::default;
use std::net::{TcpListener, TcpStream};

use crate::error::server_error::Result;
use crate::protobuf_msg::SomeMessage;

#[allow(dead_code)]
pub struct Server {
    connections: LinkedList<UserConnection>,
    pub listening_port: u16,
    port_listener: Option<Box<TcpListener>>,
    pub server_ip: String,
}

#[allow(dead_code)]
impl Server {
    pub fn new(listening_port: u16, local: bool) -> Result<Self> {
        let listening_port_ip: String;
        if local {
            listening_port_ip = "127.0.0.1:".to_string() + listening_port.to_string().as_str();
        } else {
            listening_port_ip = get_local_ip().unwrap() + ":" + listening_port.to_string().as_str();
        }
        let port_listener = TcpListener::bind(listening_port_ip.as_str());
        if port_listener.is_err() {
            return Err(("Could not bind listening port: ".to_string()
                + listening_port_ip.as_str())
            .into());
        }
        println!("server listening on ip: {}", listening_port_ip);
        Ok(Self {
            connections: default::Default::default(),
            listening_port,
            port_listener: Some(Box::new(port_listener.unwrap())),
            server_ip: listening_port_ip,
        })
    }
    fn handle_client(&mut self, stream: TcpStream) {
        stream
            .set_nonblocking(true)
            .expect("set_nonblocking call failed");
        println!("Server: Add connection");
        self.connections
            .push_back(UserConnection::new(Some(stream)));
    }
    pub fn accept_incomming_connections(&mut self) -> Result<()> {
        let mut port_listener = self.port_listener.take();
        let mut error: Option<String> = None;
        let _ = port_listener.as_mut().unwrap().set_nonblocking(true);
        // accept connections and process them serially
        for stream in port_listener.as_mut().unwrap().incoming() {
            //println!("Server: Incomming connection!");
            if stream.is_err() {
                error = Some("stream was error".to_string());
                break;
            }
            self.handle_client(stream.unwrap());
        }
        self.port_listener = port_listener;
        if error.is_some() {
            return Err(error.unwrap().into());
        }
        return Ok(());
    }
    // Take requests from clients
    pub fn update(&mut self) {
        let _ = self.accept_incomming_connections();
        for i in self.connections.iter_mut() {
            let _ = i.update();
        }
    }
}

#[allow(dead_code)]
pub struct UserConnection {
    stream: Option<TcpStream>,
    userid: String,
}

impl UserConnection {
    pub fn update(&mut self) -> Result<()> {
        // Read incomming data
        let data = read_tcp_stream_bytes(self.stream.as_mut().unwrap(), 100000000)?;
        println!("Server: Recieved message: {}", data.len());
        let msg_list = crate::protobuf_msg::decode(&data);
        if data.len() == msg_list.len {
            println!("Server: Managed to fully parse all messages!");
        }
        else {
            println!("Server: Managed to parse {} of {}", msg_list.len, data.len());
        }
        for msg in msg_list.msg {
            let h = self.handle_message(&msg);
            if h.is_err() {
                let respons = crate::protobuf_msg::SomeMessage {
                    action: crate::protobuf_msg::Action::Error as i32,
                    filename: "".into(),
                    data: format!("Error: {:?}", h.err()).as_bytes().to_vec(),
                };
                self.send(&respons)?;
            }
        }
        return Ok(());
    }
    pub fn handle_message(&mut self, msg: &SomeMessage) -> Result<()> {
        use crate::protobuf_msg::Action;
        match Action::from_i32(msg.action) {
            Some(Action::Login) => {
                self.login(msg)?;
            }
            Some(Action::GetFile) => {
                let respons = self.load_file(msg)?;
                self.send(&respons)?;
            }
            Some(Action::AddFile) => {
                self.save_file(msg)?;
            }
            Some(Action::GetFileList) => {
                let respons = self.get_file_list()?;
                self.send(&respons)?;
            }
            Some(Action::Register) => {
                self.register_new_user(msg)?;
            }
            _ => {
                let respons = crate::protobuf_msg::SomeMessage {
                    action: crate::protobuf_msg::Action::Error as i32,
                    filename: "".into(),
                    data: "could not parse action".to_string().into_bytes(),
                };
                self.send(&respons)?;
                return Err("Could not parse action: ".into());
            }
        }
        return Ok(());
    }
    pub fn new(stream: Option<TcpStream>) -> Self {
        Self {
            stream,
            userid: "".into(),
        }
    }
    pub fn login(&mut self, msg: &SomeMessage) -> Result<()> {
        // Takes in the the username and password hash
        if msg.data.len() < 128 {
            // Get a list of current users
            let user_list = crate::file::get_diretories("./user")?;

            // Convert hash to hex string
            let user_id = crate::crypto::to_hex_str(&msg.data);

            // Make sure user exists
            if user_list.contains(&user_id) {
                self.userid = user_id;
            } else {
                return Err("could not locate user".into());
            }
            println!("Server: User logged in!");
        }
        return Ok(());
    }
    pub fn register_new_user(&mut self, msg: &SomeMessage) -> Result<()> {
        // Registers a new user, done after request by user
        // Takes in the the userid
        if msg.data.len() < 128 {
            // Get a list of current users
            let user_list = crate::file::get_diretories("./user").unwrap_or(vec![]);

            // Convert hash to hex string
            let user_id = crate::crypto::to_hex_str(&msg.data);

            // Make sure user does not exist
            if !user_list.contains(&user_id) {
                let new_path = "./user/".to_string() + user_id.as_str();
                std::fs::create_dir_all(new_path)?;
                self.userid = user_id;
            } else {
                return Err("user already exists".into());
            }
            println!("Server: User registered!");
        }
        return Ok(());
    }
    // Get the path of the filename
    pub fn file_path(&self, msg: &SomeMessage) -> Result<String> {
        self.is_valid_file_name(msg)?;
        let path = self.user_path()? + "/" + &msg.filename;
        return Ok(path);
    }
    // Get the path of the user
    pub fn user_path(&self) -> Result<String> {
        self.logged_in()?;
        let path = "./user/".to_string() + &self.userid;
        return Ok(path);
    }
    // Save a file that was sent by client
    pub fn save_file(&mut self, msg: &SomeMessage) -> Result<()> {
        // Saves the file in users directory
        // There can be multiple of the same file in directory, but server wont know they are the same due to salt
        println!("Server: Saving file");
        let path = self.file_path(msg)?;
        crate::file::write_file(&msg.data, &path)?;
        return Ok(());
    }
    // Checks if the file is a valid file name(not implemented yet)
    pub fn is_valid_file_name(&self, _msg: &SomeMessage) -> Result<()> {
        // Check for / and other illegal characters
        return Ok(());
    }
    // Returns error if the user is not logged in
    pub fn logged_in(&self) -> Result<()> {
        if self.userid != "".to_string() {
            return Ok(());
        } else {
            return Err("not logged in".into());
        }
    }
    // Request sent by client to load file, read it and send to client
    pub fn load_file(&mut self, msg: &SomeMessage) -> Result<SomeMessage> {
        println!("Server: Loading file");
        let path = self.file_path(msg)?;
        let data = crate::file::read_file(&path)?;
        let respons = crate::protobuf_msg::SomeMessage {
            action: crate::protobuf_msg::Action::GetFile as i32,
            filename: msg.filename.clone(),
            data,
        };
        return Ok(respons);
    }
    // Send a list of all files that the user has available
    pub fn get_file_list(&mut self) -> Result<SomeMessage> {
        println!("Server: Return list of files");
        let file_list = crate::file::get_files(&self.user_path()?)?;
        let mut return_str = String::new();
        let mut iter_num = 0;
        for i in file_list.iter() {
            if iter_num != 0 {
                return_str += ", ";
            }
            return_str += i;
            iter_num += 1;
        }
        let respons = crate::protobuf_msg::SomeMessage {
            action: crate::protobuf_msg::Action::GetFileList as i32,
            filename: "".into(),
            data: return_str.into_bytes(),
        };
        return Ok(respons);
    }
    // Send the message to user
    pub fn send(&mut self, msg: &SomeMessage) -> Result<()> {
        let buf = crate::protobuf_msg::encode(vec!(msg));
        println!("Server: Sending respons: {}", buf.len());
        write_to_tcp_stream_bytes(self.stream.as_mut().unwrap(), &buf)?;
        return Ok(());
    }
}

use std::io::Read;
use std::io::Write;

pub fn get_local_ip() -> Result<String> {
    use local_ip_address::local_ip;
    let my_local_ip = local_ip();
    if my_local_ip.is_err() {
        return Err("Could not fetch local ip".into());
    }
    return Ok(my_local_ip.unwrap().to_string());
}
#[allow(dead_code)]
pub fn read_tcp_stream_bytes(stream: &mut TcpStream, max_read_size: usize) -> Result<Vec<u8>> {
    let mut buf = vec![];
    buf.resize(max_read_size, 0);
    //println!("read");
    match stream.read(&mut buf) {
        Ok(size) => buf.resize(size, 0),
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            return Err("socket not ready".into());
        }
        Err(e) => panic!("encountered IO error: {}", e),
    };
    if buf.len() == 0 {
        return Err("nothing to read".into());
    }
    //println!("bytes: {:?}", buf);
    return Ok(buf);
}
pub fn read_to_end_tcp_stream_bytes(
    stream: &mut TcpStream,
    max_read_size: usize,
) -> Result<Vec<u8>> {
    let mut buf = vec![];
    buf.resize(max_read_size, 0);
    //println!("read");
    match stream.read_to_end(&mut buf) {
        Ok(size) => buf.resize(size, 0),
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            return Err("socket not ready".into());
        }
        Err(e) => panic!("encountered IO error: {}", e),
    };
    if buf.len() == 0 {
        return Err("nothing to read".into());
    }
    //println!("bytes: {:?}", buf);
    return Ok(buf);
}
#[allow(dead_code)]
pub fn read_tcp_stream_string(stream: &mut TcpStream, max_read_size: usize) -> Result<String> {
    let vec = read_tcp_stream_bytes(stream, max_read_size)?;
    let result = String::from_utf8(vec);
    if result.is_err() {
        return Err("could not convert tcp read to string".into());
    }
    return Ok(result.unwrap());
}

pub fn write_to_tcp_stream_bytes(stream: &mut TcpStream, buf: &[u8]) -> Result<()> {
    let result = stream.write(&buf);
    if result.is_err() {
        return Err("Write to tcpstream failed".into());
    }
    //println!("write");
    return Ok(());
}
#[allow(dead_code)]
pub fn write_to_tcp_stream_string(stream: &mut TcpStream, buf: &str) -> Result<()> {
    let buf = buf.as_bytes();
    return write_to_tcp_stream_bytes(stream, buf);
}
