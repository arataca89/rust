// main.rs
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config =  Config::build(&args).unwrap_or_else(|err|{
        eprintln!("{err}");
        eprintln!("Uso: minigrep string arquivo");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        eprintln!("Erro:{e}");
        process::exit(1);
    }
}
