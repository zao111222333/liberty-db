use crate::{
  ast::{AttributeList, ComplexAttri, GroupComments, GroupFn, SimpleAttri},
  GroupSet,
};

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct TableLookUpMultiSegment {
  #[liberty(name)]
  #[id]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple)]
  #[id]
  segment: usize,
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  #[liberty(complex)]
  pub index_2: Vec<f64>,
  #[liberty(complex)]
  pub index_3: Vec<f64>,
  #[liberty(complex)]
  pub index_4: Vec<f64>,
  #[liberty(complex)]
  pub values: Values,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct DriverWaveform {
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[id]
  #[liberty(simple(type=Option))]
  driver_waveform_name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  #[liberty(complex)]
  pub index_2: Vec<f64>,
  #[liberty(complex)]
  pub index_3: Vec<f64>,
  #[liberty(complex)]
  pub index_4: Vec<f64>,
  #[liberty(complex)]
  pub values: Values,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct TableLookUp2D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  #[liberty(complex)]
  pub index_2: Vec<f64>,
  #[liberty(complex)]
  pub values: Values,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct Vector3D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[id]
  #[liberty(complex)]
  pub index_1: ordered_float::OrderedFloat<f64>,
  #[id]
  #[liberty(complex)]
  pub index_2: ordered_float::OrderedFloat<f64>,
  #[liberty(complex)]
  pub index_3: Vec<f64>,
  #[liberty(complex)]
  pub values: Vec<f64>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct ReferenceTimeVector3D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[id]
  #[liberty(simple)]
  pub reference_time: ordered_float::OrderedFloat<f64>,
  #[id]
  #[liberty(complex)]
  pub index_1: ordered_float::OrderedFloat<f64>,
  #[id]
  #[liberty(complex)]
  pub index_2: ordered_float::OrderedFloat<f64>,
  #[liberty(complex)]
  pub index_3: Vec<f64>,
  #[liberty(complex)]
  pub values: Vec<f64>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct Vector4D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[id]
  #[liberty(complex)]
  pub index_1: ordered_float::OrderedFloat<f64>,
  #[id]
  #[liberty(complex)]
  pub index_2: ordered_float::OrderedFloat<f64>,
  #[id]
  #[liberty(complex)]
  pub index_3: ordered_float::OrderedFloat<f64>,
  #[liberty(complex)]
  pub index_4: Vec<f64>,
  #[liberty(complex)]
  pub values: Vec<f64>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct Vector3DGrpup {
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(group(type = Set))]
  pub vector: GroupSet<Vector3D>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct ReferenceTimeVector3DGrpup {
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(group(type = Set))]
  pub vector: GroupSet<ReferenceTimeVector3D>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct Vector4DGrpup {
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(group(type = Set))]
  pub vector: GroupSet<Vector4D>,
}

impl GroupFn for Vector3DGrpup {}
impl GroupFn for Vector4DGrpup {}
impl GroupFn for ReferenceTimeVector3D {}
impl GroupFn for ReferenceTimeVector3DGrpup {}
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct TableLookUp3D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  #[liberty(complex)]
  pub index_2: Vec<f64>,
  #[liberty(complex)]
  pub index_3: Vec<f64>,
  #[liberty(complex)]
  pub values: Values,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct TableLookUp1D {
  // TODO: unit
  unit: (),
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  #[liberty(complex)]
  pub values: Vec<f64>,
}
impl GroupFn for TableLookUp1D {}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct TableLookUp {
  // TODO: unit
  unit: (),
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(complex)]
  pub index_1: Vec<f64>,
  #[liberty(complex)]
  pub index_2: Vec<f64>,
  #[liberty(complex)]
  pub index_3: Vec<f64>,
  #[liberty(complex)]
  pub index_4: Vec<f64>,
  #[liberty(complex)]
  pub values: Values,
}
#[duplicate::duplicate_item(
  AllTypes;
  [TableLookUp];
  [TableLookUpMultiSegment];
  [TableLookUp2D];
  [TableLookUp3D];
  [DriverWaveform];
)]
impl GroupFn for AllTypes {
  #[inline]
  fn post_process(&mut self) {
    match (self.index_1.len(), self.index_2.len()) {
      (0, 0) => {
        self.values.size1 = self.values.inner.len();
      }
      (l1, 0) => {
        // 1-d table
        // fall_power (passive_power_template_8x1) {
        //   index_1 (0.0023, 0.0091, 0.0228, 0.0502, 0.105, 0.2145, 0.4335, 0.8715);
        //   values ("0.000137298, 0.00013122, 0.000128847, 0.000127135, 0.000126483, 0.000125385, 0.000125261, 0.000125493");
        // }
        self.values.size1 = l1;
        self.values.size2 = 1;
      }
      (l1, l2) => {
        self.values.size1 = l2;
        self.values.size2 = l1;
      }
    }
  }
}

impl GroupFn for Vector3D {}
impl GroupFn for Vector4D {}

#[derive(Debug, Default, Clone)]
pub struct Values {
  pub size1: usize,
  pub size2: usize,
  pub inner: Vec<f64>,
}

impl ComplexAttri for Values {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, crate::ast::ComplexParseError> {
    Ok(Self {
      size1: 0,
      size2: 0,
      inner: <Vec<f64> as ComplexAttri>::parse(v)?,
    })
  }
  #[inline]
  fn to_wrapper(&self) -> crate::ast::ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    self
      .inner
      .chunks(self.size1)
      .map(|v| {
        vec![itertools::Itertools::join(
          &mut v.iter().map(|f| buffer.format(*f).to_string()),
          ", ",
        )]
      })
      .collect()
  }
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
pub struct TableTemple {
  #[id]
  #[liberty(name)]
  pub name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  #[liberty(simple(type=Option))]
  pub variable_1: Option<Variable>,
  #[liberty(simple(type=Option))]
  pub variable_2: Option<Variable>,
  #[liberty(simple(type=Option))]
  pub variable_3: Option<Variable>,
  #[liberty(simple(type=Option))]
  pub variable_4: Option<Variable>,
  #[liberty(complex(type=Option))]
  pub index_1: Option<Vec<f64>>,
  #[liberty(complex(type=Option))]
  pub index_2: Option<Vec<f64>>,
  #[liberty(complex(type=Option))]
  pub index_3: Option<Vec<f64>>,
  #[liberty(complex(type=Option))]
  pub index_4: Option<Vec<f64>>,
}
impl GroupFn for TableTemple {}

/// In Timing Delay Tables:
///
/// Following are the values that you can assign for `variable_1`, `variable_2`, and `variable_3`  
/// to the templates for timing delay tables:
/// + input_net_transition
/// + total_output_net_capacitance
/// + output_net_length
/// + output_net_wire_cap
/// + output_net_pin_cap
/// + related_out_total_output_net_capacitance
/// + related_out_output_net_length
/// + related_out_output_net_wire_cap
/// + related_out_output_net_pin_cap
///
/// The values that you can assign to the variables of a table specifying timing delay
/// depend on whether the table is one-, two-, or three-dimensional.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=67.9&end=67.20
/// ">Reference-Definition</a>
///
/// In Constraint Tables:
///
/// You can assign the following values to the `variable_1`, `variable_2`, and `variable_3` variables
/// in the templates for constraint tables:
/// + constrained_pin_transition
/// + related_pin_transition
/// + related_out_total_output_net_capacitance
/// + related_out_output_net_length
/// + related_out_output_net_wire_cap
/// + related_out_output_net_pin_cap
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=67.21&end=67.28
/// ">Reference-Definition</a>
///
/// In Wire Delay Tables:
///
/// The following is the value set that you can assign for `variable_1`, `variable_2`, and `variable_3`  
/// to the templates for wire delay tables:
/// + fanout_number
/// + fanout_pin_capacitance
/// + driver_slew
///
/// The values that you can assign to the variables of a table specifying wire delay depends on whether the table is one-, two-, or three-dimensional.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=67.29&end=67.34
/// ">Reference-Definition</a>
///
/// In Net Delay Tables:
///
/// The following is the value set that you can assign for `variable_1`  and `variable_2`  
/// to the templates for net delay tables:
/// + output_transition
/// + rc_product
///
/// The values that you can assign to the variables of a table specifying net delay depend on whether the table is one- or two-dimensional.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=68.2+67.35&end=68.3+67.38
/// ">Reference-Definition</a>
///
/// In Degradation Tables:
///
/// The following values apply only to templates for transition time degradation tables:
/// + `variable_1` : `output_pin_transition` | `connect_delay` ;
/// + `variable_2` : `output_pin_transition` | `connect_delay` ;
///
/// The cell degradation table template allows only one-dimensional tables:
/// + `variable_1` : `input_net_transition`
///
/// The following rules show the relationship between the variables and indexes:
/// + If you have `variable_1`, you must have `index_1`.
/// + If you have `variable_1`  and `variable_2`, you must have `index_1`  and `index_2`.
/// + If you have `variable_1`, `variable_2`, and `variable_3`, you must have `index_1`, `index_2`, and `index_3`.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=68.4&end=68.16
/// ">Reference-Definition</a>
///
/// Specify the following attributes in the vector group:
///
/// + The `index_1` attribute lists the `input_noise_height` values in library voltage units.
/// + The `index_2` attribute lists the `input_noise_width` values in library time units.
/// + The `index_3` attribute lists the `total_output_net_capacitance` values in library capacitance units.
/// + The `index_4` attribute lists the sampling `time` values in library time units.
///
/// The values attribute lists the voltage values, in library voltage units, that are measured at the channel-connecting block output node.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=290.11&end=290.17
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
pub enum Variable {
  /// input_voltage
  #[strum(serialize = "input_voltage")]
  InputVoltage,
  /// output_voltage
  #[strum(serialize = "output_voltage")]
  OutputVoltage,
  /// input_transition_time
  #[strum(serialize = "input_transition_time")]
  InputTransitionTime,
  /// input_net_transition
  #[strum(serialize = "input_net_transition")]
  InputNetTransition,
  /// total_output_net_capacitance
  #[strum(serialize = "total_output_net_capacitance")]
  TotalOutputNetCapacitance,
  /// output_net_length
  #[strum(serialize = "output_net_length")]
  OutputNetLength,
  /// output_net_wire_cap
  #[strum(serialize = "output_net_wire_cap")]
  OutputNetWireCap,
  /// output_net_pin_cap
  #[strum(serialize = "output_net_pin_cap")]
  OutputNetPinCap,
  /// related_out_total_output_net_capaci
  #[strum(serialize = "related_out_total_output_net_capaci")]
  RelatedOutTotalOutputNetCapacitance,
  /// related_out_output_net_length
  #[strum(serialize = "related_out_output_net_length")]
  RelatedOutOutputNetLength,
  /// related_out_output_net_wire_cap
  #[strum(serialize = "related_out_output_net_wire_cap")]
  RelatedOutOutputNetWireCap,
  /// related_out_output_net_pin_cap
  #[strum(serialize = "related_out_output_net_pin_cap")]
  RelatedOutOutputNetPinCap,
  ///constrained_pin_transition
  #[strum(serialize = "constrained_pin_transition")]
  ConstrainedPinTransition,
  ///related_pin_transition
  #[strum(serialize = "related_pin_transition")]
  RelatedPinTransition,
  /// fanout_number
  #[strum(serialize = "fanout_number")]
  FanoutNumber,
  /// fanout_pin_capacitance
  #[strum(serialize = "fanout_pin_capacitance")]
  FanoutPinCapacitance,
  /// driver_slew
  #[strum(serialize = "driver_slew")]
  DriverSlew,
  /// output_transition
  #[strum(serialize = "output_transition")]
  OutputTransition,
  /// rc_product
  #[strum(serialize = "rc_product")]
  RcProduct,
  /// output_pin_transition
  #[strum(serialize = "output_pin_transition")]
  OutputPinTransition,
  /// connect_delay
  #[strum(serialize = "connect_delay")]
  ConnectDelay,
  /// input_noise_height
  #[strum(serialize = "input_noise_height")]
  InputNoiseHeight,
  /// input_noise_width
  #[strum(serialize = "input_noise_width")]
  InputNoiseWidth,
  /// time
  #[strum(serialize = "time")]
  Time,
}
impl SimpleAttri for Variable {}
