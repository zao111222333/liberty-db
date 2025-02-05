use crate::{
  ast::{Attributes, GroupComments, GroupFn},
  common::{
    items::{Domain, NameList},
    table::TableLookUp,
  },
  expression::LogicBooleanExpression,
  timing::items::Mode,
  Ctx,
};

#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::InternalPower: serde::Serialize + serde::de::DeserializeOwned")]
pub struct InternalPower<C: Ctx> {
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  pub extra_ctx: C::InternalPower,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  // NOTICE: Simple Attributes
  // equal_or_opposite_output
  // falling_together_group
  // power_level
  #[id(
    borrow = "crate::common::items::RefNameList<'_>",
    check_fn = "NameList::as_ref",
    with_ref = false
  )]
  #[size = 64]
  #[liberty(simple)]
  pub related_pin: NameList,
  #[id(
    borrow = "crate::common::items::RefNameList<'_>",
    check_fn = "NameList::as_ref",
    with_ref = false
  )]
  #[size = 64]
  #[liberty(simple)]
  pub related_pg_pin: NameList,
  // rising_together_group
  // switching_interval
  // switching_together_group
  #[size = 80]
  #[liberty(simple(type = Option))]
  #[id(
    borrow = "Option<&LogicBooleanExpression>",
    check_fn = "mut_set::borrow_option!",
    with_ref = false
  )]
  pub when: Option<LogicBooleanExpression>,
  // NOTICE: Complex Attribute
  #[size = 16]
  #[liberty(complex(type = Option))]
  mode: Option<Mode>,
  // NOTICE: Group Statements
  #[size = 336]
  #[liberty(group(type = Option))]
  pub domain: Option<Domain<C>>,
  // fall_power (template name) {}
  // power (template name) {}
  // rise_power (template name) {}
  #[size = 336]
  #[liberty(group(type = Option))]
  pub rise_power: Option<TableLookUp<C>>,
  #[size = 336]
  #[liberty(group(type = Option))]
  pub fall_power: Option<TableLookUp<C>>,
  #[size = 336]
  #[liberty(group(type = Option))]
  pub power: Option<TableLookUp<C>>,
}

impl<C: Ctx> GroupFn for InternalPower<C> {}
