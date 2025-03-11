mod function_declaration;

use oxc::ast::ast::Declaration;

use crate::transformer::Transformer;

impl<'a> Transformer<'a> {
  pub fn transform_declaration(&self, node: &'a Declaration<'a>) -> Option<Declaration<'a>> {
    match node {
      Declaration::FunctionDeclaration(node) => self
        .transform_function_declaration(node)
        .map(Declaration::FunctionDeclaration),
      _ => unreachable!(),
    }
  }
}
