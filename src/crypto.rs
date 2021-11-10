

pub fn to_hex_str(data: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for &byte in data {
        write!(&mut s, "{:X}", byte).expect("Unable to write");
    }
    return s;
}

// Some general hash function, that takes in like a string and splits out a hash
#[allow(dead_code)]
pub fn hash(data: &[u8]) -> [u8; 64] {
    let hashed_data = sodiumoxide::crypto::hash::hash(data);
    return hashed_data.0;
}

#[allow(dead_code)]
pub fn hash_str(data: &str) -> [u8; 64] {
    return hash(data.as_bytes());
}

#[allow(dead_code)]
pub fn init() {
    let _ = sodiumoxide::init();
    let usr_id = crate::crypto::hash_str("hello world");
    println!("{}", to_hex_str(&usr_id));
    // Get list of directories in usr directory
}