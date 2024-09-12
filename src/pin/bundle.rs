use crate::{
  ast::{Attributes, GroupComments, GroupFn},
  common::items::NameList,
  expression::IdBooleanExpression,
  pin::{Direction, NextstateType},
  timing::Timing,
  ArcStr, GroupSet,
};
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Default, Clone)]
#[mut_set::derive::item(
    sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(liberty_macros::Group)]
pub struct Bundle {
  /// Name of the pin
  #[id]
  #[liberty(name)]
  pub name: NameList,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(complex)]
  pub members: Vec<ArcStr>,
  #[liberty(simple(type = Option))]
  pub direction: Option<Direction>,
  #[liberty(simple(type = Option))]
  pub capacitance: Option<f64>,
  #[liberty(simple(type = Option))]
  pub function: Option<IdBooleanExpression>,
  #[liberty(simple(type = Option))]
  pub nextstate_type: Option<NextstateType>,
  #[liberty(group(type = Set))]
  pub timing: GroupSet<Timing>,
}

impl GroupFn for Bundle {}
