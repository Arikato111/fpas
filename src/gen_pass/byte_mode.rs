use crate::hash;

pub fn byte_mode(msg: String) -> String {
    let binding = hash::to_base64(hash::sha256(&msg));
    let raw_sha256 = binding.as_bytes();
    let binding = hash::to_base64(hash::sha1(&msg));
    let raw_sha1 = binding.as_bytes();

    let raw_sha256_len = raw_sha256.len();
    let raw_sha1_len = raw_sha1.len();
    let mut raw = Vec::new();
    for i in 0..raw_sha256_len {
        raw.push(raw_sha256[i]);
        if i < raw_sha1_len {
            raw.push(raw_sha1[i])
        }
    }
    let raw = match String::from_utf8(raw) {
        Ok(v) => {
            format!("{}{}", v, hash::md5(&v))
        }
        Err(_) => panic!("cannot convert vec to string at gen_pass/normal_mode"),
    };
    let raw = raw.replace("1", "@").replace("e", "#").replace("2", "&");
    raw
}
