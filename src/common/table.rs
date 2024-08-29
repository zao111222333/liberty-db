use crate::{
  ast::{
    Attributes, ComplexAttri, ComplexParseError, GroupComments, GroupFn, ParseScope,
    SimpleAttri,
  },
  ArcStr, GroupSet, NotNan,
};
use core::fmt::{self, Write};
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableLookUpMultiSegment {
  #[liberty(name)]
  #[id]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(simple)]
  #[id]
  segment: usize,
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_2: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_3: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_4: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Values,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DriverWaveform {
  #[id]
  #[liberty(name)]
  pub name: Option<ArcStr>,
  /// The `driver_waveform_name`  string attribute differentiates the driver waveform table
  /// from other driver waveform tables when multiple tables are defined.
  /// The cell-specific, rise-specific, and fall-specific driver waveform usage modeling
  /// depend on this attribute.
  ///
  /// The `driver_waveform_name`  attribute is optional.
  /// You can define a driver waveform table without the attribute, but there can be only one table in a library,
  /// and that table is regarded as the default driver waveform table for all cells in the library.
  /// If more than one table is defined without the attribute, the last table is used.
  /// The other tables are ignored and not stored in the library database file.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=71.24&end=71.31
  /// ">Reference</a>
  #[id]
  #[liberty(simple(type=Option))]
  pub driver_waveform_name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_2: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_3: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_4: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Values,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableLookUp2D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_2: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Values,
}

/// The `compact_lut_template`  group is a lookup table template used for compact CCS timing and power modeling
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=41.20&end=41.21
/// ">Reference</a>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompactLutTemplate {
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(simple(type=Option))]
  pub base_curves_group: Option<ArcStr>,
  /// The only valid values for the `variable_1`  and `variable_2`  attributes are `input_net_transition`  and `total_output_net_capacitance`.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=42.21&end=42.22
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub variable_1: Option<VariableTypeCompactLutTemplateIndex12>,
  /// The `index_1`  and `index_2`  attributes are required.
  /// The `index_1`  and `index_2`  attributes define the
  /// `input_net_transition`  and `total_output_net_capacitance`  values.
  /// The index value for `input_net_transition`  or `total_output_net_capacitance`  
  /// is a floating-point number.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=43.7&end=43.10
  /// ">Reference</a>
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  /// The only valid values for the `variable_1`  and `variable_2`  attributes are `input_net_transition`  and `total_output_net_capacitance`.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=42.21&end=42.22
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub variable_2: Option<VariableTypeCompactLutTemplateIndex12>,
  /// The `index_1`  and `index_2`  attributes are required.
  /// The `index_1`  and `index_2`  attributes define the
  /// `input_net_transition`  and `total_output_net_capacitance`  values.
  /// The index value for `input_net_transition`  or `total_output_net_capacitance`  
  /// is a floating-point number.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=43.7&end=43.10
  /// ">Reference</a>
  #[liberty(complex)]
  pub index_2: Vec<NotNan<f64>>,
  /// The string values in `index_3`  are determined by the `base_curve_type` value
  /// in the `base_curve`  group. When `ccs_timing_half_curve` is the
  /// `base_curve_type`  value, the following six string values (parameters)
  /// should be defined: `init_current`, `peak_current`, `peak_voltage`, `peak_time`, `left_id`, `right_id`;
  /// their order is not fixed.
  #[liberty(simple(type = Option))]
  pub variable_3: Option<VariableTypeCompactLutTemplateIndex3>,
  /// The string values in `index_3`  are determined by the `base_curve_type` value
  /// in the `base_curve`  group. When `ccs_timing_half_curve` is the
  /// `base_curve_type`  value, the following six string values (parameters)
  /// should be defined: `init_current`, `peak_current`, `peak_voltage`, `peak_time`, `left_id`, `right_id`;
  /// their order is not fixed.
  ///
  /// More than six parameters are allowed if a more robust syntax is required
  /// or for circumstances where more parameters are needed to describe the original data.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=43.18+43.22&end=43.21+43.23
  /// ">Reference</a>
  #[liberty(complex)]
  pub index_3: Vec<ArcStr>,
}

impl GroupFn for CompactLutTemplate {
  #[inline]
  fn post_parse_process(&mut self, scope: &mut ParseScope) {
    let l = self.index_3.len();
    if l != 0 {
      scope.compact_lut_template_index3_len = l;
    }
  }
}

/// The only valid values for the `variable_1`  and `variable_2`  attributes are `input_net_transition`  and `total_output_net_capacitance`.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=42.21&end=42.22
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(strum_macros::Display, strum_macros::EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum VariableTypeCompactLutTemplateIndex12 {
  #[strum(serialize = "input_net_transition")]
  InputNetTransition,
  #[strum(serialize = "total_output_net_capacitance")]
  TotalOutputNetCapacitance,
}

impl SimpleAttri for VariableTypeCompactLutTemplateIndex12 {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

/// The only legal string value for the `variable_3`  attribute is `curve_parameters`.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=42.30&end=42.31
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(strum_macros::Display, strum_macros::EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum VariableTypeCompactLutTemplateIndex3 {
  #[strum(serialize = "curve_parameters")]
  CurveParameters,
}

impl SimpleAttri for VariableTypeCompactLutTemplateIndex3 {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector3D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[id]
  #[liberty(complex)]
  pub index_1: NotNan<f64>,
  #[id]
  #[liberty(complex)]
  pub index_2: NotNan<f64>,
  #[liberty(complex)]
  pub index_3: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Vec<NotNan<f64>>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReferenceTimeVector3D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[id]
  #[liberty(simple)]
  pub reference_time: NotNan<f64>,
  #[id]
  #[liberty(complex)]
  pub index_1: NotNan<f64>,
  #[id]
  #[liberty(complex)]
  pub index_2: NotNan<f64>,
  #[liberty(complex)]
  pub index_3: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Vec<NotNan<f64>>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector4D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[id]
  #[liberty(complex)]
  pub index_1: NotNan<f64>,
  #[id]
  #[liberty(complex)]
  pub index_2: NotNan<f64>,
  #[id]
  #[liberty(complex)]
  pub index_3: NotNan<f64>,
  #[liberty(complex)]
  pub index_4: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Vec<NotNan<f64>>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector3DGrpup {
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(group(type = Set))]
  pub vector: GroupSet<Vector3D>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReferenceTimeVector3DGrpup {
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(group(type = Set))]
  pub vector: GroupSet<ReferenceTimeVector3D>,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector4DGrpup {
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(group(type = Set))]
  pub vector: GroupSet<Vector4D>,
}

impl GroupFn for Vector3DGrpup {}
impl GroupFn for Vector4DGrpup {}
impl GroupFn for ReferenceTimeVector3D {}
impl GroupFn for ReferenceTimeVector3DGrpup {}
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableLookUp3D {
  // TODO: unit
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_2: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_3: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Values,
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableLookUp1D {
  // TODO: unit
  unit: (),
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub values: Vec<NotNan<f64>>,
}
impl GroupFn for TableLookUp1D {}

/// The `compact_ccs_rise`  and `compact_ccs_fall`  groups define the compact CCS timing data in the timing arc.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=352.40&end=352.41
/// ">Reference-Definition</a>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompactCcsTable {
  // TODO: unit
  unit: (),
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(simple)]
  pub base_curves_group: ArcStr,
  #[liberty(complex)]
  pub values: Values,
}
impl GroupFn for CompactCcsTable {
  #[inline]
  #[allow(clippy::integer_division)]
  #[allow(clippy::arithmetic_side_effects)]
  #[allow(clippy::integer_division_remainder_used)]
  fn post_parse_process(&mut self, scope: &mut ParseScope) {
    if scope.compact_lut_template_index3_len == 0 {
      scope.compact_lut_template_index3_len = 1;
    }
    // if scope.compact_lut_template_index3_len == 0 {
    //   self.values.size2 = 1;
    //   self.values.size1 = self.values.inner.len();
    // } else {
    self.values.size1 = scope.compact_lut_template_index3_len;
    self.values.size2 = self.values.inner.len() / scope.compact_lut_template_index3_len;
    // }
  }
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableLookUp {
  // TODO: unit
  unit: (),
  #[id]
  #[liberty(name)]
  name: Option<ArcStr>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_2: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_3: Vec<NotNan<f64>>,
  #[liberty(complex)]
  pub index_4: Vec<NotNan<f64>>,
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
  fn post_parse_process(&mut self, _scope: &mut ParseScope) {
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
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Values {
  pub size1: usize,
  pub size2: usize,
  pub inner: Vec<NotNan<f64>>,
}

impl ComplexAttri for Values {
  #[inline]
  fn parse(v: &Vec<&str>, scope: &mut ParseScope) -> Result<Self, ComplexParseError> {
    Ok(Self {
      size1: 0,
      size2: 0,
      inner: <Vec<NotNan<f64>> as ComplexAttri>::parse(v, scope)?,
    })
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.inner.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let indent = f.indentation();
    let mut buffer = ryu::Buffer::new();
    let mut iter = self.inner.chunks(self.size1);
    if let Some(v) = iter.next() {
      crate::ast::join_fmt(
        v.iter(),
        f,
        |float, ff| write!(ff, "{}", buffer.format(Into::<f64>::into(*float))),
        ", ",
      )?;
    }
    while let Some(v) = iter.next() {
      write!(f, ", \\\n{indent}")?;
      crate::ast::join_fmt(
        v.iter(),
        f,
        |float, ff| write!(ff, "{}", buffer.format(Into::<f64>::into(*float))),
        ", ",
      )?;
    }
    Ok(())
  }
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TableTemple {
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(simple(type=Option))]
  pub variable_1: Option<Variable>,
  #[liberty(simple(type=Option))]
  pub variable_2: Option<Variable>,
  #[liberty(simple(type=Option))]
  pub variable_3: Option<Variable>,
  #[liberty(simple(type=Option))]
  pub variable_4: Option<Variable>,
  #[liberty(complex(type=Option))]
  pub index_1: Option<Vec<NotNan<f64>>>,
  #[liberty(complex(type=Option))]
  pub index_2: Option<Vec<NotNan<f64>>>,
  #[liberty(complex(type=Option))]
  pub index_3: Option<Vec<NotNan<f64>>>,
  #[liberty(complex(type=Option))]
  pub index_4: Option<Vec<NotNan<f64>>>,
}
impl GroupFn for TableTemple {}

/// In Timing Delay Tables:
///
/// Following are the values that you can assign for `variable_1`, `variable_2`, and `variable_3`  
/// to the templates for timing delay tables:
/// + `input_net_transition`
/// + `total_output_net_capacitance`
/// + `output_net_length`
/// + `output_net_wire_cap`
/// + `output_net_pin_cap`
/// + `related_out_total_output_net_capacitance`
/// + `related_out_output_net_length`
/// + `related_out_output_net_wire_cap`
/// + `related_out_output_net_pin_cap`
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
/// + `constrained_pin_transition`
/// + `related_pin_transition`
/// + `related_out_total_output_net_capacitance`
/// + `related_out_output_net_length`
/// + `related_out_output_net_wire_cap`
/// + `related_out_output_net_pin_cap`
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=67.21&end=67.28
/// ">Reference-Definition</a>
///
/// In Wire Delay Tables:
///
/// The following is the value set that you can assign for `variable_1`, `variable_2`, and `variable_3`  
/// to the templates for wire delay tables:
/// + `fanout_number`
/// + `fanout_pin_capacitance`
/// + `driver_slew`
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
/// + `output_transition`
/// + `rc_product`
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
///
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
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Variable {
  Time(TimeVariable),
  Voltage(VoltageVariable),
  Capacitance(CapacitanceVariable),
  RcProduct,
  Length(LengthVariable),
  Scalar(ScalarVariable),
}
impl SimpleAttri for Variable {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

impl core::str::FromStr for Variable {
  type Err = strum::ParseError;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "input_voltage" => Self::Voltage(VoltageVariable::InputVoltage),
      "output_voltage" => Self::Voltage(VoltageVariable::OutputVoltage),
      "input_noise_height" => Self::Voltage(VoltageVariable::InputNoiseHeight),
      "input_transition_time" => Self::Time(TimeVariable::InputTransitionTime),
      "input_net_transition" => Self::Time(TimeVariable::InputNetTransition),
      "constrained_pin_transition" => Self::Time(TimeVariable::ConstrainedPinTransition),
      "related_pin_transition" => Self::Time(TimeVariable::RelatedPinTransition),
      "driver_slew" => Self::Time(TimeVariable::DriverSlew),
      "output_transition" => Self::Time(TimeVariable::OutputTransition),
      "output_pin_transition" => Self::Time(TimeVariable::OutputPinTransition),
      "connect_delay" => Self::Time(TimeVariable::ConnectDelay),
      "input_noise_width" => Self::Time(TimeVariable::InputNoiseWidth),
      "time" => Self::Time(TimeVariable::Time),
      "total_output_net_capacitance" => {
        Self::Capacitance(CapacitanceVariable::TotalOutputNetCapacitance)
      }
      "output_net_wire_cap" => Self::Capacitance(CapacitanceVariable::OutputNetWireCap),
      "output_net_pin_cap" => Self::Capacitance(CapacitanceVariable::OutputNetPinCap),
      "related_out_total_output_net_capaci" => {
        Self::Capacitance(CapacitanceVariable::RelatedOutTotalOutputNetCapacitance)
      }
      "related_out_output_net_wire_cap" => {
        Self::Capacitance(CapacitanceVariable::RelatedOutOutputNetWireCap)
      }
      "related_out_output_net_pin_cap" => {
        Self::Capacitance(CapacitanceVariable::RelatedOutOutputNetPinCap)
      }
      "fanout_pin_capacitance" => {
        Self::Capacitance(CapacitanceVariable::FanoutPinCapacitance)
      }
      "output_net_length" => Self::Length(LengthVariable::OutputNetLength),
      "related_out_output_net_length" => {
        Self::Length(LengthVariable::RelatedOutOutputNetLength)
      }
      "fanout_number" => Self::Scalar(ScalarVariable::FanoutNumber),
      "normalized_voltage" => Self::Scalar(ScalarVariable::NormalizedVoltage),
      "rc_product" => Self::RcProduct,
      _ => {
        return Err(strum::ParseError::VariantNotFound);
      }
    })
  }
}

impl fmt::Display for Variable {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Time(v) => v.fmt(f),
      Self::Voltage(v) => v.fmt(f),
      Self::Capacitance(v) => v.fmt(f),
      Self::Length(v) => v.fmt(f),
      Self::Scalar(v) => v.fmt(f),
      Self::RcProduct => write!(f, "rc_product"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TimeVariable {
  /// `input_transition_time`
  #[strum(serialize = "input_transition_time")]
  InputTransitionTime,
  /// `input_net_transition`
  #[strum(serialize = "input_net_transition")]
  InputNetTransition,
  ///`constrained_pin_transition`
  #[strum(serialize = "constrained_pin_transition")]
  ConstrainedPinTransition,
  ///`related_pin_transition`
  #[strum(serialize = "related_pin_transition")]
  RelatedPinTransition,
  /// `driver_slew`
  #[strum(serialize = "driver_slew")]
  DriverSlew,
  /// `output_transition`
  #[strum(serialize = "output_transition")]
  OutputTransition,
  /// `output_pin_transition`
  #[strum(serialize = "output_pin_transition")]
  OutputPinTransition,
  /// `connect_delay`
  #[strum(serialize = "connect_delay")]
  ConnectDelay,
  /// `input_noise_width`
  #[strum(serialize = "input_noise_width")]
  InputNoiseWidth,
  /// `time`
  #[strum(serialize = "time")]
  Time,
}

#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum VoltageVariable {
  /// `input_voltage`
  #[strum(serialize = "input_voltage")]
  InputVoltage,
  /// `output_voltage`
  #[strum(serialize = "output_voltage")]
  OutputVoltage,
  /// `input_noise_height`
  #[strum(serialize = "input_noise_height")]
  InputNoiseHeight,
}

#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CapacitanceVariable {
  /// `total_output_net_capacitance`
  #[strum(serialize = "total_output_net_capacitance")]
  TotalOutputNetCapacitance,
  /// `output_net_wire_cap`
  #[strum(serialize = "output_net_wire_cap")]
  OutputNetWireCap,
  /// `output_net_pin_cap`
  #[strum(serialize = "output_net_pin_cap")]
  OutputNetPinCap,
  /// `related_out_total_output_net_capaci`
  #[strum(serialize = "related_out_total_output_net_capaci")]
  RelatedOutTotalOutputNetCapacitance,
  /// `related_out_output_net_wire_cap`
  #[strum(serialize = "related_out_output_net_wire_cap")]
  RelatedOutOutputNetWireCap,
  /// `related_out_output_net_pin_cap`
  #[strum(serialize = "related_out_output_net_pin_cap")]
  RelatedOutOutputNetPinCap,
  /// `fanout_pin_capacitance`
  #[strum(serialize = "fanout_pin_capacitance")]
  FanoutPinCapacitance,
}

#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LengthVariable {
  /// `output_net_length`
  #[strum(serialize = "output_net_length")]
  OutputNetLength,
  /// `related_out_output_net_length`
  #[strum(serialize = "related_out_output_net_length")]
  RelatedOutOutputNetLength,
}

#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ScalarVariable {
  /// `fanout_number`
  #[strum(serialize = "fanout_number")]
  FanoutNumber,
  /// The `normalized_voltage`  variable is specified under the
  /// `lu_table_template`  table to describe a collection of waveforms under
  /// various input slew values.
  /// For a given input slew in `index_1`  (for example, `index_1`[0] = 1.0 ns),
  /// the `index_2`  values are a set of points that represent how the voltage rises from 0 to VDD in a rise arc,
  /// or from VDD to 0 in a fall arc.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=65.38&end=65.41
  /// ">Reference-Definition</a>
  #[strum(serialize = "normalized_voltage")]
  NormalizedVoltage,
}
