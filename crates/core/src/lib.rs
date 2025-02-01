mod analyzer;
mod anti_tamper;
mod io;
mod nodes;
mod transformer;
mod utils;

use std::path::PathBuf;

use anti_tamper::AntiTamper;
use oxc::{allocator::Allocator, parser::Parser, span::SourceType};

pub fn run_tamper(entrypoint_path: &PathBuf) {
  let entrypoint_readed = io::read_file(entrypoint_path);
  let source_type = SourceType::from_path(&entrypoint_path).unwrap();
  let allocator = Allocator::default();
  let anti_tamper = AntiTamper::new(&allocator);

  let parser = Parser::new(&allocator, entrypoint_readed.as_str(), source_type);
  let mut parsed = parser.parse();

  let errors = parsed
    .errors
    .iter()
    .map(|e| e.to_string())
    .collect::<Vec<_>>();

  if parsed.panicked {
    for error in errors {
      eprintln!("{error}");
    }
  }

  let result = anti_tamper.run(&mut parsed.program);
  todo!()
}
