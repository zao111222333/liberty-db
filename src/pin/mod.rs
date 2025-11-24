//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
use crate::{
  Ctx,
  ast::{
    Attributes, BuilderScope, FlattenNameAttri, GroupComments, GroupFn, LibertySet,
    RandomState,
  },
  ccsn::{CCSNStage, ReceiverCapacitance},
  common::{char_config::CharConfig, items::WordSet},
  expression::{
    BooleanExpression, LogicBooleanExpression, PowerGroundBooleanExpression, logic,
  },
  internal_power::InternalPower,
  timing::Timing,
};
mod bus;
pub use bus::{BusType, BusTypeCtx, SimpleBusType};
mod items;
pub use items::*;

#[derive(liberty_macros::Duplicate)]
#[duplicated(
  name = Bus,
  docs(
    /// A `bus` group, defined in a [`cell`](crate::cell::Cell) group or a [`model`](crate::cell::Model) group, defines the bused pins in the
    /// library. Before you can define a `bus` group you must first define a [`type`](crate::pin::BusType) group at the `library`
    /// level.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=136.4&end=136.9
    /// ">Reference</a>
  ),
  additional_attrs(
    /// The `bus_type` attribute is a required element of all bus groups. The attribute defines the
    /// type of bus. It must be the first attribute declared in a bus group.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=269.26&end=269.33
    /// ">Reference-Definition</a>
    #[liberty(simple)]
    pub bus_type: SimpleBusType,
    /// The optional `scan_start_pin` attribute specifies the scan output pin of a sequential
    /// element of a multibit scan cell, where the internal scan chain begins. This attribute applies
    /// only to output buses and bundles of multibit scan cells.
    /// Only the following scan chains are supported:
    /// + From the least significant bit (LSB) to the most significant bit (MSB) of the output bus
    /// group; and
    /// + From the MSB to the LSB of the output bus group.
    ///
    /// Therefore, for a multibit scan cell with internal scan chain, the value of the
    /// `scan_start_pin` attribute can either be the LSB, or the MSB output pin.
    /// Specifying the LSB scan output pin as the value of the `scan_start_pin` attribute indicates
    /// that the scan signal shifts from the LSB sequential element to the MSB sequential element
    /// of the multibit scan cell.
    ///
    /// Specifying the MSB scan output pin as the value of the `scan_start_pin` attribute
    /// indicates that the scan signal shifts from the MSB sequential element to the LSB
    /// sequential element of the multibit scan cell.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=269.26&end=269.33
    /// ">Reference-Definition</a>
    #[liberty(simple)]
    pub scan_start_pin: Option<String>,
    /// The optional `scan_pin_inverted` attribute specifies that the scan signal is inverted (after
    /// the first sequential element of the multibit scan cell). This attribute applies only to output
    /// buses and bundles of multibit scan cells. The default is false.
    ///
    /// If you specify the `scan_pin_inverted` attribute value as true, you must specify the value
    /// of the `signal_type` attribute as `test_scan_out_inverted`.
    ///
    /// If you specify the `scan_pin_inverted` attribute, you must specify the `scan_start_pin`
    /// attribute in the same bus group.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=269.26&end=269.33
    /// ">Reference-Definition</a>
    #[liberty(simple)]
    #[liberty(default = false)]
    pub scan_pin_inverted: bool,
    #[liberty(group)]
    pub memory_write: Option<MemoryWrite<C>>,
    #[liberty(group)]
    pub memory_read: Option<MemoryRead<C>>,
    /// You can define a `pin` group within a [`cell`](crate::cell::Cell),
    /// [`test_cell`](crate::cell::TestCell), [`model`](crate::cell::Model),
    /// or [`bus`](crate::pin::Bus) group.
    #[liberty(group)]
    pub pin: LibertySet<Pin<C>>,
  )
)]
#[duplicated(
  name = Bundle,
  exclude(complex, group),
  not_exclude(timing),
  docs(
    /// A bundle group uses the members complex attribute (unique to bundles) to group together
    /// in multibit cells—such as quad latches and 4-bit registers—several pins that have similar
    /// timing or functionality.
    /// The bundle group contains the following elements:
    /// + The members complex attribute. It must be declared first in a bundle group.
    /// + All simple attributes that also appear in a pin group.
    /// + The pin group statement (including all the pin group simple and complex attributes,
    /// and group statements).
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=129.30+130.3&end=129.39+130.23
    /// ">Reference</a>
  ),
  additional_attrs(
    #[liberty(complex)]
    pub members: Vec<String>,
    #[liberty(group)]
    pub pin: LibertySet<Pin<C>>,
  )
)]
/// You can define a `pin` group within a [`cell`](crate::cell::Cell),
/// [`test_cell`](crate::cell::TestCell), [`model`](crate::cell::Model),
/// or [`bus`](crate::pin::Bus) group.
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
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Pin: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Pin<C: 'static + Ctx> {
  /// Name of the pin
  /// `pin (name | name_list)`
  ///
  /// `name_list` cases will be flatten into multiple Pin
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=227.10&end=227.25
  /// ">Reference-Definition</a>
  #[id(borrow = str)]
  #[liberty(name(flatten))]
  pub name: String,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Pin,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[liberty(simple)]
  pub driver_waveform_rise: Option<String>,
  #[liberty(simple)]
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
  #[liberty(simple)]
  pub related_bias_pin: WordSet,
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
  /// ### Syntax
  /// ``` text
  /// retention_pin (pin_class, disable_value) ;
  /// ```
  /// ### Example
  /// ``` text
  /// retention_pin (save | restore | save_restore, enumerated_type) ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=282.3&end=282.23
  /// ">Reference-Definition</a>
  #[liberty(complex)]
  pub retention_pin: Option<RetentionPin>,
  // NOTICE: Simple Attributes in a pin Group
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =227.33
  /// &end
  /// =227.33
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub alive_during_partial_power_down: Option<bool>,
  #[liberty(simple)]
  pub power_down_function: Option<PowerGroundBooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.2
  /// &end
  /// =228.2
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub alive_during_power_up: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.3
  /// &end
  /// =228.3
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub always_on: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.4
  /// &end
  /// =228.4
  /// ">Reference-Instance</a>
  #[liberty(simple)]
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
  #[liberty(simple)]
  pub bit_width: Option<usize>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.8
  /// &end
  /// =228.8
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.9
  /// &end
  /// =228.9
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clamp_0_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.10
  /// &end
  /// =228.10
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clamp_1_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.11
  /// &end
  /// =228.11
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clamp_latch_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.12
  /// &end
  /// =228.12
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clamp_z_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.13
  /// &end
  /// =228.13
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clock: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.14
  /// &end
  /// =228.14
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clock_gate_clock_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.15
  /// &end
  /// =228.15
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clock_gate_enable_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.16
  /// &end
  /// =228.16
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clock_gate_test_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.17
  /// &end
  /// =228.17
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clock_gate_obs_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.18
  /// &end
  /// =228.18
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clock_gate_out_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.19
  /// &end
  /// =228.19
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub clock_isolation_cell_clock_pin: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.20
  /// &end
  /// =228.20
  /// ">Reference-Instance</a>
  #[liberty(simple)]
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
  #[liberty(simple)]
  pub direction: Option<Direction>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.23
  /// &end
  /// =228.23
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub dont_fault: Option<DontFault>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.24
  /// &end
  /// =228.24
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub drive_current: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.25
  /// &end
  /// =228.27
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub driver_type: Option<AllDriverType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.28
  /// &end
  /// =228.28
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub fall_capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.29
  /// &end
  /// =228.29
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub fall_current_slope_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.30
  /// &end
  /// =228.30
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub fall_current_slope_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.31
  /// &end
  /// =228.31
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub fall_time_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.32
  /// &end
  /// =228.32
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub fall_time_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.33
  /// &end
  /// =228.33
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub fanout_load: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.34
  /// &end
  /// =228.34
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub fault_model: Option<TwoValue>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.35
  /// &end
  /// =228.35
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub function: Option<LogicBooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.36
  /// &end
  /// =228.36
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub has_builtin_pad: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.37
  /// &end
  /// =228.37
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub hysteresis: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.38
  /// &end
  /// =228.38
  /// ">Reference-Instance</a>
  #[liberty(simple)]
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
  #[liberty(simple)]
  pub input_signal_level: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.43
  /// &end
  /// =228.43
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub input_voltage: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.44
  /// &end
  /// =228.46
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub internal_node: Option<String>, /* Required in statetable cells */
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.47
  /// &end
  /// =228.47
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub inverted_output: Option<bool>, /* Required in statetable cells */
  /// The `is_pad`  attribute identifies a pad pin on
  /// any I/O cell. You can also specify the `is_pad` attribute
  /// on PG pins.
  /// The valid values are `true`  and `false`.
  /// If the cell-level `pad_cell` attribute is specified on
  /// a I/O cell, the `is_pad`  attribute must be set to `true`
  /// in either a `pg_pin`  group or on a signal pin for that cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=111.5&end=111.8
  /// ">Reference</a>
  #[liberty(simple)]
  pub is_pad: Option<bool>,
  /// The `is_pll_reference_pin` Boolean attribute tags a pin as a reference pin on the phaselocked loop.
  /// In a phase-locked loop cell group, the `is_pll_reference_pin` attribute
  /// should be set to true in only one input pin group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=256.17&end=256.19
  /// ">Reference</a>
  #[liberty(simple)]
  pub is_pll_reference_pin: Option<bool>,
  /// The `is_pll_feedback_pin`  Boolean attribute tags a pin as a feedback pin on a phase-locked loop.
  /// In a phase-locked loop cell group, the `is_pll_feedback_pin`  attribute should
  /// be set to true in only one input pin group
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=257.3&end=257.5
  /// ">Reference</a>
  #[liberty(simple)]
  pub is_pll_feedback_pin: Option<bool>,
  /// The `is_pll_output_pin`  Boolean attribute tags a pin as an output pin on a phase-locked loop.
  /// In a phase-locked loop cell group, the `is_pll_output_pin`  attribute
  /// should be set to true in one or more output pin groups.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=257.35&end=257.37
  /// ">Reference</a>
  #[liberty(simple)]
  pub is_pll_output_pin: Option<bool>,
  /// The `is_unbuffered`  attribute specifies the pin as unbuffered.
  /// You can specify this optional attribute on the pins of any library cell.
  /// The default is false.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=258.39&end=258.40
  /// ">Reference</a>
  #[liberty(simple)]
  pub is_unbuffered: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.49
  /// &en7
  /// =228.49
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub is_unconnected: Option<bool>,
  /// The `isolation_cell_data_pin`  attribute identifies the data pin of any isolation cell.The valid values of this attribute are true  or false. If this attribute is not specified, all the input pins of the isolation cell are considered to be data pins.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=259.12&end=259.14
  /// ">Reference</a>
  #[liberty(simple)]
  pub isolation_cell_data_pin: Option<bool>,
  /// The `isolation_cell_enable_pin`  attribute specifies the enable input pin of an isolation cell including a clock isolation cell. For more information about isolation cells,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=258.39&end=258.40
  /// ">Reference</a>
  #[liberty(simple)]
  pub isolation_cell_enable_pin: Option<bool>,
  /// The `isolation_enable_condition`  attribute specifies the isolation condition for internally-isolated pins, buses, and bundles of a cell. When this attribute is defined in a pin group, the corresponding Boolean expression can include only input and inout pins. Do not include the output pins of an internally isolated cell in the Boolean expression
  ///
  /// The attribute is applicable to pins of macro cells
  ///
  /// When the `isolation_enable_condition`  attribute is defined in a bus  or  bundle  group, the corresponding Boolean expression can include pins, and buses and bundles of the same bit-width. For example, when the Boolean expression includes a bus and a bundle, both of them must have the same bit-width.
  ///
  /// Pins, buses, and bundles that have the `isolation_enable_condition`  attribute must also have the `always_on`  attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=260.3&end=260.13
  /// ">Reference</a>
  #[liberty(simple)]
  pub isolation_enable_condition: Option<BooleanExpression>,
  /// The `level_shifter_data_pin`  attribute specifies the input data pin on a level shifter cell
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=260.19&end=260.20
  /// ">Reference</a>
  #[liberty(simple)]
  pub level_shifter_data_pin: Option<bool>,
  /// The `level_shifter_enable_pin`  attribute specifies the enable input pin on a level shifter cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=260.30&end=260.31
  /// ">Reference</a>
  #[liberty(simple)]
  pub level_shifter_enable_pin: Option<bool>,
  /// The `map_to_logic`  attribute specifies which logic level to tie a pin when a power-switch cell functions as a normal cell. For more information about power-switch cells
  ///
  /// Valid values are 1 and 0.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=261.11+261.17&end=261.12+261.18
  /// ">Reference</a>
  #[liberty(simple)]
  pub map_to_logic: Option<OneZero>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.50
  /// &end
  /// =228.50
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub max_capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.51
  /// &end
  /// =228.41
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub max_fanout: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.52
  /// &end
  /// =228.52
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub max_input_delta_overdrive_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.53
  /// &end
  /// =228.53
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub max_input_delta_underdrive_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.54
  /// &end
  /// =228.54
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub max_transition: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.55
  /// &end
  /// =228.55
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub min_capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.56
  /// &end
  /// =228.56
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub min_fanout: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.57
  /// &end
  /// =228.57
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub min_period: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.58
  /// &end
  /// =228.58
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub min_pulse_width_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.59
  /// &end
  /// =228.59
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub min_pulse_width_low: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.60
  /// &end
  /// =228.60
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub min_transition: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.61
  /// &end
  /// =228.61
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub multicell_pad_pin: Option<bool>,
  /// In a pin  group, the `nextstate_type` attribute defines the type of the `next_state` attribute.
  /// You define a `next_state`  attribute in an `ff`  group or an `ff_bank`  group.
  ///
  /// Note:
  ///
  /// Specify a `nextstate_type`  attribute to ensure that the synchronous set (or synchronous reset) pin
  /// and the D pin of a sequential cell are not swapped when the design is instantiated.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=228.62+265.12&end=228.62+265.17
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub nextstate_type: Option<NextstateType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.63
  /// &end
  /// =228.63
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub output_signal_level: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.64
  /// &end
  /// =228.64
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub output_signal_level_high: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.65
  /// &end
  /// =228.65
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub output_signal_level_low: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =228.66
  /// &end
  /// =228.66
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub output_voltage: Option<String>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.2
  /// &end
  /// =229.3
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub pin_func_type: Option<PinFuncType>,
  /// The `prefer_tied` attribute describes an input pin of a flip-flop or latch.
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
  #[liberty(simple)]
  pub prefer_tied: Option<OneZero>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.5
  /// &end
  /// =229.5
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub primary_output: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.6
  /// &end
  /// =229.6
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub pulling_current: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.7
  /// &end
  /// =229.7
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub pulling_resistance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.8
  /// &end
  /// =229.8
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub restore_action: Option<logic::Normal>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.9
  /// &end
  /// =229.9
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub restore_edge_type: Option<RestoreEdgeType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.10
  /// &end
  /// =229.10
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub rise_capacitance: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.11
  /// &end
  /// =229.11
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub rise_current_slope_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.12
  /// &end
  /// =229.12
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub rise_current_slope_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.13
  /// &end
  /// =229.13
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub rise_time_after_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.14
  /// &end
  /// =229.14
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub rise_time_before_threshold: Option<f64>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.15
  /// &end
  /// =229.15
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub save_action: Option<logic::Normal>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.16
  /// &end
  /// =229.19
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub signal_type: Option<SignalType>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.20
  /// &end
  /// =229.20
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub slew_control: Option<SlewControl>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.21
  /// &end
  /// =229.21
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub state_function: Option<BooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.22
  /// &end
  /// =229.22
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub test_output_only: Option<bool>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.23
  /// &end
  /// =229.23
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub three_state: Option<LogicBooleanExpression>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.24
  /// &end
  /// =229.24
  /// ">Reference-Instance</a>
  #[liberty(simple)]
  pub x_function: Option<BooleanExpression>,
  /// The `switch_pin`  attribute is a pin-level Boolean attribute.
  /// When it is set to true, it is used to identify the pin as
  /// the switch pin of a coarse-grain switch cell
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=279.19&end=279.20
  /// ">Reference-Definition</a>
  #[liberty(simple)]
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
  #[liberty(complex)]
  pub fall_capacitance_range: Option<(f64, f64)>,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =229.29
  /// &end
  /// =229.29
  /// ">Reference-Instance</a>
  #[liberty(complex)]
  pub rise_capacitance_range: Option<(f64, f64)>,
  // NOTICE: Group Attributes in a pin Group
  // electromigration () { }
  /// The `char_config` group is a group of attributes including simple and complex attributes.
  /// These attributes represent library characterization configuration, and specify the settings
  /// to characterize the library. Use the `char_config` group syntax to apply an attribute value
  /// to a specific characterization model. You can specify multiple complex attributes in the
  /// `char_config` group. You can also specify a single complex attribute multiple times for
  /// different characterization models.
  /// You can also define the `char_config` group within the cell, pin, and timing groups.
  /// However, when you specify the same attribute in multiple `char_config` groups at different
  /// levels, such as at the `library`, `cell`, `pin`, and `timing` levels, the attribute specified at the lower
  /// level gets priority over the ones specified at the higher levels. For example, the pin-level
  /// `char_config` group attributes have higher priority over the library-level `char_config`
  /// group attributes.
  ///
  /// ### Syntax
  /// ``` text
  /// library (library_name) {
  ///   char_config() {
  ///     /* characterization configuration attributes */
  ///   }
  ///   ...
  ///   cell (cell_name) {
  ///     char_config() {
  ///       /* characterization configuration attributes */
  ///     }
  ///     ...
  ///     pin(pin_name) {
  ///       char_config() {
  ///         /* characterization configuration attributes */
  ///       }
  ///       timing() {
  ///         char_config() {
  ///           /* characterization configuration attributes */
  ///         }
  ///       } /* end of timing */
  ///       ...
  ///     } /* end of pin */
  ///     ...
  ///   } /* end of cell */
  ///   ...
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=43.30+44.2&end=43.31+44.37
  /// ">Reference</a>
  #[liberty(group)]
  pub char_config: Option<CharConfig<C>>,
  #[liberty(group)]
  pub internal_power: LibertySet<InternalPower<C>>,
  // TODO
  // max_trans () { }
  // TODO
  // min_pulse_width ()  { }
  // TODO
  // minimum_period ()  { }
  // TODO
  /// In timing analysis, use a tlatch group to describe the relationship between the data pin
  /// and the enable pin on a transparent level-sensitive latch.
  /// You define the tlatch group in a pin group, but it is only effective if you also define the
  /// `timing_model_type` attribute in the cell that the pin belongs to. For more information
  /// about the `timing_model_type` attribute,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=372.33&end=372.37
  /// ">Reference-Definition</a>
  #[liberty(group)]
  pub tlatch: LibertySet<TLatch<C>>,
  /// A `timing` group is defined in a `bundle`, a `bus`, or a `pin` group within a `cell`. The `timing`
  /// group can be used to identify the name or names of multiple `timing` arcs. A `timing` group
  /// identifies multiple `timing` arcs, by identifying a `timing` arc in a `pin` group that has more than
  /// one `related pin` or when the timing arc is part of a `bundle` or a `bus`.
  /// The following syntax shows a `timing` group in a `pin` group within a `cell` group.
  ///
  /// ### Syntax
  /// ``` text
  /// library (namestring) {
  ///   cell (name) {
  ///     pin (name) {
  ///       timing (name | name_list) {
  ///         ... timing description ...
  ///       }
  ///     }
  ///   }
  /// }
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=90.29+91.2&end=90.41+91.5
  /// ">Reference</a>
  #[liberty(group)]
  pub timing: LibertySet<Timing<C>>,
  /// Use the `receiver_capacitance`  group to specify capacitance values
  /// for composite current source (CCS) receiver modeling at the pin level.
  ///
  /// Groups
  ///
  /// For two-segment receiver capacitance model
  /// + `receiver_capacitance1_fall`
  /// + `receiver_capacitance1_rise`
  /// + `receiver_capacitance2_fall`
  /// + `receiver_capacitance2_rise`
  ///
  /// For multisegment receiver capacitance model
  /// + `receiver_capacitance_fall`
  /// + `receiver_capacitance_rise`
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=316.5&end=316.31
  /// ">Reference-Definition</a>
  #[liberty(group)]
  pub receiver_capacitance: LibertySet<ReceiverCapacitance<C>>,
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
  #[liberty(group)]
  pub input_ccb: LibertySet<CCSNStage<C>>,
  #[liberty(group)]
  pub output_ccb: LibertySet<CCSNStage<C>>,
  #[liberty(group)]
  pub ccsn_first_stage: LibertySet<CCSNStage<C>>,
  #[liberty(group)]
  pub ccsn_last_stage: LibertySet<CCSNStage<C>>,
}

impl<C: 'static + Ctx> GroupFn<C> for Pin<C> {}
#[duplicate::duplicate_item(
  BusBundle;
  [Bus];
  [Bundle];
)]
impl<C: 'static + Ctx> GroupFn<C> for BusBundle<C> {
  fn after_build(&mut self, _: &mut BuilderScope<C>) {
    let mut pin =
      LibertySet::with_capacity_and_hasher(self.pin.len(), RandomState::default());
    for p in core::mem::take(&mut self.pin) {
      if let Some(names) = FlattenNameAttri::ungroup(&p.name) {
        pin.extend(names.map(|name| {
          let mut _p = p.clone();
          _p.name = name;
          _p
        }));
      } else {
        _ = pin.insert(p);
      }
    }
    self.pin = pin;
  }
}

#[cfg(test)]
mod test {
  use crate::DefaultCtx;

  #[test]
  fn pin_name_list() {
    // let mut scope = Default::default();
    // let builder = super::PinBuilder::<DefaultCtx>::default();
    // let a = <super::Pin as crate::ast::ParsingBuilder>::build_iter(builder, &mut scope);
    // a.flat_map(f)
    let cell = crate::ast::test_parse_fmt::<crate::Cell<DefaultCtx>>(
      r#"(test_cell){
        pin (A) {}
        pin (B,C,D,E) {}
      }"#,
      r#"
liberty_db::cell::Cell (test_cell) {
| pin (A) {
| }
| pin (B) {
| }
| pin (C) {
| }
| pin (D) {
| }
| pin (E) {
| }
}"#,
    );
  }
}
