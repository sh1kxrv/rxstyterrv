use oxc::{
  allocator::{Allocator, CloneIn},
  ast::{ast::Program, AstBuilder},
  span,
};

use crate::analyzer::Analyzer;

pub struct Transformer<'a> {
  pub allocator: &'a Allocator,
  pub ast_builder: AstBuilder<'a>,
}

impl<'a> Transformer<'a> {
  pub fn new(analyzer: Analyzer<'a>) -> Self {
    let Analyzer { allocator, .. } = analyzer;
    Self {
      allocator,
      ast_builder: AstBuilder::new(analyzer.allocator),
    }
  }

  fn transform(&self, node: &'a mut Program<'a>) -> Program<'a> {
    let Program {
      span,
      source_text,
      source_type,
      comments,
      hashbang,
      directives,
      body,
      ..
    } = node;

    let mut body = self.

    self.ast_builder.program(
      *span,
      *source_type,
      *source_text,
      self.clone_node(comments),
      self.clone_node(hashbang),
      self.clone_node(directives),
      body,
    )
  }
}

impl<'a> Transformer<'a> {
  pub fn clone_node<T: CloneIn<'a>>(&self, node: &T) -> T::Cloned {
    node.clone_in(self.allocator)
  }
}
