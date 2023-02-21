use strum_macros::{Display, EnumString};
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum AntennaDiodeType{
    #[strum(serialize = "power")]
    Power,
    #[strum(serialize = "ground")]
    Ground,
    #[strum(serialize = "power_and_ground")]
    PowerAndGround,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum Direction{
    #[strum(serialize = "input")]
    Input,
    #[strum(serialize = "output")]
    Output,
    #[strum(serialize = "inout")]
    Inout,
    #[strum(serialize = "internal")]
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum DontFault{
    #[strum(serialize = "sa0")]
    Sa0,
    #[strum(serialize = "sa1")]
    Sa1,
    #[strum(serialize = "sao1")]
    Sao1,
}
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum DriverType{
    #[strum(serialize = "pull_up")]
    PullUp,
    #[strum(serialize = "pull_down")]
    PullDown,
    #[strum(serialize = "open_drain")]
    OpenDrain,
    #[strum(serialize = "open_source")]
    OpenSource,
    #[strum(serialize = "bus_hold")]
    BusHold,
    #[strum(serialize = "resistive")]
    Resistive,
    #[strum(serialize = "resistive_0")]
    Resistive0,
    #[strum(serialize = "resistive_1")]
    Resistive1,
}
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum NextstateType{
    #[strum(serialize = "data")]
    Data,
    #[strum(serialize = "preset")]
    Preset,
    #[strum(serialize = "clear")]
    Clear,
    #[strum(serialize = "load")]
    Load,
    #[strum(serialize = "scan_in")]
    ScanIn,
    #[strum(serialize = "scan_enable")]
    ScanEnable,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum PinFuncType{
    #[strum(serialize = "clock_enable")]
    ClockEnable,
    #[strum(serialize = "active_high")]
    ActiveHigh,
    #[strum(serialize = "active_low")]
    ActiveLow,
    #[strum(serialize = "active_rising")]
    ActiveRising,
    #[strum(serialize = "active_falling")]
    ActiveFalling,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum RestoreEdgeType{
    #[strum(serialize = "edge_trigger")]
    EdgeTrigger,
    #[strum(serialize = "leading")]
    Leading,
    #[strum(serialize = "trailing")]
    Trailing,
}

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum SignalType{
    #[strum(serialize = "test_scan_in")]
    TestScanIn,
    #[strum(serialize = "test_scan_in_inverted")]
    TestScanInInverted,
    #[strum(serialize = "test_scan_out")]
    TestScanOut,
    #[strum(serialize = "test_scan_out_inverted")]
    TestScanOutInverted,
    #[strum(serialize = "test_scan_enable")]
    TestScanEnable,
    #[strum(serialize = "test_scan_enable_inverted")]
    TestScanEnableInverted,
    #[strum(serialize = "test_scan_clock")]
    TestScanClock,
    #[strum(serialize = "test_scan_clock_a")]
    TestScanClockA,
    #[strum(serialize = "test_scan_clock_b")]
    TestScanClockB,
    #[strum(serialize = "test_clock")]
    TestClock,
}

#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum SlewControl{
    #[strum(serialize = "low")]
    Low,
    #[strum(serialize = "medium")]
    Medium,
    #[strum(serialize = "high")]
    High,
    #[default]
    #[strum(serialize = "none")]
    None,
}