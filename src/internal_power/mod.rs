use crate::{
  ast::{Attributes, GroupComments, GroupFn},
  common::{items::NameList, table::TableLookUp},
  expression::LogicBooleanExpression,
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
  #[size = 48]
  #[liberty(complex(type = Option))]
  pub mode: Option<[String; 2]>,
  // NOTICE: Group Statements
  // #[size = 336]
  // #[liberty(group(type = Option))]
  // pub domain: Option<Domain<C>>,
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

impl<C: Ctx> GroupFn<C> for InternalPower<C> {}
