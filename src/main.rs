/*
 * Cryptodropfile-klient
 */

use std::{io::{Read, Write, ErrorKind, stdin}, os::unix::process};
use std::time::*;
use rustls::*;

use rustls::client::{
    ServerCertVerified, ServerCertVerifier,
};
//use webpki::*;
use std::net::TcpStream;
use std::sync::*;
use std::convert::TryInto;
use protobuf_msg::Action;
use prost::{Message, Enumeration};

use bytes::*;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::pwhash;

use libflate::zlib::{Encoder, Decoder};

mod protobuf_msg;

const _DROPFILE_HOST:&str = "dropfile.ghaglund.se";
const _DROPFILE_PORT:&str = ":4443";

struct MyVerifier {}

impl MyVerifier {
    fn new() -> Self {
        Self {}
    }
}

impl ServerCertVerifier for MyVerifier {
    fn verify_server_cert(
        &self, 
        _end_entity: &Certificate, 
        _intermediates: &[Certificate], 
        _server_name: &ServerName, 
        _scts: &mut dyn Iterator<Item = &[u8]>, 
        _ocsp_response: &[u8], 
        _now: SystemTime
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn request_scts(&self) -> bool {
        false
    }
}

/* Kod från servern */
pub fn read_tcp_stream_bytes(stream: &mut rustls::Stream<ClientConnection, TcpStream>, max_read_size: usize) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = vec![];
    buf.resize(max_read_size, 0);
    //println!("read");
    match stream.read(&mut buf) {
        Ok(size) => buf.resize(size, 0),
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // wait until network socket is ready, typically implemented
            // via platform-specific APIs such as epoll or IOCP
            return Err(std::io::Error::new::<String>(ErrorKind::Other, "socket not ready".into()));
        }
        Err(e) => panic!("encountered IO error: {}", e),
    };
    if buf.len() == 0 {
        return Err(std::io::Error::new::<String>(ErrorKind::Other, "nothing to read".into()));
    }
    //println!("bytes: {:?}", buf);
    return Ok(buf);
}

fn main() {
    let mut root_store = rustls::RootCertStore::empty();
	root_store.add_server_trust_anchors(
    	webpki_roots::TLS_SERVER_ROOTS
        	.0
        	.iter()
        	.map(|ta| {
            	rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                	ta.subject,
                	ta.spki,
                	ta.name_constraints,
            	)
        	})
	); 

    let myverifier = Arc::new(MyVerifier::new());

	let mut config = rustls::ClientConfig::builder()
    .with_safe_defaults().with_root_certificates(root_store)
    .with_no_client_auth();

    /* Nu skiter vi i all säkerhet */
    config.dangerous()
    .set_certificate_verifier(myverifier);

	let server_name:ServerName = _DROPFILE_HOST.try_into().unwrap();
	let rc_config = Arc::new(config);
    
	let mut conn = rustls::ClientConnection::new(rc_config, server_name).unwrap();
    let mut sock = TcpStream::connect(format!("{}{}", _DROPFILE_HOST, _DROPFILE_PORT)).unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);

    let mut input = String::new();
	let mut msg:protobuf_msg::SomeMessage;

	let mut passwd = String::new();
	let mut file = String::new();
	let mut resulting_file = String::new();
	let mut resulting_file_hash:sodiumoxide::crypto::hash::Digest;

    loop {
        std::io::stdin().read_line(&mut input).expect("Ajdå");
		input = input.trim().into();

		let mut processing_flag:Action = Action::OK;

        match input.as_str() {
			"login" | "register" => {
				let mut udetails_input = String::new();
				let udetails:Vec<&str>;
				print!("Användarnamn:lösenord väntas: ");
				std::io::stdout().flush();
				std::io::stdin().read_line(&mut udetails_input).expect("Ajdå");
				udetails_input = udetails_input.trim().into();
				udetails = udetails_input.split(":").collect();
				passwd = udetails[1].into();

				msg = protobuf_msg::SomeMessage {
					action: if input.as_str() == "login" { Action::Login.into() } else { Action::Register.into() },
					filename: "".into(),
					data: sodiumoxide::crypto::hash::hash(udetails[0].as_bytes()).0.into()
				};
			},

			"filelist" => {
				msg = protobuf_msg::SomeMessage {
					action: Action::GetFileList.into(),
					filename: "".into(),
					data: "".into()
				};
				processing_flag = Action::GetFileList;
			},

			"addfile" => {
				print!("Fil väntas: ");
				std::io::stdout().flush();
				std::io::stdin().read_line(&mut file).expect("Ajdå");
				file = file.trim().into();
				let plaintext = std::fs::read_to_string(&file).expect("ajdå");
				let mut encoder = Encoder::new(Vec::new()).unwrap();

				let nonce = secretbox::gen_nonce();
				//let nonce_file = secretbox::gen_nonce();
				let salt = pwhash::gen_salt();
				let salt_file = pwhash::gen_salt();

    			let secretbox::Key(ref mut kb) = secretbox::Key([0; secretbox::KEYBYTES]);
				let secretbox::Key(ref mut kb_file) = secretbox::Key([0; secretbox::KEYBYTES]);

    			pwhash::derive_key(kb, passwd.as_bytes(), &salt,
                       pwhash::OPSLIMIT_INTERACTIVE,
                       pwhash::MEMLIMIT_INTERACTIVE).unwrap();

				pwhash::derive_key(kb_file, passwd.as_bytes(), &salt_file,
            		pwhash::OPSLIMIT_INTERACTIVE,
            		pwhash::MEMLIMIT_INTERACTIVE).unwrap();

				let ciphertext:Vec<u8> = secretbox::seal(plaintext.as_bytes(), &nonce, &secretbox::Key::from_slice(kb).unwrap());
				/* should work I think, reusing salt as nonce */

				std::io::copy(&mut file.as_bytes(), &mut encoder).unwrap();
				let encoded_filename = encoder.finish().into_result().unwrap();

				let ciphertext_filename:Vec<u8> = secretbox::seal(&encoded_filename, &secretbox::Nonce::from_slice(salt_file[0..24].as_ref()).unwrap(), &secretbox::Key::from_slice(kb_file).unwrap());

				let mut tosave:BytesMut = BytesMut::new();
				tosave.put(nonce.as_ref());
				tosave.put(salt.as_ref());
				tosave.put(ciphertext.as_ref());

				resulting_file_hash = sodiumoxide::crypto::hash::hash(tosave.as_ref());

				let mut tosave_filename:BytesMut = BytesMut::new();
				//tosave_filename.put(nonce_file.as_ref());
				tosave_filename.put(salt_file.as_ref());
				tosave_filename.put(ciphertext_filename.as_ref());

				resulting_file = hex::encode(&tosave_filename.as_ref());

				msg = protobuf_msg::SomeMessage {
					action: Action::AddFile.into(),
					filename: (&*resulting_file).to_string(),
					data: tosave.as_ref().into()
				};

				processing_flag = Action::AddFile.into();
			},

			"getfile" => {
				let mut file = String::new();
				print!("Fil väntas (behöver vara filnamn som finns på servern): ");
				std::io::stdout().flush();
				std::io::stdin().read_line(&mut file).expect("Ajdå");
				file = file.trim().into();

				msg = protobuf_msg::SomeMessage {
					action: Action::GetFile.into(),
					filename: file.into(),
					data: "".into()
				};
				processing_flag = Action::GetFile.into();
			},

			"validaterecord" => {
				print!("Fil väntas: ");
				std::io::stdout().flush();

				msg = protobuf_msg::SomeMessage {
					action: Action::ValidateRecord.into(),
					filename: "".into(),
					data: "".into()
				};
			}

			_ => {
				println!("Va? {}", input);
				input.clear();
				continue;
			}
        }

		let mut buf: Vec<u8> = protobuf_msg::encode(vec!(&msg));
		let c: &[u8] = &buf;
    
    	tls.write(
        c
    	)
    	.unwrap();

		let ciphersuite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();
    
    	writeln!(
        	&mut std::io::stderr(),
        	"Current ciphersuite: {:?}",
        	ciphersuite.suite()
    	)
    	.unwrap();

    	let r = read_tcp_stream_bytes(&mut tls, 1000000);

	    if r.is_ok() {
    	    let s = protobuf_msg::decode(&r.unwrap());
        	//println!("reply {}", std::str::from_utf8(&s.msg[0].data).unwrap());

			if Action::from_i32(s.msg[0].action) == Some(Action::Error) {
				println!("Fel från server {}", std::str::from_utf8(&s.msg[0].data).unwrap());
				processing_flag = Action::Error;
			} else {
				println!("Önskad åtgärd genomfördes framgångsrikt");
			}

			match processing_flag {
				Action::GetFile => {
					let f = Bytes::from(s.msg[0].data.clone()); // meh clone :(
					let fname_bytes = hex::decode(s.msg[0].filename.clone()).unwrap();
					let fname = Bytes::from(fname_bytes);

					let fname_salt = fname.slice(0..32);
					let fname_nonce = fname.slice(0..24);
					let fname_ciphertext = fname.slice(32..fname.len());

					let nonce = f.slice(0..24);
					let salt = f.slice(24..56);
					let ciphertext = f.slice(56..f.len());

					println!("Nonce: {:?}", nonce.as_ref());
					println!("Salt: {:?}", salt.as_ref());

    				let secretbox::Key(ref mut kb) = secretbox::Key([0; secretbox::KEYBYTES]);
					let secretbox::Key(ref mut kb_file) = secretbox::Key([0; secretbox::KEYBYTES]);

    				pwhash::derive_key(kb, passwd.as_bytes(), &pwhash::Salt::from_slice(salt.as_ref()).unwrap(),
                       pwhash::OPSLIMIT_INTERACTIVE,
                       pwhash::MEMLIMIT_INTERACTIVE).unwrap();
					pwhash::derive_key(kb_file, passwd.as_bytes(), &pwhash::Salt::from_slice(fname_salt.as_ref()).unwrap(),
                       pwhash::OPSLIMIT_INTERACTIVE,
                       pwhash::MEMLIMIT_INTERACTIVE).unwrap();

					let plaintext = secretbox::open(&ciphertext, &secretbox::Nonce::from_slice(nonce.as_ref()).unwrap(), &secretbox::Key::from_slice(kb).unwrap()).unwrap();
					let plaintext_fname = secretbox::open(&fname_ciphertext, &secretbox::Nonce::from_slice(fname_nonce.as_ref()).unwrap(), &secretbox::Key::from_slice(kb_file).unwrap()).unwrap();

					let mut decoder_fname = Decoder::new(&plaintext_fname[..]).unwrap();
					let mut final_fname = Vec::new();
					decoder_fname.read_to_end(&mut final_fname).unwrap();

					println!("Decrypted filename is {:?}", std::str::from_utf8(&final_fname).unwrap());
					println!("Decrypted content is: {:?}", std::str::from_utf8(&plaintext).unwrap());
				},

				Action::AddFile => {
					/*let hashes_from_server = Bytes::from(s.msg[0].data.clone());
					println!("Antal hashar: {}", hashes_from_server.len()/64);*/

					println!("Din valda fil {} sparades ner som {}", file, resulting_file);
				},

				Action::GetFileList => {
					let listing = std::str::from_utf8(&s.msg[0].data).unwrap().split(",");
					for fname_src in listing {
						let fname_hex = hex::decode(fname_src);
						let fname_bytes:Vec<u8>;
						if fname_hex.is_ok() {
							fname_bytes = fname_hex.unwrap();
						} else {
							continue;
						}
						let fname = Bytes::from(fname_bytes);
						let fname_salt = fname.slice(0..32);
						let fname_nonce = fname.slice(0..24);
						let fname_ciphertext = fname.slice(32..fname.len());

						let secretbox::Key(ref mut kb_file) = secretbox::Key([0; secretbox::KEYBYTES]);
						pwhash::derive_key(kb_file, passwd.as_bytes(), &pwhash::Salt::from_slice(fname_salt.as_ref()).unwrap(),
                       		pwhash::OPSLIMIT_INTERACTIVE,
                       		pwhash::MEMLIMIT_INTERACTIVE).unwrap();

						let plaintext_fname = secretbox::open(&fname_ciphertext, &secretbox::Nonce::from_slice(fname_nonce.as_ref()).unwrap(), &secretbox::Key::from_slice(kb_file).unwrap()).unwrap();

						let mut decoder_fname = Decoder::new(&plaintext_fname[..]).unwrap();
						let mut final_fname = Vec::new();
						decoder_fname.read_to_end(&mut final_fname).unwrap();

						println!("Fil {} (på server som {})", std::str::from_utf8(&final_fname).unwrap(), fname_src);
					}
				}

				_ => {}
			}
    	}

		input.clear();
    }
}