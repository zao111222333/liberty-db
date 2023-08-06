use super::{logic, BooleanExpression, BooleanExpressionLike};
use std::fmt;

#[derive(Debug, Clone)]
pub struct LatchFfId {
  var_1: String,
  var_2: String,
}

#[derive(Debug, Clone)]
pub struct Latch {
  name_pair: [String; 2],
  clock_on: BooleanExpression,
  next_state: BooleanExpression,
}

#[derive(Debug, Clone)]
pub struct LatchExpression {
  pub latch: Box<Latch>,
  pub is_inverse: bool,
}

impl fmt::Display for LatchExpression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_inverse {
      return write!(f, "{}", self.latch.name_pair[1]);
    } else {
      return write!(f, "{}", self.latch.name_pair[0]);
    }
  }
}

impl BooleanExpressionLike for LatchExpression {
  fn table(&self) -> logic::Table {
    todo!()
  }
}
