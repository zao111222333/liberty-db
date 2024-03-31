use crate::{
  ast::{AttributeList, GroupComments},
  common::items::{Domain, WordSet},
  expression::IdBooleanExpression,
  timing::items::Mode,
};

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  macro(derive(Debug, Clone,Default);)
)]
pub struct InternalPower {
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  // NOTICE: Simple Attributes
  // equal_or_opposite_output
  // falling_together_group
  // power_level
  #[liberty(simple(type = Option))]
  #[id]
  pub related_pin: Option<WordSet>,
  #[liberty(simple(type = Option))]
  #[id]
  pub related_pg_pin: Option<WordSet>,
  // rising_together_group
  // switching_interval
  // switching_together_group
  #[liberty(simple(type = Option))]
  #[id]
  pub when: Option<IdBooleanExpression>,
  // NOTICE: Complex Attribute
  #[liberty(complex(type = Option))]
  mode: Option<Mode>,
  // NOTICE: Group Statements
  #[liberty(group(type = Option))]
  pub domain: Option<Domain>,
  // fall_power (template name) {}
  // power (template name) {}
  // rise_power (template name) {}
}
