use oxc::{
  ast::{ast::Statement, match_declaration},
  span::GetSpan,
};

use crate::transformer::Transformer;

impl<'a> Transformer<'a> {
  pub fn transform_stmt(&self, node: &'a Statement<'a>) -> Option<Statement<'a>> {
    let span = node.span();
    match node {
      match_declaration!(Statement) => self
        .transform_declaration(node.to_declaration())
        .map(Statement::from),
      _ => unreachable!(),
    }
  }
}
