use crate::{
  ast::{
    join_fmt, AttriValue, CodeFormatter, ComplexAttri, ComplexParseError, GroupComments,
    GroupFn, Indentation, NamedGroup, SimpleAttri,
  },
  common::items::WordSet,
  expression::IdBooleanExpression,
  pin::Direction,
  timing::items::Mode,
  ArcStr,
};
use core::{
  fmt::{self, Write},
  hash::Hash,
  str::FromStr,
};
/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=199.5&end=199.6
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct LeakagePower {
  #[id]
  #[liberty(name)]
  pub name: Vec<ArcStr>,
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

#[cfg(test)]
mod test_sort {
  use super::*;

  #[test]
  fn test_leakage_sort() {
    let cell = crate::ast::test_parse::<crate::Cell>(
      r#"(CELL) {
      leakage_power () {
        related_pg_pin : VDD;
        value : 1;
      }
      leakage_power () {
        related_pg_pin : VDD;
        when : "A*B*Y";
        value : 2;
      }
      leakage_power () {
        related_pg_pin : VDD;
        when : "!A*B*!Y";
        value : 3;
      }
      leakage_power () {
        related_pg_pin : VDD;
        when : "A*!B*!Y";
        value : 4;
      }
      leakage_power () {
        related_pg_pin : VDD;
        when : "!A*!B*!Y";
        value : 5;
      }
      leakage_power () {
        related_pg_pin : VSS;
        value : 6;
      }
      leakage_power () {
        related_pg_pin : VSS;
        when : "A*B*Y";
        value : 7;
      }
      leakage_power () {
        related_pg_pin : VSS;
        when : "!A*B*!Y";
        value : 8;
      }
      leakage_power () {
        related_pg_pin : VSS;
        when : "A*!B*!Y";
        value : 9;
      }
      leakage_power () {
        related_pg_pin : VSS;
        when : "!A*!B*!Y";
        value : 10;
      }
    }
  "#,
    );
    assert_eq!(
      vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
      cell
        .leakage_power
        .into_iter_sort()
        .map(|leakage| leakage.value as i8)
        .collect::<Vec<_>>()
    );
  }
}
/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.4&end=141.5
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default, Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
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
  fn parse_set_name(&mut self, mut v: Vec<&str>) -> Result<(), crate::ast::IdError> {
    let l = v.len();
    if l == 2 {
      v.pop()
        .map_or(Err(crate::ast::IdError::Other("Unkown pop error".into())), |var2| {
          v.pop().map_or(
            Err(crate::ast::IdError::Other("Unkown pop error".into())),
            |var1| {
              self.input_nodes =
                var1.split_ascii_whitespace().map(ArcStr::from).collect();
              self.internal_nodes =
                var2.split_ascii_whitespace().map(ArcStr::from).collect();
              Ok(())
            },
          )
        })
    } else {
      Err(crate::ast::IdError::length_dismatch(2, l, v))
    }
  }
  #[inline]
  #[allow(clippy::indexing_slicing)]
  fn fmt_name<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if self.input_nodes.len() == 1 {
      write!(f, "{}", self.input_nodes[0])?;
    } else {
      join_fmt(self.input_nodes.iter(), f, |s, ff| write!(ff, "{s}"), " ")?;
    }
    write!(f, ", ")?;
    if self.internal_nodes.len() == 1 {
      write!(f, "{}", self.internal_nodes[0])
    } else {
      join_fmt(self.internal_nodes.iter(), f, |s, ff| write!(ff, "{s}"), " ")
    }
  }
}

/// `StateTable` Table
#[derive(Default, Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Table {
  pub v: Vec<ArcStr>,
}

impl fmt::Display for Table {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.v, f)
  }
}

impl FromStr for Table {
  type Err = fmt::Error;
  /// To prevent syntax errors, the line continuation character
  /// must be followed immediately by the next line character.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.18&end=141.21
  /// ">Reference</a>
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      v: s
        .split("\\\n")
        .filter_map(|line| {
          let _l = line
            .trim_start()
            .trim_end_matches(|c: char| c == ',' || c.is_ascii_whitespace());
          if _l.is_empty() {
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
  fn is_set(&self) -> bool {
    !self.v.is_empty()
  }
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> nom::IResult<&'a str, Result<Self, AttriValue>, nom::error::Error<&'a str>> {
    let (input, simple_multi) = crate::ast::parser::simple_multi(i, line_num)?;
    simple_multi
      .parse()
      .map_or(Ok((input, Err(AttriValue::Simple(ArcStr::from(simple_multi))))), |s| {
        Ok((input, Ok(s)))
      })
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let indent = f.indentation();
    join_fmt(
      self.v.iter(),
      f,
      |i, ff| write!(ff, "{i}"),
      format!(" ,\\\n{indent}         ").as_str(),
    )
    // let mut iter = self.v.iter();
    // if let Some(first) = iter.next() {
    //   write!(f, "\"{first}")?;
    //   while let Some(next) = iter.next() {
    //     write!(f, " ,\\\n{indent}         {next}")?;
    //   }
    //   write!(f, "\"")
    // } else {
    //   Ok(())
    // }
  }
}

#[cfg(test)]
mod test_statetable {
  use super::*;
  #[test]
  fn statetable_test() {
    _ = crate::ast::test_parse_fmt::<Statetable>(
      r#"(" CLK EN SE",ENL) {
        table : "	H   L  L : - : L ,\
        H   L  H : - : H ,\
        H   H  L : - : H ,\
        H   H  H : - : H ,\
        L   -  - : - : N ";
    }
  "#,
      r#"
liberty_db::cell::items::Statetable ("CLK EN SE", ENL) {
| table : "H   L  L : - : L ,\
|          H   L  H : - : H ,\
|          H   H  L : - : H ,\
|          H   H  H : - : H ,\
|          L   -  - : - : N";
}"#,
    );
  }
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
#[mut_set::derive::item(
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
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.22
  /// &end
  /// =228.22
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub direction: Option<Direction>,
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
  /// The `std_cell_main_rail`  Boolean attribute is defined in a `primary_power`
  /// power pin. When the attribute is set to true, the power and ground pin
  /// is used to determine which side of the voltage boundary
  /// the power and ground pin is connected.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=278.29&end=278.31
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub std_cell_main_rail: Option<bool>,
  /// The `pg_function`  attribute models the logical function
  /// of a virtual or derived power and ground (PG) pin as a Boolean expression
  /// involving the cells input signal pins, internal signal pins, PG pins.
  /// The attribute Boolean expression is checked during library compile to
  /// ensure that only one `pg_pin`  is always active at this virtual or derived PG pin.
  /// If more than one `pg_pin`  is found to be active at the virtual or the derived
  /// pg_pin  output, the `read_lib` command generates an error
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=266.21&end=266.26
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub pg_function: Option<IdBooleanExpression>,
  /// The `switch_function`  string attribute identifies the condition
  /// when the attached design partition is turned off by the input `switch_pin`.
  /// For a coarse-grain switch cell, the `switch_function`  attribute can be defined
  /// at both controlled power and ground pins (virtual VDD and virtual VSS for `pg_pin`) and
  /// the output pins.
  /// When the `switch_function`  attribute is defined in the controlled power and ground pin,
  /// it is used to specify the Boolean condition under which the cell switches off
  /// (or drives an X to) the controlled design partitions, including the traditional signal
  /// input pins only (with no related power pins to this output).
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=279.5&end=279.13
  /// ">Reference-Definition</a>
  #[liberty(simple(type = Option))]
  pub switch_function: Option<IdBooleanExpression>,
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
  /// `primary_power`
  #[strum(serialize = "primary_power")]
  PrimaryPower,
  /// `primary_ground`
  #[strum(serialize = "primary_ground")]
  PrimaryGround,
  /// `backup_power`
  #[strum(serialize = "backup_power")]
  BackupPower,
  /// `backup_ground`
  #[strum(serialize = "backup_ground")]
  #[default]
  BackupGround,
  /// `internal_power`
  #[strum(serialize = "internal_power")]
  InternalPower,
  /// `internal_ground`
  #[strum(serialize = "internal_ground")]
  InternalGround,
  /// `pwell`
  #[strum(serialize = "pwell")]
  Pwell,
  /// `nwell`
  #[strum(serialize = "nwell")]
  Nwell,
  /// `deepnwell`
  #[strum(serialize = "deepnwell")]
  DeepNwell,
  /// `deeppwell`
  #[strum(serialize = "deeppwell")]
  DeepPwell,
}
impl SimpleAttri for PgType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

/// The `switch_cell_type`  cell-level attribute specifies
/// the type of the switch cell for direct inference.
///
/// Syntax:
/// ``` text
/// switch_cell_type : coarse_grain | fine_grain;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=210.8&end=210.13
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SwitchCellType {
  /// `coarse_grain`
  #[strum(serialize = "coarse_grain")]
  CoarseGrain,
  /// `fine_grain`
  #[strum(serialize = "fine_grain")]
  FineGrain,
}
impl SimpleAttri for SwitchCellType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

/// interprets a combination timing arc between the clock pin and the output pin as a rising edge arc or as a falling edge arc
///
/// Valid values are `rising_edge_clock_cell`  and `falling_edge_clock_cell`.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.29+109.36&end=109.30+109.37
/// ">Reference</a>
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FpgaCellType {
  /// `rising_edge_clock_cell`
  #[strum(serialize = "rising_edge_clock_cell")]
  RisingEdgeClockCell,
  /// `falling_edge_clock_cell`
  #[strum(serialize = "falling_edge_clock_cell")]
  FallingEdgeClockCell,
}
impl SimpleAttri for FpgaCellType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
/// The `level_shifter_type`  attribute specifies the
/// voltage conversion type that is supported.
/// Valid values are:
///
/// + `LH`: Low to High
/// + `HL`: High to Low
/// + `HL_LH`: High to Low and Low to High
///
/// The `level_shifter_type`  attribute is optional
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=115.9&end=115.17
/// ">Reference</a>
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LevelShifterType {
  /// `LH`: Low to High
  #[strum(serialize = "LH")]
  LH,
  /// `HL`: High to Low
  #[strum(serialize = "HL")]
  HL,
  /// `HL_LH`: High to Low and Low to High
  #[strum(serialize = "HL_LH")]
  HL_LH,
}
impl SimpleAttri for LevelShifterType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

/// You can use the `clock_gating_integrated_cell` attribute to enter specific
/// values that determine which integrated cell functionality the clock-gating tool uses.
///
/// Syntax:
/// ```text
/// clock_gating_integrated_cell:generic|value_id;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=103.19&end=103.24
/// ">Reference</a>
#[derive(Debug, Clone)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ClockGatingIntegratedCell {
  ContainlatchNegedge,
  RegisterslatchPosedgePostcontrol,
  LatchlatchNegedgePrecontrol,
  LatchnonePosedgeControlObs,
  /// by accessing the state tables and state functions of the library cell pins
  Generic(ArcStr),
}
impl FromStr for ClockGatingIntegratedCell {
  type Err = ();
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "containlatch_negedge" => Ok(Self::ContainlatchNegedge),
      "registerslatch_posedge_postcontrol" => Ok(Self::RegisterslatchPosedgePostcontrol),
      "latchlatch_negedge_precontrol" => Ok(Self::LatchlatchNegedgePrecontrol),
      "latchnone_posedge_control_obs" => Ok(Self::LatchnonePosedgeControlObs),
      _ => Ok(Self::Generic(s.into())),
    }
  }
}

impl fmt::Display for ClockGatingIntegratedCell {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ContainlatchNegedge => write!(f, "containlatch_negedge"),
      Self::RegisterslatchPosedgePostcontrol => {
        write!(f, "registerslatch_posedge_postcontrol")
      }
      Self::LatchlatchNegedgePrecontrol => write!(f, "latchlatch_negedge_precontrol"),
      Self::LatchnonePosedgeControlObs => write!(f, "latchnone_posedge_control_obs"),
      Self::Generic(s) => write!(f, "{s}"),
    }
  }
}

impl SimpleAttri for ClockGatingIntegratedCell {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

/// Use the pin_opposite attribute to describe functionally opposite (logically inverse) groups
/// of input or output pins.
/// Syntax
///
/// ``` text
/// pin_opposite ("name_list1", "name_list2") ;
/// ```
///
/// + `name_list1`: A `name_list` of output pins requires the supplied output values to be opposite.
/// + `name_list2`: A `name_list` of input pins requires the supplied input values to be opposite.
///
/// In the following example, pins IP and OP are logically inverse.
/// ``` text
/// pin_opposite ("IP", "OP") ;
/// ```
/// The `pin_opposite` attribute also incorporates the functionality of the `pin_equal` complex
/// attribute.
///
/// In the following example, Q1, Q2, and Q3 are equal; QB1 and QB2 are equal; and the pins
/// in the first group are opposite of the pins in the second group.
/// ``` text
/// pin_opposite ("Q1 Q2 Q3", "QB1 QB2") ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=124.9&end=124.22
/// ">Reference</a>
#[derive(Debug, Clone)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]

pub struct PinOpposite {
  pub name_list1: WordSet,
  pub name_list2: WordSet,
}

impl ComplexAttri for PinOpposite {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let name_list1: WordSet = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Other),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let name_list2: WordSet = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Other),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      Err(ComplexParseError::LengthDismatch)
    } else {
      Ok(Self { name_list1, name_list2 })
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}, {}", self.name_list1, self.name_list2)
  }
}
