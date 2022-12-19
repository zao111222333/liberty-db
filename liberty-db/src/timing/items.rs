//! All item structure inside
//! `Timing`.<script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>

use crate::common::Group;
use compact_str::CompactString;
use strum_macros::{Display, EnumString};
/// The `timing_sense` attribute describes the way an input pin logically affects an output pin.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2b0b.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y293f.ff7.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.x9.h7.y2b32.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y293f.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Default, Display, EnumString)]
pub enum TimingSenseType {
    /// positive_unate :
    #[strum(serialize = "positive_unate")]
    PositiveUnate,
    /// negative_unate :
    #[strum(serialize = "negative_unate")]
    NegativeUnate,
    /// non_unate :
    #[default]
    #[strum(serialize = "non_unate")]
    NonUnate,
}

/// The `timing_type` attribute distinguishes between combinational
/// and sequential cells by defining the type of timing arc.
/// If this attribute is not assigned, the cell is considered combinational.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2b33.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y2940.ff7.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h4.y2b47.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y294e.ff8.fs2.fc2.sc0.ls0.ws0
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Default, Display, EnumString)]
pub enum TimingType {
    /// combinational :
    #[strum(serialize = "combinational")]
    #[default]
    Combinational,
    /// combinational_rise :
    #[strum(serialize = "combinational_rise")]
    CombinationalRise,
    /// combinational_fall :
    #[strum(serialize = "combinational_fall")]
    CombinationalFall,
    /// three_state_disable :
    #[strum(serialize = "three_state_disable")]
    ThreeStateDisable,
    /// three_state_disable_rise :
    #[strum(serialize = "three_state_disable_rise")]
    ThreeStateDisableRise,
    /// three_state_disable_fall :
    #[strum(serialize = "three_state_disable_fall")]
    ThreeStateDisableFall,
    /// three_state_enable :
    #[strum(serialize = "three_state_enable")]
    ThreeStateEnable,
    /// three_state_enable_rise :
    #[strum(serialize = "three_state_enable_rise")]
    ThreeStateEnableRise,
    /// three_state_enable_fall :
    #[strum(serialize = "three_state_enable_fall")]
    ThreeStateEnableFall,
    /// rising_edge :
    #[strum(serialize = "rising_edge")]
    RisingEdge,
    /// falling_edge :
    #[strum(serialize = "falling_edge")]
    FallingEdge,
    /// preset :
    #[strum(serialize = "preset")]
    Preset,
    /// clear :
    #[strum(serialize = "clear")]
    Clear,
    /// hold_rising :
    #[strum(serialize = "hold_rising")]
    HoldRising,
    /// hold_falling :
    #[strum(serialize = "hold_falling")]
    HoldFalling,
    /// setup_rising :
    #[strum(serialize = "setup_rising")]
    SetupRising,
    /// setup_falling :
    #[strum(serialize = "setup_falling")]
    SetupFalling,
    /// recovery_rising :
    #[strum(serialize = "recovery_rising")]
    RecoveryRising,
    /// recovery_falling :
    #[strum(serialize = "recovery_falling")]
    RecoveryFalling,
    /// skew_rising :
    #[strum(serialize = "skew_rising")]
    SkewRising,
    /// skew_falling :
    #[strum(serialize = "skew_falling")]
    SkewFalling,
    /// removal_rising :
    #[strum(serialize = "removal_rising")]
    RemovalRising,
    /// removal_falling :
    #[strum(serialize = "removal_falling")]
    RemovalFalling,
    /// min_pulse_width :
    #[strum(serialize = "min_pulse_width")]
    MinPulseWidth,
    /// minimum_period :
    #[strum(serialize = "minimum_period")]
    MinimumPeriod,
    /// max_clock_tree_path :
    #[strum(serialize = "max_clock_tree_path")]
    MaxClockTreePath,
    /// min_clock_tree_path :
    #[strum(serialize = "min_clock_tree_path")]
    MinClockTreePath,
    /// non_seq_setup_rising :
    #[strum(serialize = "non_seq_setup_rising")]
    NonSeqSetupRising,
    /// non_seq_setup_falling :
    #[strum(serialize = "non_seq_setup_falling")]
    NonSeqSetupFalling,
    /// non_seq_hold_rising :
    #[strum(serialize = "non_seq_hold_rising")]
    NonSeqHoldRising,
    /// non_seq_hold_falling :
    #[strum(serialize = "non_seq_hold_falling")]
    NonSeqHoldFalling,
    /// nochange_high_high :
    #[strum(serialize = "nochange_high_high")]
    NochangeHighHigh,
    /// nochange_high_low :
    #[strum(serialize = "nochange_high_low")]
    NochangeHighLow,
    /// nochange_low_high :
    #[strum(serialize = "nochange_low_high")]
    NochangeLowHigh,
    /// nochange_low_low :
    #[strum(serialize = "nochange_low_low")]
    NochangeLowLow,
}

/// You define the mode attribute within a timing group. A mode attribute pertains to an individual timing arc. The timing arc is active when mode is instantiated with a name and a value. You can specify multiple instances of the mode attribute, but only one instance for each timing arc.
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2c4c.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x9.h7.y2c58.ff1.fs2.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h8.y2c56.ff7.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y2c8e.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference</a>
#[derive(Debug, Clone, Copy, Default)]
pub struct Mode {}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CellDegradation {}

/// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
/// **Note: **
///
/// The same k-factors that scale the cell_fall and cell_rise values also scale the
/// retaining_fall and retaining_rise values. There are no separate k-factors for
/// the retaining_fall and retaining_rise values.
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2d8f.ff1.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h4.y2daa.ff1.fs2.fc2.sc0.ls0.ws0
/// ">Reference</a>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CallFall {
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dac.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dac.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_1: Vec<f64>,
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dad.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dad.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_2: Vec<f64>,
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dae.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dae.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_3: Vec<f64>,
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2daf.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2daf.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub values: Vec<Vec<Vec<f64>>>,
}

impl Group for CallFall {
    fn name(&self) -> &CompactString {
        todo!()
    }
}
