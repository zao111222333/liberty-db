use super::{logic, BooleanExpression, BooleanExpressionLike};
use std::fmt;

#[derive(Debug, Clone)]
pub struct TriState {
  enable: Box<BooleanExpression>,
  logic: Box<BooleanExpression>,
}

impl TriState {
  #[inline]
  pub fn new() -> Self {
    todo!()
  }
  #[inline]
  pub fn to_box(self) -> Box<Self> {
    Box::new(self)
  }
}

impl fmt::Display for TriState {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}{}{}@Z{}{}{}",
      BooleanExpression::BRACKET_L,
      self.logic,
      BooleanExpression::BRACKET_R,
      BooleanExpression::BRACKET_L,
      self.enable,
      BooleanExpression::BRACKET_R
    )
  }
}

impl BooleanExpressionLike for TriState {
  #[inline]
  fn table(&self) -> logic::Table {
    todo!()
  }
}
