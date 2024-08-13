//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
pub mod logic;
mod port;
mod test;
use enum_dispatch::enum_dispatch;
pub use port::Port;

mod statetable;
pub use statetable::*;

mod ff;
pub use ff::{Ff, FfExpression};
mod latch;
pub use latch::{Latch, LatchExpression, LatchFfId};

mod function;
pub use function::FunctionExpression;

mod condition;
pub use condition::ConditionExpression;

mod tri_state;
// use strum_macros::Display;
use std::{
  collections::HashMap,
  fmt::{Debug, Display},
  hash::Hash,
};
pub use tri_state::TriState;
/// BooleanExpressionLike
#[enum_dispatch(BooleanExpression)]
pub trait BooleanExpressionLike: Display + Debug + Clone {
  /// get table with function
  fn table(&self) -> logic::Table;
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =132.36
/// &end
/// =132.38
/// ">Reference</a>
#[derive(Debug, Clone)]
#[enum_dispatch]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BooleanExpression {
  Port(Port),
  FF(FfExpression),
  Latch(LatchExpression),
  Function(FunctionExpression),
  TriState(TriState),
}

impl Display for BooleanExpression {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      BooleanExpression::Port(exp) => core::fmt::Display::fmt(&exp, f),
      BooleanExpression::FF(exp) => core::fmt::Display::fmt(&exp, f),
      BooleanExpression::Latch(exp) => core::fmt::Display::fmt(&exp, f),
      BooleanExpression::Function(exp) => core::fmt::Display::fmt(&exp, f),
      BooleanExpression::TriState(exp) => core::fmt::Display::fmt(&exp, f),
    }
  }
}

impl PartialEq for BooleanExpression {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.table() == other.table()
  }
}

impl Eq for BooleanExpression {}

impl Hash for BooleanExpression {
  #[inline]
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.table().hash(state);
  }
}
impl std::str::FromStr for BooleanExpression {
  type Err = core::fmt::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    todo!()
  }
}
const BRACKET_L: char = '(';
const BRACKET_R: char = ')';
impl BooleanExpression {
  // TODO:
  pub fn from_str(
    s: &str,
    ff_map: &HashMap<LatchFfId, Ff>,
    latch_map: &HashMap<LatchFfId, Latch>,
  ) -> Result<Self, core::fmt::Error> {
    let l_pos_list = s.match_indices(BRACKET_L).map(|(i, _)| i).collect::<Vec<usize>>();
    let r_pos_list = s.match_indices(BRACKET_R).map(|(i, _)| i).collect::<Vec<usize>>();
    // match (s.find(Self::BRACKET_L),s.find(Self::BRACKET_R)){
    //     (None, None) => todo!(),
    //     (None, Some(_)) => Err(core::fmt::Error),
    //     (Some(_), None) => Err(core::fmt::Error),
    //     (Some(idx_l), Some(idx_r)) => todo!(),
    // }
    todo!()
  }
}
