use crate::{
  ast::{
    self, Attributes, CodeFormatter, ComplexAttri, GroupComments, GroupFn, Indentation,
    ParseScope, SimpleAttri,
  },
  expression::logic::Edge,
  ArcStr,
};
use core::fmt::{self, Write};

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =228.4
/// &end
/// =228.4
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum AntennaDiodeType {
  /// `power`
  #[token("power")]
  Power,
  /// `ground`
  #[token("ground")]
  Ground,
  /// `power_and_ground`
  #[token("power_and_ground")]
  PowerAndGround,
}
impl SimpleAttri for AntennaDiodeType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

/// In timing analysis, use a tlatch group to describe the relationship between the data pin
/// and the enable pin on a transparent level-sensitive latch.
/// You define the tlatch group in a pin group, but it is only effective if you also define the
/// timing_model_type attribute in the cell that the pin belongs to. For more information
/// about the timing_model_type attribute,
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=test&bgn=372.33&end=372.37
/// ">Reference-Definition</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct TLatch {
  /// Name of the pin
  #[liberty(name)]
  #[id(borrow = "&str")]
  #[size = 8]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// Valid values are rising and falling.
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub edge_type: Option<Edge>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub tdisable: Option<bool>,
}

impl GroupFn for TLatch {}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =228.22
/// &end
/// =228.22
/// ">Reference-Instance</a>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Direction {
  #[token("input")]
  Input,
  #[token("output")]
  Output,
  #[token("inout")]
  Inout,
  #[token("internal")]
  Internal,
}
impl SimpleAttri for Direction {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DontFault {
  #[token("sa0")]
  Sa0,
  #[token("sa1")]
  Sa1,
  #[token("sao1")]
  Sao1,
}
impl SimpleAttri for DontFault {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum DriverType {
  #[token("pull_up")]
  PullUp,
  #[token("pull_down")]
  PullDown,
  #[token("open_drain")]
  OpenDrain,
  #[token("open_source")]
  OpenSource,
  #[token("bus_hold")]
  BusHold,
  #[token("resistive")]
  Resistive,
  #[token("resistive_0")]
  Resistive0,
  #[token("resistive_1")]
  Resistive1,
}
impl SimpleAttri for DriverType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NextstateType {
  #[token("data")]
  Data,
  #[token("preset")]
  Preset,
  #[token("clear")]
  Clear,
  #[token("load")]
  Load,
  #[token("scan_in")]
  ScanIn,
  #[token("scan_enable")]
  ScanEnable,
}
impl SimpleAttri for NextstateType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PinFuncType {
  #[token("clock_enable")]
  ClockEnable,
  #[token("active_high")]
  ActiveHigh,
  #[token("active_low")]
  ActiveLow,
  #[token("active_rising")]
  ActiveRising,
  #[token("active_falling")]
  ActiveFalling,
}
impl SimpleAttri for PinFuncType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RestoreEdgeType {
  #[token("edge_trigger")]
  EdgeTrigger,
  #[token("leading")]
  Leading,
  #[token("trailing")]
  Trailing,
}
impl SimpleAttri for RestoreEdgeType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SignalType {
  #[token("test_scan_in")]
  TestScanIn,
  #[token("test_scan_in_inverted")]
  TestScanInInverted,
  #[token("test_scan_out")]
  TestScanOut,
  #[token("test_scan_out_inverted")]
  TestScanOutInverted,
  #[token("test_scan_enable")]
  TestScanEnable,
  #[token("test_scan_enable_inverted")]
  TestScanEnableInverted,
  #[token("test_scan_clock")]
  TestScanClock,
  #[token("test_scan_clock_a")]
  TestScanClockA,
  #[token("test_scan_clock_b")]
  TestScanClockB,
  #[token("test_clock")]
  TestClock,
}
impl SimpleAttri for SignalType {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}
#[derive(Default)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SlewControl {
  #[token("low")]
  Low,
  #[token("medium")]
  Medium,
  #[token("high")]
  High,
  #[default]
  #[token("none")]
  None,
}
impl SimpleAttri for SlewControl {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}

/// The `prefer_tied` attribute describes an input pin of a flip-flop or latch.
///
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum OneZero {
  /// 1
  #[token("1")]
  One,
  /// 0
  #[token("0")]
  Zero,
}
impl SimpleAttri for OneZero {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(liberty_macros::EnumToken)]
pub enum OneValue {
  #[token("1")]
  One,
  #[token("0")]
  Zero,
  #[token("x")]
  Unkown,
}
/// Two values that define the value of the differential signals.
///
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
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}
impl fmt::Display for TwoValue {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.0, self.1)
  }
}
impl ast::NomParseTerm for TwoValue {
  fn nom_parse<'a>(
    i: &'a str,
  ) -> nom::IResult<&'a str, Self, nom::error::Error<&'a str>> {
    nom::combinator::map(
      nom::sequence::pair(
        <OneValue as ast::NomParseTerm>::nom_parse,
        <OneValue as ast::NomParseTerm>::nom_parse,
      ),
      |(v1, v2)| Self(v1, v2),
    )(i)
  }
}

/// Example
/// ``` text
/// retention_pin (save | restore | save_restore, enumerated_type) ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=282.3&end=282.23
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PinClass {
  /// `save_restore`
  #[token("save_restore")]
  SaveRestore,
  /// `save`
  #[token("save")]
  Save,
  /// `restore`
  #[token("restore")]
  Restore,
}
/// The `retention_pin` complex attribute identifies the retention pins of a retention cell. The
/// attribute defines the following information:
/// + pin class
///
///   Valid values:
///   + `restore`: Restores the state of the cell.
///   + `save`: Saves the state of the cell.
///   + `save_restore`: Saves and restores the state of the cell.
/// + disable value
///
/// Defines the value of the retention pin when the cell works in normal mode. The valid
/// values are 0 and 1.
///
/// Syntax
/// ``` text
/// retention_pin (pin_class, disable_value) ;
/// ```
/// Example
/// ``` text
/// retention_pin (save | restore | save_restore, enumerated_type) ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=282.3&end=282.23
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RetentionPin {
  /// `pin_class`
  pub pin_class: PinClass,
  /// `disable_value`
  pub disable_value: OneZero,
}

impl ComplexAttri for RetentionPin {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      <PinClass as ast::NomParseTerm>::nom_parse,
      <OneZero as ast::NomParseTerm>::nom_parse,
      |pin_class, disable_value| Self { pin_class, disable_value },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}, {}", self.pin_class, self.disable_value)
  }
}

#[cfg(test)]
mod test {

  use super::*;
  #[test]
  fn two_value() {
    assert_eq!(
      Ok(("", TwoValue(OneValue::Unkown, OneValue::One))),
      <TwoValue as ast::NomParseTerm>::nom_parse("x1")
    );
    assert_eq!(
      Ok(("", TwoValue(OneValue::Zero, OneValue::One))),
      <TwoValue as ast::NomParseTerm>::nom_parse("01")
    );
    assert!(<TwoValue as ast::NomParseTerm>::nom_parse("1").is_err());
    assert!(<TwoValue as ast::NomParseTerm>::nom_parse("1-").is_err());
  }
  #[test]
  fn retention_pin() {
    let pin = ast::test_parse_fmt::<crate::Cell>(
      r#"(cell1){
        pin(A){
          retention_pin (save_restore, 1);
        }
        pin(B){
          retention_pin (restore, 0);
        }
        pin(C){
          retention_pin ("save", 0);
        }
      }"#,
      r#"
liberty_db::cell::Cell (cell1) {
| pin (A) {
| | retention_pin (save_restore, 1);
| }
| pin (B) {
| | retention_pin (restore, 0);
| }
| pin (C) {
| | retention_pin (save, 0);
| }
}"#,
    );
  }
}
