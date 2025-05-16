use crate::{
  Ctx,
  ast::{
    CodeFormatter, ComplexAttri, ComplexParseError, GroupComments, GroupFn, GroupSet,
    Indentation, NamedGroup, ParseScope, SimpleAttri, SimpleParseRes, join_fmt,
  },
  common::items::{NameList, WordSet},
  expression::{LogicBooleanExpression, PowerGroundBooleanExpression, logic},
  pin::Direction,
  table::CompactCcsPower,
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
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct LeakagePower<C: Ctx> {
  #[id]
  #[liberty(name)]
  pub name: Vec<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[id]
  #[liberty(simple(type = Option))]
  pub power_level: Option<String>,
  #[id]
  #[liberty(simple)]
  pub related_pg_pin: NameList,
  #[id]
  #[liberty(simple(type = Option))]
  pub when: Option<LogicBooleanExpression>,
  #[liberty(simple)]
  pub value: f64,
  #[liberty(complex(type = Option))]
  pub mode: Option<[String; 2]>,
}
impl<C: Ctx> GroupFn<C> for LeakagePower<C> {}

#[cfg(test)]
mod test_sort {
  use super::*;
  use crate::DefaultCtx;

  #[test]
  fn test_leakage_sort() {
    let cell = crate::ast::test_parse::<crate::Cell<DefaultCtx>>(
      r#"(CELL) {
      pin(A){}
      pin(B){}
      pin(Y){}
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
        .iter()
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
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Statetable<C: Ctx> {
  #[id]
  #[liberty(name)]
  pub input_nodes: Vec<String>,
  #[id]
  #[liberty(name)]
  pub internal_nodes: Vec<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[liberty(simple)]
  pub table: Table,
}
impl<C: Ctx> GroupFn<C> for Statetable<C> {}

impl<C: Ctx> NamedGroup<C> for Statetable<C> {
  #[inline]
  fn parse_set_name(
    builder: &mut Self::Builder,
    mut v: Vec<&str>,
  ) -> Result<(), crate::ast::IdError> {
    let l = v.len();
    if l == 2 {
      v.pop()
        .map_or(Err(crate::ast::IdError::Other("Unkown pop error".into())), |var2| {
          v.pop().map_or(
            Err(crate::ast::IdError::Other("Unkown pop error".into())),
            |var1| {
              builder.input_nodes =
                var1.split_ascii_whitespace().map(String::from).collect();
              builder.internal_nodes =
                var2.split_ascii_whitespace().map(String::from).collect();
              Ok(())
            },
          )
        })
    } else {
      Err(crate::ast::IdError::length_dismatch(2, l, v))
    }
  }
  #[inline]
  #[expect(clippy::indexing_slicing)]
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
  pub v: Vec<String>,
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
          if _l.is_empty() { None } else { Some(String::from(_l)) }
        })
        .collect(),
    })
  }
}
crate::ast::impl_self_builder!(Table);
impl<C: Ctx> SimpleAttri<C> for Table {
  #[inline]
  fn is_set(&self) -> bool {
    !self.v.is_empty()
  }
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> SimpleParseRes<'a, Self> {
    let (input, simple_multi) =
      crate::ast::parser::simple_multi(i, &mut scope.loc.line_num)?;
    simple_multi
      .parse()
      .map_or(Ok((input, Err(String::from(simple_multi)))), |s| Ok((input, Ok(s))))
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
  use crate::DefaultCtx;
  #[test]
  fn statetable_test() {
    _ = crate::ast::test_parse_fmt::<Statetable<DefaultCtx>>(
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
///
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
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct PgPin<C: Ctx> {
  #[liberty(name)]
  #[id(borrow = str)]
  pub name: String,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
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
  pub voltage_name: String,
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
  pub user_pg_type: String,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=209.30&end=209.32
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub physical_connection: String,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=209.30&end=209.32
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub related_bias_pin: String,
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
  pub pg_function: Option<PowerGroundBooleanExpression>,
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
  pub switch_function: Option<LogicBooleanExpression>,
}
impl<C: Ctx> GroupFn<C> for PgPin<C> {}

/// Use the `dynamic_current` group to specify a current waveform vector when the power
/// and ground current is dependent on the logical condition of a cell. A `dynamic_current`
/// group is defined in a cell group, as shown here:
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=147.3&end=147.5
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct DynamicCurrent<C: Ctx> {
  #[liberty(name)]
  #[id]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[id]
  #[liberty(simple(type = Option))]
  pub when: Option<LogicBooleanExpression>,
  #[id]
  #[liberty(simple)]
  pub related_inputs: NameList,
  #[id]
  #[liberty(simple)]
  pub related_outputs: NameList,
  #[liberty(complex(type = Option))]
  pub typical_capacitances: Option<Vec<f64>>,
  /// Use the switching_group group to specify a current waveform vector when the power
  /// and ground current is dependent on pin switching conditions.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=150.18&end=150.19
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub switching_group: GroupSet<SwitchingGroup<C>>,
}
impl<C: Ctx> GroupFn<C> for DynamicCurrent<C> {}

/// Use the switching_group group to specify a current waveform vector when the power
/// and ground current is dependent on pin switching conditions.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=150.18&end=150.19
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct SwitchingGroup<C: Ctx> {
  #[liberty(name)]
  #[id]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  /// The `input_switching_condition` attribute specifies the sense of the toggling input. If
  /// more than one `switching_group` group is specified within the `dynamic_current` group,
  /// you can place the attribute in any order.
  /// The valid values are rise and fall. rise represents a rising pin and fall represents a
  /// falling pin.
  /// ### Syntax
  /// `input_switching_condition (enum(rise, fall));`
  ///
  /// `enum(rise, fall)`
  /// Enumerated type specifying the rise or fall condition.
  ///
  /// ### Example
  /// `input_switching_condition (rise);`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=151.5&end=151.15
  /// ">Reference</a>
  #[id]
  #[liberty(complex(type = Option))]
  pub input_switching_condition: Option<logic::Edge>,
  /// Use the `output_switching_condition` attribute to specify the sense of the toggling
  /// output. If there is more than one `switching_group` group specified within the
  /// `dynamic_current` group, you can place the attribute in any order. The order in the list of
  /// the `output_switching_condition` attribute is mapped to the same order of output pins in
  /// the `related_outputs` attribute.
  /// The valid values are rise and fall. rise represents a rising pin and fall represents a
  /// falling pin.
  /// ### Syntax
  /// `output_switching_condition (enum(rise, fall));`
  ///
  /// `enum(rise, fall)`
  /// Enumerated type specifying the rise or fall condition.
  ///
  /// ### Example
  /// `output_switching_condition (rise, fall);`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=151.17&end=151.29
  /// ">Reference</a>
  #[id]
  #[liberty(complex(type = Option))]
  pub output_switching_condition: Option<logic::Edge>,
  /// The `min_input_switching_count` attribute specifies the minimum number of
  /// bits in the input bus that are switching simultaneously. The following applies to the
  /// `min_input_switching_count` attribute:
  /// + The count must be an integer.
  /// + The count must be greater than 0 and less than the `max_input_switching_count`
  /// value.
  /// ### Syntax
  /// ``` text
  /// switching_group() {
  /// min_input_switching_count : integer ;
  /// max_input_switching_count : integer ;
  /// ...
  /// }
  /// ```
  /// ### Example
  /// ``` text
  /// switching_group() {
  /// min_input_switching_count : 1 ;
  /// max_input_switching_count : 3 ;
  /// ...
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=152.3&end=152.22
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub min_input_switching_count: Option<usize>,
  /// The `max_input_switching_count` attribute specifies the maximum number of
  /// bits in the input bus that are switching simultaneously. The following applies to the
  /// `max_input_switching_count` attribute:
  /// + The count must be an integer.
  /// + The count must be greater than the `min_input_switching_count` value.
  /// + The count within a `dynamic_current` should cover the total number of input bits
  /// specified in `related_inputs`.
  /// ### Syntax
  /// ``` text
  /// switching_group() {
  /// min_input_switching_count : integer ;
  /// max_input_switching_count : integer ;
  /// ...
  /// }
  /// ```
  /// ### Example
  /// ``` text
  /// switching_group() {
  /// min_input_switching_count : 1 ;
  /// max_input_switching_count : 3 ;
  /// ...
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=152.24+153.2&end=152.38+153.7
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub max_input_switching_count: Option<usize>,
  /// Use the `pg_current` group to specify current waveform data in a vector group. If all
  /// vectors under the group are dense, data in this group is represented as a dense table. If
  /// all vectors under the group are sparse in cross type, data in this group is represented as a
  /// sparse cross table. If all vectors under the group are sparse in diagonal type, data in this
  /// group is represented as a sparse diagonal table.
  /// ``` text
  /// library (name) {
  /// cell (name) {
  /// dynamic_current () {
  /// ...
  /// switching_group() {
  /// ...
  /// pg_current () {}
  /// ...
  /// }
  /// }
  /// }
  /// }
  /// }
  /// ```
  /// Group
  ///
  /// `compact_ccs_power`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=153.9&end=153.28
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub pg_current: GroupSet<PgCurrent<C>>,
}
impl<C: Ctx> GroupFn<C> for SwitchingGroup<C> {}

/// Use the `pg_current` group to specify current waveform data in a vector group. If all
/// vectors under the group are dense, data in this group is represented as a dense table. If
/// all vectors under the group are sparse in cross type, data in this group is represented as a
/// sparse cross table. If all vectors under the group are sparse in diagonal type, data in this
/// group is represented as a sparse diagonal table.
/// ``` text
/// library (name) {
/// cell (name) {
/// dynamic_current () {
/// ...
/// switching_group() {
/// ...
/// pg_current () {}
/// ...
/// }
/// }
/// }
/// }
/// }
/// ```
/// Group
///
/// `compact_ccs_power`
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=153.9&end=153.28
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct PgCurrent<C: Ctx> {
  #[liberty(name)]
  #[id]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  /// The `compact_ccs_power` group contains a detailed description for compact CCS
  /// power data. The `compact_ccs_power` group includes the following optional attributes:
  /// `base_curves_group`, `index_1`, `index_2`, `index_3` and `index_4`. The description for these
  /// attributes in the `compact_ccs_power` group is the same as in the `compact_lut_template`
  /// group. However, the attributes have a higher priority in the `compact_ccs_power` group.
  /// For more information, see `compact_lut_template` Group on page 41.
  /// The `index_output` attribute is also optional. It is used only on cross type tables. For
  /// more information about the `index_output` attribute, see `index_output` Simple Attribute on
  /// page 156.
  /// ``` text
  /// library (name) {
  ///   cell(cell_name) {
  ///     dynamic_current() {
  ///       switching_group() {
  ///         pg_current(pg_pin_name) {
  ///           compact_ccs_power (template_name) {
  ///             base_curves_group : bc_name;
  ///             index_output : pin_name;
  ///             index_1 ("float, ..., float");
  ///             index_2 ("float, ..., float");
  ///             index_3 ("float, ..., float");
  ///             index_4 ("string, ..., string");
  ///             values ("float | integer, ..., float | integer");
  ///           } /* end of compact_ccs_power */
  ///         }
  ///       }
  ///     }
  ///   }
  /// }
  /// ```
  /// Complex Attributes
  /// `base_curves_group : bc_name;`
  /// `index_output : pin_name;`
  /// `index_1 ("float, ..., float");`
  /// `index_2 ("float, ..., float");`
  /// `index_3 ("float, ..., float");`
  /// `index_4 ("string, ..., string");`
  /// `values ("float | integer, ..., float | integer");`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=153.30+154.2&end=153.40+154.25
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  #[liberty(after_build = crate::table::use_compact_template!)]
  pub compact_ccs_power: GroupSet<CompactCcsPower<C>>,
}
impl<C: Ctx> GroupFn<C> for PgCurrent<C> {}

/// The `intrinsic_parasitic` group specifies the state-dependent intrinsic capacitance and
/// intrinsic resistance of a `cell`.
/// ### Syntax
/// ``` text
/// library( library_name ) {
///   ......
///   lu_table_template ( template_name ) {
///     variable_1 : pg_voltage | pg_voltage_difference;
///     index_1 ( "float, ..., float" );
///   }
///   cell (cell_name) {
///     mode_definition (mode_name) {
///       mode_value (mode_value) {
///         when : boolean_expression ;
///         sdf_cond : boolean_expression ;
///       }
///     }
///     ...
///     intrinsic_parasitic () {
///       mode (mode_name, mode_value) ;
///       when : boolean expression ;
///       intrinsic_resistance(pg_pin_name) {
///         related_output : output_pin_name ;
///         value : float ;
///         reference_pg_pin : pg_pin_name;
///         lut_values ( template_name ) {
///           index_1 ("float, ... float" );
///           values ("float, ... float" );
///         }
///       }
///       intrinsic_capacitance(pg_pin_name) {
///         value : float ;
///         reference_pg_pin : pg_pin_name;
///         lut_values ( template_name ) {
///           index_1 ("float, ... float" );
///           values ("float, ... float" );
///         }
///       }
///     }
///   }
/// }
/// ```
/// Simple Attributes
/// + when
/// + reference_pg_pin
///
/// Complex Attribute
/// + mode
///
/// Groups
/// + intrinsic_capacitance
/// + intrinsic_resistance
/// + total_capacitance
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=176.24+177.2&end=176.49+177.25
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct IntrinsicParasitic<C: Ctx> {
  #[liberty(name)]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[id]
  #[liberty(simple(type = Option))]
  pub when: Option<LogicBooleanExpression>,
  /// The `reference_pg_pin` attribute specifies the reference pin for the
  /// `intrinsic_resistance` and `intrinsic_capacitance` groups. The reference pin must
  /// be a valid PG pin.
  ///
  /// ### Syntax
  /// `reference_pg_pin : pg_pin_name ;`
  ///
  /// ### Example
  /// `reference_pg_pin : G1 ;`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=178.3&end=178.9
  /// ">Reference</a>
  #[id]
  #[liberty(simple(type = Option))]
  pub reference_pg_pin: Option<String>,
  /// The `mode` attribute pertains to an individual `cell`. The cell is active when the `mode` attribute
  /// is instantiated with a name and a value. You can specify multiple instances of this attribute.
  /// However, specify only one instance for each `cell`.
  /// Define the mode attribute within an `intrinsic_parasitic` group.
  ///
  /// ### Syntax
  /// `mode (mode_name, mode_value) ;`
  ///
  /// ### Example
  /// `mode (rw, read) ;`
  ///
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=178.11&end=178.18
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub mode: Option<[String; 2]>,
  /// Use this group to specify the intrinsic capacitance of a `cell`.
  /// ### Syntax
  /// ``` text
  /// intrinsic_parasitic () {
  ///   intrinsic_capacitance (pg_pin_name) {
  ///     value : float ;
  ///     reference_pg_pin : pg_pin_name;
  ///     lut_values ( template_name ) {
  ///       index_1 ("float, ... float" );
  ///       values ("float, ... float" );
  ///     }
  ///   }
  /// }
  /// ```
  /// The `pg_pin_name` specifies a power and ground pin where the capacitance is derived.
  /// You can have more than one `intrinsic_capacitance` group. You can place these
  /// groups in any order within an `intrinsic_parasitic` group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=178.20&end=178.34
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub intrinsic_capacitance: GroupSet<IntrinsicCapacitance<C>>,
  /// Use this group to specify the intrinsic resistance between a power pin and an output pin of
  /// a cell.
  ///
  /// ### Syntax
  /// ``` text
  /// intrinsic_parasitic () {
  ///   intrinsic_resistance (pg_pin_name) {
  ///     related_output : output_pin_name ;
  ///     value : float ;
  ///     reference_pg_pin : pg_pin_name;
  ///     lut_values ( template_name ) {
  ///       index_1 ("float, ... float" );
  ///       values ("float, ... float" );
  ///     }
  ///   }
  /// }
  /// ```
  /// The `pg_pin_name` specifies a power or ground pin. You can place the
  /// `intrinsic_resistance` groups in any order within an `intrinsic_parasitic` group. If
  /// some of the `intrinsic_resistance` group is not defined, the value of resistance defaults
  /// to +infinity. The channel connection between the power and ground pins and the output
  /// pin is defined as a closed channel if the resistance value is greater than 1 megaohm.
  /// Otherwise, the channel is opened. The `intrinsic_resistance` group is not required if
  /// the channel is closed.
  ///
  /// Simple Attributes
  /// + `related_output`
  /// + `value`
  /// + `reference_pg_pin`
  /// Group
  /// + `lut_values`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=180.10&end=180.36
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub intrinsic_resistance: GroupSet<IntrinsicResistance<C>>,
  /// The `total_capacitance` group specifies the macro cell’s total capacitance on a power
  /// or ground net within the `intrinsic_parasitic` group. The following applies to the
  /// `total_capacitance` group:
  /// + The `total_capacitance` group can be placed in any order if there is more than one
  /// `total_capacitance` group within an `intrinsic_parasitic` group.
  /// + The total capacitance parasitics modeling in macro cells is not state dependent, which
  /// means that there is no state condition specified in `intrinsic_parasitic`.
  ///
  /// ### Syntax
  /// ``` text
  /// cell (cell_name) {
  ///   ...
  ///   intrinsic_parasitic () {
  ///     total_capacitance (pg_pin_name) {
  ///       value : float ;
  ///     }
  ///   ...
  ///   }
  ///   ...
  /// }
  /// ```
  ///
  /// ### Example
  /// ``` text
  /// cell (my_cell) {
  ///   ...
  ///   intrinsic_parasitic () {
  ///     total_capacitance (VDD) {
  ///       value : 0.2 ;
  ///     }
  ///   ...
  ///   }
  /// ...
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=182.12&end=182.42
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub total_capacitance: GroupSet<PgPinWithValue<C>>,
}
impl<C: Ctx> GroupFn<C> for IntrinsicParasitic<C> {}

/// Use this group to specify the intrinsic capacitance of a `cell`.
/// ### Syntax
/// ``` text
/// intrinsic_parasitic () {
///   intrinsic_capacitance (pg_pin_name) {
///     value : float ;
///     reference_pg_pin : pg_pin_name;
///     lut_values ( template_name ) {
///       index_1 ("float, ... float" );
///       values ("float, ... float" );
///     }
///   }
/// }
/// ```
/// The `pg_pin_name` specifies a power and ground pin where the capacitance is derived.
/// You can have more than one `intrinsic_capacitance` group. You can place these
/// groups in any order within an `intrinsic_parasitic` group.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=178.20&end=178.34
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct IntrinsicCapacitance<C: Ctx> {
  #[liberty(name)]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  /// The `value` attribute specifies the value of the intrinsic capacitance.
  /// By default, the intrinsic capacitance value is zero.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=179.8&end=179.9
  /// ">Reference</a>
  #[id(into_hash_ord_fn = crate::common::f64_into_hash_ord_fn)]
  #[liberty(simple)]
  pub value: f64,
  /// The `reference_pg_pin` attribute specifies the reference pin for the
  /// `intrinsic_resistance` and `intrinsic_capacitance` groups. The reference pin must
  /// be a valid PG pin.
  /// ### Syntax
  /// `reference_pg_pin : pg_pin_name ;`
  ///
  /// ### Example
  /// `reference_pg_pin : G1 ;`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=179.15&end=179.21
  /// ">Reference</a>
  #[id]
  #[liberty(simple(type = Option))]
  pub reference_pg_pin: Option<String>,
  /// Voltage-dependent intrinsic parasitics are modeled by lookup tables. A lookup table
  /// consists of intrinsic parasitic values for different values of VDD. To use these lookup
  /// tables, define the lut_values group. You can add the `lut_values` group to both the
  /// `intrinsic_resistance` and `intrinsic_capacitance` groups. The `lut_values` group
  /// uses the `variable_1` variable, which is defined within the lu_table_template group,
  /// at the library level. The valid values of the `variable_1` variable are `pg_voltage` and
  /// `pg_voltage_difference`.
  ///
  /// ### Syntax
  /// ``` text
  /// lut_values ( template_name ) {
  /// index_1 ("float, ... float" );
  /// values ("float, ... float" );
  /// }
  /// ```
  ///
  /// ### Example
  /// ``` text
  /// lut_values ( test_voltage ) {
  /// index_1 ( "0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0" );
  /// values ( "0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0" );
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=179.23+180.2&end=179.34+180.8
  /// ">Reference</a>
  #[liberty(group(type = Option))]
  pub lut_values: Option<LutValues<C>>,
}
impl<C: Ctx> GroupFn<C> for IntrinsicCapacitance<C> {}

/// Use this group to specify the intrinsic resistance between a power pin and an output pin of
/// a cell.
///
/// ### Syntax
/// ``` text
/// intrinsic_parasitic () {
///   intrinsic_resistance (pg_pin_name) {
///     related_output : output_pin_name ;
///     value : float ;
///     reference_pg_pin : pg_pin_name;
///     lut_values ( template_name ) {
///       index_1 ("float, ... float" );
///       values ("float, ... float" );
///     }
///   }
/// }
/// ```
/// The `pg_pin_name` specifies a power or ground pin. You can place the
/// `intrinsic_resistance` groups in any order within an `intrinsic_parasitic` group. If
/// some of the `intrinsic_resistance` group is not defined, the value of resistance defaults
/// to +infinity. The channel connection between the power and ground pins and the output
/// pin is defined as a closed channel if the resistance value is greater than 1 megaohm.
/// Otherwise, the channel is opened. The `intrinsic_resistance` group is not required if
/// the channel is closed.
///
/// Simple Attributes
/// + `related_output`
/// + `value`
/// + `reference_pg_pin`
/// Group
/// + `lut_values`
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=180.10&end=180.36
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct IntrinsicResistance<C: Ctx> {
  #[liberty(name)]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  /// Specifies the value of the intrinsic resistance. If this attribute is not defined, the value of
  /// the intrinsic resistance defaults to +infinity.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=181.9&end=181.10
  /// ">Reference</a>
  #[liberty(simple)]
  #[liberty(default = f64::INFINITY)]
  pub value: f64,
  /// Use this attribute to specify the output pin.
  /// ### Syntax
  /// ``` text
  /// related_output : output_pin_name ;
  /// ``` text
  ///
  /// `output_pin_name`
  /// The name of the output pin.
  /// ### Example
  /// `related_output : "A & B" ;`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=179.15&end=179.21
  /// ">Reference</a>
  #[id]
  #[liberty(simple(type = Option))]
  pub related_output: Option<String>,
  /// The `reference_pg_pin` attribute specifies the reference pin for the
  /// `intrinsic_resistance` and `intrinsic_capacitance` groups. The reference pin must
  /// be a valid PG pin.
  /// ### Syntax
  /// `reference_pg_pin : pg_pin_name ;`
  ///
  /// ### Example
  /// `reference_pg_pin : G1 ;`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=179.15&end=179.21
  /// ">Reference</a>
  #[id]
  #[liberty(simple(type = Option))]
  pub reference_pg_pin: Option<String>,
  /// Voltage-dependent intrinsic parasitics are modeled by lookup tables. A lookup table
  /// consists of intrinsic parasitic values for different values of VDD. To use these lookup
  /// tables, define the lut_values group. You can add the `lut_values` group to both the
  /// `intrinsic_resistance` and `intrinsic_capacitance` groups. The `lut_values` group
  /// uses the `variable_1` variable, which is defined within the lu_table_template group,
  /// at the library level. The valid values of the `variable_1` variable are `pg_voltage` and
  /// `pg_voltage_difference`.
  ///
  /// ### Syntax
  /// ``` text
  /// lut_values ( template_name ) {
  /// index_1 ("float, ... float" );
  /// values ("float, ... float" );
  /// }
  /// ```
  ///
  /// ### Example
  /// ``` text
  /// lut_values ( test_voltage ) {
  /// index_1 ( "0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0" );
  /// values ( "0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0" );
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=179.23+180.2&end=179.34+180.8
  /// ">Reference</a>
  #[liberty(group(type = Option))]
  pub lut_values: Option<LutValues<C>>,
}
impl<C: Ctx> GroupFn<C> for IntrinsicResistance<C> {}

/// The `total_capacitance` group specifies the macro cell’s total capacitance on a power
/// or ground net within the `intrinsic_parasitic` group. The following applies to the
/// `total_capacitance` group:
/// + The `total_capacitance` group can be placed in any order if there is more than one
/// `total_capacitance` group within an `intrinsic_parasitic` group.
/// + The total capacitance parasitics modeling in macro cells is not state dependent, which
/// means that there is no state condition specified in `intrinsic_parasitic`.
///
/// ### Syntax
/// ``` text
/// cell (cell_name) {
///   ...
///   intrinsic_parasitic () {
///     total_capacitance (pg_pin_name) {
///       value : float ;
///     }
///   ...
///   }
///   ...
/// }
/// ```
///
/// ### Example
/// ``` text
/// cell (my_cell) {
///   ...
///   intrinsic_parasitic () {
///     total_capacitance (VDD) {
///       value : 0.2 ;
///     }
///   ...
///   }
/// ...
/// }
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=182.12&end=182.42
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct PgPinWithValue<C: Ctx> {
  #[id]
  #[liberty(name)]
  pg_pin_name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[liberty(simple)]
  pub value: f64,
}
impl<C: Ctx> GroupFn<C> for PgPinWithValue<C> {}

/// The `gate_leakage` group specifies the cell’s gate leakage current on input or inout pins
/// within the `leakage_current` group in a cell. The following applies to `gate_leakage`
///
/// groups:
/// + Groups can be placed in any order if there is more than one `gate_leakage` group
/// within a `leakage_current` group.
/// + The leakage current of a cell is characterized with opened outputs, which means that
/// modeling cell outputs do not drive any other cells. Outputs are assumed to have zero
/// static current during the measurement.
/// + A missing `gate_leakage` group is allowed for certain pins.
/// + Current conservation is applicable if it can be applied to higher error tolerance.
/// ### Syntax
/// `gate_leakage (input_pin_name)`
///
/// ### Example
/// ``` text
/// cell (my_cell) {
///   ...
///   leakage_current {
///     ...
///   }
///   ...
///   gate_leakage (A) {
///     input_low_value : -0.5 ;
///     input_high_value : 0.6 ;
///   }
/// }
/// ```
/// Simple Attributes
/// + `input_low_value`
/// + `input_high_value`
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=197.9&end=197.38
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct GateLeakage<C: Ctx> {
  #[id]
  #[liberty(name)]
  input_pin_name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  /// The `input_low_value` attribute specifies gate leakage current on an input or inout pin
  /// when the pin is in a low state condition.
  /// The following applies to the `input_low_value` attribute:
  /// + A negative floating-point number value is required.
  /// + The gate leakage current flow is measured from the power pin of a cell to the ground
  /// pin of its driver cell.
  /// + The input pin is pulled up to low.
  /// + The `input_low_value` attribute is not required for a gate_leakage group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=198.3&end=198.22
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub input_low_value: Option<f64>,
  /// The `input_high_value` attribute specifies gate leakage current on an input or inout pin
  /// when the pin is in a high state condition.
  /// + The gate leakage current flow is measured from the power pin of its driver cell to the
  /// ground pin of the cell itself.
  /// + A positive floating-point number value is required.
  /// + The input pin is pulled up to high.
  /// + The `input_high_value` attribute is not required for a gate_leakage group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=198.24&end=198.38
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub input_high_value: Option<f64>,
}
impl<C: Ctx> GroupFn<C> for GateLeakage<C> {}

/// A `leakage_current` group is defined within a cell group or a model group to specify
/// leakage current values that are dependent on the state of the cell.
///
/// ### Syntax
/// ``` text
/// library (name) {
/// cell(cell_name) {
///   ...
///   leakage_current() {
///     when : boolean expression;
///     pg_current(pg_pin_name) {
///       value : float;
///     }
///     ...
///   }
/// }
/// ```
/// Simple Attributes
/// + when
/// + value
///
/// Complex Attribute
/// + mode
///
/// Group
/// + pg_current
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=194.49+195.2&end=194.50+195.20
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct LeakageCurrent<C: Ctx> {
  #[liberty(name)]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[id]
  #[liberty(simple(type = Option))]
  pub when: Option<LogicBooleanExpression>,
  /// When a cell has a single power and ground pin, omit the `pg_current` group and specify
  /// the leakage current value. Otherwise, specify the value in the `pg_current` group. Current
  /// conservation is applied for each leakage_current group. The value attribute specifies
  /// the absolute value of leakage current on a single power and ground pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=195.34+196.2&end=195.35+196.9
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub value: Option<f64>,
  /// The `mode` attribute pertains to an individual `cell`. The cell is active when the `mode` attribute
  /// is instantiated with a name and a value. You can specify multiple instances of this attribute.
  /// However, specify only one instance for each `cell`.
  /// Define the mode attribute within an `intrinsic_parasitic` group.
  ///
  /// ### Syntax
  /// `mode (mode_name, mode_value) ;`
  ///
  /// ### Example
  /// `mode (rw, read) ;`
  ///
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=178.11&end=178.18
  /// ">Reference</a>
  #[liberty(complex(type = Option))]
  pub mode: Option<[String; 2]>,
  /// Use this group to specify a power or ground pin where leakage current is to be measured.
  ///
  /// ### Syntax
  /// ``` text
  /// cell(cell_name) {
  ///   ...
  ///   leakage_current() {
  ///     when : boolean expression;
  ///     pg_current(pg_pin_name) {
  ///       value : float;
  ///     }
  ///   }
  /// }
  /// ```
  /// `pg_pin_name`
  ///
  /// Specifies the power or ground pin where the leakage current is to be measured.
  /// Simple Attribute
  ///
  /// `value`
  ///
  /// Use this attribute in the `pg_current` group to specify the leakage current value when a cell
  /// has multiple power and ground pins. The leakage current is measured toward a cell. For
  /// power pins, the current is positive if it is dragged into a cell. For ground pins, the current
  /// is negative, indicating that current flows out of a cell. If all power and ground pins are
  /// specified within a `leakage_current` group, the sum of the leakage currents should be
  /// zero.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=196.17&end=196.36
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub pg_current: GroupSet<PgPinWithValue<C>>,
  /// The `gate_leakage` group specifies the cell’s gate leakage current on input or inout pins
  /// within the `leakage_current` group in a cell. The following applies to `gate_leakage`
  ///
  /// groups:
  /// + Groups can be placed in any order if there is more than one `gate_leakage` group
  /// within a `leakage_current` group.
  /// + The leakage current of a cell is characterized with opened outputs, which means that
  /// modeling cell outputs do not drive any other cells. Outputs are assumed to have zero
  /// static current during the measurement.
  /// + A missing `gate_leakage` group is allowed for certain pins.
  /// + Current conservation is applicable if it can be applied to higher error tolerance.
  /// ### Syntax
  /// `gate_leakage (input_pin_name)`
  ///
  /// ### Example
  /// ``` text
  /// cell (my_cell) {
  ///   ...
  ///   leakage_current {
  ///     ...
  ///   }
  ///   ...
  ///   gate_leakage (A) {
  ///     input_low_value : -0.5 ;
  ///     input_high_value : 0.6 ;
  ///   }
  /// }
  /// ```
  /// Simple Attributes
  /// + `input_low_value`
  /// + `input_high_value`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=197.9&end=197.38
  /// ">Reference</a>
  #[liberty(group(type = Set))]
  pub gate_leakage: GroupSet<GateLeakage<C>>,
}
impl<C: Ctx> GroupFn<C> for LeakageCurrent<C> {}

/// Voltage-dependent intrinsic parasitics are modeled by lookup tables. A lookup table
/// consists of intrinsic parasitic values for different values of VDD. To use these lookup
/// tables, define the `lut_values` group. You can add the `lut_values` group to both the
/// `intrinsic_resistance` and `intrinsic_capacitance` groups. The `lut_values` group
/// uses the `variable_1` variable, which is defined within the `lu_table_template` group,
/// at the library level. The valid values of the `variable_1` variable are `pg_voltage` and
/// `pg_voltage_difference`.
///
/// ### Syntax
/// ``` text
/// lut_values ( template_name ) {
/// index_1 ("float, ... float" );
/// values ("float, ... float" );
/// }
/// ```
///
/// ### Example
/// ``` text
/// lut_values ( test_voltage ) {
/// index_1 ( "0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0" );
/// values ( "0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0" );
/// }
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=179.23+180.2&end=179.34+180.8
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct LutValues<C: Ctx> {
  #[liberty(name)]
  pub name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  #[liberty(complex)]
  pub values: Vec<f64>,
}
impl<C: Ctx> GroupFn<C> for LutValues<C> {}

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
#[derive(strum::EnumString, strum::EnumIter, strum::Display, strum::AsRefStr)]
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
crate::ast::impl_self_builder!(PgType);
impl<C: Ctx> SimpleAttri<C> for PgType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
  }
}

/// The `switch_cell_type`  cell-level attribute specifies
/// the type of the switch cell for direct inference.
///
/// ### Syntax:
/// ``` text
/// switch_cell_type : coarse_grain | fine_grain;
/// ``` text
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=210.8&end=210.13
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum::EnumString, strum::EnumIter, strum::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SwitchCellType {
  /// `coarse_grain`
  #[strum(serialize = "coarse_grain")]
  CoarseGrain,
  /// `fine_grain`
  #[strum(serialize = "fine_grain")]
  FineGrain,
}
crate::ast::impl_self_builder!(SwitchCellType);
impl<C: Ctx> SimpleAttri<C> for SwitchCellType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
  }
}

/// interprets a combination timing arc between the clock pin and the output pin as a rising edge arc or as a falling edge arc
///
/// Valid values are `rising_edge_clock_cell`  and `falling_edge_clock_cell`.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.29+109.36&end=109.30+109.37
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum::EnumString, strum::EnumIter, strum::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FpgaCellType {
  /// `rising_edge_clock_cell`
  #[strum(serialize = "rising_edge_clock_cell")]
  RisingEdgeClockCell,
  /// `falling_edge_clock_cell`
  #[strum(serialize = "falling_edge_clock_cell")]
  FallingEdgeClockCell,
}
crate::ast::impl_self_builder!(FpgaCellType);
impl<C: Ctx> SimpleAttri<C> for FpgaCellType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
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
#[expect(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum::EnumString, strum::EnumIter, strum::Display)]
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
crate::ast::impl_self_builder!(LevelShifterType);
impl<C: Ctx> SimpleAttri<C> for LevelShifterType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
  }
}

/// You can use the `clock_gating_integrated_cell` attribute to enter specific
/// values that determine which integrated cell functionality the clock-gating tool uses.
///
/// ### Syntax:
/// ```text
/// clock_gating_integrated_cell:generic|value_id;
/// ``` text
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
  Generic(String),
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
crate::ast::impl_self_builder!(ClockGatingIntegratedCell);
impl<C: Ctx> SimpleAttri<C> for ClockGatingIntegratedCell {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
  }
}

/// Use the `pin_opposite` attribute to describe functionally opposite (logically inverse) groups
/// of input or output pins.
/// ### Syntax
///
/// ``` text
/// pin_opposite ("name_list1", "name_list2") ;
/// ``` text
///
/// + `name_list1`: A `name_list` of output pins requires the supplied output values to be opposite.
/// + `name_list2`: A `name_list` of input pins requires the supplied input values to be opposite.
///
/// In the following example, pins IP and OP are logically inverse.
/// ``` text
/// pin_opposite ("IP", "OP") ;
/// ``` text
/// The `pin_opposite` attribute also incorporates the functionality of the `pin_equal` complex
/// attribute.
///
/// In the following example, Q1, Q2, and Q3 are equal; QB1 and QB2 are equal; and the pins
/// in the first group are opposite of the pins in the second group.
/// ``` text
/// pin_opposite ("Q1 Q2 Q3", "QB1 QB2") ;
/// ``` text
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
crate::ast::impl_self_builder!(PinOpposite);
impl<C: Ctx> ComplexAttri<C> for PinOpposite {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let name_list1: WordSet = match iter.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(_) => return Err(ComplexParseError::Other),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let name_list2: WordSet = match iter.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(_) => return Err(ComplexParseError::Other),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
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
