use super::{logic, BooleanExpression, BooleanExpressionLike, FunctionExpression};
use core::fmt;

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Ff {
  name_pair: [String; 2],
  clock_on: FunctionExpression,
  next_state: FunctionExpression,
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FfExpression {
  pub ff: Box<Ff>,
  pub is_inverse: bool,
}

impl FfExpression {
  #[inline]
  pub fn new() -> Self {
    todo!()
  }
  #[inline]
  pub fn to_box(self) -> Box<Self> {
    Box::new(self)
  }
}

impl fmt::Display for FfExpression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.is_inverse {
      return write!(f, "{}", self.ff.name_pair[1]);
    } else {
      return write!(f, "{}", self.ff.name_pair[0]);
    }
  }
}

impl BooleanExpressionLike for FfExpression {
  fn table(&self) -> logic::Table {
    todo!()
  }
}
