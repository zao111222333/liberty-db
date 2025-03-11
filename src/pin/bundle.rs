use crate::{
  ast::{Attributes, GroupComments, GroupFn, GroupSet},
  common::items::NameList,
  expression::LogicBooleanExpression,
  pin::{Direction, NextstateType},
  timing::Timing,
  Ctx,
};
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Pin: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Bundle<C: Ctx> {
  /// Name of the pin
  #[id(
    borrow = "crate::common::items::RefNameList<'_>",
    check_fn = "NameList::as_ref",
    with_ref = false
  )]
  #[size = 48]
  #[liberty(name)]
  pub name: NameList,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Pin,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[size = 24]
  #[liberty(complex)]
  pub members: Vec<String>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub direction: Option<Direction>,
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub capacitance: Option<f64>,
  #[size = 80]
  #[liberty(simple(type = Option))]
  pub function: Option<LogicBooleanExpression>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub nextstate_type: Option<NextstateType>,
  #[size = 88]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Timing<C>>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Timing<C>>::deserialize_with")]
  pub timing: GroupSet<Timing<C>>,
}

impl<C: Ctx> GroupFn<C> for Bundle<C> {}
