//! All item structure inside
//! `Timing`.<script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>

use crate::common::{Group, Domain};
use compact_str::CompactString;
use strum_macros::{Display, EnumString};
/// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2b0b.ff1.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.x9.h7.y2b32.ff1.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.xb.h8.y293f.ff7.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.xb.h8.y293f.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum TimingSenseType {
    /// Combines incoming `rise` delays with local `rise` delays 
    /// and compares incoming `fall` delays with local `fall` delays.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2b33.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x9.ha.y2b48.ffc.fs2.fc1.sc0.ls0.ws0
/// +t.m0.x9.ha.y2b5e.ffc.fs2.fc1.sc0.ls0.ws0
/// +t.m0.x9.ha.y2bc9.ffc.fs2.fc1.sc0.ls0.ws0
/// +t.m0.x9.ha.y2bde.ffc.fs2.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h4.y2b47.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h4.y2b5d.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x39.h8.y2bc6.ff7.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x43.h4.y2bdd.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x43.h4.y2bee.ff1.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Difinition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.xb.h8.y2940.ff7.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.xb.h8.y294e.ff8.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
#[derive(Debug, Clone, Copy, PartialEq, Default, Display, EnumString)]
pub enum TimingType {
    /// `combinational`: Combinational Timing Arc 
    /// `Defualt` value.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
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
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2c4c.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x9.h7.y2c58.ff1.fs2.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h8.y2c56.ff7.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y2c8e.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.xb.h8.y2953.ff7.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.xb.h8.y2953.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
#[derive(Debug, Clone, Copy, Default)]
pub struct Mode {}

/// The `cell_degradation` group describes a cell performance degradation
/// design rule for compiling a design. A cell degradation design rule
/// specifies the maximum capacitive load a cell can drive without causing
/// cell performance degradation during the fall transition.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2cf6.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x9.ha.y2d0d.ffc.fs2.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h8.y2d0b.ff7.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y2d8d.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.xb.h8.y2957.ff7.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.xb.h8.y2957.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Instance</a>
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CellDegradation {}

/// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2d8f.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x9.ha.y2db1.ffc.fs2.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h8.y2daf.ff7.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h4.y2dd4.ff1.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.xb.h8.y2958.ff7.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.xb.h8.y2958.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference-Instance</a>
/// 
/// **Note:**
/// The same k-factors that scale the cell_fall and cell_rise values also scale the
/// retaining_fall and retaining_rise values. There are no separate k-factors for
/// the retaining_fall and retaining_rise values.
/// 
/// **Used By:**
/// [Timing](crate::timing::Timing)
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CallFall {
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dac.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dac.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_1: Vec<f64>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dad.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dad.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_2: Vec<f64>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dae.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dae.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_3: Vec<f64>,
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2daf.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2daf.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub values: Vec<Vec<Vec<f64>>>,
    // TODO:
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x9.ha.y2db1.ffc.fs2.fc1.sc0.ls0.ws0
    /// &end
    /// =t.m0.xb.h4.y2dd4.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference-Definition</a>
    pub domain: Domain,
}

impl Group for CallFall {
    fn name(&self) -> &CompactString {
        todo!()
    }
}
