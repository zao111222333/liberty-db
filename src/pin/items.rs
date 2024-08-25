use crate::ast::SimpleAttri;
use core::{fmt, str::FromStr};
use strum_macros::{Display, EnumString};

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =228.4
/// &end
/// =228.4
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AntennaDiodeType {
  /// `power`
  #[strum(serialize = "power")]
  Power,
  /// `ground`
  #[strum(serialize = "ground")]
  Ground,
  /// `power_and_ground`
  #[strum(serialize = "power_and_ground")]
  PowerAndGround,
}
impl SimpleAttri for AntennaDiodeType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =228.22
/// &end
/// =228.22
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Direction {
  #[strum(serialize = "input")]
  Input,
  #[strum(serialize = "output")]
  Output,
  #[strum(serialize = "inout")]
  Inout,
  #[strum(serialize = "internal")]
  Internal,
}
impl SimpleAttri for Direction {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DontFault {
  #[strum(serialize = "sa0")]
  Sa0,
  #[strum(serialize = "sa1")]
  Sa1,
  #[strum(serialize = "sao1")]
  Sao1,
}
impl SimpleAttri for DontFault {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DriverType {
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
impl SimpleAttri for DriverType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NextstateType {
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
impl SimpleAttri for NextstateType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PinFuncType {
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
impl SimpleAttri for PinFuncType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RestoreEdgeType {
  #[strum(serialize = "edge_trigger")]
  EdgeTrigger,
  #[strum(serialize = "leading")]
  Leading,
  #[strum(serialize = "trailing")]
  Trailing,
}
impl SimpleAttri for RestoreEdgeType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SignalType {
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
impl SimpleAttri for SignalType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
#[derive(Default)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SlewControl {
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
impl SimpleAttri for SlewControl {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}

/// The `prefer_tied` attribute describes an input pin of a flip-flop or latch.
/// It indicates what the library developer wants this pin connected to.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =229.4
/// &end
/// =229.4
/// ">Reference-Instance</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=267.24&end=267.26
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PreferTied {
  /// 1
  #[strum(serialize = "1")]
  One,
  /// 0
  #[strum(serialize = "0")]
  Zero,
}
impl SimpleAttri for PreferTied {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
enum OneValue {
  #[strum(serialize = "1")]
  One,
  #[strum(serialize = "0")]
  Zero,
  #[strum(serialize = "x")]
  Unkown,
}
/// Two values that define the value of the differential signals
/// when both inputs are driven to the same value. The first value
/// represents the value when both input pins are at logic 0;
/// the second value represents the value when both input pins are at logic 1.
/// Valid values for the two-value string are any two-value combinations
/// made up of 0, 1, and x.
/// If you do not enter a `fault_model` attribute value, the signal
/// pin value goes to x when both input pins are 0 or 1.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=248.6&end=248.13
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TwoValue(OneValue, OneValue);
impl SimpleAttri for TwoValue {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
impl fmt::Display for TwoValue {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.0, self.1)
  }
}
impl FromStr for TwoValue {
  type Err = strum::ParseError;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 2 {
      return Err(strum::ParseError::VariantNotFound);
    }
    let mut i = s.chars();
    if let Some(c1) = i.next() {
      if let Some(c2) = i.next() {
        let mut tmp = [0; 1];
        return Ok(Self(
          OneValue::from_str(c1.encode_utf8(&mut tmp))?,
          OneValue::from_str(c2.encode_utf8(&mut tmp))?,
        ));
      }
    }
    Err(strum::ParseError::VariantNotFound)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn two_value() {
    assert_eq!(Ok(TwoValue(OneValue::Unkown, OneValue::One)), TwoValue::from_str("x1"));
    assert_eq!(Ok(TwoValue(OneValue::Zero, OneValue::One)), TwoValue::from_str("01"));
    assert_eq!(Err(strum::ParseError::VariantNotFound), TwoValue::from_str("1"));
    assert_eq!(Err(strum::ParseError::VariantNotFound), TwoValue::from_str("111"));
    assert_eq!(Err(strum::ParseError::VariantNotFound), TwoValue::from_str("1-"));
  }
}
