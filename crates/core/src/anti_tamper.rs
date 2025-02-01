use std::rc::Rc;

use oxc::{
  allocator::Allocator, ast::ast::Program, codegen::CodegenReturn, semantic::SemanticBuilder,
};

use crate::{
  analyzer::Analyzer,
  transformer::{self, Transformer},
};

pub struct AntiTamperInner<'a> {
  pub allocator: &'a Allocator,
}

pub struct AntiTamperReturn {
  pub codegen_return: CodegenReturn,
}

#[derive(Clone)]
pub struct AntiTamper<'a>(pub Rc<AntiTamperInner<'a>>);

impl<'a> AntiTamper<'a> {
  pub fn new(allocator: &'a Allocator) -> Self {
    Self(Rc::new(AntiTamperInner { allocator }))
  }

  pub fn run(self, ast: &'a mut Program<'a>) -> AntiTamperReturn {
    let AntiTamperInner { allocator } = &*self.0;

    let semantic_builder = SemanticBuilder::new();
    let semantic = semantic_builder.build(ast).semantic;

    let mut analyzer = Analyzer::new(self.clone(), semantic);
    analyzer.analyze(ast);

    let transformer = Transformer::new(analyzer);
    allocator.alloc(transformer.transform(ast))
  }
}
