//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
use crate::{
  ast::{AttributeList, GroupComments, GroupFn},
  ccsn::{CCSNStage, ReceiverCapacitance},
  common::items::{DummyGroup, WordSet},
  expression::{logic, BooleanExpression},
  internal_power::InternalPower,
  timing::Timing,
  GroupSet,
};
mod items;
// use crate::units;
pub use items::*;
/// You can define a `pin` group within a [`cell`](crate::cell::Cell),
/// [`test_cell`](crate::test_cell), [`model`](crate::model),
/// or [`bus`](crate::bus::Bus) group.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =227.0
/// &end
/// =227.8
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
///
/// + An example of the `pin` group syntax showing the attribute
/// and group statements that you can use within the `pin` group
/// + Descriptions of the attributes and groups you can use in a `pin` group
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pin {
  /// Name of the pin
  #[id]
  #[liberty(name)]
  pub name: String,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  #[liberty(simple(type = Option))]
  pub driver_waveform_rise: Option<String>,
  #[liberty(simple(type = Option))]
  pub driver_waveform_fall: Option<String>,
  /// The `related_power_pin`  and `related_ground_pin`  attributes
  /// are defined at the `pin` level for `output`, `input`, and `inout` pins.
  /// The `related_power_pin`  and `related_ground_pin` attributes are used
  /// to associate a predefined power and ground pin with the signal pin,
  /// in which they are defined. This behavior only applies to standard cells.
  /// For special cells, you must specify this relationship explicitly.
  /// The `pg_pin`  groups are mandatory for each cell.
  /// Because a cell must have at least one `primary_power`  and
  /// at least one `primary_ground`  pin,
  /// a default `related_power_pin`  and `related_ground_pin`  always exists in any cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=269.26&end=269.33
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub related_ground_pin: String,
  /// The `related_power_pin`  and `related_ground_pin`  attributes
  /// are defined at the `pin` level for `output`, `input`, and `inout` pins.
  /// The `related_power_pin`  and `related_ground_pin` attributes are used
  /// to associate a predefined power and ground pin with the signal pin,
  /// in which they are defined. This behavior only applies to standard cells.
  /// For special cells, you must specify this relationship explicitly.
  /// The `pg_pin`  groups are mandatory for each cell.
  /// Because a cell must have at least one `primary_power`  and
  /// at least one `primary_ground`  pin,
  /// a default `related_power_pin`  and `related_ground_pin`  always exists in any cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=269.26&end=269.33
  /// ">Reference-Definition</a>
  #[liberty(simple)]
  pub related_power_pin: String,
  // NOTICE: Simple Attributes in a pin Group
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =227.33
  /// &end
  /// =227.33
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub alive_during_partial_power_down: Option<bool>,
  // TODO
  #[liberty(simple(type = Option))]
  pub power_down_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.2
  /// &end
  /// =228.2
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub alive_during_power_up: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.3
  /// &end
  /// =228.3
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub always_on: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.4
  /// &end
  /// =228.4
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub antenna_diode_type: Option<AntennaDiodeType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.5
  /// &end
  /// =228.5
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub antenna_diode_related_ground_pins: WordSet,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.6
  /// &end
  /// =228.6
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub antenna_diode_related_power_pins: WordSet,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.7
  /// &end
  /// =228.7
  /// ">Reference-Instance</a>
  /* bus cells */
  #[liberty(simple(type = Option))]
  pub bit_width: Option<usize>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.8
  /// &end
  /// =228.8
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.9
  /// &end
  /// =228.9
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clamp_0_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.10
  /// &end
  /// =228.10
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clamp_1_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.11
  /// &end
  /// =228.11
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clamp_latch_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.12
  /// &end
  /// =228.12
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clamp_z_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.13
  /// &end
  /// =228.13
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clock: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.14
  /// &end
  /// =228.14
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clock_gate_clock_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.15
  /// &end
  /// =228.15
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clock_gate_enable_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.16
  /// &end
  /// =228.16
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clock_gate_test_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.17
  /// &end
  /// =228.17
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clock_gate_obs_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.18
  /// &end
  /// =228.18
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clock_gate_out_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.19
  /// &end
  /// =228.19
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub clock_isolation_cell_clock_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.20
  /// &end
  /// =228.20
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub complementary_pin: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.21
  /// &end
  /// =228.21
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub connection_class: WordSet,
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
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.23
  /// &end
  /// =228.23
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub dont_fault: Option<DontFault>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.24
  /// &end
  /// =228.24
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub drive_current: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.25
  /// &end
  /// =228.27
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub driver_type: Option<DriverType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.28
  /// &end
  /// =228.28
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub fall_capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.29
  /// &end
  /// =228.29
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub fall_current_slope_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.30
  /// &end
  /// =228.30
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub fall_current_slope_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.31
  /// &end
  /// =228.31
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub fall_time_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.32
  /// &end
  /// =228.32
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub fall_time_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.33
  /// &end
  /// =228.33
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub fanout_load: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.34
  /// &end
  /// =228.34
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub fault_model: Option<TwoValue>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.35
  /// &end
  /// =228.35
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.36
  /// &end
  /// =228.36
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub has_builtin_pad: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.37
  /// &end
  /// =228.37
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub hysteresis: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.38
  /// &end
  /// =228.38
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub illegal_clamp_condition: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.39
  /// &end
  /// =228.41
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub input_map: WordSet,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.42
  /// &end
  /// =228.42
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub input_signal_level: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.43
  /// &end
  /// =228.43
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub input_voltage: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.44
  /// &end
  /// =228.46
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub internal_node: Option<String>, /* Required in statetable cells */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.47
  /// &end
  /// =228.47
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub inverted_output: Option<bool>, /* Required in statetable cells */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.48
  /// &end
  /// =228.48
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub is_pad: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.49
  /// &en7
  /// =228.49
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub is_unconnected: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.50
  /// &end
  /// =228.50
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub max_capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.51
  /// &end
  /// =228.41
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub max_fanout: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.52
  /// &end
  /// =228.52
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub max_input_delta_overdrive_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.53
  /// &end
  /// =228.53
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub max_input_delta_underdrive_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.54
  /// &end
  /// =228.54
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub max_transition: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.55
  /// &end
  /// =228.55
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub min_capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.56
  /// &end
  /// =228.56
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub min_fanout: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.57
  /// &end
  /// =228.57
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub min_period: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.58
  /// &end
  /// =228.58
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub min_pulse_width_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.59
  /// &end
  /// =228.59
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub min_pulse_width_low: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.60
  /// &end
  /// =228.60
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub min_transition: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.61
  /// &end
  /// =228.61
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub multicell_pad_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.62
  /// &end
  /// =228.62
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub nextstate_type: Option<NextstateType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.63
  /// &end
  /// =228.63
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub output_signal_level: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.64
  /// &end
  /// =228.64
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub output_signal_level_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.65
  /// &end
  /// =228.65
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub output_signal_level_low: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.66
  /// &end
  /// =228.66
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub output_voltage: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.2
  /// &end
  /// =229.3
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub pin_func_type: Option<PinFuncType>,
  /// The prefer_tied attribute describes an input pin of a flip-flop or latch.
  /// It indicates what the library developer wants this pin connected to.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.4
  /// &end
  /// =229.4
  /// ">Reference-Instance</a>
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=267.24&end=267.26
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub prefer_tied: Option<PreferTied>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.5
  /// &end
  /// =229.5
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub primary_output: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.6
  /// &end
  /// =229.6
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub pulling_current: Option<f64>,
  // pub pulling_current: Option<units::ElectricCurrent>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.7
  /// &end
  /// =229.7
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub pulling_resistance: Option<f64>,
  // pub pulling_resistance: Option<units::ElectricalResistance>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.8
  /// &end
  /// =229.8
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub restore_action: Option<logic::Normal>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.9
  /// &end
  /// =229.9
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub restore_edge_type: Option<RestoreEdgeType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.10
  /// &end
  /// =229.10
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub rise_capacitance: Option<f64>,
  // pub rise_capacitance: Option<units::Capacitance>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.11
  /// &end
  /// =229.11
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub rise_current_slope_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.12
  /// &end
  /// =229.12
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub rise_current_slope_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.13
  /// &end
  /// =229.13
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub rise_time_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.14
  /// &end
  /// =229.14
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub rise_time_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.15
  /// &end
  /// =229.15
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub save_action: Option<logic::Normal>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.16
  /// &end
  /// =229.19
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub signal_type: Option<SignalType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.20
  /// &end
  /// =229.20
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub slew_control: Option<SlewControl>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.21
  /// &end
  /// =229.21
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub state_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.22
  /// &end
  /// =229.22
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub test_output_only: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.23
  /// &end
  /// =229.23
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub three_state: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.24
  /// &end
  /// =229.24
  /// ">Reference-Instance</a>
  #[liberty(simple(type = Option))]
  pub x_function: Option<BooleanExpression>,
  // /* Complex Attributes in a pin Group */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.28
  /// &end
  /// =229.28
  /// ">Reference-Instance</a>
  // NOTICE: Complex Attributes in a pin Group
  #[liberty(complex(type = Option))]
  pub fall_capacitance_range: Option<(f64, f64)>,
  // pub fall_capacitance_range: Option<(units::Capacitance, units::Capacitance)>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.29
  /// &end
  /// =229.29
  /// ">Reference-Instance</a>
  #[liberty(complex(type = Option))]
  pub rise_capacitance_range: Option<(f64, f64)>,
  // NOTICE: Group Attributes in a pin Group
  // electromigration () { }
  // input_ccb (string) { }
  #[liberty(group(type=Set))]
  pub internal_power: GroupSet<InternalPower>,
  // TODO
  // max_trans () { }
  // TODO
  // min_pulse_width ()  { }
  // TODO
  // minimum_period ()  { }
  // TODO
  // output_ccb (string) { }
  // pub output_ccb: (),
  // TODO
  pub tlatch: (),
  /// A timing group is defined in a [bundle](crate::bundle::Bundle), a [bus](crate::bus::Bus), or a [pin](crate::pin::Pin) group within a cell.
  /// The timing group can be used to identify the name or names of multiple timing arcs.
  /// A timing group identifies multiple timing arcs, by identifying a timing arc in a [pin](crate::pin::Pin) group
  /// that has more than one related pin or when the timing arc is part of a [bundle](crate::bundle::Bundle) or a [bus](crate::bus::Bus).
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =67.26
  /// &end
  /// =67.43
  /// ">Reference-Definition</a>
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
  /// ?field=test
  /// &bgn
  /// =203.8
  /// &end
  /// =203.29
  /// ">Reference-Instatnce-In-Pin</a>
  ///
  #[liberty(group(type=Set))]
  pub timing: GroupSet<Timing>,
  #[liberty(group(type=Set))]
  /// Use the `receiver_capacitance`  group to specify capacitance values
  /// for composite current source (CCS) receiver modeling at the pin level.
  ///
  /// Groups
  ///
  /// For two-segment receiver capacitance model
  /// + receiver_capacitance1_fall
  /// + receiver_capacitance1_rise
  /// + receiver_capacitance2_fall
  /// + receiver_capacitance2_rise
  ///
  /// For multisegment receiver capacitance model
  /// + receiver_capacitance_fall
  /// + receiver_capacitance_rise
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=316.5&end=316.31
  /// ">Reference-Definition</a>
  pub receiver_capacitance: GroupSet<ReceiverCapacitance>,
  /// In referenced CCS noise modeling,
  /// use the `input_ccb`  group to specify the CCS noise for
  /// an input channel-connected block (CCB).
  /// You must name the `input_ccb`  group so that it can be referenced.
  /// The `input_ccb`  group includes all the attributes and subgroups
  /// of the `ccsn_first_stage` Group  on page 283.
  /// The `input_ccb`  group also includes the `related_ccb_node`  simple attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=296.7&end=296.12
  /// ">Reference-Instance</a>
  #[liberty(group(type=Set))]
  pub input_ccb: GroupSet<CCSNStage>,
  #[liberty(group(type=Set))]
  pub output_ccb: GroupSet<CCSNStage>,
  #[liberty(group(type=Set))]
  pub ccsn_first_stage: GroupSet<CCSNStage>,
  #[liberty(group(type=Set))]
  pub ccsn_last_stage: GroupSet<CCSNStage>,
}

impl GroupFn for Pin {}

// #[test]
// fn test_link() {
//   use crate::ast::{LinkError, LinkedGroup};
//   use std::cell::RefCell;
//   use std::collections::HashSet;
//   use std::sync::Arc;
//   let arc_set: Arc<RefCell<GroupMap<Pin>>>;
//   {
//     let mut pin_a = Pin::default();
//     pin_a._id = String::from("A").into();
//     let mut pin_b = Pin::default();
//     pin_b._id = String::from("B").into();
//     let mut set = GroupMap::<Pin>::default();
//     let _ = set.insert(pin_a);
//     let _ = set.insert(pin_b);
//     arc_set = Arc::new(RefCell::new(set));
//   }
//   let pin_a_link = LinkedGroup::<Pin>::new(String::from("A").into(), &arc_set);
//   pin_a_link.get_linked(|r| {
//     println!("{:?}", r);
//   });
//   println!("---------");
//   {
//     let mut pin_a = Pin::default();
//     pin_a._id = String::from("A").into();
//     pin_a.bit_width = 12345;
//     let _ = arc_set.borrow_mut().insert(pin_a);
//     pin_a_link.get_linked(|r| {
//       assert!(matches!(r, Ok(_)));
//       assert_eq!(r.unwrap().bit_width, 12345);
//     });
//   }
//   println!("---------");
//   {
//     let xxx = arc_set.borrow_mut();
//     let pin_a_link__ = LinkedGroup::<Pin>::new(String::from("A").into(), &arc_set);
//     pin_a_link__.get_linked(|r| assert!(matches!(r, Err(LinkError::BorrowError(_)))));
//   }
//   println!("---------");
//   {
//     let pin_c_link__ = LinkedGroup::<Pin>::new(String::from("C").into(), &arc_set);
//     pin_c_link__.get_linked(|r| assert!(matches!(r, Err(LinkError::NotFind))));
//   }
// }
