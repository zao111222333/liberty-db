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
  #[size = 120]
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[size = 48]
  #[liberty(attributes)]
  pub attributes: Attributes,
  // NOTICE: Simple Attributes
  // equal_or_opposite_output
  // falling_together_group
  // power_level
  #[id]
  #[size = 48]
  #[liberty(simple)]
  pub related_pin: WordSet,
  #[id]
  #[size = 48]
  #[liberty(simple)]
  pub related_pg_pin: WordSet,
  // rising_together_group
  // switching_interval
  // switching_together_group
  #[size = 80]
  #[liberty(simple(type = Option))]
  #[id(borrow = "Option<&IdBooleanExpression>", check_fn = "mut_set::borrow_option!")]
  pub when: Option<IdBooleanExpression>,
  // NOTICE: Complex Attribute
  #[size = 16]
  #[liberty(complex(type = Option))]
  mode: Option<Mode>,
  // NOTICE: Group Statements
  #[size = 336]
  #[liberty(group(type = Option))]
  pub domain: Option<Domain>,
  // fall_power (template name) {}
  // power (template name) {}
  // rise_power (template name) {}
  #[size = 336]
  #[liberty(group(type = Option))]
  pub rise_power: Option<TableLookUp>,
  #[size = 336]
  #[liberty(group(type = Option))]
  pub fall_power: Option<TableLookUp>,
  #[size = 336]
  #[liberty(group(type = Option))]
  pub power: Option<TableLookUp>,
}

impl GroupFn for InternalPower {}
