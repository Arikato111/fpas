use std::io;

use crate::gen_pass;
use clap::{command, CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, Shell};

#[derive(Parser, Default)]
#[command(name = "fpas",author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t, value_enum)]
    /// Mode for generate password
    mode: Mode,
    msg: Option<String>,
    #[clap(long, short)]
    /// genrate completions for any shell
    completions: Option<Shell>,
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

    if let Some(shell) = cli.completions {
        let mut cli_gen = Cli::command();
        generate(shell, &mut cli_gen, "fpas", &mut io::stdout());
        return;
    }

    if let Some(msg) = cli.msg {
        let raw_hash = match cli.mode {
            Mode::B | Mode::Byte => gen_pass::byte_mode(msg),
            Mode::N | Mode::Normal => gen_pass::normal_mode(msg),
        };
        println!("{}", raw_hash);
    } else {
        // show help 
        Cli::parse_from(&[std::env::args().nth(0).unwrap().as_str(), "--help"]);
    }
}
