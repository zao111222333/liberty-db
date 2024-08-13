//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use crate::{
  ast::{AttributeList, GroupComments, GroupFn},
  expression::{FFBank, Latch, LatchBank, FF},
  pin::Pin,
  ArcStr, GroupSet,
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
  #[liberty(simple(type = Option))]
  pub driver_waveform_rise: Option<ArcStr>,
  #[liberty(simple(type = Option))]
  pub driver_waveform_fall: Option<ArcStr>,
  #[liberty(simple(type = Option))]
  pub cell_footprint: Option<ArcStr>,
  #[liberty(simple(type=Option))]
  pub cell_leakage_power: Option<f64>,
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
}
impl GroupFn for Cell {}

#[allow(dead_code, unused_imports, unused)]
mod test {
  use super::Cell;
  /// Example 23 A multibit register containing four rising-edge-triggered D flip-flops
  /// with clear  and preset is shown in Figure 1 and Example 23
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=167.32&end=167.33
  /// ">Reference</a>
  #[test]
  fn example23() {
    let (cell, _) = &mut crate::ast::test_parse_group::<Cell>(
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
    );
  }
  /// Example 27 shows a latch_bank  group for a multibit register containing four rising-edge-triggered D latches
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=187.42&end=187.43
  /// ">Reference</a>
  #[test]
  fn example27() {
    let (cell, _) = &mut crate::ast::test_parse_group::<Cell>(
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
    );
  }
  /// Example 28 a multibit register containing four high-enable D latches with the clear  attribute.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=190.11&end=190.12
  /// ">Reference</a>
  #[test]
  fn example28() {
    let (cell, _) = &mut crate::ast::test_parse_group::<Cell>(
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
    );
  }
}
