//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use std::collections::HashSet;

use crate::{
  ast::{AttributeList, GroupId, GroupMap, HashedGroup},
  pin::Pin,
};
mod items;
pub use items::*;

/// cell
#[derive(Debug, Default)]
#[derive(liberty_macros::Group)]
pub struct Cell {
  #[liberty(id(auto_impl_len = 1))]
  _id: GroupId<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type = Option))]
  pub area: Option<f64>,
  #[liberty(group(type=Map))]
  pub pin: GroupMap<Pin>,
  #[liberty(group(type=Option))]
  pub statetable: Option<Statetable>,
}
