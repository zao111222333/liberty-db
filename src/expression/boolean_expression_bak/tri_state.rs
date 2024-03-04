use super::{logic, BooleanExpression, BooleanExpressionLike, BRACKET_L, BRACKET_R};
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
      BRACKET_L, self.logic, BRACKET_R, BRACKET_L, self.enable, BRACKET_R
    )
  }
}

impl BooleanExpressionLike for TriState {
  #[inline]
  fn table(&self) -> logic::Table {
    todo!()
  }
}
