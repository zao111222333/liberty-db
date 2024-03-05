use std::{collections::HashSet, fmt::Debug, rc::Rc};

use itertools::Itertools;
use strum_macros::{Display, EnumString};

use crate::ast::{GroupComments, GroupId};

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
pub struct SdfExpression {}
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
pub struct Domain {
  #[liberty(id(title = 1))]
  _id: GroupId<Self>,
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

/// sth. like "A B C" will save as set{A B C}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct WordSet {
  _set: HashSet<String>,
}
impl std::fmt::Display for WordSet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Display::fmt(&self._set.iter().join(" "), f)
  }
}

impl std::hash::Hash for WordSet {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    let mut sum = 0_u64;
    for item in &self._set {
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
      _set: s.split(' ').map(ToString::to_string).collect(),
    })
  }
}

// impl crate::ast::HashedGroup for TableLookUp {
//   type Id = Option<String>;
//   #[inline]
//   fn title(&self) -> Vec<String> {
//     match Option::as_ref(&self._id) {
//       Some(s) => vec![s.clone()],
//       None => vec![],
//     }
//   }
//   #[inline]
//   fn id(&self) -> crate::ast::GroupId<Self> {
//     self._id.clone()
//   }
//   #[inline]
//   fn gen_id(&self, title: Vec<String>) -> Result<Self::Id, crate::ast::IdError> {
//     // Ok(title.pop())
//     todo!()
//   }
// }

#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[readonly::make]
pub struct TableLookUpMultiSegment {
  #[liberty(title)]
  #[liberty(id)]
  #[readonly]
  _title: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
  #[liberty(simple)]
  #[liberty(id)]
  #[readonly]
  segment: usize,
  #[liberty(complex(type=Default))]
  pub index_1: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_2: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_3: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_4: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub values: Vec<f64>,
}
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
pub struct TableLookUp {
  #[liberty(id)]
  _id: GroupId<Self>,
  #[liberty(title)]
  _title: Option<String>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
  #[liberty(complex(type=Default))]
  pub index_1: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_2: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_3: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub index_4: Vec<f64>,
  #[liberty(complex(type=Default))]
  pub values: Vec<f64>,
}
