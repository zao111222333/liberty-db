//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
//! </script>
// use std::ops::DerefMut;

use crate::{
  ast::{
    self, Attributes, CodeFormatter, ComplexAttri, DefinedType, GroupComments, GroupFn,
    GroupSet, Indentation, ParseScope, SimpleAttri,
  },
  common::items::{Formula, IdVector},
  expression::logic,
  ArcStr, NotNan,
};
use core::fmt::{self, Write};

/// The `sensitization` group defined at the library level describes.
///
/// the complete state patterns for a specific list of pins (defined by the `pin_names` attribute)
/// that are referenced and instantiated as stimuli in the timing arc.
///
/// Vector attributes in the group define all possible pin states used as stimuli.
/// Actual stimulus waveforms can be described by a combination of these vectors.
/// Multiple sensitization groups are allowed in a library. Each `sensitization` group
/// can be referenced by multiple cells, and each cell can make reference to
/// multiple `sensitization`  groups.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=88.10&end=88.16
/// ">Reference</a>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(sort)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sensitization {
  /// name
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The `pin_names` attribute specified at the library level defines
  /// a default list of pin names. All vectors in this `sensitization` group
  /// are the exhaustive list of all possible transitions of the input pins
  /// and their subsequent output response.
  ///
  /// The `pin_names` attribute is required, and it must be declared in
  /// the `sensitization` group before all vector declarations.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=88.28&end=88.32
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex)]
  pub pin_names: Vec<ArcStr>,
  /// # vector Complex Attribute
  ///
  /// Similar to the `pin_names` attribute,
  /// the `vector` attribute describes a transition pattern for the specified pins.
  /// The stimulus is described by an ordered list of vectors.
  ///
  /// The arguments for the `vector` attribute are as follows:
  ///
  /// `vector id`
  ///
  /// The `vector id`  argument is an identifier to the vector string (a number tag
  /// that defines the list of possible sensitization combinations in a cell).
  /// The vector id value must be an integer greater than or equal to zero and
  /// unique among all vectors in the current `sensitization` group. It is recommended
  /// that you start numbering from 0 or 1.
  ///
  /// `vector string`
  ///
  /// The `vector string` argument represents a pin transition state. The string consists
  /// of the following transition status values: 0, 1, X, and Z where each character is separated by a space.
  /// The number of elements in the vector string must equal the number of arguments in `pin_names`.
  ///
  /// The `vector` attribute can also be declared as:
  ///
  /// `vector (positive_integer, "{0|1|X|Z} [0|1|X|Z]…");`
  ///
  /// ## Syntax
  ///
  /// `vector (integer, string);`
  ///
  /// ## Example
  /// ``` text
  /// sensitization(sensitization_nand2) {
  ///   pin_names ( IN1, IN2, OUT1 );
  ///   vector ( 1, "0 0 1" );
  ///   vector ( 2, "0 1 1" );
  ///   vector ( 3, "1 0 1" );
  ///   vector ( 4, "1 1 0" );
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=89.5&end=89.29
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex(type = Vec))]
  pub vector: Vec<SensitizationVector>,
}

/// # vector Complex Attribute
///
/// Similar to the `pin_names` attribute,
/// the `vector` attribute describes a transition pattern for the specified pins.
/// The stimulus is described by an ordered list of vectors.
///
/// The arguments for the `vector` attribute are as follows:
///
/// `vector id`
///
/// The `vector id`  argument is an identifier to the vector string (a number tag
/// that defines the list of possible sensitization combinations in a cell).
/// The vector id value must be an integer greater than or equal to zero and
/// unique among all vectors in the current `sensitization` group. It is recommended
/// that you start numbering from 0 or 1.
///
/// `vector string`
///
/// The `vector string` argument represents a pin transition state. The string consists
/// of the following transition status values: 0, 1, X, and Z where each character is separated by a space.
/// The number of elements in the vector string must equal the number of arguments in `pin_names`.
///
/// The `vector` attribute can also be declared as:
///
/// `vector (positive_integer, "{0|1|X|Z} [0|1|X|Z]…");`
///
/// ## Syntax
///
/// `vector (integer, string);`
///
/// ## Example
/// ``` text
/// sensitization(sensitization_nand2) {
///   pin_names ( IN1, IN2, OUT1 );
///   vector ( 1, "0 0 1" );
///   vector ( 2, "0 1 1" );
///   vector ( 3, "1 0 1" );
///   vector ( 4, "1 1 0" );
/// }
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=89.5&end=89.29
/// ">Reference</a>
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SensitizationVector {
  id: usize,
  states: Vec<logic::Static>,
}

impl ComplexAttri for SensitizationVector {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    use nom::{
      branch::alt,
      character::complete::{char, space1},
      combinator::map,
      multi::separated_list1,
    };
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::unquote_usize,
      ast::parser::unquote_f(separated_list1(
        space1,
        alt((
          map(char('0'), |_| logic::Static::L),
          map(char('1'), |_| logic::Static::H),
          map(char('X'), |_| logic::Static::X),
          map(char('Z'), |_| logic::Static::Z),
        )),
      )),
      |id, states| Self { id, states },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(self.id)?;
    f.write_str(", ")?;
    ast::join_fmt(
      self.states.iter(),
      f,
      |state, ff| {
        write!(
          ff,
          "{}",
          match state {
            logic::Static::Z => "Z",
            logic::Static::X => "X",
            logic::Static::H => "1",
            logic::Static::L => "0",
          }
        )
      },
      " ",
    )
  }
}

#[cfg(test)]
mod test_sensitization {
  use super::*;

  #[test]
  fn sensitization() {
    let sense = ast::test_parse_fmt::<Sensitization>(
      r#"(sensitization_nand2) {
        pin_names ( IN1, IN2, OUT1 );
        vector ( 1, "0 0 1" );
        vector ( 2, "0 X 1" );
        vector ( 3, "Z 0 1" );
        vector ( 4, "1 1 0" );
      }"#,
      r#"
liberty_db::library::items::Sensitization (sensitization_nand2) {
| pin_names (IN1, IN2, OUT1);
| vector (1, "0 0 1");
| vector (2, "0 X 1");
| vector (3, "Z 0 1");
| vector (4, "1 1 0");
}"#,
    );
    assert_eq!(
      sense.vector,
      vec![
        SensitizationVector {
          id: 1,
          states: vec![logic::Static::L, logic::Static::L, logic::Static::H,]
        },
        SensitizationVector {
          id: 2,
          states: vec![logic::Static::L, logic::Static::X, logic::Static::H,]
        },
        SensitizationVector {
          id: 3,
          states: vec![logic::Static::Z, logic::Static::L, logic::Static::H,]
        },
        SensitizationVector {
          id: 4,
          states: vec![logic::Static::H, logic::Static::H, logic::Static::L,]
        }
      ]
    );
    let sense1 = ast::test_parse_fmt::<Sensitization>(
      r#"(sensitization_nand2) {
        vector ( 1, "0 0 1" );
        vector ( 2, "0 X 9" );
        vector ( 3, "Z 0 1" );
        vector ( 4, "1 1 0" );
      }"#,
      r#"
liberty_db::library::items::Sensitization (sensitization_nand2) {
| vector (1, "0 0 1");
| vector (3, "Z 0 1");
| vector (4, "1 1 0");
| vector (2, "0 X 9"); /* user defined attribute */
}"#,
    );
    assert!(sense1.attributes.len() == 1);
  }
}
impl GroupFn for Sensitization {}

/// Use the `voltage_map`  attribute to associate a voltage name
/// with relative voltage values referenced by the cell-level `pg_pin`  groups.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.15&end=39.16
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoltageMap {
  /// name
  #[id(borrow = "&str")]
  #[size = 8]
  pub name: ArcStr,
  /// voltage
  pub voltage: NotNan<f64>,
}
impl ComplexAttri for VoltageMap {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::unquote_arcstr,
      ast::parser::unquote_float,
      |name, voltage| Self { name, voltage },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}, ", self.name)?;
    f.write_float(self.voltage.into_inner())
  }
}

/// An `input_voltage`  group is defined in the library  group to designate
/// a set of input voltage ranges for your cells.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=61.32&end=61.33
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputVoltage {
  /// name
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The maximum input voltage for which the input to the core is guaranteed to be a logic 0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.7&end=62.8
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub vil: Formula,
  /// The minimum input voltage for which the input to the core is guaranteed to be a logic 1
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.10&end=62.11
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub vih: Formula,
  /// The minimum acceptable input voltage.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.13&end=62.13
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub vimin: Formula,
  /// The maximum acceptable input voltage.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.15&end=62.16
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub vimax: Formula,
}
impl GroupFn for InputVoltage {}

/// You define an `output_voltage` group in the `library` group to designate a set of output
/// voltage level ranges to drive output cells.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=75.22&end=75.23
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputVoltage {
  /// name
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The maximum output voltage generated to represent a logic 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=75.45&end=75.46
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub vol: Formula,
  /// The minimum output voltage generated to represent a logic 1.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=76.3&end=76.4
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub voh: Formula,
  /// The minimum output voltage the pad can generate.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=76.5&end=76.6
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub vomin: Formula,
  /// The maximum output voltage the pad can generate.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=76.7&end=76.8
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub vomax: Formula,
}
impl GroupFn for OutputVoltage {}

/// Use the `delay_model`  attribute to specify which delay model
/// to use in the delay calculations.
///
/// The `delay_model`  attribute must be the first attribute in the library
/// if a technology attribute is not present.
/// Otherwise, it should follow the technology attribute.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=24.3&end=24.6
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq, Default)]
#[derive(Ord, PartialOrd)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DelayModel {
  /// `table_lookup`
  #[default]
  #[token("table_lookup")]
  TableLookup,
}
impl SimpleAttri for DelayModel {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

/// Use this group to define operating conditions;
///
/// that is, `process`, `voltage`, and `temperature`.
/// You define an `operating_conditions`  group at the library-level, as shown here:
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.3&end=72.4
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OperatingConditions {
  /// name
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// An optional attribute, you can use calc_mode  to specify an associated process mode.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.28&end=72.28
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub calc_mode: Option<ArcStr>,
  /// Use this optional attribute to specify values for up to five user-defined variables.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.36&end=72.37
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub parameteri: Option<NotNan<f64>>,
  /// Use the `process`  attribute to specify a scaling factor to account for variations in the outcome of the actual semiconductor manufacturing steps.
  ///
  /// A floating-point number from 0 through 100.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.44+73.7&end=72.45+73.8
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub process: NotNan<f64>,
  /// Use the process_label  attribute to specify the name of the current process.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.9&end=73.10
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub process_label: Option<ArcStr>,
  /// Use the `temperature`  attribute to specify the ambient temperature in which the design is to operate.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.15&end=73.16
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub temperature: NotNan<f64>,
  /// Use the `tree_type`  attribute to specify the environment interconnect model.
  ///
  /// Valid values are `best_case_tree`, `balanced_tree`, and `worst_case_tree`.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.24+73.30&end=73.25+73.31
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub tree_type: Option<TreeType>,
  /// Use the `voltage`  attribute to specify the operating voltage of the design; typically 5 volts for a CMOS library.
  ///
  /// A floating-point number from 0 through 1000, representing the absolute value of the actual voltage
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.32+74.3&end=73.33+74.4
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  #[default = "unsafe { NotNan::<f64>::new_unchecked(5.0) }"]
  pub voltage: NotNan<f64>,
}
impl GroupFn for OperatingConditions {}

/// You can define one or more `fpga_isd`  groups at the library level
/// to specify the drive current, I/O voltages, and slew rates for FPGA parts and cells
///
/// When you specify more than one `fpga_isd`  group, you **must** also define
/// the library-level `default_fpga_isd`  attribute to specify which `fpga_isd`
/// group is the default
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=63.22+63.25&end=63.23+63.27
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FpgaIsd {
  /// name
  #[size = 8]
  #[liberty(name)]
  #[id(borrow = "&str")]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The `drive`  attribute is optional and specifies the output current of the FPGA part or the FPGA cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=64.7&end=64.8
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub drive: ArcStr,
  /// The `io_type`  attribute is required and specifies the input or output voltage of the FPGA part or the FPGA cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=64.17&end=64.18
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub io_type: ArcStr,
  /// The `slew`  attribute is optional and specifies whether the slew of the FPGA part or the FPGA cell is FAST or SLOW.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=64.27&end=64.28
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub slew: Option<FPGASlew>,
}
impl GroupFn for FpgaIsd {}

/// The `slew`  attribute is optional and specifies whether the slew of the FPGA part or the FPGA cell is FAST or SLOW.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=64.27&end=64.28
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FPGASlew {
  /// `FAST`
  #[token("FAST")]
  FAST,
  /// `SLOW`
  #[token("SLOW")]
  SLOW,
}
impl SimpleAttri for FPGASlew {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

/// Use the `tree_type`  attribute to specify the environment interconnect model.
///
/// Valid values are `best_case_tree`, `balanced_tree`, and `worst_case_tree`.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.24+73.30&end=73.25+73.31
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TreeType {
  /// `best_case_tree`
  #[token("best_case_tree")]
  BestCaseTree,
  /// `balanced_tree`
  #[token("balanced_tree")]
  BalancedTree,
  /// `worst_case_tree`
  #[token("worst_case_tree")]
  WorstCaseTree,
}
impl SimpleAttri for TreeType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

/// Use this attribute to define new, temporary, or user-defined attributes
/// for use in symbol and technology libraries.
///
/// You can use either a space or a comma to separate the arguments.
/// The following example shows how to define a new string attribute called `bork`,
/// which is valid in a `pin`  group:
///
/// Example
/// ``` liberty
/// define ("bork", "pin", "string") ;
/// ```
/// You give the new library attribute a value by using the simple attribute syntax:
/// ``` liberty
/// bork : "nimo" ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.5&end=36.21
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Define {
  /// The name of the attribute you are creating.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.10&end=36.11
  /// ">Reference</a>
  #[id(borrow = "&str")]
  #[size = 8]
  pub attribute_name: ArcStr,
  /// The name of the group statement in which the attribute is to be used.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.12&end=36.13
  /// ">Reference</a>
  #[id(borrow = "&str")]
  #[size = 8]
  pub group_name: ArcStr,
  /// The type of the attribute that you are creating; valid values are Boolean, string, integer, or float
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.14&end=36.15
  /// ">Reference</a>
  #[size = 1]
  pub attribute_type: AttributeType,
}
/// The type of the attribute that you are creating; valid values are Boolean, string, integer, or float
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.14&end=36.15
/// ">Reference</a>
#[derive(Debug, Clone, Copy, Default)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttributeType {
  /// Boolean
  #[default]
  #[token("Boolean", "boolean")]
  Boolean,
  /// string
  #[token("string")]
  String,
  /// integer
  #[token("integer")]
  Integer,
  /// float
  #[token("float")]
  Float,
}
impl ComplexAttri for Define {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    ast::parser::complex3(
      i,
      &mut scope.line_num,
      ast::parser::unquote_arcstr,
      ast::parser::unquote_arcstr,
      <AttributeType as ast::NomParseTerm>::nom_parse,
      |attribute_name, group_name, attribute_type| {
        let define_id = ast::define_id(&scope.hasher, &group_name, &attribute_name);
        _ = scope
          .define_map
          .insert(define_id, DefinedType::Simple(attribute_type));
        Self { attribute_name, group_name, attribute_type }
      },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}, {}, {}", self.attribute_name, self.group_name, self.attribute_type)
  }
}

/// Use this special attribute to define new, temporary, or user-defined groups
/// for use in technology libraries.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.24&end=37.25
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefineGroup {
  /// The name of the user-defined group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.33&end=37.34
  /// ">Reference</a>
  #[id(borrow = "&str")]
  #[size = 8]
  pub group: ArcStr,
  /// The name of the group statement in which the attribute is to be used.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.35&end=37.36
  /// ">Reference</a>
  #[id(borrow = "&str")]
  #[size = 8]
  pub parent_name: ArcStr,
}
impl ComplexAttri for DefineGroup {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::unquote_arcstr,
      ast::parser::unquote_arcstr,
      |group, parent_name| {
        let define_id = ast::define_id(&scope.hasher, &parent_name, &group);
        _ = scope.define_map.insert(define_id, DefinedType::Group);
        Self { group, parent_name }
      },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}, {}", self.group, self.parent_name)
  }
}

/// The `define_cell_area`  attribute defines the area resources a `cell` uses,
/// such as the number of pad slots.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.23&end=36.24
/// ">Reference</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefineCellArea {
  /// A name of a resource type.
  /// You can associate more than one `area_name` attribute with each of the predefined resource types.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.28&end=36.29
  /// ">Reference</a>
  #[id(borrow = "&str")]
  #[size = 8]
  pub area_name: ArcStr,
  /// The resource type can be
  /// + `pad_slots`
  /// + `pad_input_driver_sites`
  /// + `pad_output_driver_sites`
  /// + `pad_driver_sites`
  ///
  /// Use the `pad_driver_sites` type when you do not need to discriminate between
  /// input and output pad driver sites.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.3&end=37.11
  /// ">Reference</a>
  pub resource_type: ResourceType,
}
/// The resource type can be
/// + `pad_slots`
/// + `pad_input_driver_sites`
/// + `pad_output_driver_sites`
/// + `pad_driver_sites`
///
/// Use the `pad_driver_sites` type when you do not need to discriminate between
/// input and output pad driver sites.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.3&end=37.11
/// ">Reference</a>
#[derive(Debug, Clone, Copy, Default)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ResourceType {
  /// `pad_slots`
  #[default]
  #[token("pad_slots")]
  PadSlots,
  /// `pad_input_driver_sites`
  #[token("pad_input_driver_sites")]
  PadInputDriverSites,
  /// `pad_output_driver_sites`
  #[token("pad_output_driver_sites")]
  PadOutputDriverSites,
  /// `pad_driver_sites`
  #[token("pad_driver_sites")]
  PadDriverSites,
}
impl ComplexAttri for DefineCellArea {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::unquote_arcstr,
      <ResourceType as ast::NomParseTerm>::nom_parse,
      |area_name, resource_type| Self { area_name, resource_type },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}, {}", self.area_name, self.resource_type)
  }
}

/// A `wire_load`  group is defined in a `library`  group, as follows.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
/// ">Reference</a>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(sort)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WireLoad {
  /// name
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// Use this attribute to specify area per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.31&end=94.32
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub area: NotNan<f64>,
  /// Use this attribute to specify capacitance per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=95.5&end=95.6
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub capacitance: NotNan<f64>,
  /// Use this attribute to specify wire resistance per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=95.15&end=95.16
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub resistance: NotNan<f64>,
  /// Use this attribute to characterize linear fanout length behavior
  /// beyond the scope of the longest length specified
  /// in the `fanout_length`  attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=95.25&end=95.26
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple)]
  pub slope: NotNan<f64>,
  /// Use this attribute to define values for fanout and length
  /// when you create the wire load manually.
  /// fanoutAn integer representing the total number of pins, minus one, on the net driven by the given output.lengthA floating-point number representing the estimated amount of metal that is statistically found on a network with the given number of pins.
  ///
  /// Examples
  /// ``` liberty
  /// library (example)  
  ///   ...
  ///   wire_load (small) {  
  ///     area : 0.0 ;  
  ///     capacitance : 1.0 ;  
  ///     resistance : 0.0 ;  
  ///     slope : 0.0 ;  
  ///     fanout_length (1,1.68) ;
  ///   }
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=96.3&end=96.34
  /// ">Reference</a>
  #[size = 64]
  #[liberty(complex(type = Set))]
  #[serde(serialize_with = "GroupSet::<FanoutLength>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<FanoutLength>::deserialize_with")]
  pub fanout_length: GroupSet<FanoutLength>,
}
impl GroupFn for WireLoad {}

/// Use this attribute to define values for fanout and length
/// when you create the wire load manually.
///
/// fanout: An integer representing the total number of pins, minus one, on the net driven by the given output.lengthA floating-point number representing the estimated amount of metal that is statistically found on a network with the given number of pins.
///
/// Examples
/// ``` liberty
/// library (example)  
///   ...
///   wire_load (small) {  
///     area : 0.0 ;  
///     capacitance : 1.0 ;  
///     resistance : 0.0 ;  
///     slope : 0.0 ;  
///     fanout_length (1,1.68) ;
///   }
/// }
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=96.3&end=96.34
/// ">Reference</a>
#[derive(Debug, Clone, Default, Copy)]
#[mut_set::derive::item(sort)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FanoutLength {
  /// An integer representing the total number of pins, minus one, on the net driven by the given output
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=96.19&end=96.20
  /// ">Reference</a>
  #[id]
  pub fanout: usize,
  /// A floating-point number representing the estimated amount of metal
  /// that is statistically found on a network with the given number of pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=96.22&end=96.23
  /// ">Reference</a>
  pub length: NotNan<f64>,
  /// + average_capacitance
  /// + standard_deviation
  /// + number_of_nets
  pub extra_info: Option<FanoutLengthExtra>,
}
#[derive(Debug, Clone, Default, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FanoutLengthExtra {
  /// average_capacitance
  pub average_capacitance: NotNan<f64>,
  /// standard_deviation
  pub standard_deviation: NotNan<f64>,
  /// number_of_nets
  pub number_of_nets: usize,
}
impl ComplexAttri for FanoutLength {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    ast::parser::complex5_opt(
      i,
      &mut scope.line_num,
      ast::parser::unquote_usize,
      ast::parser::unquote_float,
      ast::parser::unquote_float,
      ast::parser::unquote_float,
      ast::parser::unquote_usize,
      |fanout, length, extra_info| Self {
        fanout,
        length,
        extra_info: extra_info.map(
          |(average_capacitance, standard_deviation, number_of_nets)| FanoutLengthExtra {
            average_capacitance,
            standard_deviation,
            number_of_nets,
          },
        ),
      },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(self.fanout)?;
    f.write_str(", ")?;
    f.write_float(self.length.into_inner())?;
    if let Some(extra_info) = self.extra_info {
      f.write_str(", ")?;
      f.write_float(extra_info.average_capacitance.into_inner())?;
      f.write_str(", ")?;
      f.write_float(extra_info.standard_deviation.into_inner())?;
      f.write_str(", ")?;
      f.write_int(extra_info.number_of_nets)?;
    }
    Ok(())
  }
}

/// A `wire_load_selection`  group is defined in a `library`  group, as follows.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
/// ">Reference</a>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(sort)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WireLoadSection {
  /// name
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// Use this attribute to specify area per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.31&end=94.32
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex)]
  pub wire_load_from_area: (NotNan<f64>, NotNan<f64>, ArcStr),
}
impl GroupFn for WireLoadSection {}

/// The `base_curve_type` attribute specifies the type of base curve.
///
/// The valid values for `base_curve_type`  are `ccs_timing_half_curve`  and `ccs_half_curve`.
/// The `ccs_half_curve`  value allows you to model compact CCS power
/// and compact CCS timing data within the same `base_curves`  group.
/// You must specify `ccs_half_curve` before specifying `ccs_timing_half_curve`.
///
/// **Syntax**
/// ``` text
/// base_curve_type: enum (ccs_half_curve, ccs_timing_half_curve);
/// ```
/// **Example**
/// ``` text
/// base_curve_type : ccs_timing_half_curve ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=40.21+40.26&end=40.25+40.29
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd, Default)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BaseCurveType {
  /// The `ccs_half_curve`  value allows you to model compact CCS power
  #[token("ccs_half_curve")]
  #[default]
  CcsHalfCurve,
  /// You must specify `ccs_half_curve` before specifying `ccs_timing_half_curve`.
  #[token("ccs_timing_half_curve")]
  CcsTimingHalfCurve,
}

impl SimpleAttri for BaseCurveType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

/// The `base_curves`  group is a library-level group that contains
/// the detailed description of normalized base curves.
///
/// **Syntax**
/// ``` text
/// library (my_compact_ccs_lib) {
///   …
///   base_curves (base_curves_name) {
///     …
///   }
/// }
/// ```
/// **Example**
/// ``` text
/// library(my_lib) {
///   …
///   base_curves (ctbct1) {
///     …
///   }
/// }
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.32+40.2&end=39.33+40.15
/// ">Reference</a>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(sort)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BaseCurves {
  /// name
  #[size = 8]
  #[liberty(name)]
  #[id(borrow = "&str")]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// The `base_curve_type` attribute specifies the type of base curve.
  /// The valid values for `base_curve_type`  are `ccs_timing_half_curve`  and `ccs_half_curve`.
  /// The `ccs_half_curve`  value allows you to model compact CCS power
  /// and compact CCS timing data within the same `base_curves`  group.
  /// You must specify `ccs_half_curve` before specifying `ccs_timing_half_curve`.
  ///
  /// **Syntax**
  /// ``` text
  /// base_curve_type: enum (ccs_half_curve, ccs_timing_half_curve);
  /// ```
  /// **Example**
  /// ``` text
  /// base_curve_type : ccs_timing_half_curve ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=40.21+40.26&end=40.25+40.29
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple)]
  pub base_curve_type: BaseCurveType,
  #[size = 24]
  #[liberty(complex)]
  pub curve_x: Vec<NotNan<f64>>,
  #[size = 64]
  #[liberty(complex(type = Set))]
  #[serde(serialize_with = "GroupSet::<IdVector>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<IdVector>::deserialize_with")]
  pub curve_y: GroupSet<IdVector>,
}

impl GroupFn for BaseCurves {}

#[cfg(test)]
mod test {

  #[test]
  fn input_voltage() {
    let g = crate::ast::test_parse_fmt::<super::InputVoltage>(
      r#"(cmos_schmitt) {
        vil : 0.3 * VDD ;
        vih : 0.7 * VDD ;
        vimin : -0.5 ;
        vimax : VDD + 0.5 ;
      }"#,
      r#"
liberty_db::library::items::InputVoltage (cmos_schmitt) {
| vil : 0.3 * VDD ;
| vih : 0.7 * VDD ;
| vimin : -0.5 ;
| vimax : VDD + 0.5 ;
}"#,
    );
  }
  #[test]
  fn base_curves() {
    let g = crate::ast::test_parse_fmt::<super::BaseCurves>(
      r#"("nc_compact_ccs_curve_1") {
      base_curve_type : "ccs_timing_half_curve";
      curve_x("0.1000000, 0.2000000, 0.3000000, 0.4000000, 0.5000000, 0.6000000, 0.7000000, 0.8000000, 0.9000000");
      curve_y(1, \
        "0.9965371, 0.9930742, 0.9584770, 0.9165637, 0.8271961, 0.7425452, 0.6009643, 0.4459254, 0.2653107");
      curve_y(2, \
        "0.9887274, 0.9695129, 0.9443244, 0.9183546, 0.8705093, 0.8062681, 0.6984753, 0.5213233, 0.2657268");
      curve_y(3, \
        "0.9895478, 0.9774914, 0.9389569, 0.8934003, 0.8125975, 0.7144581, 0.5786802, 0.4298566, 0.2542494");
      curve_y(4, \
        "0.9944934, 0.9784088, 0.9620733, 0.9304195, 0.8888662, 0.8329558, 0.7240709, 0.5580780, 0.3037784");
      curve_y(5, \
        "0.9922672, 0.9664605, 0.9307680, 0.8888898, 0.8146837, 0.7076250, 0.5811826, 0.4366006, 0.2619239");   
    }"#,
      r#"
liberty_db::library::items::BaseCurves (nc_compact_ccs_curve_1) {
| base_curve_type : ccs_timing_half_curve;
| curve_x ("0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9");
| curve_y (1, \
| | "0.9965371, 0.9930742, 0.958477, 0.9165637, 0.8271961, 0.7425452, 0.6009643, 0.4459254, 0.2653107");
| curve_y (2, \
| | "0.9887274, 0.9695129, 0.9443244, 0.9183546, 0.8705093, 0.8062681, 0.6984753, 0.5213233, 0.2657268");
| curve_y (3, \
| | "0.9895478, 0.9774914, 0.9389569, 0.8934003, 0.8125975, 0.7144581, 0.5786802, 0.4298566, 0.2542494");
| curve_y (4, \
| | "0.9944934, 0.9784088, 0.9620733, 0.9304195, 0.8888662, 0.8329558, 0.7240709, 0.558078, 0.3037784");
| curve_y (5, \
| | "0.9922672, 0.9664605, 0.930768, 0.8888898, 0.8146837, 0.707625, 0.5811826, 0.4366006, 0.2619239");
}"#,
    );
  }
}
