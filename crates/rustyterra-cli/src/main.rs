use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about = "RustyTerra CLI", long_about = None)]
struct Cli {
  #[arg(short, long, value_name = "FILE")]
  wasm: PathBuf,

  #[arg(short, long, value_name = "COMPRESS-TYPE", default_value = "gzip")]
  compress: Option<String>,
}

fn main() {
  let cli = Cli::parse();

  println!("WASM: {}", cli.wasm.display());
  println!("COMPRESS: {}", cli.compress.unwrap_or("gzip".to_string()));
}
