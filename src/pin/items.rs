use crate::{
  Ctx,
  ast::{
    Attributes, CodeFormatter, ComplexAttri, ComplexParseError, GroupComments, GroupFn,
    Indentation, ParseScope, SimpleAttri,
  },
  expression::logic::{Edge, Static},
};
use core::{
  fmt::{self, Write},
  str::FromStr,
};
use strum::{Display, EnumString};

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
crate::ast::impl_self_builder!(AntennaDiodeType);
crate::ast::impl_simple!(AntennaDiodeType);

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
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct TLatch<C: Ctx> {
  /// Name of the pin
  #[liberty(name)]
  #[id(borrow = str)]
  pub name: String,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: Attributes,
  /// Valid values are rising and falling.
  #[liberty(simple(type = Option))]
  pub edge_type: Option<Edge>,
  #[liberty(simple(type = Option))]
  pub tdisable: Option<bool>,
}

impl<C: Ctx> GroupFn<C> for TLatch<C> {}

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
  #[strum(serialize = "inoutput", to_string = "inout")]
  Inout,
  #[strum(serialize = "internal")]
  Internal,
}
crate::ast::impl_self_builder!(Direction);
crate::ast::impl_simple!(Direction);

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
crate::ast::impl_self_builder!(DontFault);
crate::ast::impl_simple!(DontFault);

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

bitflags::bitflags! {
  /// Represents a set of flags.
  #[derive(Debug, Clone, Copy, Eq, PartialEq)]
  #[derive(serde::Serialize, serde::Deserialize)]
  #[serde(transparent)]
  pub struct AllDriverType: u8 {
    const pull_up = 0b0000_0001;
    const pull_down = 0b0000_0010;
    const open_drain = 0b0000_0100;
    const open_source = 0b0000_1000;
    const bus_hold = 0b0000_1000;
    const resistive = 0b0001_0000;
    const resistive_0 = 0b0100_0000;
    const resistive_1 = 0b1000_0000;
  }
}

crate::ast::impl_self_builder!(AllDriverType);
impl<C: Ctx> SimpleAttri<C> for AllDriverType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if self.bits().is_power_of_two() {
      write!(f, "{self}")
    } else {
      write!(f, "\"{self}\"")
    }
  }
}
impl FromStr for AllDriverType {
  type Err = <DriverType as FromStr>::Err;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut out = Self::empty();
    for t in s.split_ascii_whitespace() {
      match t.parse()? {
        DriverType::PullUp => out.insert(Self::pull_up),
        DriverType::PullDown => out.insert(Self::pull_down),
        DriverType::OpenDrain => out.insert(Self::open_drain),
        DriverType::OpenSource => out.insert(Self::open_source),
        DriverType::BusHold => out.insert(Self::bus_hold),
        DriverType::Resistive => out.insert(Self::resistive),
        DriverType::Resistive0 => out.insert(Self::resistive_0),
        DriverType::Resistive1 => out.insert(Self::resistive_1),
      }
    }
    Ok(out)
  }
}
impl fmt::Display for AllDriverType {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut has_write = false;
    if self.contains(Self::pull_up) {
      has_write = true;
      write!(f, "{}", DriverType::PullUp)?;
    }
    if self.contains(Self::pull_down) {
      if has_write {
        write!(f, " ")?;
      } else {
        has_write = true;
      }
      write!(f, "{}", DriverType::PullDown)?;
    }
    if self.contains(Self::open_drain) {
      if has_write {
        write!(f, " ")?;
      } else {
        has_write = true;
      }
      write!(f, "{}", DriverType::OpenDrain)?;
    }
    if self.contains(Self::open_source) {
      if has_write {
        write!(f, " ")?;
      } else {
        has_write = true;
      }
      write!(f, "{}", DriverType::OpenSource)?;
    }
    if self.contains(Self::bus_hold) {
      if has_write {
        write!(f, " ")?;
      } else {
        has_write = true;
      }
      write!(f, "{}", DriverType::BusHold)?;
    }
    if self.contains(Self::resistive) {
      if has_write {
        write!(f, " ")?;
      } else {
        has_write = true;
      }
      write!(f, "{}", DriverType::Resistive)?;
    }
    if self.contains(Self::resistive_0) {
      if has_write {
        write!(f, " ")?;
      } else {
        has_write = true;
      }
      write!(f, "{}", DriverType::Resistive0)?;
    }
    if self.contains(Self::resistive_1) {
      if has_write {
        write!(f, " ")?;
      }
      write!(f, "{}", DriverType::Resistive1)?;
    }
    Ok(())
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
crate::ast::impl_self_builder!(NextstateType);
crate::ast::impl_simple!(NextstateType);

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
crate::ast::impl_self_builder!(PinFuncType);
crate::ast::impl_simple!(PinFuncType);

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
crate::ast::impl_self_builder!(RestoreEdgeType);
crate::ast::impl_simple!(RestoreEdgeType);

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
crate::ast::impl_self_builder!(SignalType);
crate::ast::impl_simple!(SignalType);

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
crate::ast::impl_self_builder!(SlewControl);
crate::ast::impl_simple!(SlewControl);

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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum OneZero {
  /// 1
  #[strum(serialize = "1")]
  One,
  /// 0
  #[strum(serialize = "0")]
  Zero,
}
crate::ast::impl_self_builder!(OneZero);
crate::ast::impl_simple!(OneZero);

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
crate::ast::impl_self_builder!(TwoValue);
crate::ast::impl_simple!(TwoValue);

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

/// ### Example
/// ``` text
/// retention_pin (save | restore | save_restore, enumerated_type) ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=282.3&end=282.23
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PinClass {
  /// `save`
  Save,
  /// `restore`
  Restore,
  /// `save_restore`
  SaveRestore,
  PinName(String),
}
impl FromStr for PinClass {
  type Err = ();
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "save" => Ok(Self::Save),
      "restore" => Ok(Self::Restore),
      "save_restore" => Ok(Self::SaveRestore),
      _ => Ok(Self::PinName(s.to_owned())),
    }
  }
}
impl fmt::Display for PinClass {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Save => write!(f, "save"),
      Self::Restore => write!(f, "restore"),
      Self::SaveRestore => write!(f, "save_restore"),
      Self::PinName(pin) => write!(f, "{pin}"),
    }
  }
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
/// ### Syntax
/// ``` text
/// retention_pin (pin_class, disable_value) ;
/// ```
/// ### Example
/// ``` text
/// retention_pin (save | restore | save_restore, enumerated_type) ;
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=282.3&end=282.23
/// ">Reference-Definition</a>
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RetentionPin {
  /// `pin_class`
  pub pin_class: PinClass,
  /// `disable_value`
  pub disable_value: Static,
}
crate::ast::impl_self_builder!(RetentionPin);
impl<C: Ctx> ComplexAttri<C> for RetentionPin {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let pin_class: PinClass = match iter.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(_) => return Err(ComplexParseError::Other),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let disable_value = match iter.next() {
      Some(&s) => match s {
        "1" => Static::H,
        "0" => Static::L,
        "X" | "x" => Static::X,
        "Z" | "z" => Static::Z,
        _ => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      Err(ComplexParseError::LengthDismatch)
    } else {
      Ok(Self { pin_class, disable_value })
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(
      f,
      "{}, {}",
      self.pin_class,
      match self.disable_value {
        Static::X => "x",
        Static::Z => "z",
        Static::H => "1",
        Static::L => "0",
      }
    )
  }
}

#[cfg(test)]
mod test {
  use crate::DefaultCtx;

  use super::*;
  #[test]
  fn two_value() {
    assert_eq!(Ok(TwoValue(OneValue::Unkown, OneValue::One)), TwoValue::from_str("x1"));
    assert_eq!(Ok(TwoValue(OneValue::Zero, OneValue::One)), TwoValue::from_str("01"));
    assert_eq!(Err(strum::ParseError::VariantNotFound), TwoValue::from_str("1"));
    assert_eq!(Err(strum::ParseError::VariantNotFound), TwoValue::from_str("111"));
    assert_eq!(Err(strum::ParseError::VariantNotFound), TwoValue::from_str("1-"));
  }
  #[test]
  fn retention_pin() {
    let cell = crate::ast::test_parse_fmt::<crate::Cell<DefaultCtx>>(
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
  #[test]
  fn driver_type() {
    let cell = crate::ast::test_parse_fmt::<crate::Cell<DefaultCtx>>(
      r#"(cell1){
        pin(A){
          direction: inout;
          driver_type : "pull_up pull_down open_drain open_source bus_hold";
        }
        pin(B){
          driver_type : pull_up;
        }
        pin(C){
          driver_type : "bus_hold";
        }
      }"#,
      r#"
liberty_db::cell::Cell (cell1) {
| pin (A) {
| | direction : inout;
| | driver_type : "pull_up pull_down open_drain open_source bus_hold";
| }
| pin (B) {
| | driver_type : pull_up;
| }
| pin (C) {
| | driver_type : open_source bus_hold;
| }
}"#,
    );
  }
}
