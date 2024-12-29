use std::io::{self, Read};

use crate::gen_pass::{self, Mode};
use clap::{command, CommandFactory, Parser};
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
    #[clap(short = 'l', long = "loop", value_name = "uint32")]
    /// Loop to generate password
    lop: Option<u32>,
}

pub fn system() {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        let mut cli_gen = Cli::command();
        generate(shell, &mut cli_gen, "fpas", &mut io::stdout());
        return;
    }

    let msg = if let Some(msg) = cli.msg {
        msg
    } else {
        let mut buf = Vec::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        if let Err(err) = handle.read_to_end(&mut buf) {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }

        // makes user can input binary data from file.
        // likes `cat ./file.bin | fpas`
        unsafe { String::from_utf8_unchecked(buf) }
    };
    let lop = match cli.lop {
        Some(l) => l,
        None => 1,
    };

    let passwd = gen_pass::process(msg, cli.mode, lop);
    print!("{}", passwd);
}
