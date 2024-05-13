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
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: crate::ast::AttributeList,
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
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: crate::ast::AttributeList,
}
impl GroupFn for DummyGroup {}
