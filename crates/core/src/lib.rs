mod io;
mod transformer;

use std::path::PathBuf;

use oxc::{
  allocator::Allocator,
  ast::{ast::Function, visit::walk, Visit},
  parser::{ParseOptions, Parser},
  semantic::ScopeFlags,
  span::SourceType,
};

pub fn run(entrypoint_path: &PathBuf) -> Result<(), String> {
  let entrypoint_readed = io::read_file(entrypoint_path);
  let source_type = SourceType::from_path(&entrypoint_path).unwrap();

  let allocator = Allocator::new();
  let ret = Parser::new(&allocator, &entrypoint_readed, source_type)
    .with_options(ParseOptions {
      parse_regular_expression: true,
      ..ParseOptions::default()
    })
    .parse();

  if ret.errors.is_empty() {
    println!("Parsed Successfully.");
  } else {
    for error in ret.errors {
      let error = error.with_source_code(entrypoint_readed.clone());
      println!("{error:?}");
      println!("Parsed with Errors.");
    }
  }

  let program = ret.program;

  let mut ast_pass = TestingAST::default();
  ast_pass.visit_program(&program);

  Ok(())
}

#[derive(Debug, Default)]
struct TestingAST {}

impl<'a> Visit<'a> for TestingAST {
  fn visit_function(&mut self, func: &Function<'a>, flags: ScopeFlags) {
    println!("Visiting function {}", func.name().unwrap());
    walk::walk_function(self, func, flags);
  }
}
