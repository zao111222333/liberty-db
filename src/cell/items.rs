use std::{hash::Hash, str::FromStr, sync::Arc};

use crate::{
  ast::{AttriValue, GroupComments, GroupFn, NamedGroup, SimpleAttri, SimpleWrapper},
  common::items::WordSet,
  expression::IdBooleanExpression,
  timing::items::Mode,
  ArcStr,
};
use itertools::Itertools;

/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=199.5&end=199.6
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LeakagePower {
  #[id]
  #[liberty(name)]
  name: Vec<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: crate::ast::AttributeList,
  #[id]
  #[liberty(simple(type = Option))]
  power_level: Option<ArcStr>,
  #[id]
  #[liberty(simple)]
  related_pg_pin: WordSet,
  #[id]
  #[liberty(simple(type = Option))]
  when: Option<IdBooleanExpression>,
  #[liberty(simple)]
  value: f64,
  #[liberty(complex(type = Option))]
  mode: Option<Mode>,
}
impl GroupFn for LeakagePower {}

/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.4&end=141.5
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Statetable {
  #[id]
  #[liberty(name)]
  pub input_nodes: Vec<ArcStr>,
  #[id]
  #[liberty(name)]
  pub internal_nodes: Vec<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: crate::ast::AttributeList,
  #[liberty(simple)]
  pub table: Table,
}
impl GroupFn for Statetable {}

impl NamedGroup for Statetable {
  #[inline]
  fn parse(mut v: Vec<ArcStr>) -> Result<Self::Name, crate::ast::IdError> {
    let l = v.len();
    if l != 2 {
      return Err(crate::ast::IdError::LengthDismatch(2, l, v));
    }
    if let Some(var2) = v.pop() {
      if let Some(var1) = v.pop() {
        Ok(Self::Name {
          input_nodes: var1.split_ascii_whitespace().map(ArcStr::from).collect(),
          internal_nodes: var2.split_ascii_whitespace().map(ArcStr::from).collect(),
        })
      } else {
        Err(crate::ast::IdError::Other("Unkown pop error".into()))
      }
    } else {
      Err(crate::ast::IdError::Other("Unkown pop error".into()))
    }
  }
  #[inline]
  fn name2vec(name: Self::Name) -> Vec<ArcStr> {
    vec![name.input_nodes.join(" ").into(), name.internal_nodes.join(" ").into()]
  }
}

/// StateTable Table
#[derive(Default, Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Table {
  pub v: Vec<ArcStr>,
}

impl std::fmt::Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.v, f)
  }
}

impl FromStr for Table {
  type Err = std::fmt::Error;
  /// To prevent syntax errors, the line continuation character
  /// must be followed immediately by the next line character.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.18&end=141.21
  /// ">Reference</a>
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      v: s
        .split("\\\n")
        .filter_map(|line| {
          let _l = line
            .trim_start()
            .trim_end_matches(|c: char| c == ',' || c.is_ascii_whitespace());
          if _l == "" {
            None
          } else {
            Some(ArcStr::from(_l))
          }
        })
        .collect(),
    })
  }
}

impl SimpleAttri for Table {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> nom::IResult<
    &'a str,
    Result<Self, (Self::Err, AttriValue)>,
    nom::error::Error<&'a str>,
  > {
    let (input, simple_multi) = crate::ast::parser::simple_multi(i, line_num)?;
    match Self::parse(simple_multi) {
      Ok(s) => Ok((input, Ok(s))),
      Err(e) => {
        Ok((input, Err((e, crate::ast::AttriValue::Simple(ArcStr::from(simple_multi))))))
      }
    }
  }
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    self.v.join(" ,\\\n").into()
  }
}

#[test]
fn statetable_test() {
  let _ = crate::ast::test_parse_group::<Statetable>(
    r#"(" CLK EN SE",ENL) {
        table : "	H   L  L : - : L ,\
        H   L  H : - : H ,\
        H   H  L : - : H ,\
        H   H  H : - : H ,\
        L   -  - : - : N ";
    }
  "#,
  );
}

/// Use the `pg_pin` group to specify power and ground pins.
/// The library cells can have multiple `pg_pin` groups.
/// A `pg_pin` group is mandatory for each cell.
/// A cell must have at least one `primary_power` pin
/// specified in the `pg_type` attribute and
/// at least one `primary_ground` pin specified in the `pg_type` attribute.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=209.3&end=209.6
/// ">Reference-Definition</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
    sort,
    macro(derive(Debug, Clone,Default);)
  )]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PgPin {
  #[liberty(name)]
  #[id]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: crate::ast::AttributeList,
  /// Use the `voltage_name`  attribute to specify an associated voltage.
  /// This attribute is optional in the `pg_pin`  group of a level-shifter cell
  /// not powered by the switching power domains,
  /// where the `pg_pin`  group has the `std_cell_main_rail`  attribute
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=209.30&end=209.32
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub voltage_name: ArcStr,
  /// Use the optional `pg_type`  attribute to specify the type of power and ground pin.
  /// The `pg_type`  attribute also supports back-bias modeling.
  /// The `pg_type`  attribute can have the following values:
  /// + `primary_power`
  /// + `primary_ground`
  /// + `backup_power`
  /// + `backup_ground`
  /// + `internal_power`
  /// + `internal_ground`
  /// + `pwell`
  /// + `nwell`
  /// + `deepnwell`
  /// + `deeppwell`
  ///
  /// The `pwell`  and `nwell`  values specify regular wells,
  /// and the `deeppwell`  and `deepnwell`  values specify isolation wells.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=210.8&end=210.13
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub pg_type: PgType,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=209.30&end=209.32
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub user_pg_type: ArcStr,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=209.30&end=209.32
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub physical_connection: ArcStr,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=209.30&end=209.32
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub related_bias_pin: ArcStr,
}
impl GroupFn for PgPin {}

/// Use the optional `pg_type`  attribute to specify the type of power and ground pin.
/// The `pg_type`  attribute also supports back-bias modeling.
/// The `pg_type`  attribute can have the following values:
/// + `primary_power`
/// + `primary_ground`
/// + `backup_power`
/// + `backup_ground`
/// + `internal_power`
/// + `internal_ground`
/// + `pwell`
/// + `nwell`
/// + `deepnwell`
/// + `deeppwell`
///
/// The `pwell`  and `nwell`  values specify regular wells,
/// and the `deeppwell`  and `deepnwell`  values specify isolation wells.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=210.8&end=210.13
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd, Default)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PgType {
  /// primary_power
  #[strum(serialize = "primary_power")]
  PrimaryPower,
  /// primary_ground
  #[strum(serialize = "primary_ground")]
  PrimaryGround,
  /// backup_power
  #[strum(serialize = "backup_power")]
  BackupPower,
  /// backup_ground
  #[strum(serialize = "backup_ground")]
  #[default]
  BackupGround,
  /// internal_power
  #[strum(serialize = "internal_power")]
  InternalPower,
  /// internal_ground
  #[strum(serialize = "internal_ground")]
  InternalGround,
  /// pwell
  #[strum(serialize = "pwell")]
  Pwell,
  /// nwell
  #[strum(serialize = "nwell")]
  Nwell,
  /// deepnwell
  #[strum(serialize = "deepnwell")]
  DeepNwell,
  /// deeppwell
  #[strum(serialize = "deeppwell")]
  DeepPwell,
}
impl SimpleAttri for PgType {}
