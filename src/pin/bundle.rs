use crate::{
  Ctx,
  ast::{Attributes, GroupComments, GroupFn, GroupSet},
  common::items::NameList,
  expression::LogicBooleanExpression,
  pin::{Direction, NextstateType},
  timing::Timing,
};
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Pin: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Bundle<C: Ctx> {
  /// Name of the pin
  #[id]
  #[liberty(name)]
  pub name: NameList,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Pin,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(complex)]
  pub members: Vec<String>,
  #[liberty(simple(type = Option))]
  pub direction: Option<Direction>,
  #[liberty(simple(type = Option))]
  pub capacitance: Option<f64>,
  #[liberty(simple(type = Option))]
  pub function: Option<LogicBooleanExpression>,
  #[liberty(simple(type = Option))]
  pub nextstate_type: Option<NextstateType>,
  #[liberty(group(type = Set))]
  pub timing: GroupSet<Timing<C>>,
}

impl<C: Ctx> GroupFn<C> for Bundle<C> {}
