use oxc::{allocator::Allocator, ast::ast::Program, semantic::Semantic};

use crate::anti_tamper::AntiTamper;

pub struct Analyzer<'a> {
  pub tamper: AntiTamper<'a>,
  pub allocator: &'a Allocator,
  pub semantic: Semantic<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn new(anti_tamper: AntiTamper<'a>, semantic: Semantic<'a>) -> Self {
    let allocator = anti_tamper.0.allocator;
    Self {
      tamper: anti_tamper,
      semantic,
      allocator,
    }
  }

  pub fn analyze(&mut self, node: &'a Program<'a>) {}
}
