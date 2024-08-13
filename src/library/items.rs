//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
//! </script>
// use std::ops::DerefMut;

use itertools::Itertools;

use crate::{
  ast::{
    AttributeList, ComplexAttri, ComplexParseError, ComplexWrapper, GroupComments,
    GroupFn, SimpleAttri,
  },
  expression::logic,
  ArcStr, GroupSet,
};

/// The `sensitization` group defined at the library level describes
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
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sensitization {
  /// name
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
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
#[derive(Debug, Clone, Default, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SensitizationVector {
  id: usize,
  states: Vec<logic::Static>,
}

impl ComplexAttri for SensitizationVector {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let id: usize = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Int(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let states = match i.next() {
      Some(&s) => match s
        .split_ascii_whitespace()
        .map(|term| term.parse::<logic::Static>())
        .collect::<Result<Vec<logic::Static>, _>>()
      {
        Ok(states) => states,
        Err(_) => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { id, states })
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = itoa::Buffer::new();
    vec![vec![
      ArcStr::from(buffer.format(self.id)),
      self
        .states
        .iter()
        .map(|state| match state {
          logic::Static::UnInit(logic::UnInit::HighImpedance) => "Z",
          logic::Static::UnInit(logic::UnInit::Unknown(_)) => "X",
          logic::Static::Level(logic::Level::High) => "1",
          logic::Static::Level(logic::Level::Low) => "0",
        })
        .join(" ")
        .into(),
    ]]
  }
}

#[test]
fn sensitization() {
  let (sense, _) = crate::ast::test_parse_group::<Sensitization>(
    r#"(sensitization_nand2) {
        pin_names ( IN1, IN2, OUT1 );
        vector ( 1, "0 0 1" );
        vector ( 2, "0 X 1" );
        vector ( 3, "Z 0 1" );
        vector ( 4, "1 1 0" );
      }"#,
  );
  assert_eq!(
    sense.vector,
    vec![
      SensitizationVector {
        id: 1,
        states: vec![
          logic::Static::Level(logic::Level::Low),
          logic::Static::Level(logic::Level::Low),
          logic::Static::Level(logic::Level::High),
        ]
      },
      SensitizationVector {
        id: 2,
        states: vec![
          logic::Static::Level(logic::Level::Low),
          logic::Static::UnInit(logic::UnInit::Unknown(None)),
          logic::Static::Level(logic::Level::High),
        ]
      },
      SensitizationVector {
        id: 3,
        states: vec![
          logic::Static::UnInit(logic::UnInit::HighImpedance),
          logic::Static::Level(logic::Level::Low),
          logic::Static::Level(logic::Level::High),
        ]
      },
      SensitizationVector {
        id: 4,
        states: vec![
          logic::Static::Level(logic::Level::High),
          logic::Static::Level(logic::Level::High),
          logic::Static::Level(logic::Level::Low),
        ]
      }
    ]
  );
  let (sense1, _) = crate::ast::test_parse_group::<Sensitization>(
    r#"(sensitization_nand2) {
        vector ( 1, "0 0 1" );
        vector ( 2, "0 X 9" );
        vector ( 3, "Z 0 1" );
        vector ( 4, "1 1 0" );
      }"#,
  );
  assert!(sense1.undefined.len() == 1);
}

impl GroupFn for Sensitization {}

/// Use the `voltage_map`  attribute to associate a voltage name
/// with relative voltage values referenced by the cell-level `pg_pin`  groups
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=39.15&end=39.16
/// ">Reference</a>
#[derive(Debug, Clone, Default)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VoltageMap {
  /// name
  #[id]
  pub name: ArcStr,
  /// voltage
  pub voltage: f64,
}
impl ComplexAttri for VoltageMap {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let name = match i.next() {
      Some(&s) => ArcStr::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let voltage = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => {
          return Err(ComplexParseError::Float(
            ordered_float::ParseNotNanError::ParseFloatError(e),
          ))
        }
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { name, voltage })
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![self.name.clone(), ArcStr::from(buffer.format(self.voltage))]]
  }
}

/// An `input_voltage`  group is defined in the library  group to designate
/// a set of input voltage ranges for your cells.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=61.32&end=61.33
/// ">Reference</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputVoltage {
  /// name
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  /// The maximum input voltage for which the input to the core is guaranteed to be a logic 0
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.7&end=62.8
  /// ">Reference</a>
  #[liberty(simple)]
  pub vil: ArcStr,
  /// The minimum input voltage for which the input to the core is guaranteed to be a logic 1
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.10&end=62.11
  /// ">Reference</a>
  #[liberty(simple)]
  pub vih: ArcStr,
  /// The minimum acceptable input voltage.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.13&end=62.13
  /// ">Reference</a>
  #[liberty(simple)]
  pub vimin: ArcStr,
  /// The maximum acceptable input voltage.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=62.15&end=62.16
  /// ">Reference</a>
  #[liberty(simple)]
  pub vimax: ArcStr,
}
impl GroupFn for InputVoltage {}

/// You define an `output_voltage` group in the `library` group to designate a set of output
/// voltage level ranges to drive output cells.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=75.22&end=75.23
/// ">Reference</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutputVoltage {
  /// name
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  /// The maximum output voltage generated to represent a logic 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=75.45&end=75.46
  /// ">Reference</a>
  #[liberty(simple)]
  pub vol: ArcStr,
  /// The minimum output voltage generated to represent a logic 1.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=76.3&end=76.4
  /// ">Reference</a>
  #[liberty(simple)]
  pub voh: ArcStr,
  /// The minimum output voltage the pad can generate.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=76.5&end=76.6
  /// ">Reference</a>
  #[liberty(simple)]
  pub vomin: ArcStr,
  /// The maximum output voltage the pad can generate.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=76.7&end=76.8
  /// ">Reference</a>
  #[liberty(simple)]
  pub vomax: ArcStr,
}
impl GroupFn for OutputVoltage {}

/// Use the `delay_model`  attribute to specify which delay model
/// to use in the delay calculations.
/// The `delay_model`  attribute must be the first attribute in the library
/// if a technology attribute is not present.
/// Otherwise, it should follow the technology attribute.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=24.3&end=24.6
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq, Default)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DelayModel {
  ///     table_lookup
  #[default]
  #[strum(serialize = "table_lookup")]
  TableLookup,
}
impl SimpleAttri for DelayModel {}

/// Use this group to define operating conditions;
/// that is, `process`, `voltage`, and `temperature`.
/// You define an `operating_conditions`  group at the library-level, as shown here:
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.3&end=72.4
/// ">Reference</a>
#[derive(Debug, Clone, derivative::Derivative)]
#[derivative(Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone);
        derive(derivative::Derivative);
        derivative(Default);),
  attr_filter(derivative;)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OperatingConditions {
  /// name
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  /// An optional attribute, you can use calc_mode  to specify an associated process mode.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.28&end=72.28
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub calc_mode: Option<ArcStr>,
  /// Use this optional attribute to specify values for up to five user-defined variables.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.36&end=72.37
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub parameteri: Option<f64>,
  /// Use the `process`  attribute to specify a scaling factor to account for variations in the outcome of the actual semiconductor manufacturing steps.
  ///
  /// A floating-point number from 0 through 100.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=72.44+73.7&end=72.45+73.8
  /// ">Reference</a>
  #[liberty(simple)]
  pub process: f64,
  /// Use the process_label  attribute to specify the name of the current process.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.9&end=73.10
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub process_label: Option<ArcStr>,
  /// Use the `temperature`  attribute to specify the ambient temperature in which the design is to operate.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.15&end=73.16
  /// ">Reference</a>
  #[liberty(simple)]
  pub temperature: f64,
  /// Use the `tree_type`  attribute to specify the environment interconnect model.
  ///
  /// Valid values are `best_case_tree`, `balanced_tree`, and `worst_case_tree`.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.24+73.30&end=73.25+73.31
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub tree_type: Option<TreeType>,
  /// Use the `voltage`  attribute to specify the operating voltage of the design; typically 5 volts for a CMOS library.
  ///
  /// A floating-point number from 0 through 1000, representing the absolute value of the actual voltage
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.32+74.3&end=73.33+74.4
  /// ">Reference</a>
  #[liberty(simple)]
  #[derivative(Default(value = "5.0"))]
  pub voltage: f64,
}
impl GroupFn for OperatingConditions {}

/// Use the `tree_type`  attribute to specify the environment interconnect model.
///
/// Valid values are `best_case_tree`, `balanced_tree`, and `worst_case_tree`.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=73.24+73.30&end=73.25+73.31
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TreeType {
  /// best_case_tree
  #[strum(serialize = "best_case_tree")]
  BestCaseTree,
  /// balanced_tree
  #[strum(serialize = "balanced_tree")]
  BalancedTree,
  /// worst_case_tree
  #[strum(serialize = "worst_case_tree")]
  WorstCaseTree,
}
impl SimpleAttri for TreeType {}

/// Use this attribute to define new, temporary, or user-defined attributes
/// for use in symbol and technology libraries.
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
#[derive(Debug, Clone)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Define {
  /// The name of the attribute you are creating.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.10&end=36.11
  /// ">Reference</a>
  #[id]
  pub attribute_name: ArcStr,
  /// The name of the group statement in which the attribute is to be used.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.12&end=36.13
  /// ">Reference</a>
  #[id]
  pub group_name: ArcStr,
  /// The type of the attribute that you are creating; valid values are Boolean, string, integer, or float
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.14&end=36.15
  /// ">Reference</a>
  pub attribute_type: AttributeType,
}
/// The type of the attribute that you are creating; valid values are Boolean, string, integer, or float
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.14&end=36.15
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttributeType {
  /// Boolean
  #[strum(serialize = "Boolean", serialize = "boolean")]
  Boolean,
  /// string
  #[strum(serialize = "string")]
  String,
  /// integer
  #[strum(serialize = "integer")]
  Integer,
  /// float
  #[strum(serialize = "float")]
  Float,
}
impl ComplexAttri for Define {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let attribute_name = match i.next() {
      Some(&s) => ArcStr::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let group_name = match i.next() {
      Some(&s) => ArcStr::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let attribute_type = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(_) => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { attribute_name, group_name, attribute_type })
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![vec![
      self.attribute_name.clone(),
      self.group_name.clone(),
      self.attribute_type.to_string().into(),
    ]]
  }
}

/// The `define_cell_area`  attribute defines the area resources a `cell` uses,
/// such as the number of pad slots.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.23&end=36.24
/// ">Reference</a>
#[derive(Debug, Clone)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefineCellArea {
  /// A name of a resource type.
  /// You can associate more than one `area_name` attribute with each of the predefined resource types.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=36.28&end=36.29
  /// ">Reference</a>
  #[id]
  pub area_name: ArcStr,
  /// The resource type can be
  /// + pad_slots
  /// + pad_input_driver_sites
  /// + pad_output_driver_sites
  /// + pad_driver_sites
  ///
  /// Use the `pad_driver_sites` type when you do not need to discriminate between
  /// input and output pad driver sites.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.3&end=37.11
  /// ">Reference</a>
  pub resource_type: ResourceType,
}
/// The resource type can be
/// + pad_slots
/// + pad_input_driver_sites
/// + pad_output_driver_sites
/// + pad_driver_sites
///
/// Use the `pad_driver_sites` type when you do not need to discriminate between
/// input and output pad driver sites.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.3&end=37.11
/// ">Reference</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ResourceType {
  /// pad_slots
  #[strum(serialize = "pad_slots")]
  PadSlots,
  /// pad_input_driver_sites
  #[strum(serialize = "pad_input_driver_sites")]
  PadInputDriverSites,
  /// pad_output_driver_sites
  #[strum(serialize = "pad_output_driver_sites")]
  PadOutputDriverSites,
  /// pad_driver_sites
  #[strum(serialize = "pad_driver_sites")]
  PadDriverSites,
}
impl ComplexAttri for DefineCellArea {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let area_name = match i.next() {
      Some(&s) => ArcStr::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let resource_type = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(_) => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { area_name, resource_type })
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![vec![self.area_name.clone(), self.resource_type.to_string().into()]]
  }
}

/// Use this special attribute to define new, temporary, or user-defined groups
/// for use in technology libraries.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.24&end=37.25
/// ">Reference</a>
#[derive(Debug, Clone)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefineGroup {
  /// The name of the user-defined group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.33&end=37.34
  /// ">Reference</a>
  #[id]
  pub group: ArcStr,
  /// The name of the group statement in which the attribute is to be used.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=37.35&end=37.36
  /// ">Reference</a>
  #[id]
  pub parent_name: ArcStr,
}
impl ComplexAttri for DefineGroup {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let group = match i.next() {
      Some(&s) => ArcStr::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let parent_name = match i.next() {
      Some(&s) => ArcStr::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { group, parent_name })
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![vec![self.group.clone(), self.parent_name.clone()]]
  }
}

/// A `wire_load`  group is defined in a `library`  group, as follows.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
/// ">Reference</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WireLoad {
  /// name
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  /// Use this attribute to specify area per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.31&end=94.32
  /// ">Reference</a>
  #[liberty(simple)]
  pub area: f64,
  /// Use this attribute to specify capacitance per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=95.5&end=95.6
  /// ">Reference</a>
  #[liberty(simple)]
  pub capacitance: f64,
  /// Use this attribute to specify wire resistance per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=95.15&end=95.16
  /// ">Reference</a>
  #[liberty(simple)]
  pub resistance: f64,
  /// Use this attribute to characterize linear fanout length behavior
  /// beyond the scope of the longest length specified
  /// in the `fanout_length`  attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=95.25&end=95.26
  /// ">Reference</a>
  #[liberty(simple)]
  pub slope: f64,
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
  #[liberty(complex(type = Set))]
  pub fanout_length: GroupSet<FanoutLength>,
}
impl GroupFn for WireLoad {}

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
#[derive(Debug, Clone, Default, Copy)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default, Copy);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FanoutLength {
  /// An integer representing the total number of pins, minus one, on the net driven by the given output
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=96.19&end=96.20
  /// ">Reference</a>
  #[id]
  pub fanout: u32,
  /// A floating-point number representing the estimated amount of metal
  /// that is statistically found on a network with the given number of pins
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=96.22&end=96.23
  /// ">Reference</a>
  pub length: f64,
  /// average_capacitance
  pub average_capacitance: Option<f64>,
  /// standard_deviation
  pub standard_deviation: Option<f64>,
  /// number_of_nets
  pub number_of_nets: Option<u32>,
}
impl ComplexAttri for FanoutLength {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let fanout = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Int(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let length = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => {
          return Err(ComplexParseError::Float(
            ordered_float::ParseNotNanError::ParseFloatError(e),
          ))
        }
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let average_capacitance = i.next().and_then(|s| match s.parse() {
      Ok(f) => Some(f),
      Err(_) => None,
    });
    let standard_deviation = i.next().and_then(|s| match s.parse() {
      Ok(f) => Some(f),
      Err(_) => None,
    });
    let number_of_nets = i.next().and_then(|s| match s.parse() {
      Ok(f) => Some(f),
      Err(_) => None,
    });

    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self {
      fanout,
      length,
      average_capacitance,
      standard_deviation,
      number_of_nets,
    })
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer_f = ryu::Buffer::new();
    let mut buffer_i = itoa::Buffer::new();
    match (self.average_capacitance, self.standard_deviation, self.number_of_nets) {
      (Some(average_capacitance), Some(standard_deviation), Some(number_of_nets)) => {
        vec![vec![
          ArcStr::from(buffer_i.format(self.fanout)),
          ArcStr::from(buffer_f.format(self.length)),
          ArcStr::from(buffer_f.format(average_capacitance)),
          ArcStr::from(buffer_f.format(standard_deviation)),
          ArcStr::from(buffer_i.format(number_of_nets)),
        ]]
      }
      _ => {
        vec![vec![
          ArcStr::from(buffer_i.format(self.fanout)),
          ArcStr::from(buffer_f.format(self.length)),
        ]]
      }
    }
  }
}

/// A `wire_load_selection`  group is defined in a `library`  group, as follows.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.16&end=94.17
/// ">Reference</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WireLoadSection {
  /// name
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  /// Use this attribute to specify area per unit length of interconnect wire.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=94.31&end=94.32
  /// ">Reference</a>
  #[liberty(complex)]
  pub wire_load_from_area: (f64, f64, ArcStr),
}
impl GroupFn for WireLoadSection {}
