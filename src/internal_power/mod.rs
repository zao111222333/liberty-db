use crate::{
  Ctx,
  ast::{Attributes, GroupComments, GroupFn},
  common::items::WordSet,
  expression::LogicBooleanExpression,
  table::TableLookUp,
};

#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::InternalPower: serde::Serialize + serde::de::DeserializeOwned")]
pub struct InternalPower<C: 'static + Ctx> {
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::InternalPower,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  // NOTICE: Simple Attributes
  // equal_or_opposite_output
  // falling_together_group
  // power_level
  #[id]
  #[liberty(simple)]
  pub related_pin: WordSet,
  #[id]
  #[liberty(simple)]
  pub related_pg_pin: WordSet,
  // rising_together_group
  // switching_interval
  // switching_together_group
  #[liberty(simple)]
  #[id]
  pub when: Option<LogicBooleanExpression>,
  // NOTICE: Complex Attribute
  /// The `mode` attribute specifies the current mode of operation of the cell. Use this attribute in
  /// the `internal_power` group to define the internal power in the specified mode.
  ///
  /// ### Syntax
  /// ``` text
  /// mode (mode_name, mode_value) ;
  /// ```
  /// ### Example
  /// ``` text
  /// mode (rw, read) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=305.9&end=305.14
  /// ">Reference-Definition</a>
  #[liberty(complex)]
  pub mode: Option<[String; 2]>,
  // NOTICE: Group Statements
  // #[liberty(group)]
  // pub domain: Option<Domain<C>>,
  #[liberty(group)]
  #[liberty(after_build = TableLookUp::use_power_template)]
  pub rise_power: Option<TableLookUp<C>>,
  #[liberty(group)]
  #[liberty(after_build = TableLookUp::use_power_template)]
  pub fall_power: Option<TableLookUp<C>>,
  #[liberty(group)]
  #[liberty(after_build = TableLookUp::use_power_template)]
  pub power: Option<TableLookUp<C>>,
}

impl<C: 'static + Ctx> GroupFn<C> for InternalPower<C> {}
