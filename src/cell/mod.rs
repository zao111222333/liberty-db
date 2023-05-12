//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use std::collections::HashSet;

use crate::{
  ast::{AttributeList, HashedGroup},
  pin::Pin,
};
mod items;
pub use items::*;

/// cell
#[derive(Debug, Default)]
#[derive(liberty_macros::Group)]
pub struct Cell {
  #[id_len(1)]
  _id: <Self as HashedGroup>::Id,
  _undefined: AttributeList,

  #[arrti_type(simple)]
  pub area: Option<f64>,
  #[arrti_type(group)]
  pub pin: HashSet<Pin>,
  #[arrti_type(group)]
  pub statetable: Option<Statetable>,
}
