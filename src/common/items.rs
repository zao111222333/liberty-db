use strum_macros::{Display, EnumString};

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
#[derive(Debug, Clone)]
#[derive(Default)]
#[derive(liberty_macros::Group)]
// #[derive(liberty_macros::NameIdx)]
pub struct Domain {
  #[id_len(1)]
  _id: <Self as crate::ast::HashedGroup>::Id,
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
