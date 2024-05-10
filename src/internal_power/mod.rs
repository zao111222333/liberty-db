use crate::{
  ast::{AttributeList, GroupComments, GroupFn},
  common::{
    items::{Domain, WordSet},
    table::TableLookUp,
  },
  expression::IdBooleanExpression,
  timing::items::Mode,
};

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
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
  #[liberty(simple)]
  #[id]
  pub related_pin: WordSet,
  #[liberty(simple)]
  #[id]
  pub related_pg_pin: WordSet,
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
  #[liberty(group(type = Option))]
  pub rise_power: Option<TableLookUp>,
  #[liberty(group(type = Option))]
  pub fall_power: Option<TableLookUp>,
  #[liberty(group(type = Option))]
  pub power: Option<TableLookUp>,
}

impl GroupFn for InternalPower {}
