//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
use crate::{
  ast::{Attributes, GroupComments, GroupFn, GroupSet},
  ccsn::{CCSNStage, ReceiverCapacitance},
  common::items::{NameList, WordSet},
  expression::{logic, BooleanExpression},
  internal_power::InternalPower,
  timing::Timing,
  ArcStr, NotNan,
};
mod bundle;
mod items;
pub use bundle::*;
// use crate::units;
pub use items::*;
/// You can define a `pin` group within a [`cell`](crate::cell::Cell),
/// [`test_cell`](crate::test_cell), [`model`](crate::model),
/// or [`bus`](crate::bus::Bus) group.
///
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
#[mut_set::derive::item(sort)]
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Pin {
  /// Name of the pin
  /// `pin (name | name_list)`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=227.10&end=227.25
  /// ">Reference-Definition</a>
  #[id]
  #[size = 48]
  #[liberty(name)]
  pub name: NameList,
  /// group comments
  #[size = 2376]
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub driver_waveform_rise: Option<ArcStr>,
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub driver_waveform_fall: Option<ArcStr>,
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
  #[size = 8]
  #[liberty(simple)]
  pub related_ground_pin: ArcStr,
  /// The `retention_pin` complex attribute identifies the retention pins of a retention cell. The
  /// attribute defines the following information:
  /// + pin class
  ///
  ///   Valid values:
  ///   + `restore`: Restores the state of the cell.
  ///   + `save`: Saves the state of the cell.
  ///   + `save_restore`: Saves and restores the state of the cell.
  /// + disable value
  ///
  /// Defines the value of the retention pin when the cell works in normal mode. The valid
  /// values are 0 and 1.
  ///
  /// Syntax
  /// ``` text
  /// retention_pin (pin_class, disable_value) ;
  /// ```
  /// Example
  /// ``` text
  /// retention_pin (save | restore | save_restore, enumerated_type) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=282.3&end=282.23
  /// ">Reference-Definition</a>
  #[size = 2]
  #[liberty(complex(type = Option))]
  pub retention_pin: Option<RetentionPin>,
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
  #[size = 8]
  #[liberty(simple)]
  pub related_power_pin: ArcStr,
  // NOTICE: Simple Attributes in a pin Group
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =227.33
  /// &end
  /// =227.33
  /// ">Reference-Instance</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub alive_during_partial_power_down: Option<bool>,
  // TODO
  #[size = 32]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 64]
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
  #[size = 64]
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
  #[size = 16]
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
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub capacitance: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.9
  /// &end
  /// =228.9
  /// ">Reference-Instance</a>
  #[size = 32]
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
  #[size = 32]
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
  #[size = 32]
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
  #[size = 32]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub complementary_pin: Option<ArcStr>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.21
  /// &end
  /// =228.21
  /// ">Reference-Instance</a>
  #[size = 64]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub drive_current: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.25
  /// &end
  /// =228.27
  /// ">Reference-Instance</a>
  #[size = 1]
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
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub fall_capacitance: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.29
  /// &end
  /// =228.29
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub fall_current_slope_after_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.30
  /// &end
  /// =228.30
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub fall_current_slope_before_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.31
  /// &end
  /// =228.31
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub fall_time_after_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.32
  /// &end
  /// =228.32
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub fall_time_before_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.33
  /// &end
  /// =228.33
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub fanout_load: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.34
  /// &end
  /// =228.34
  /// ">Reference-Instance</a>
  #[size = 2]
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
  #[size = 32]
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
  #[size = 32]
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
  #[size = 1]
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
  #[size = 32]
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
  #[size = 64]
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
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub input_signal_level: Option<ArcStr>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.43
  /// &end
  /// =228.43
  /// ">Reference-Instance</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub input_voltage: Option<ArcStr>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.44
  /// &end
  /// =228.46
  /// ">Reference-Instance</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub internal_node: Option<ArcStr>, /* Required in statetable cells */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.47
  /// &end
  /// =228.47
  /// ">Reference-Instance</a>
  #[size = 1]
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
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_pad: Option<bool>,
  /// The `is_pll_reference_pin` Boolean attribute tags a pin as a reference pin on the phaselocked loop.
  /// In a phase-locked loop cell group, the is_pll_reference_pin attribute
  /// should be set to true in only one input pin group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=256.17&end=256.19
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub is_pll_reference_pin: Option<bool>,
  /// The `is_pll_feedback_pin`  Boolean attribute tags a pin as a feedback pin on a phase-locked loop.
  /// In a phase-locked loop cell group, the `is_pll_feedback_pin`  attribute should
  /// be set to true in only one input pin group
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=257.3&end=257.5
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub is_pll_feedback_pin: Option<bool>,
  /// The `is_pll_output_pin`  Boolean attribute tags a pin as an output pin on a phase-locked loop.
  /// In a phase-locked loop cell group, the `is_pll_output_pin`  attribute
  /// should be set to true in one or more output pin groups.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=257.35&end=257.37
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub is_pll_output_pin: Option<bool>,
  /// The `is_unbuffered`  attribute specifies the pin as unbuffered.
  /// You can specify this optional attribute on the pins of any library cell.
  /// The default is false.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=258.39&end=258.40
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub is_unbuffered: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.49
  /// &en7
  /// =228.49
  /// ">Reference-Instance</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_unconnected: Option<bool>,
  /// The `isolation_cell_data_pin`  attribute identifies the data pin of any isolation cell.The valid values of this attribute are true  or false. If this attribute is not specified, all the input pins of the isolation cell are considered to be data pins.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=259.12&end=259.14
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub isolation_cell_data_pin: Option<bool>,
  /// The `isolation_cell_enable_pin`  attribute specifies the enable input pin of an isolation cell including a clock isolation cell. For more information about isolation cells,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=258.39&end=258.40
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub isolation_cell_enable_pin: Option<bool>,
  /// The `isolation_enable_condition`  attribute specifies the isolation condition for internally-isolated pins, buses, and bundles of a cell. When this attribute is defined in a pin group, the corresponding Boolean expression can include only input and inout pins. Do not include the output pins of an internally isolated cell in the Boolean expression
  ///
  /// The attribute is applicable to pins of macro cells
  ///
  /// When the isolation_enable_condition  attribute is defined in a bus  or  bundle  group, the corresponding Boolean expression can include pins, and buses and bundles of the same bit-width. For example, when the Boolean expression includes a bus and a bundle, both of them must have the same bit-width.
  ///
  /// Pins, buses, and bundles that have the isolation_enable_condition  attribute must also have the always_on  attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=260.3&end=260.13
  /// ">Reference</a>
  #[size = 32]
  #[liberty(simple(type=Option))]
  pub isolation_enable_condition: Option<BooleanExpression>,
  /// The `level_shifter_data_pin`  attribute specifies the input data pin on a level shifter cell
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=260.19&end=260.20
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub level_shifter_data_pin: Option<bool>,
  /// The `level_shifter_enable_pin`  attribute specifies the enable input pin on a level shifter cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=260.30&end=260.31
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub level_shifter_enable_pin: Option<bool>,
  /// The `map_to_logic`  attribute specifies which logic level to tie a pin when a power-switch cell functions as a normal cell. For more information about power-switch cells
  ///
  /// Valid values are 1 and 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=261.11+261.17&end=261.12+261.18
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type=Option))]
  pub map_to_logic: Option<OneZero>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.50
  /// &end
  /// =228.50
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub max_capacitance: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.51
  /// &end
  /// =228.41
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub max_fanout: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.52
  /// &end
  /// =228.52
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub max_input_delta_overdrive_high: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.53
  /// &end
  /// =228.53
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub max_input_delta_underdrive_high: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.54
  /// &end
  /// =228.54
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub max_transition: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.55
  /// &end
  /// =228.55
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub min_capacitance: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.56
  /// &end
  /// =228.56
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub min_fanout: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.57
  /// &end
  /// =228.57
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub min_period: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.58
  /// &end
  /// =228.58
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub min_pulse_width_high: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.59
  /// &end
  /// =228.59
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub min_pulse_width_low: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.60
  /// &end
  /// =228.60
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub min_transition: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.61
  /// &end
  /// =228.61
  /// ">Reference-Instance</a>
  #[size = 1]
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
  #[size = 1]
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
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub output_signal_level: Option<ArcStr>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.64
  /// &end
  /// =228.64
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub output_signal_level_high: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.65
  /// &end
  /// =228.65
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub output_signal_level_low: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.66
  /// &end
  /// =228.66
  /// ">Reference-Instance</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub output_voltage: Option<ArcStr>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.2
  /// &end
  /// =229.3
  /// ">Reference-Instance</a>
  #[size = 1]
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
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub prefer_tied: Option<OneZero>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.5
  /// &end
  /// =229.5
  /// ">Reference-Instance</a>
  #[size = 1]
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
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub pulling_current: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.7
  /// &end
  /// =229.7
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub pulling_resistance: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.8
  /// &end
  /// =229.8
  /// ">Reference-Instance</a>
  #[size = 2]
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
  #[size = 1]
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
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub rise_capacitance: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.11
  /// &end
  /// =229.11
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub rise_current_slope_after_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.12
  /// &end
  /// =229.12
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub rise_current_slope_before_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.13
  /// &end
  /// =229.13
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub rise_time_after_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.14
  /// &end
  /// =229.14
  /// ">Reference-Instance</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub rise_time_before_threshold: Option<NotNan<f64>>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.15
  /// &end
  /// =229.15
  /// ">Reference-Instance</a>
  #[size = 2]
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
  #[size = 1]
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
  #[size = 1]
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
  #[size = 32]
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
  #[size = 1]
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
  #[size = 32]
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
  #[size = 32]
  #[liberty(simple(type = Option))]
  pub x_function: Option<BooleanExpression>,
  /// The `switch_pin`  attribute is a pin-level Boolean attribute.
  /// When it is set to true, it is used to identify the pin as
  /// the switch pin of a coarse-grain switch cell
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=279.19&end=279.20
  /// ">Reference-Definition</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub switch_pin: Option<bool>,
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
  #[size = 24]
  #[liberty(complex(type = Option))]
  pub fall_capacitance_range: Option<(NotNan<f64>, NotNan<f64>)>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.29
  /// &end
  /// =229.29
  /// ">Reference-Instance</a>
  #[size = 24]
  #[liberty(complex(type = Option))]
  pub rise_capacitance_range: Option<(NotNan<f64>, NotNan<f64>)>,
  // NOTICE: Group Attributes in a pin Group
  // electromigration () { }
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<InternalPower>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<InternalPower>::deserialize_with")]
  pub internal_power: GroupSet<InternalPower>,
  // TODO
  // max_trans () { }
  // TODO
  // min_pulse_width ()  { }
  // TODO
  // minimum_period ()  { }
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
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Timing>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Timing>::deserialize_with")]
  pub timing: GroupSet<Timing>,
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
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<ReceiverCapacitance>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<ReceiverCapacitance>::deserialize_with")]
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
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<CCSNStage>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<CCSNStage>::deserialize_with")]
  pub input_ccb: GroupSet<CCSNStage>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<CCSNStage>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<CCSNStage>::deserialize_with")]
  pub output_ccb: GroupSet<CCSNStage>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<CCSNStage>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<CCSNStage>::deserialize_with")]
  pub ccsn_first_stage: GroupSet<CCSNStage>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<CCSNStage>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<CCSNStage>::deserialize_with")]
  pub ccsn_last_stage: GroupSet<CCSNStage>,
}

impl GroupFn for Pin {}

#[cfg(test)]
mod test {

  #[test]
  fn pin_name_list() {
    let cell = crate::ast::test_parse_fmt::<crate::Cell>(
      r#"(test_cell){
        pin (A) {}
        pin (B,C,D,E) {}
      }"#,
      r#"
liberty_db::cell::Cell (test_cell) {
| pin (A) {
| }
| pin (B, C, D, E) {
| }
}"#,
    );
  }
}

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
//      _ = set.insert(pin_a);
//      _ = set.insert(pin_b);
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
//      _ = arc_set.borrow_mut().insert(pin_a);
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
