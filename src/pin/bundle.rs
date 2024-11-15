use crate::{
  ast::{Attributes, GroupComments, GroupFn, GroupSet},
  common::items::NameList,
  expression::IdBooleanExpression,
  pin::{Direction, NextstateType},
  timing::Timing,
  ArcStr, NotNan,
};
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[mut_set::derive::item(sort)]
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Bundle {
  /// Name of the pin
  #[id]
  #[size = 48]
  #[liberty(name)]
  pub name: NameList,
  /// group comments
  #[size = 144]
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[size = 48]
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[size = 24]
  #[liberty(complex)]
  pub members: Vec<ArcStr>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub direction: Option<Direction>,
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub capacitance: Option<NotNan<f64>>,
  #[size = 80]
  #[liberty(simple(type = Option))]
  pub function: Option<IdBooleanExpression>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub nextstate_type: Option<NextstateType>,
  #[size = 48]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Timing>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Timing>::deserialize_with")]
  pub timing: GroupSet<Timing>,
}

impl GroupFn for Bundle {}
