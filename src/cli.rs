use crate::gen_pass;
use clap::{command, Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "tenjin",author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t, value_enum)]
    /// Mode for generate password
    mode: Mode,
    msg: String,
}

#[derive(Clone, Debug, Default, ValueEnum)]
enum Mode {
    #[default]
    N,
    Normal,
    B,
    Byte,
}

pub fn system() {
    let cli = Cli::parse();
    let raw_hash = match cli.mode {
        Mode::B | Mode::Byte => gen_pass::byte_mode(cli.msg),
        Mode::N | Mode::Normal => gen_pass::normal_mode(cli.msg),
    };
    println!("{}", raw_hash);
}
