pub fn chain_mode<T: Fn(String) -> String>(mode: T, msg: String) -> String {
    let (last_four, msg) = split_string(msg);

    format!("{}{}", msg, mode(last_four))
}

fn split_string(msg: String) -> (String, String) {
    let point = msg.len() - 4;
    let last_four = &msg[point..];
    let first_msg = &msg[..point];

    (last_four.to_string(), first_msg.to_string())
}
