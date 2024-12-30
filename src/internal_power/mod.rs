use crate::{
  ast::{Attributes, GroupComments, GroupFn},
  common::{
    items::{Domain, WordSet},
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
#[serde(bound = "C::Dummy: serde::Serialize + serde::de::DeserializeOwned")]
pub struct InternalPower<C: Ctx> {
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  extra_ctx: C::Dummy,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  // NOTICE: Simple Attributes
  // equal_or_opposite_output
  // falling_together_group
  // power_level
  #[id]
  #[size = 64]
  #[liberty(simple)]
  pub related_pin: WordSet,
  #[id]
  #[size = 64]
  #[liberty(simple)]
  pub related_pg_pin: WordSet,
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
