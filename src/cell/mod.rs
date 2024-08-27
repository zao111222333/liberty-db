//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use crate::{
  ast::{AttributeList, GroupComments, GroupFn},
  expression::{FFBank, Latch, LatchBank, FF},
  pin::{Bundle, Pin},
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
  /// CellId
  #[liberty(simple(type=Option))]
  pub single_bit_degenerate: Option<ArcStr>,
  #[liberty(simple(type = Option))]
  pub driver_waveform_rise: Option<ArcStr>,
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
  pub driver_waveform_fall: Option<ArcStr>,
  #[liberty(simple(type = Option))]
  pub cell_footprint: Option<ArcStr>,
  #[liberty(simple(type=Option))]
  pub cell_leakage_power: Option<f64>,
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
  #[liberty(complex(type=Option))]
  pub output_voltage_range: Option<(NotNan<f64>, NotNan<f64>)>,
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
  #[liberty(group(type=Set))]
  // TODO:
  pub bundle: GroupSet<Bundle>,
}
impl GroupFn for Cell {}

#[cfg(test)]
mod test {
  use super::Cell;
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
| | | timing_type : combinational;
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
| | | timing_type : combinational;
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
