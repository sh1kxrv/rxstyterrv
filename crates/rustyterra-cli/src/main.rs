mod brotli;

use std::{
  fs,
  path::{Path, PathBuf},
};

use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about = "RustyTerra CLI", long_about = None)]
struct Cli {
  #[arg(short, long, value_name = "FILE")]
  input_wasm: PathBuf,

  #[arg(short, long, value_name = "FILE")]
  output: PathBuf,

  #[arg(short, long, value_name = "MANGLE", default_value = "compress")]
  mangle: String,
}

fn read_wasm(wasm_path: &PathBuf) -> Vec<u8> {
  let path = Path::new(wasm_path);
  if !path.exists() {
    panic!("wasm file does not exist");
  }
  let data = fs::read(path).unwrap();

  data
}

fn compress(wasm_path: &PathBuf, output: &PathBuf) {
  let data = read_wasm(wasm_path);
  let compressed_data = brotli::compress_with_brotli(&data, 10).unwrap();

  let output = Path::new(output);
  if output.exists() {
    fs::remove_file(output).unwrap()
  }

  fs::write(output, compressed_data).unwrap();
}

fn chunkify(wasm_path: &PathBuf, output: &PathBuf) {
  todo!()
}

fn main() {
  let cli = Cli::parse();
  let mangle_type = cli.mangle.as_str();

  match mangle_type {
    "compress" => compress(&cli.input_wasm, &cli.output),
    "chunkify" => chunkify(&cli.input_wasm, &cli.output),
    _ => panic!("unknown mangle type {mangle_type}"),
  }
}
