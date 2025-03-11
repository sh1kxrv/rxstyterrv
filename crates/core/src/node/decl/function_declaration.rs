use oxc::{
  allocator,
  ast::ast::{Function, TSThisParameter, TSTypeAnnotation, TSTypeParameterDeclaration},
};

use crate::transformer::Transformer;

impl<'a> Transformer<'a> {
  pub fn transform_function_declaration(
    &self,
    node: &'a Function<'a>,
  ) -> Option<allocator::Box<'a, Function<'a>>> {
    let Function {
      r#type,
      span,
      id,
      generator,
      r#async,
      params,
      body,
      ..
    } = node;

    // TODO:
  }
}
