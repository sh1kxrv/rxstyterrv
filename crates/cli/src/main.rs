mod brotli;

use std::{
  fs,
  path::{Path, PathBuf},
};

use clap::{command, Parser as ClapParser};
use oxc::{
  allocator::Allocator,
  parser::{Parser, ParserReturn},
  semantic::{SemanticBuilder, SemanticBuilderReturn},
  span::SourceType,
};

#[derive(ClapParser)]
#[command(version, about = "RustyTerra CLI", long_about = None)]
struct Cli {
  #[arg(short, long, value_name = "FILE")]
  entrypoint: PathBuf,
}

fn file2string(path: &PathBuf) -> String {
  let path = Path::new(path);
  if !path.exists() {
    panic!("file does not exist");
  }
  fs::read_to_string(path).unwrap()
}

fn main() {
  let cli = Cli::parse();
  rustyterra_core::run_tamper(&cli.entrypoint);
}
