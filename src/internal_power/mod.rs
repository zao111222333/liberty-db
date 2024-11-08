use crate::{
  ast::{Attributes, GroupComments, GroupFn},
  common::{
    items::{Domain, WordSet},
    table::TableLookUp,
  },
  expression::IdBooleanExpression,
  timing::items::Mode,
};

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InternalPower {
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
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
  #[id(borrow = "Option<&IdBooleanExpression>", check_fn = "mut_set::borrow_option!")]
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
