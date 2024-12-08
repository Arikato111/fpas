use base64::{engine::general_purpose, Engine};
use sha1::Digest;

pub fn md5(msg: &String) -> String {
    format!("{:x}", md5::compute(msg))
}

pub fn sha1(msg: &String) -> String {
    let mut sha1 = sha1::Sha1::new();
    sha1.update(&msg);
    let raw_sha1 = sha1.finalize();
    format!("{:x}", raw_sha1)
}

pub fn sha256(msg: &String) -> String {
    sha256::digest(msg)
}

pub fn to_base64(hex: String) -> String {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    }
    general_purpose::STANDARD.encode(&bytes)
}
