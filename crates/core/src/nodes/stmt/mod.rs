use oxc::{ast::ast::Statement, syntax::node};

use crate::transformer::Transformer;

mod statement_vec;

impl<'a> Transformer<'a> {
  pub fn transform_statement(&self, node: &'a Statement<'a>) -> Option<Statement<'a>> {
    let span = node.span();
  }
}
