use crate::{DefaultCtx, Group};

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
  let cell = crate::ast::test_parse_fmt::<Cell<DefaultCtx>>(
    r#"(test) {
  pin_opposite ("Q1 Q2 Q3 ", "QB1 QB2 ") ;
}"#,
    r#"
liberty_db::cell::Cell (test) {
| pin_opposite ("Q1 Q2 Q3", "QB1 QB2");
}"#,
  );
}
/// ### Example 23 A multibit register containing four rising-edge-triggered D flip-flops
/// with clear  and preset is shown in Figure 1 and Example 23
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=167.32&end=167.33
/// ">Reference</a>
#[test]
fn example23() {
  let cell = crate::ast::test_parse_fmt::<Cell<DefaultCtx>>(
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
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : CLK;
| | | timing_type : setup_rising;
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_fall (scalar) {
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
/// ### Example 27 shows a `latch_bank`  group for a multibit register containing four rising-edge-triggered D latches
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=187.42&end=187.43
/// ">Reference</a>
#[test]
fn example27() {
  let cell = crate::ast::test_parse_fmt::<Cell<DefaultCtx>>(
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
/// ### Example PLL
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=112.3+113.2&end=112.53+113.6
/// ">Reference</a>
#[test]
fn example_pll() {
  let cell = crate::ast::test_parse_fmt::<Cell<DefaultCtx>>(
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
/// ### Example 28 a multibit register containing four high-enable D latches with the clear  attribute.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=190.11&end=190.12
/// ">Reference</a>
#[test]
fn example28() {
  let cell = crate::ast::test_parse_fmt::<Cell<DefaultCtx>>(
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
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
| | | | values ("1.0");
| | | }
| | }
| | timing () {
| | | related_pin : EN;
| | | timing_type : setup_falling;
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("1.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : EN;
| | | timing_type : rising_edge;
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_fall (scalar) {
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
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_fall (scalar) {
| | | | values ("2.0");
| | | }
| | }
| | timing () {
| | | related_pin : EN;
| | | timing_type : rising_edge;
| | | cell_rise (scalar) {
| | | | values ("2.0");
| | | }
| | | cell_fall (scalar) {
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

/// ### Example 30 Using the short Attribute in a model Group
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=226.14&end=226.42
/// ">Reference</a>
#[test]
fn example30() {
  let cell = crate::ast::test_parse_fmt::<Cell<DefaultCtx>>(
    r#"(cellA) {
    area : 0.4;
    short(b, c, y);
    short(c, y);
    short(b, c);
    pin(y) {
        direction : output;
        timing() {
            related_pin : a;
        }
    }
    pin(a) {
        direction : input;
        capacitance : 0.1;
    }
    pin(b) {
        direction : input;
        capacitance : 0.1;
    }
    pin(c) {
        direction : input;
        capacitance : 0.1;
        clock : true;
    }
}"#,
    r#"
liberty_db::cell::Cell (cellA) {
| area : 0.4;
| short (b, c, y);
| short (c, y);
| short (b, c);
| pin (a) {
| | capacitance : 0.1;
| | direction : input;
| }
| pin (b) {
| | capacitance : 0.1;
| | direction : input;
| }
| pin (c) {
| | capacitance : 0.1;
| | clock : true;
| | direction : input;
| }
| pin (y) {
| | direction : output;
| | timing () {
| | | related_pin : a;
| | }
| }
}"#,
  );
}
