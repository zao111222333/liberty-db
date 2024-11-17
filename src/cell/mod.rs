//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
mod items;
#[cfg(test)]
mod test;
pub use items::*;

use crate::{
  ast::{Attributes, GroupComments, GroupFn, GroupSet},
  common::table::TableLookUp2D,
  expression::{FFBank, Latch, LatchBank, FF},
  pin::{AntennaDiodeType, Bundle, Pin},
  ArcStr, NotNan,
};

/// cell
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cell {
  #[id(borrow = "&str")]
  #[size = 8]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub area: Option<NotNan<f64>>,
  /// The `dont_use`  attribute with a true value indicates
  /// that a cell should not be added to a design
  /// during optimization
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=107.3&end=107.4
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub dont_use: Option<bool>,
  /// The `dont_touch`  attribute with a true
  /// value indicates that all instances of the cell must
  /// remain in the network.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=106.21&end=106.22
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub dont_touch: Option<bool>,
  /// CellId
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub single_bit_degenerate: Option<ArcStr>,
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub driver_waveform_rise: Option<ArcStr>,
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub driver_waveform_fall: Option<ArcStr>,
  /// The `always_on`  simple attribute models always-on cells or signal pins. Specify the attribute at the cell level to determine whether a cell is an always-on cell. Specify the attribute at the pin level to determine whether a pin is an always-on signal pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=100.73&end=100.75
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
  /// You can use the `clock_gating_integrated_cell` attribute to enter specific
  /// values that determine which integrated cell functionality the clock-gating tool uses.
  ///
  /// Syntax:
  /// ```text
  /// clock_gating_integrated_cell:generic|value_id;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=103.19&end=103.24
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub clock_gating_integrated_cell: Option<ClockGatingIntegratedCell>,
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub cell_footprint: Option<ArcStr>,
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub cell_leakage_power: Option<NotNan<f64>>,
  /// The `em_temp_degradation_factor` attribute specifies the electromigration
  /// exponential degradation factor
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.17&end=109.18
  /// ">Reference</a>
  #[size = 16]
  #[liberty(simple(type = Option))]
  pub em_temp_degradation_factor: Option<NotNan<f64>>,
  /// interprets a combination timing arc between the clock pin and the output pin as a rising edge arc or as a falling edge arc
  ///
  /// Valid values are `rising_edge_clock_cell`  and `falling_edge_clock_cell`.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.29+109.36&end=109.30+109.37
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub fpga_cell_type: Option<FpgaCellType>,
  /// Use the `fpga_isd`  attribute to reference the drive,
  /// `io_type`, and `slew`  information contained in a library-level `fpga_isd`  group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.3&end=110.4
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub fpga_isd: Option<ArcStr>,
  /// Indicates that the `timing` arcs are interpreted according
  ///  to interface `timing` specifications semantics.
  /// If this attribute is missing or its value is set to false,
  /// the `timing` relationships are interpreted as those of
  /// a regular cell rather than according to interface timing
  /// specification semantics.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.14&end=110.17
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub interface_timing: Option<bool>,
  /// Use the io_type  attribute to define the I/O standard used by this I/O cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.25&end=110.26
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub io_type: Option<ArcStr>,
  /// The `is_pad`  attribute identifies a pad pin on
  /// any I/O cell. You can also specify the `is_pad` attribute
  /// on PG pins.
  /// The valid values are true  and false.
  /// If the cell-level `pad_cell` attribute is specified on
  /// a I/O cell, the `is_pad`  attribute must be set to true
  /// in either a `pg_pin`  group or on a signal pin for that cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=111.5&end=111.8
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_pad: Option<bool>,
  /// The `is_pll_cell`  Boolean attribute identifies a phase-locked loop
  /// cell. A phase-locked loop (PLL) is a feedback control system
  /// that automatically adjusts the phase of a locally-generated signal
  /// to match the phase of an input signal.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=111.5&end=111.8
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_pll_cell: Option<bool>,
  /// The cell-level `is_clock_gating_cell` attribute specifies that a cell is for clock gating.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.8&end=113.9
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_clock_gating_cell: Option<bool>,
  /// The `is_clock_isolation_cell`  attribute identifies a cell as a clock-isolation cell.
  /// The default is false, meaning that the cell is a standard cell.
  /// For information about pin-level attributes of the clock-isolation cell,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.18&end=113.20
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_clock_isolation_cell: Option<bool>,
  /// The cell-level `is_isolation_cell`  attribute specifies that a
  /// cell is an isolation cell.
  /// The pin-level `isolation_cell_enable_pin`  attribute specifies
  /// the enable input pin for the isolation cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.27&end=113.29
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_isolation_cell: Option<bool>,
  /// The cell-level `is_level_shifter`  attribute specifies
  /// that a cell is a level shifter cell.
  /// The pin-level `level_shifter_enable_pin`  
  /// attribute specifies the enable input pin for
  /// the level shifter cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.5&end=114.7
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_level_shifter: Option<bool>,
  /// The `is_macro_cell`  simple Boolean attribute identifies
  /// whether a cell is a macro cell.
  /// If the attribute is set to true, the cell is a macro cell.
  /// If it is set to false, the cell is not a macro cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.15&end=114.17
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_macro_cell: Option<bool>,
  /// The `is_soi`  attribute specifies that the cell is a
  /// silicon-on-insulator (SOI) cell.
  /// The default is false, which means that the cell is a
  /// bulk-CMOS cell.
  ///
  /// If the `is_soi`  attribute is specified at both the
  /// library and cell levels,
  /// the cell-level value overrides the library-level value
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.25&end=114.28
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub is_soi: Option<bool>,
  /// The `level_shifter_type`  attribute specifies the
  /// voltage conversion type that is supported.
  /// Valid values are:
  ///
  /// + `LH`: Low to High
  /// + `HL`: High to Low
  /// + `HL_LH`: High to Low and Low to HighThe
  ///
  /// `level_shifter_type`  attribute is optional
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=115.9&end=115.17
  /// ">Reference</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub level_shifter_type: Option<LevelShifterType>,
  /// The `retention_cell`  attribute identifies a retention cell. The `retention_cell_style` value is a random string
  ///
  /// Syntax
  /// ``` text
  /// retention_cell : retention_cell_style ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=118.21+118.24&end=118.22+118.25
  /// ">Reference</a>
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub retention_cell: Option<ArcStr>,
  /// The `switch_cell_type`  cell-level attribute specifies
  /// the type of the switch cell for direct inference.
  ///
  /// Syntax:
  /// ``` text
  /// switch_cell_type : coarse_grain | fine_grain;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=210.8&end=210.13
  /// ">Reference-Definition</a>
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub switch_cell_type: Option<SwitchCellType>,
  /// Use the `dc_current`  group to specify the input and output voltage values
  /// of a two-dimensional current table for a channel-connecting block.
  ///
  /// Use `index_1`  to represent the input voltage
  /// and `index_2`  to represent the output voltage.
  /// The `values`  attribute of the group lists the relative
  /// channel-connecting block DC current values in library units measured
  /// at the channel-connecting block output node.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=289.2+288.24&end=289.4+288.25
  /// ">Reference-Definition</a>
  #[size = 240]
  #[liberty(group(type = Option))]
  pub dc_current: Option<TableLookUp2D>,
  /// The `input_voltage_range`  attribute specifies the allowed
  /// voltage range of the level-shifter input pin and the voltage
  /// range for all input pins of the cell under all possible operating conditions
  /// (defined across multiple libraries).
  ///
  /// The attribute defines two floating values:
  ///  the first is the lower bound, and second is the upper bound.
  ///
  /// The `input_voltage_range`  syntax differs from the pin-level
  /// `input_signal_level_low` and `input_signal_level_high`  syntax in the following ways:
  ///
  /// + The `input_signal_level_low`  and `input_signal_level_high`  attributes are defined
  /// on the input pins under one operating condition.
  /// + The `input_signal_level_low`  and `input_signal_level_high`  attributes are used
  /// to specify the partial voltage swing of an input pin (that is, to prevent from
  /// swinging from ground rail VSS to full power rail VDD).
  /// Note that `input_voltage_range`  is not related to the voltage swing.
  ///
  /// Note:
  ///
  /// The `input_voltage_range`  and `output_voltage_range`  attributes
  /// should always be defined together.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=122.7&end=122.23
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex(type = Option))]
  pub input_voltage_range: Option<(NotNan<f64>, NotNan<f64>)>,
  /// The `input_voltage_range`  and `output_voltage_range`  attributes
  /// should always be defined together.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=122.7&end=122.23
  /// ">Reference</a>
  #[size = 24]
  #[liberty(complex(type = Option))]
  pub output_voltage_range: Option<(NotNan<f64>, NotNan<f64>)>,
  /// Use the pin_opposite attribute to describe functionally opposite (logically inverse) groups
  /// of input or output pins.
  /// Syntax
  ///
  /// ``` text
  /// pin_opposite ("name_list1", "name_list2") ;
  /// ```
  ///
  /// + `name_list1`: A name_list of output pins requires the supplied output values to be opposite.
  /// + `name_list2`: A name_list of input pins requires the supplied input values to be opposite.
  ///
  /// In the following example, pins IP and OP are logically inverse.
  /// ``` text
  /// pin_opposite ("IP", "OP") ;
  /// ```
  /// The pin_opposite attribute also incorporates the functionality of the `pin_equal` complex
  /// attribute.
  ///
  /// In the following example, Q1, Q2, and Q3 are equal; QB1 and QB2 are equal; and the pins
  /// in the first group are opposite of the pins in the second group.
  /// ``` text
  /// pin_opposite ("Q1 Q2 Q3", "QB1 QB2") ;
  /// ```
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=124.9&end=124.22
  /// ">Reference</a>
  #[size = 96]
  #[liberty(complex(type = Option))]
  pub pin_opposite: Option<PinOpposite>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<PgPin>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<PgPin>::deserialize_with")]
  pub pg_pin: GroupSet<PgPin>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<FF>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<FF>::deserialize_with")]
  pub ff: GroupSet<FF>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<FFBank>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<FFBank>::deserialize_with")]
  pub ff_bank: GroupSet<FFBank>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Latch>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Latch>::deserialize_with")]
  pub latch: GroupSet<Latch>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<LatchBank>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<LatchBank>::deserialize_with")]
  pub latch_bank: GroupSet<LatchBank>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<LeakagePower>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<LeakagePower>::deserialize_with")]
  pub leakage_power: GroupSet<LeakagePower>,
  #[size = 168]
  #[liberty(group(type = Option))]
  pub statetable: Option<Statetable>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Pin>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Pin>::deserialize_with")]
  pub pin: GroupSet<Pin>,
  #[size = 24]
  #[liberty(group(type = Vec))]
  /// The `test_cell`  group is in a `cell` group or `model` group.
  /// It models only the nontest behavior of a scan cell, which
  /// is described by an `ff`, `ff_bank`, `latch`, `latch_bank`  or `statetable`  statement
  /// and `pin` function attributes
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=218.9&end=218.11
  /// ">Reference</a>
  pub test_cell: Vec<TestCell>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Bundle>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Bundle>::deserialize_with")]
  pub bundle: GroupSet<Bundle>,
}
impl GroupFn for Cell {}

/// The `test_cell`  group is in a `cell` group or `model` group.
///
/// It models only the nontest behavior of a scan cell, which
/// is described by an `ff`, `ff_bank`, `latch`, `latch_bank`  or `statetable`  statement
/// and `pin` function attributes
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=218.9&end=218.11
/// ">Reference</a>
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(mut_set::derive::Dummy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TestCell {
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<FF>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<FF>::deserialize_with")]
  pub ff: GroupSet<FF>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<FFBank>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<FFBank>::deserialize_with")]
  pub ff_bank: GroupSet<FFBank>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Latch>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Latch>::deserialize_with")]
  pub latch: GroupSet<Latch>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<LatchBank>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<LatchBank>::deserialize_with")]
  pub latch_bank: GroupSet<LatchBank>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Pin>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Pin>::deserialize_with")]
  pub pin: GroupSet<Pin>,
  #[size = 64]
  #[liberty(group(type = Set))]
  #[serde(serialize_with = "GroupSet::<Statetable>::serialize_with")]
  #[serde(deserialize_with = "GroupSet::<Statetable>::deserialize_with")]
  pub statetable: GroupSet<Statetable>,
}

impl GroupFn for TestCell {}
