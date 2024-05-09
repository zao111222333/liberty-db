use std::{cmp::Ordering, collections::HashSet, fmt::Debug, rc::Rc};

use itertools::Itertools;
use strum_macros::{Display, EnumString};

use crate::{
  ast::{ComplexAttri, GroupComments, GroupFn, SimpleAttri},
  GroupSet,
};

/// The expression must conform to `OVI SDF 2.1 timing-check condition syntax`.
///
/// #### Example
/// ``` liberty
/// sdf_cond_end : "SIG_0 == 1’b1" ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =210.10
/// &end
/// =210.19
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.45
/// &end
/// =203.45
/// ">Reference-Instance</a>
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct SdfExpression {
  inner: String,
}
impl std::fmt::Display for SdfExpression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.inner, f)
  }
}
impl std::str::FromStr for SdfExpression {
  type Err = core::convert::Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self { inner: String::from_str(s)? })
  }
}
impl SimpleAttri for SdfExpression {}
/// The `sdf_edges` attribute defines the edge specification on both
/// the start pin and the end pin. The default is noedge.
///
/// #### Syntax
/// `sdf_edges : sdf_edge_type;`
///
/// `sdf_edge_type`: One of these four edge types: `noedge`, `start_edge`,
/// `end_edge`, or `both_edges`. The default is `noedge`.
///
/// #### Example
/// ``` liberty
/// sdf_edges : both_edges;
/// sdf_edges : start_edge ; /* edge specification on starting pin */
/// sdf_edges : end_edge ; /* edge specification on end pin */
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =210.31
/// &end
/// =211.2
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.47
/// &end
/// =203.47
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default, EnumString, Display)]
pub enum SdfEdgeType {
  #[default]
  #[strum(serialize = "noedge")]
  Noedge,
  #[strum(serialize = "start_edge")]
  StartEdge,
  #[strum(serialize = "end_edge")]
  EndEdge,
  #[strum(serialize = "both_edges")]
  BothEdges,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Display, EnumString)]
pub enum VariableType {
  #[strum(serialize = "input_net_transition")]
  InputNetTransition,
  #[strum(serialize = "normalized_voltage")]
  NormalizedVoltage,
  #[strum(serialize = "total_output_net_capacitance")]
  TotalOutputNetCapacitance,
  #[strum(serialize = "related_out_total_output_net_capacitance")]
  RelatedOutTotalOutputNetCapacitance,
  #[strum(serialize = "constrained_pin_transition")]
  ConstrainedPinTransition,
  #[strum(serialize = "fanout_number")]
  FanoutNumber,
  #[strum(serialize = "fanout_pin_capacitance")]
  FanoutPinCapacitance,
  #[strum(serialize = "driver_slew")]
  DriverSlew,
  #[strum(serialize = "input_transition_time")]
  InputTransitionTime,
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =38.48
/// &end
/// =39.24
/// ">Reference-Definition</a>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct Domain {
  #[liberty(name)]
  #[id]
  pub name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
  pub group_name: String,
  pub calc_mode: Option<String>,
  pub variable_1: Option<VariableType>,
  pub variable_2: Option<VariableType>,
  pub variable_3: Option<VariableType>,
  pub index_1: Vec<f64>,
  pub index_2: Vec<f64>,
  pub index_3: Vec<f64>,
}
impl GroupFn for Domain {}
/// sth. like "A B C" will save as set{A B C}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct WordSet {
  pub inner: HashSet<String>,
}
impl std::fmt::Display for WordSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self.inner.iter().join(" "), f)
  }
}
impl Ord for WordSet {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    if self.inner.is_subset(&other.inner) {
      Ordering::Less
    } else if self.inner.is_superset(&other.inner) {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  }
}

impl PartialOrd for WordSet {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self == other {
      Some(Ordering::Equal)
    } else if self.inner.is_subset(&other.inner) {
      Some(Ordering::Less)
    } else if self.inner.is_superset(&other.inner) {
      Some(Ordering::Greater)
    } else {
      None
    }
  }
}

impl SimpleAttri for WordSet {}
impl std::hash::Hash for WordSet {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    let mut sum = 0_u64;
    for item in &self.inner {
      let mut hasher = std::hash::DefaultHasher::new();
      item.hash(&mut hasher);
      sum = sum.wrapping_add(std::hash::Hasher::finish(&hasher));
    }
    state.write_u64(sum);
  }
}

impl std::str::FromStr for WordSet {
  type Err = std::fmt::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      inner: s.split(' ').map(ToString::to_string).collect(),
    })
  }
}

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct DummyGroup {
  #[liberty(name)]
  #[id]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
}
impl GroupFn for DummyGroup {}
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
  _undefined: crate::ast::AttributeList,
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
pub struct TableLookUp {
  // TODO: unit
  unit: (),
  #[id]
  #[liberty(name)]
  name: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
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
  [TableLookUp2D];
  [TableLookUp3D];
  [DriverWaveform];
  [TableLookUpMultiSegment];
)]
impl GroupFn for AllTypes {
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
      .map(|v| vec![v.iter().map(|f| buffer.format(*f).to_string()).join(", ")])
      .collect()
  }
}
