mod brotli;

use std::path::PathBuf;

use clap::{command, Parser as ClapParser};

#[derive(ClapParser)]
#[command(version, about = "RustyTerra CLI", long_about = None)]
struct Cli {
  #[arg(short, long, value_name = "FILE")]
  entrypoint: PathBuf,
}

fn main() {
  let cli = Cli::parse();
  let result = rustyterra_core::run(&cli.entrypoint);
}
