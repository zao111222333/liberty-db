//! All item structure inside
//! `Timing`.<script src="https://zao111222333.github.io/liberty-rs/js/iframe.js"></script>

use crate::common::{traits::Group, items::Domain};
use compact_str::CompactString;
use strum_macros::{Display, EnumString};
/// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =213.11
/// &end
/// =214.6
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.55
/// &end
/// =203.55
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/js/iframe.js"></script>
/// 
/// #### Syntax 
/// `timing_sense : positive_unate | negative_unate | non_unate ;`
/// 
/// `positive_unate`: Combines incoming rise delays with local rise delays and 
/// compares incoming fall delays with local fall delays.
/// 
/// `negative_unate`: Combines incoming rise delays with local fall delays and 
/// compares incoming fall delays with local rise delays.
/// 
/// `non_unate`: Combines local delays with the worst-case incoming delay value.
/// The non-unate timing sense represents a function whose output value change cannot 
/// be determined from the direction of the change in the input value.
/// 
/// Timing sense is derived from the logic function of a pin. For example, the value derived for 
/// an AND gate is `positive_unate`, the value for a NAND gate is `negative_unate`, and the value 
/// for an XOR gate is `non_unate`.
/// 
/// A function is said to be unate if a rising (falling) change on a positive (negative) unate 
/// input variable causes the output function variable to rise (fall) or not change. 
/// For a non-unate variable, further state information is required to determine the effects of 
/// a particular state transition.
/// 
/// You can specify half-unate sequential timing arcs if the `timing_type` value is either 
/// `rising_edge` or `falling_edge` and the `timing_sense` value is either `positive_unate` 
/// or `negative_unate`.
/// + In the case of `rising_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition` 
/// information is required.
/// + In the case of `rising_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition` 
/// information is required.
/// + In the case of `falling_edge` and `positive_unate` values, only the `cell_rise` and `rise_transition` 
/// information is required.
/// + In the case of `falling_edge` and `negative_unate` values, only the `cell_fall` and `fall_transition`
/// information is required.
/// 
/// Do not define the `timing_sense` value of a pin, except when you need to override the derived value 
/// or when you are characterizing a noncombinational gate such as a three-state component. For example, 
/// you might want to define the timing sense manually when you model multiple paths between 
/// an input pin and an output pin, such as in an XOR gate.
/// 
/// It is possible that one path is positive unate while another is negative unate. In this case, 
/// the first timing arc is given a `positive_unate` designation and the second is given a `negative_unate` 
/// designation.
/// 
/// Timing arcs with a timing type of `clear` or `preset` require a `timing_sense` attribute. 
/// If `related_pin` is an output pin, you must define a `timing_sense`` attribute for that pin.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =213.11
/// &end
/// =214.6
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.55
/// &end
/// =203.55
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum TimingSenseType {
    /// Combines incoming `rise` delays with local `rise` delays 
    /// and compares incoming `fall` delays with local `fall` delays.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x45.ha.y2b10.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x37.h4.y2b12.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "positive_unate")]
    PositiveUnate,
    /// Combines incoming `rise` delays with local `fall` delays 
    /// and compares incoming `fall` delays with local `rise` delays.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x45.ha.y2b13.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x37.h4.y2b15.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "negative_unate")]
    NegativeUnate,
    /// Combines local delays with the `worst-case` incoming delay value. 
    /// The non-unate timing sense represents a function whose 
    /// output value change cannot be determined from the direction 
    /// of the change in the input value.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x45.ha.y2b16.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x37.h4.y2b19.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "non_unate")]
    NonUnate,
}

/// The `timing_type` attribute distinguishes between combinational
/// and sequential cells by defining the type of timing arc.
/// If this attribute is not assigned, the cell is considered combinational (Default).
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =214.7
/// +214.28
/// +214.50
/// +216.63
/// +217.19
/// &end
/// =214.27
/// +214.49
/// +216.60
/// +217.18
/// +217.35
/// ">Reference-Difinition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.56
/// &end
/// =203.70
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/js/iframe.js"></script>
/// 
/// #### Syntax
/// `timing_type : combinational | combinational_rise | combinational_fall | three_state_disable | 
/// three_state_disable_rise | three_state_disable_fall | three_state_enable | three_state_enable_rise | 
/// three_state_enable_fall | rising_edge | falling_edge | preset | clear | hold_rising | hold_falling | 
/// setup_rising | setup_falling | recovery_rising | recovery_falling | skew_rising | skew_falling | 
/// removal_rising | removal_falling | min_pulse_width | minimum_period | max_clock_tree_path |
/// min_clock_tree_path |non_seq_setup_rising | non_seq_setup_falling | non_seq_hold_rising | 
/// non_seq_hold_falling | nochange_high_high | nochange_high_low | nochange_low_high | nochange_low_low ;`
/// 
/// #### Combinational Timing Arcs
/// The timing type and timing sense define the signal propagation pattern. The default timing type is combinational.
/// <table>
///     <thead>
///         <tr>
///             <th>Timing type</th>
///             <th></th>
///             <th>Timing sense</th>
///             <th></th>
///         </tr>
///         <tr>
///             <th></th>
///             <th><code>positive_unate</code></th>
///             <th><code>negative_unate</code></th>
///             <th><code>non_unate</code></th>
///         </tr>
///     </thead>
///     <tbody>
///         <tr>
///             <td><code>combinational</code></td>
///             <td>R->R, F->F</td>
///             <td>R->F, F->R</td>
///             <td>{R,F}->{R,F}</td>
///         </tr>
///         <tr>
///             <td><code>combinational_rise</code></td>
///             <td>R->R</td>
///             <td>F->R</td>
///             <td>{R,F}->R</td>
///         </tr>
///         <tr>
///             <td><code>combinational_fall</code></td>
///             <td>F->F</td>
///             <td>R->F</td>
///             <td>{R,F}->F</td>
///         </tr>
///         <tr>
///             <td><code>three_state_disable</code></td>
///             <td>R->{0Z,1Z}</td>
///             <td>F->{0Z,1Z}</td>
///             <td>{R,F}->{0Z,1Z}</td>
///         </tr>
///         <tr>
///             <td><code>three_state_enable</code></td>
///             <td>R->{Z0,Z1}</td>
///             <td>F->{Z0,Z1}</td>
///             <td>{R,F}->{Z0,Z1}</td>
///         </tr>
///         <tr>
///             <td><code>three_state_disable_rise</code></td>
///             <td>R->0Z</td>
///             <td>F->0Z</td>
///             <td>{R,F}->0Z</td>
///         </tr>
///         <tr>
///             <td><code>three_state_disable_fall</code></td>
///             <td>R->1Z</td>
///             <td>F->1Z</td>
///             <td>{R,F}->1Z</td>
///         </tr>
///         <tr>
///             <td><code>three_state_enable_rise</code></td>
///             <td>R->Z1</td>
///             <td>F->Z1</td>
///             <td>{R,F}->Z1</td>
///         </tr>
///         <tr>
///             <td><code>three_state_enable_fall</code></td>
///             <td>R->Z0</td>
///             <td>F->Z0</td>
///             <td>{R,F}->Z0</td>
///         </tr>
///     </tbody>
/// </table>
/// 
/// #### Sequential Timing Arcs
/// + `rising_edge`: Identifies a timing arc whose output pin is sensitive to 
/// a rising signal at the input pin.
/// + `falling_edge`: Identifies a timing arc whose output pin is sensitive to 
/// a falling signal at the input pin.
/// + `preset`: Preset arcs affect only the rise arrival time of the arc’s endpoint pin. 
/// A preset arc implies that you are asserting a logic 1 on the output pin when 
/// the designated `related_pin` is asserted.
/// + `clear`: Clear arcs affect only the fall arrival time of the arc’s endpoint pin.
/// A clear arc implies that you are asserting a logic 0 on the output pin when 
/// the designated `related_pin` is asserted.
/// + `hold_rising`: Designates the rising edge of the related pin for the hold check.
/// + `hold_falling`: Designates the falling edge of the related pin for the hold check.
/// + `setup_rising`: Designates the rising edge of the related pin for the setup check on clocked elements.
/// + `setup_falling`: Designates the falling edge of the related pin for the setup check on clocked elements.
/// + `recovery_rising`: Uses the rising edge of the related pin for the recovery time check.
/// The clock is rising-edge-triggered.
/// + `recovery_falling`: Uses the falling edge of the related pin for the recovery time check. 
/// The clock is falling-edge-triggered.
/// + `skew_rising`: The timing constraint interval is measured from the rising edge of 
/// the reference pin (specified in `related_pin`) to a transition edge of the parent pin of 
/// the timing group. The `intrinsic_rise` value is the maximum skew time between the reference pin rising 
/// and the parent pin rising. The `intrinsic_fall` value is the maximum skew time between the reference pin rising 
/// and the parent pin falling.
/// + `skew_falling`: The timing constraint interval is measured from the falling edge of 
/// the reference pin (specified in `related_pin`) to a transition edge of the parent pin of 
/// the timing group. The `intrinsic_rise` value is the maximum skew time between the reference pin falling 
/// and the parent pin rising. The `intrinsic_fall` value is the maximum skew time between the reference pin falling 
/// and the parent pin falling.
/// + `removal_rising`: Used when the cell is a low-enable latch or a rising-edge-triggered flip-flop. 
/// For active-low asynchronous control signals, define the removal time with the `intrinsic_rise` attribute. 
/// For active-high asynchronous control signals, define the removal time with the `intrinsic_fall` attribute.
/// + `removal_falling`: Used when the cell is a high-enable latch or a falling-edge-triggered flip-flop. 
/// For active-low asynchronous control signals, define the removal time with the `intrinsic_rise` attribute. 
/// For active-high asynchronous control signals, define the removal time with the `intrinsic_fall` attribute.
/// + `minimum_pulse_width`: This value, together with the `minimum_period` value, lets you specify 
/// the minimum pulse width for a clock pin. The timing check is performed on the pin itself, 
/// so the related pin should be the same. As with other timing checks, you can include rise and fall constraints. 
/// + `minimum_period`: This value, together with the `minimum_pulse_width` value, lets you specify 
/// the minimum pulse width for a clock pin. The timing check is performed on the pin itself, 
/// so the related pin should be the same. As with other timing checks, you can include rise and fall constraints.
/// + `max_clock_tree_path`: Used in timing groups under a clock pin. Defines the maximum clock tree path constraint.
/// + `min_clock_tree_path`: Used in timing groups under a clock pin. Defines the minimum clock tree path constraint.
/// #### Example
/// A sample library with the timing_type attribute and minimum_pulse_width and minimum_period values.
/// ```
/// library(ASIC) {  
///     ...  
///     delay_model : table_lookup;  
///     ...  
///     lu_table_template(pulse_width_template) {  
///         variable_1 : related_pin_transition;  
///         index_1 ("1.0, 2.0, 3.0");  
///     }  
///     cell(flop) {  
///         ...   
///         pin(CK) {  
///             direction : input;  
///             capacitance : 0.00707171;  
///             timing() {  
///                 timing_type : "min_pulse_width";  
///                 related_pin : "CK";  
///                 ...  
///                 rise_constraint("pulse_width_template") {  
///                     index_1("0.000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///                 fall_constraint("pulse_width_template") {  
///                     index_1("0.000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///             }  
///             timing() {  
///                 timing_type : "minimum_period";  
///                 related_pin : "CK";  
///                 rise_constraint("pulse_width_template") {  
///                     index_1(" .000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///                 fall_constraint("pulse_width_template") {  
///                     index_1("0.000000, 1.000000, 2.00000");  
///                     values ("6.000000, 6.250000, 7.2500000" );  
///                 }  
///             }  
///         }  
///     ...  
///     } /* end cell */ 
/// } /* end library */
/// ```
/// #### Nonsequential Timing Arcs
/// In some nonsequential cells, the setup and hold timing constraints are specified on
/// the data pin with a nonclock pin as the `related_pin`. It requires the signal of 
/// a pin to be stable for a specified period of time before and after another pin of 
/// the same cell range state so that the cell can function as expected.
/// + `non_seq_setup_rising`: Defines (with non_seq_setup_falling) the timing arcs used 
/// for setup checks between pins with nonsequential behavior. The related pin in 
/// a timing arc is used for the timing check. 
/// + `non_seq_setup_falling`: Defines (with non_seq_setup_rising) the timing arcs used 
/// for setup checks between pins with nonsequential behavior. The related pin in 
/// a timing arc is used for the timing check.
/// + `non_seq_hold_rising`: Defines (with non_seq_hold_falling) the timing arcs used 
/// for hold checks between pins with nonsequential behavior. The related pin in 
/// a timing arc is used for the timing check.
/// + `non_seq_hold_falling`: Defines (with non_seq_hold_rising) the timing arcs used 
/// for hold checks between pins with nonsequential behavior. The related pin in 
/// a timing arc is used for the timing check.
/// #### No-Change Timing Arcs
/// This feature models the timing requirement of latch devices with latch-enable signals. 
/// The four no-change timing types define the pulse waveforms of both the constrained 
/// signal and the related signal in standard CMOS and nonlinear CMOS delay models. 
/// The information is used in static timing verification during synthesis.
/// + `nochange_high_high` (positive/positive): Indicates a positive pulse on 
/// the constrained pin and a positive pulse on the related pin.
/// + `nochange_high_low` (positive/negative): Indicates a positive pulse on 
/// the constrained pin and a negative pulse on the related pin.
/// + `nochange_low_high` (negative/positive): Indicates a negative pulse on 
/// the constrained pin and a positive pulse on the related pin.
/// + `nochange_low_low` (negative/negative): Indicates a negative pulse on 
/// the constrained pin and a negative pulse on the related pin.
#[derive(Debug, Clone, Copy, PartialEq, Default, Display, EnumString)]
pub enum TimingType {
    /// `combinational`: Combinational Timing Arc 
    /// `Defualt` value.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b4e.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x61.hf.y2b4e.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "combinational")]
    #[default]
    Combinational,
    /// `combinational_rise`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b4f.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x61.hf.y2b4f.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "combinational_rise")]
    CombinationalRise,
    /// `combinational_fall`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b50.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x61.hf.y2b50.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "combinational_fall")]
    CombinationalFall,
    /// `three_state_disable`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b51.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x1c.hf.y2b52.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "three_state_disable")]
    ThreeStateDisable,
    /// `three_state_disable_rise`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b57.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x16.hf.y2b59.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "three_state_disable_rise")]
    ThreeStateDisableRise,
    /// `three_state_disable_fall`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b5a.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x61.hf.y2b5a.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "three_state_disable_fall")]
    ThreeStateDisableFall,
    /// `three_state_enable`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b54.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x1c.hf.y2b55.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "three_state_enable")]
    ThreeStateEnable,
    /// `three_state_enable_rise`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b5b.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x61.hf.y2b5b.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "three_state_enable_rise")]
    ThreeStateEnableRise,
    /// `three_state_enable_fall`: Combinational Timing Arc
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x61.hf.y2b5c.ff1.fs7.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x61.hf.y2b5c.ff1.fs7.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "three_state_enable_fall")]
    ThreeStateEnableFall,
    /// `rising_edge`: Sequential Timing Arc
    /// 
    /// Identifies a timing arc whose output pin is sensitive to a rising signal at the input pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b5f.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b61.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "rising_edge")]
    RisingEdge,
    /// `falling_edge`: Sequential Timing Arc
    /// 
    /// Identifies a timing arc whose output pin is sensitive to a falling signal at the input pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b62.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y6c.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "falling_edge")]
    FallingEdge,
    /// `preset`: Sequential Timing Arc
    /// 
    /// Preset arcs affect only the rise arrival time of the arc’s endpoint pin. 
    /// A preset arc implies that you are asserting a logic 1 on the output pin 
    /// when the designated related_pin is asserted.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y1f79.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y1f7c.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "preset")]
    Preset,
    /// `clear`: Sequential Timing Arc
    /// 
    /// Clear arcs affect only the fall arrival time of the arc’s endpoint pin. 
    /// A clear arc implies that you are asserting a logic 0 on the output pin 
    /// when the designated related_pin is asserted.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b64.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y1f80.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "clear")]
    Clear,
    /// `hold_rising`: Sequential Timing Arc
    /// 
    /// Designates the rising edge of the related pin for the hold check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b67.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b68.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "hold_rising")]
    HoldRising,
    /// `hold_falling`: Sequential Timing Arc
    /// 
    /// Designates the falling edge of the related pin for the hold check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b69.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b6a.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "hold_falling")]
    HoldFalling,
    /// `setup_rising`: Sequential Timing Arc
    /// 
    /// Designates the rising edge of the related pin for the setup check on clocked elements.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b6b.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b6d.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "setup_rising")]
    SetupRising,
    /// `setup_falling`: Sequential Timing Arc
    /// 
    /// Designates the falling edge of the related pin for the setup check on clocked elements.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b6e.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b70.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "setup_falling")]
    SetupFalling,
    /// `recovery_rising`: Sequential Timing Arc
    /// 
    /// Uses the rising edge of the related pin for the recovery time check. 
    /// The clock is rising-edge-triggered.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b71.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b73.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "recovery_rising")]
    RecoveryRising,
    /// `recovery_falling`: Sequential Timing Arc
    /// 
    /// Uses the falling edge of the related pin for the recovery time check. 
    /// The clock is falling-edge-triggered.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b74.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b76.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "recovery_falling")]
    RecoveryFalling,
    /// `skew_rising`: Sequential Timing Arc
    /// 
    /// The timing constraint interval is measured from the rising edge of 
    /// the reference pin (specified in `related_pin`) to a transition edge of 
    /// the parent pin of the timing group. The `intrinsic_rise` value is 
    /// the maximum skew time between the reference pin rising and 
    /// the parent pin rising. The `intrinsic_fall` value is the maximum skew time 
    /// between the reference pin rising and the parent pin falling.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b77.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b7d.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "skew_rising")]
    SkewRising,
    /// `skew_falling`: Sequential Timing Arc
    /// 
    /// The timing constraint interval is measured from the falling edge of 
    /// the reference pin (specified in `related_pin`) to a transition edge of 
    /// the parent pin of the timing group. The `intrinsic_rise` value is 
    /// the maximum skew time between the reference pin falling and 
    /// the parent pin rising. The `intrinsic_fall` value is the maximum skew 
    /// time between the reference pin falling and the parent pin falling.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b7e.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b84.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "skew_falling")]
    SkewFalling,
    /// `removal_rising`: Sequential Timing Arc
    /// 
    /// Used when the cell is a low-enable latch or a rising-edge-triggered flip-flop. 
    /// For active-low asynchronous control signals, define the removal time with 
    /// the `intrinsic_rise` attribute. For active-high asynchronous control signals, 
    /// define the removal time with the `intrinsic_fall` attribute.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b85.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h7.y2b89.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "removal_rising")]
    RemovalRising,
    /// `removal_falling`: Sequential Timing Arc
    /// 
    /// Used when the cell is a high-enable latch or a falling-edge-triggered flip-flop. 
    /// For active-low asynchronous control signals, define the removal time with 
    /// the `intrinsic_rise` attribute. For active-high asynchronous control signals, 
    /// define the removal time with the `intrinsic_fall` attribute.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b8a.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h7.y2b8e.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "removal_falling")]
    RemovalFalling,
    /// `min_pulse_width`: Sequential Timing Arc
    /// 
    /// This value, together with the `minimum_period` value, lets you specify 
    /// the minimum pulse width for a clock pin. The timing check is performed on 
    /// the pin itself, so the related pin should be the same. 
    /// As with other timing checks, you can include rise and fall constraints.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b8f.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y1d46.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "min_pulse_width")]
    MinPulseWidth,
    /// `minimum_period`: Sequential Timing Arc
    /// 
    /// This value, together with the `minimum_pulse_width` value, 
    /// lets you specify the minimum pulse width for a clock pin. 
    /// The timing check is performed on the pin itself, so 
    /// the related pin should be the same. As with other timing checks, 
    /// you can include rise and fall constraints.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b91.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b32.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "minimum_period")]
    MinimumPeriod,
    /// `max_clock_tree_path`: Sequential Timing Arc
    /// 
    /// Used in timing groups under a clock pin. Defines the maximum clock tree path constraint.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y1d4c.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2b95.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "max_clock_tree_path")]
    MaxClockTreePath,
    /// `min_clock_tree_path`: Sequential Timing Arc
    /// 
    /// Used in timing groups under a clock pin. Defines the minimum clock tree path constraint.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2b96.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y84e.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "min_clock_tree_path")]
    MinClockTreePath,
    /// `non_seq_setup_rising`: Nonsequential Timing Arc
    /// 
    /// Defines (with `non_seq_setup_falling`) the timing arcs used for setup checks between 
    /// pins with nonsequential behavior. The related pin in a timing arc is used for the timing check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2bce.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2bd1.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "non_seq_setup_rising")]
    NonSeqSetupRising,
    /// `non_seq_setup_falling`: Nonsequential Timing Arc
    /// 
    /// Defines (with `non_seq_setup_rising`) the timing arcs used for setup checks between 
    /// pins with nonsequential behavior. The related pin in a timing arc is used for the timing check. 
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2bd2.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2bd5.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "non_seq_setup_falling")]
    NonSeqSetupFalling,
    /// `non_seq_hold_rising`: Nonsequential Timing Arc
    /// 
    /// Defines (with `non_seq_hold_falling`) the timing arcs used for hold checks between 
    /// pins with nonsequential behavior. The related pin in a timing arc is used for the timing check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2bd6.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2bd9.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "non_seq_hold_rising")]
    NonSeqHoldRising,
    /// `non_seq_hold_falling`: Nonsequential Timing Arc
    /// 
    /// Defines (with `non_seq_hold_rising`) the timing arcs used for hold checks between 
    /// pins with nonsequential behavior. The related pin in a timing arc is used for the timing check.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2bda.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2bdd.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "non_seq_hold_falling")]
    NonSeqHoldFalling,
    /// `nochange_high_high`: No-Change Timing Arc
    /// 
    /// (positive/positive) Indicates a positive pulse on the constrained pin and a positive pulse on the related pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2be3.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2be5.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "nochange_high_high")]
    NochangeHighHigh,
    /// `nochange_high_low`(positive/negative): No-Change Timing Arc
    /// 
    /// Indicates a positive pulse on the constrained pin and a negative pulse on the related pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2be6.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2be8.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "nochange_high_low")]
    NochangeHighLow,
    /// `nochange_low_high`(negative/positive): No-Change Timing Arc
    /// 
    /// Indicates a negative pulse on the constrained pin and a positive pulse on the related pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2be9.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2beb.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "nochange_low_high")]
    NochangeLowHigh,
    /// `nochange_low_low`(negative/negative): No-Change Timing Arc
    /// 
    /// Indicates a negative pulse on the constrained pin and a negative pulse on the related pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x42.ha.y2bec.ffc.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x43.h4.y2bee.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    #[strum(serialize = "nochange_low_low")]
    NochangeLowLow,
}

/// You define the mode attribute within a timing group.
/// A mode attribute pertains to an individual timing arc.
/// The timing arc is active when mode is instantiated with a name and a value.
/// You can specify multiple instances of the mode attribute,
/// but only one instance for each timing arc.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =219.39
/// +220.11
/// &end
/// =220.9
/// +222.73
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =204.5
/// &end
/// =204.5
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/js/iframe.js"></script>
#[derive(Debug, Clone, Copy, Default)]
pub struct Mode {}

/// The `cell_degradation` group describes a cell performance degradation
/// design rule for compiling a design. A cell degradation design rule
/// specifies the maximum capacitive load a cell can drive without causing
/// cell performance degradation during the fall transition.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =225.4
/// +225.27
/// &end
/// =225.25
/// +227.51
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =204.9
/// &end
/// =204.9
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/js/iframe.js"></script>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CellDegradation {
    pub group_name: CompactString,
    /* polynomial model */ 
    pub coefs: Vec<f64>,
    /* polynomial model */ 
    pub orders: Vec<usize>,
    /* lookup table */ 
    pub index_1: Vec<f64>,
    /* lookup table */ 
    pub values: Vec<f64>,
    /* polynomial model */
    pub variable_n_range: Option<(f64,f64)>, 
    pub domain: Option<Domain>,
}

/// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =227.53
/// +228.27
/// &end
/// =228.25
/// +228.62
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =204.10
/// &end
/// =204.10
/// ">Reference-Instance</a>
/// 
/// **Note:**
/// The same k-factors that scale the cell_fall and cell_rise values also scale the
/// retaining_fall and retaining_rise values. There are no separate k-factors for
/// the retaining_fall and retaining_rise values.
/// 
/// **Used By:**
/// [Timing](crate::timing::Timing)
/// <script src="https://zao111222333.github.io/liberty-rs/js/iframe.js"></script>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CellFall {
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =228.22
    /// &end
    /// =228.22
    /// ">Reference</a>
    pub index_1: Vec<f64>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =228.23
    /// &end
    /// =228.23
    /// ">Reference</a>
    pub index_2: Vec<f64>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =228.24
    /// &end
    /// =228.24
    /// ">Reference</a>
    pub index_3: Vec<f64>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =228.25
    /// &end
    /// =228.25
    /// ">Reference</a>
    pub values: Vec<Vec<Vec<f64>>>,
    // TODO:
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =228.27
    /// &end
    /// =228.62
    /// ">Reference-Definition</a>
    pub domain: Domain,
}
