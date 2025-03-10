use oxc::{
  allocator::{self, Allocator, CloneIn},
  ast::{ast::Program, AstBuilder},
};

pub struct Transformer<'a> {
  pub ast_builder: AstBuilder<'a>,
  pub allocator: &'a Allocator,
}

impl<'a> Transformer<'a> {
  pub fn new(allocator: &'a Allocator) -> Self {
    Self {
      ast_builder: AstBuilder::new(allocator),
      allocator,
    }
  }

  pub fn transform_program(&self, node: &'a Program<'a>) -> Program<'a> {
    let Program {
      span,
      source_type,
      source_text,
      comments,
      hashbang,
      directives,
      body,
      ..
    } = node;

    let mut transformed_body = self.ast_builder.vec();

    for statement in body {
      println!("Transforming statement {statement:?}");
    }

    self.ast_builder.program(
      *span,
      *source_type,
      *source_text,
      self.clone_node(comments),
      self.clone_node(hashbang),
      self.clone_node(directives),
      transformed_body,
    )
  }
}

impl<'a> Transformer<'a> {
  pub fn clone_node<T: CloneIn<'a>>(&self, node: &T) -> T::Cloned {
    node.clone_in(self.allocator)
  }
}
