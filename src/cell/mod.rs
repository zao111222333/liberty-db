//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use std::collections::HashSet;

use crate::{
  ast::{AttributeList, GroupComments},
  pin::Pin,
};
mod items;
pub use items::*;
use mut_set::MutSet;

/// cell
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(derive(liberty_macros::Nothing, Debug, Clone))]
pub struct Cell {
  #[id]
  #[liberty(name)]
  pub name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type = Option))]
  pub area: Option<f64>,
  #[liberty(group(type=Set))]
  pub pin: MutSet<Pin>,
  #[liberty(group(type=Set))]
  pub leakage_power: MutSet<LeakagePower>,
  #[liberty(group(type=Option))]
  pub statetable: Option<Statetable>,
}
