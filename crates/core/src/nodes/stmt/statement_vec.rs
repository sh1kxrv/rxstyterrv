use oxc::{allocator::Vec, ast::ast::Statement};

use crate::{transformer::Transformer, utils::data::StatementVecData};

impl<'a> Transformer<'a> {
  pub fn transform_statement_vec(
    &self,
    data: &StatementVecData,
    statements: &'a Vec<'a, Statement<'a>>,
  ) -> Vec<'a, Statement<'a>> {
    let mut result = self.ast_builder.vec();

    if data.last_stmt.is_none() {
      return result;
    }

    let mut exited = false;
    for (index, statement) in statements.iter().enumerate() {
      if !exited {
        if let Some(statement) = self.transform_statement(statement) {
          result.push(statement);
        }
      } else if is_declaration(statement) {
        self.declaration_only.set(true);
        if let Some(statement) = self.transform_statement(statement) {
          result.push(statement);
        }
        self.declaration_only.set(false);
      }

      if data.last_stmt == Some(index) {
        exited = true;
      }
    }

    result
  }
}
