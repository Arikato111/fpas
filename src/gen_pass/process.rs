use clap::ValueEnum;

use super::{byte_mode, normal_mode};
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Mode {
    #[default]
    N,
    Normal,
    B,
    Byte,
}

pub fn process(msg: String, mode: Mode, looping: u32) -> String {
    let mut msg = msg;
    for _ in 0..looping {
        msg = match mode {
            Mode::Normal | Mode::N => normal_mode(msg),
            Mode::B | Mode::Byte => byte_mode(msg),
        }
    }
    msg
}
