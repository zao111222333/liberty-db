use strum_macros::{Display, EnumString};
/// timing_sense : positive_unate| negative_unate| non_unate;
#[derive(Debug, Clone, PartialEq)]
#[derive(Default, Display, EnumString)]
pub enum TimingSenseType{
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

/// # Timing Type
/// 
/// combinational | combinational_rise |
/// combinational_fall | three_state_disable |
/// three_state_disable_rise | three_state_disable_fall |
/// three_state_enable | three_state_enable_rise |
/// three_state_enable_fall |rising_edge | falling_edge |
/// preset | clear | hold_rising | hold_falling |
/// setup_rising | setup_falling | recovery_rising |
/// recovery_falling | skew_rising | skew_falling |
/// removal_rising | removal_falling | min_pulse_width |
/// minimum_period | max_clock_tree_path |
/// min_clock_tree_path |non_seq_setup_rising |
/// non_seq_setup_falling | non_seq_hold_rising |
/// non_seq_hold_falling | nochange_high_high |
/// nochange_high_low | nochange_low_high |
/// nochange_low_low ;
/// 
/// Reference:
///  <iframe
///   src="https://zao111222333.github.io/liberty-db/pdfjs/web/viewer.html?file=/liberty-db/reference/liberty07_03.pdf#page=203"
///   frameBorder="0"
///   scrolling="auto"
///   height="600px"
///   width="100%"
/// ></iframe>
#[derive(Debug, Clone, PartialEq)]
#[derive(Default, Display, EnumString)]
pub enum TimingType{
    /// combinational : 
    #[default]
    #[strum(serialize = "combinational")]
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