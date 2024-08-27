//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use crate::{
  ast::{AttributeList, GroupComments, GroupFn},
  common::table::TableLookUp2D,
  expression::{FFBank, Latch, LatchBank, FF},
  pin::{AntennaDiodeType, Bundle, Pin},
  ArcStr, GroupSet, NotNan,
};
mod items;
pub use items::*;

/// cell
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item(
  sort,
  macro(derive(Debug, Clone, Default);)
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cell {
  #[id]
  #[liberty(name)]
  pub name: ArcStr,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  #[liberty(simple(type=Option))]
  pub area: Option<f64>,
  /// The `dont_use`  attribute with a true value indicates
  /// that a cell should not be added to a design
  /// during optimization
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=107.3&end=107.4
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub dont_use: Option<bool>,
  /// The `dont_touch`  attribute with a true
  /// value indicates that all instances of the cell must
  /// remain in the network.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=106.21&end=106.22
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub dont_touch: Option<bool>,
  /// CellId
  #[liberty(simple(type=Option))]
  pub single_bit_degenerate: Option<ArcStr>,
  #[liberty(simple(type = Option))]
  pub driver_waveform_rise: Option<ArcStr>,
  #[liberty(simple(type = Option))]
  pub driver_waveform_fall: Option<ArcStr>,
  /// The `always_on`  simple attribute models always-on cells or signal pins. Specify the attribute at the cell level to determine whether a cell is an always-on cell. Specify the attribute at the pin level to determine whether a pin is an always-on signal pin.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=100.73&end=100.75
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
  #[liberty(simple(type = Option))]
  pub clock_gating_integrated_cell: Option<ClockGatingIntegratedCell>,
  #[liberty(simple(type = Option))]
  pub cell_footprint: Option<ArcStr>,
  #[liberty(simple(type=Option))]
  pub cell_leakage_power: Option<f64>,
  /// The `em_temp_degradation_factor` attribute specifies the electromigration
  /// exponential degradation factor
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.17&end=109.18
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub em_temp_degradation_factor: Option<f64>,
  /// interprets a combination timing arc between the clock pin and the output pin as a rising edge arc or as a falling edge arc
  ///
  /// Valid values are `rising_edge_clock_cell`  and `falling_edge_clock_cell`.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=109.29+109.36&end=109.30+109.37
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub fpga_cell_type: Option<FpgaCellType>,
  /// Use the `fpga_isd`  attribute to reference the drive,
  /// `io_type`, and `slew`  information contained in a library-level `fpga_isd`  group.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.3&end=110.4
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
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
  #[liberty(simple(type=Option))]
  pub interface_timing: Option<bool>,
  /// Use the io_type  attribute to define the I/O standard used by this I/O cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=110.25&end=110.26
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
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
  #[liberty(simple(type=Option))]
  pub is_pad: Option<bool>,
  /// The `is_pll_cell`  Boolean attribute identifies a phase-locked loop
  /// cell. A phase-locked loop (PLL) is a feedback control system
  /// that automatically adjusts the phase of a locally-generated signal
  /// to match the phase of an input signal.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=111.5&end=111.8
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub is_pll_cell: Option<bool>,
  /// The cell-level `is_clock_gating_cell` attribute specifies that a cell is for clock gating.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.8&end=113.9
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub is_clock_gating_cell: Option<bool>,
  /// The `is_clock_isolation_cell`  attribute identifies a cell as a clock-isolation cell.
  /// The default is false, meaning that the cell is a standard cell.
  /// For information about pin-level attributes of the clock-isolation cell,
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.18&end=113.20
  /// ">Reference</a>
  #[liberty(simple(type = Option))]
  pub is_clock_isolation_cell: Option<bool>,
  /// The cell-level `is_isolation_cell`  attribute specifies that a
  /// cell is an isolation cell.
  /// The pin-level `isolation_cell_enable_pin`  attribute specifies
  /// the enable input pin for the isolation cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=113.27&end=113.29
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub is_isolation_cell: Option<bool>,
  /// The cell-level `is_level_shifter`  attribute specifies
  /// that a cell is a level shifter cell.
  /// The pin-level `level_shifter_enable_pin`  
  /// attribute specifies the enable input pin for
  /// the level shifter cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.5&end=114.7
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub is_level_shifter: Option<bool>,
  /// The `is_macro_cell`  simple Boolean attribute identifies
  /// whether a cell is a macro cell.
  /// If the attribute is set to true, the cell is a macro cell.
  /// If it is set to false, the cell is not a macro cell.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=114.15&end=114.17
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
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
  #[liberty(simple(type=Option))]
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
  #[liberty(simple(type=Option))]
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
  #[liberty(simple(type=Option))]
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
  #[liberty(complex(type=Option))]
  pub input_voltage_range: Option<(NotNan<f64>, NotNan<f64>)>,
  /// The `input_voltage_range`  and `output_voltage_range`  attributes
  /// should always be defined together.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=122.7&end=122.23
  /// ">Reference</a>
  #[liberty(complex(type=Option))]
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
  #[liberty(complex(type=Option))]
  pub pin_opposite: Option<PinOpposite>,
  #[liberty(group(type=Set))]
  pub pg_pin: GroupSet<PgPin>,
  #[liberty(group(type=Set))]
  pub ff: GroupSet<FF>,
  #[liberty(group(type=Set))]
  pub ff_bank: GroupSet<FFBank>,
  #[liberty(group(type=Set))]
  pub latch: GroupSet<Latch>,
  #[liberty(group(type=Set))]
  pub latch_bank: GroupSet<LatchBank>,
  #[liberty(group(type=Set))]
  pub leakage_power: GroupSet<LeakagePower>,
  #[liberty(group(type=Option))]
  pub statetable: Option<Statetable>,
  #[liberty(group(type=Set))]
  pub pin: GroupSet<Pin>,
  #[liberty(group(type=Option))]
  /// The `test_cell`  group is in a `cell` group or `model` group.
  /// It models only the nontest behavior of a scan cell, which
  /// is described by an `ff`, `ff_bank`, `latch`, `latch_bank`  or `statetable`  statement
  /// and `pin` function attributes
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=218.9&end=218.11
  /// ">Reference</a>
  pub test_cell: Option<TestCell>,
  #[liberty(group(type=Set))]
  // TODO:
  pub bundle: GroupSet<Bundle>,
}
impl GroupFn for Cell {}

/// The `test_cell`  group is in a `cell` group or `model` group.
/// It models only the nontest behavior of a scan cell, which
/// is described by an `ff`, `ff_bank`, `latch`, `latch_bank`  or `statetable`  statement
/// and `pin` function attributes
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=218.9&end=218.11
/// ">Reference</a>
#[derive(Debug, Default, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TestCell {
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: AttributeList,
  #[liberty(group(type=Set))]
  pub ff: GroupSet<FF>,
  #[liberty(group(type=Set))]
  pub ff_bank: GroupSet<FFBank>,
  #[liberty(group(type=Set))]
  pub latch: GroupSet<Latch>,
  #[liberty(group(type=Set))]
  pub latch_bank: GroupSet<LatchBank>,
  #[liberty(group(type=Set))]
  pub pin: GroupSet<Pin>,
  #[liberty(group(type=Set))]
  pub statetable: GroupSet<Statetable>,
}

impl GroupFn for TestCell {}
#[cfg(test)]
mod test {
  use super::Cell;
  /// In the following example, pins IP and OP are logically inverse.
  /// ``` text
  /// pin_opposite ("IP", "OP") ;
  /// ```
  /// The `pin_opposite` attribute also incorporates the functionality of the `pin_equal` complex
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
  #[test]
  fn example_pin_opposite() {
    let cell = crate::ast::test_parse_fmt::<Cell>(
      r#"(test) {
        pin_opposite ("Q1 Q2 Q3", "QB1 QB2") ;
      }"#,
      r#"
liberty_db::cell::Cell (test) {
| pin_opposite ("Q1 Q2 Q3", "QB1 QB2");
}"#,
    );
  }
  /// Example 23 A multibit register containing four rising-edge-triggered D flip-flops
  /// with clear  and preset is shown in Figure 1 and Example 23
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=167.32&end=167.33
  /// ">Reference</a>
  #[test]
  fn example23() {
    let cell = crate::ast::test_parse_fmt::<Cell>(
      r#"(dff4) {
        area : 1 ;
        pin (CLK) {
            direction : input ;
            capacitance : 0 ;
            min_pulse_width_low  : 3 ;
            min_pulse_width_high : 3 ;
        }
        bundle (D) {
            members(D1, D2, D3, D4);
            nextstate_type : data;
            direction : input ;
            capacitance : 0 ;
            timing() {
                related_pin     : "CLK" ;
                timing_type     : setup_rising ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
            timing() {
                related_pin     : "CLK" ;
                timing_type     : hold_rising ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
        pin (CLR) {
            direction : input ;
            capacitance : 0 ;
            timing() {
                related_pin     : "CLK" ;
                timing_type     : recovery_rising ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
        pin (PRE) {
            direction : input ;
            capacitance : 0 ;
            timing() {
                related_pin     : "CLK" ;
                timing_type     : recovery_rising ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
        ff_bank (IQ, IQN, 4) {
            next_state : "D" ;
            clocked_on : "CLK" ;
            clear : "CLR’" ;
            preset : "PRE’" ;
            clear_preset_var1 : L ;
            clear_preset_var2 : L ;
        }
        bundle (Q) {
            members(Q1, Q2, Q3, Q4);
            direction : output ;
            function : "(IQ)" ;
            timing() {
                related_pin     : "CLK" ;
                timing_type     : rising_edge ;
                cell_rise(scalar) {
                    values (" 2.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 2.0 ") ;
                }
            }
            timing() {
                related_pin     : "PRE" ;
                timing_type     : preset ;
                timing_sense    : negative_unate ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
            }
            timing() {
                related_pin     : "CLR" ;
                timing_type     : clear ;
                timing_sense    : positive_unate ;
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
        bundle (QN) {
            members(Q1N, Q2N, Q3N, Q4N);
            direction : output ;
            function : "IQN" ;
            timing() {
                related_pin     : "CLK" ;
                timing_type     : rising_edge ;
                cell_rise(scalar) {
                    values (" 2.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 2.0 ") ;
                }
            }
            timing() {
                related_pin     : "PRE" ;
                timing_type     : clear ;
                timing_sense    : positive_unate ;
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
            timing() {
                related_pin     : "CLR" ;
                timing_type     : preset ;
                timing_sense    : negative_unate ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
    } /* end of cell dff4 */
    "#,
      r#"
liberty_db::cell::Cell (dff4) {
| area : 1.0;
| ff_bank (IQ, IQN, 4) {
| | clear : "!CLR";
| | clear_preset_var1 : L;
| | clear_preset_var2 : L;
| | clocked_on : "CLK";
| | next_state : "D";
| | preset : "!PRE";
| }
| pin (CLK) {
| | capacitance : 0.0;
| | direction : input;
| | min_pulse_width_high : 3.0;
| | min_pulse_width_low : 3.0;
| }
| pin (CLR) {
| | capacitance : 0.0;
| | direction : input;
| | timing () {
| | | related_pin : CLK;
| | | timing_type : recovery_rising;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| pin (PRE) {
| | capacitance : 0.0;
| | direction : input;
| | timing () {
| | | related_pin : CLK;
| | | timing_type : recovery_rising;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| bundle (D) {
| | members (D1, D2, D3, D4);
| | direction : input;
| | capacitance : 0.0;
| | nextstate_type : data;
| | timing () {
| | | related_pin : CLK;
| | | timing_type : hold_rising;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : CLK;
| | | timing_type : setup_rising;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| bundle (Q) {
| | members (Q1, Q2, Q3, Q4);
| | direction : output;
| | function : "IQ";
| | timing () {
| | | related_pin : CLK;
| | | timing_type : rising_edge;
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : CLR;
| | | timing_sense : positive_unate;
| | | timing_type : clear;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : PRE;
| | | timing_sense : negative_unate;
| | | timing_type : preset;
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| bundle (QN) {
| | members (Q1N, Q2N, Q3N, Q4N);
| | direction : output;
| | function : "IQN";
| | timing () {
| | | related_pin : CLK;
| | | timing_type : rising_edge;
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : CLR;
| | | timing_sense : negative_unate;
| | | timing_type : preset;
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : PRE;
| | | timing_sense : positive_unate;
| | | timing_type : clear;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
}"#,
    );
  }
  /// Example 27 shows a `latch_bank`  group for a multibit register containing four rising-edge-triggered D latches
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=187.42&end=187.43
  /// ">Reference</a>
  #[test]
  fn example27() {
    let cell = crate::ast::test_parse_fmt::<Cell>(
      r#"(latch4) {
        area: 16;
        pin (G) {     /* gate enable signal, active-high */
            direction : input;
        }
        bundle (D) {       /* data input with four member pins */
            members(D1, D2, D3, D4);/*must be 1st bundle attribute*/
            direction : input;
        }
        bundle (Q) {
            members(Q1, Q2, Q3, Q4);
            direction : output;
            function : "IQ" ;
        }
        bundle (QN) {
            members (Q1N, Q2N, Q3N, Q4N);
            direction : output;
            function : "IQN";
        }
        latch_bank(IQ, IQN, 4) {
            enable : "G" ;
            data_in : "D" ;
        }
    }
    "#,
      r#"
liberty_db::cell::Cell (latch4) {
| area : 16.0;
| latch_bank (IQ, IQN, 4) {
| | enable : "G";
| | data_in : "D";
| }
| pin (G) {
| | direction : input;
| }
| bundle (D) {
| | members (D1, D2, D3, D4);
| | direction : input;
| }
| bundle (Q) {
| | members (Q1, Q2, Q3, Q4);
| | direction : output;
| | function : "IQ";
| }
| bundle (QN) {
| | members (Q1N, Q2N, Q3N, Q4N);
| | direction : output;
| | function : "IQN";
| }
}"#,
    );
  }
  /// Example PLL
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=112.3+113.2&end=112.53+113.6
  /// ">Reference</a>
  #[test]
  fn example_pll() {
    let cell = crate::ast::test_parse_fmt::<Cell>(
      r#"(my_pll) {
        is_pll_cell : true;
        pin( REFCLK ) {
            direction : input;
            is_pll_reference_pin : true;
        }
        pin( FBKCLK ) {
            direction : input;
            is_pll_feedback_pin : true;
        }
        pin (OUTCLK1) {
            direction : output;
            is_pll_output_pin : true;
            timing() { /*Timing Arc*/
                related_pin: "REFCLK";
                timing_type: combinational_rise;
                timing_sense: positive_unate;
                cell_rise(scalar) { /*Can be a LUT as well to support NLDM and CCS models*/
                    values("0.0")
                }
            }
            timing() { /*Timing Arc*/
                related_pin: "REFCLK";
                timing_type: combinational_fall;
                timing_sense: positive_unate;
                cell_fall(scalar) {
                    values("0.0")
                }
            }
        }
        pin (OUTCLK2) {
            direction : output;
            is_pll_output_pin : true;
            timing() { /*Timing Arc*/
                related_pin: "REFCLK";
                timing_type: combinational_rise;
                timing_sense: positive_unate;
                cell_rise(scalar) { /*Can be a LUT as well to support NLDM and CCS models*/
                    values("0.0")
                }
            }
            timing() { /*Timing Arc*/
                related_pin: "REFCLK";
                timing_type: combinational_fall;
                timing_sense: positive_unate;
                cell_fall(scalar) {
                    values("0.0")
                }
            }
        }
    }"#,
      r#"
liberty_db::cell::Cell (my_pll) {
| is_pll_cell : true;
| pin (FBKCLK) {
| | direction : input;
| | is_pll_feedback_pin : true;
| }
| pin (OUTCLK1) {
| | direction : output;
| | is_pll_output_pin : true;
| | timing () {
| | | related_pin : REFCLK;
| | | timing_sense : positive_unate;
| | | timing_type : combinational_fall;
| | | cell_fall (scalar) {
| | | | values ("0.0");
| | | }
| | }
| | timing () {
| | | related_pin : REFCLK;
| | | timing_sense : positive_unate;
| | | timing_type : combinational_rise;
| | | cell_rise (scalar) {
| | | | values ("0.0");
| | | }
| | }
| }
| pin (OUTCLK2) {
| | direction : output;
| | is_pll_output_pin : true;
| | timing () {
| | | related_pin : REFCLK;
| | | timing_sense : positive_unate;
| | | timing_type : combinational_fall;
| | | cell_fall (scalar) {
| | | | values ("0.0");
| | | }
| | }
| | timing () {
| | | related_pin : REFCLK;
| | | timing_sense : positive_unate;
| | | timing_type : combinational_rise;
| | | cell_rise (scalar) {
| | | | values ("0.0");
| | | }
| | }
| }
| pin (REFCLK) {
| | direction : input;
| | is_pll_reference_pin : true;
| }
}"#,
    );
  }
  /// Example 28 a multibit register containing four high-enable D latches with the clear  attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=190.11&end=190.12
  /// ">Reference</a>
  #[test]
  fn example28() {
    let cell = crate::ast::test_parse_fmt::<Cell>(
      r#"(DLT2) {/* note: 0 hold time */
        area : 1 ;
        single_bit_degenerate : FDB ;
        pin (EN) {
            direction : input ;
            capacitance : 0 ;
            min_pulse_width_low : 3 ;
            min_pulse_width_high : 3 ;
        }
        bundle (D) {
            members(DA, DB, DC, DD);
            direction : input ;
            capacitance : 0 ;
            timing() {
                related_pin : "EN" ;
                timing_type : setup_falling ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
            timing() {
                related_pin : "EN" ;
                timing_type : hold_falling ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
        bundle (CLR) {
            members(CLRA, CLRB, CLRC, CLRD);
            direction : input ;
            capacitance : 0 ;
            timing() {
                related_pin : "EN" ;
                timing_type : recovery_falling ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
        bundle (PRE) {
        members(PREA, PREB, PREC, PRED);
        direction : input ;
        capacitance : 0 ;
        timing() {
            related_pin : "EN" ;
            timing_type : recovery_falling ;
            cell_rise(scalar) {
                values (" 1.0 ") ;
            }
            cell_fall(scalar) {
                values (" 1.0 ") ;
            }
            }
        }
        latch_bank(IQ, IQN, 4) {
            data_in : "D" ;
            enable : "EN" ;
            clear : "CLR’" ;
            preset : "PRE’" ;
            clear_preset_var1 : H ;
            clear_preset_var2 : H ;
        }
        bundle (Q) {
            members(QA, QB, QC, QD);
            direction : output ;
            function : "IQ" ;
            timing() {
                related_pin : "D" ;
                cell_rise(scalar) {
                    values (" 2.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 2.0 ") ;
                }
            }
            timing() {
                related_pin : "EN" ;
                timing_type : rising_edge ;
                cell_rise(scalar) {
                    values (" 2.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 2.0 ") ;
                }
            }
            timing() {
                related_pin : "CLR" ;
                timing_type : clear ;
                timing_sense : positive_unate ;
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
            timing() {
                related_pin : "PRE" ;
                timing_type : preset ;
                timing_sense : negative_unate ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
        bundle (QN) {
            members(QNA, QNB, QNC, QND);
            direction : output ;
            function : "IQN" ;
            timing() {
                related_pin : "D" ;
                cell_rise(scalar) {
                    values (" 2.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 2.0 ") ;
                }
            }
            timing() {
                related_pin : "EN" ;
                timing_type : rising_edge ;
                cell_rise(scalar) {
                    values (" 2.0 ") ;
                }
                cell_fall(scalar) {
                    values (" 2.0 ") ;
                }
            }
            timing() {
                related_pin : "CLR" ;
                timing_type : preset ;
                timing_sense : negative_unate ;
                cell_rise(scalar) {
                    values (" 1.0 ") ;
                }
            }
            timing() {
                related_pin : "PRE" ;
                timing_type : clear ;
                timing_sense : positive_unate ;
                cell_fall(scalar) {
                    values (" 1.0 ") ;
                }
            }
        }
    } /* end of cell DLT2
    "#,
      r#"
liberty_db::cell::Cell (DLT2) {
| area : 1.0;
| single_bit_degenerate : FDB;
| latch_bank (IQ, IQN, 4) {
| | clear : "!CLR";
| | clear_preset_var1 : H;
| | clear_preset_var2 : H;
| | enable : "EN";
| | data_in : "D";
| | preset : "!PRE";
| }
| pin (EN) {
| | capacitance : 0.0;
| | direction : input;
| | min_pulse_width_high : 3.0;
| | min_pulse_width_low : 3.0;
| }
| bundle (CLR) {
| | members (CLRA, CLRB, CLRC, CLRD);
| | direction : input;
| | capacitance : 0.0;
| | timing () {
| | | related_pin : EN;
| | | timing_type : recovery_falling;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| bundle (D) {
| | members (DA, DB, DC, DD);
| | direction : input;
| | capacitance : 0.0;
| | timing () {
| | | related_pin : EN;
| | | timing_type : hold_falling;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : EN;
| | | timing_type : setup_falling;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| bundle (PRE) {
| | members (PREA, PREB, PREC, PRED);
| | direction : input;
| | capacitance : 0.0;
| | timing () {
| | | related_pin : EN;
| | | timing_type : recovery_falling;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| bundle (Q) {
| | members (QA, QB, QC, QD);
| | direction : output;
| | function : "IQ";
| | timing () {
| | | related_pin : CLR;
| | | timing_sense : positive_unate;
| | | timing_type : clear;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : D;
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : EN;
| | | timing_type : rising_edge;
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : PRE;
| | | timing_sense : negative_unate;
| | | timing_type : preset;
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
| bundle (QN) {
| | members (QNA, QNB, QNC, QND);
| | direction : output;
| | function : "IQN";
| | timing () {
| | | related_pin : CLR;
| | | timing_sense : negative_unate;
| | | timing_type : preset;
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : D;
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : EN;
| | | timing_type : rising_edge;
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : PRE;
| | | timing_sense : positive_unate;
| | | timing_type : clear;
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | }
| }
}"#,
    );
  }
}
