//! <script>
//! IFRAME('https://en.wikipedia.org/wiki/International_System_of_Units');
//! </script>

use crate::{
  ast::{
    CodeFormatter, ComplexAttri, ComplexParseError, Indentation, ParseScope, SimpleAttri,
  },
  common::parse_f64,
  NotNan,
};
use core::fmt::{self, Write};

/// Valid values are 1ps, 10ps, 100ps, and 1ns. The default is 1ns.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=42.25&end=42.30
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Default, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TimeUnit {
  /// 1ps, 1e-12
  #[strum(serialize = "1ps")]
  _1ps,
  /// 10ps, 1e-11
  #[strum(serialize = "10ps")]
  _10ps,
  /// 100ps, 1e-10
  #[strum(serialize = "100ps")]
  _100ps,
  /// 1ns, 1e-9
  #[default]
  #[strum(serialize = "1ns")]
  _1ns,
}

impl TimeUnit {
  #[inline]
  pub fn value(&self) -> NotNan<f64> {
    match self {
      Self::_1ps => unsafe { NotNan::new_unchecked(1E-12_f64) },
      Self::_10ps => unsafe { NotNan::new_unchecked(1E-11_f64) },
      Self::_100ps => unsafe { NotNan::new_unchecked(1E-10_f64) },
      Self::_1ns => unsafe { NotNan::new_unchecked(1E-9_f64) },
    }
  }
}

impl SimpleAttri for TimeUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

/// Valid values are 1mV, 10mV, 100mV, and 1V. The default is 1V.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.2&end=43.9
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Default, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum VoltageUnit {
  /// 1mV, 1e-3
  #[strum(serialize = "1mV")]
  _1mV,
  /// 10mV, 1e-2
  #[strum(serialize = "10mV")]
  _10mV,
  /// 100mV, 1e-1
  #[strum(serialize = "100mV")]
  _100mV,
  /// 1V, 1e0
  #[default]
  #[strum(serialize = "1V")]
  _1V,
}

impl VoltageUnit {
  #[inline]
  pub fn value(&self) -> NotNan<f64> {
    match self {
      VoltageUnit::_1mV => unsafe { NotNan::new_unchecked(1E-3_f64) },
      VoltageUnit::_10mV => unsafe { NotNan::new_unchecked(1E-2_f64) },
      VoltageUnit::_100mV => unsafe { NotNan::new_unchecked(1E-1_f64) },
      VoltageUnit::_1V => unsafe { NotNan::new_unchecked(1E0_f64) },
    }
  }
}

impl SimpleAttri for VoltageUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

/// The valid values are 1uA, 10uA, 100uA, 1mA, 10mA, 100mA, and 1A.
///
/// **No default exists for the `current_unit` attribute if the attribute is omitted.**
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.12&end=43.24
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CurrentUnit {
  /// 1uA, 1e-6
  #[strum(serialize = "1uA")]
  _1uA,
  /// 10uA, 1e-5
  #[strum(serialize = "10uA")]
  _10uA,
  /// 100uA, 1e-4
  #[strum(serialize = "100uA")]
  _100uA,
  /// 1mA, 1e-3
  #[strum(serialize = "1mA")]
  _1mA,
  /// 10mA, 1e-2
  #[strum(serialize = "10mA")]
  _10mA,
  /// 100mA, 1e-1
  #[strum(serialize = "100mA")]
  _100mA,
  /// 1A, 1e0
  #[strum(serialize = "1A")]
  _1A,
}

impl CurrentUnit {
  #[inline]
  pub fn value(&self) -> NotNan<f64> {
    match self {
      Self::_1uA => unsafe { NotNan::new_unchecked(1E-6_f64) },
      Self::_10uA => unsafe { NotNan::new_unchecked(1E-5_f64) },
      Self::_100uA => unsafe { NotNan::new_unchecked(1E-4_f64) },
      Self::_1mA => unsafe { NotNan::new_unchecked(1E-3_f64) },
      Self::_10mA => unsafe { NotNan::new_unchecked(1E-2_f64) },
      Self::_100mA => unsafe { NotNan::new_unchecked(1E-1_f64) },
      Self::_1A => unsafe { NotNan::new_unchecked(1E0_f64) },
    }
  }
}

impl SimpleAttri for CurrentUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

/// Valid unit values are 1ohm, 10ohm, 100ohm, and 1kohm.
///
/// **No default exists for `pulling_resistance_unit` if the attribute is omitted.**
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.25&end=44.4
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PullingResistanceUnit {
  /// 1ohm, 1
  #[strum(serialize = "1ohm")]
  _1ohm,
  /// 10ohm, 10
  #[strum(serialize = "10ohm")]
  _10ohm,
  /// 100ohm, 100
  #[strum(serialize = "100ohm")]
  _100ohm,
  /// 1kohm, 1000
  #[strum(serialize = "1kohm")]
  _1kohm,
}

impl PullingResistanceUnit {
  #[inline]
  pub fn value(&self) -> NotNan<f64> {
    match self {
      Self::_1ohm => unsafe { NotNan::new_unchecked(1E0_f64) },
      Self::_10ohm => unsafe { NotNan::new_unchecked(1E1_f64) },
      Self::_100ohm => unsafe { NotNan::new_unchecked(1E2_f64) },
      Self::_1kohm => unsafe { NotNan::new_unchecked(1E3_f64) },
    }
  }
}

impl SimpleAttri for PullingResistanceUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

/// This attribute specifies the unit for all capacitance
/// values within the logic library, including
/// default capacitances, `max_fanout` capacitances,
/// pin capacitances, and wire capacitances.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.7&end=44.19
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Default, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapacitiveLoadUnit {
  /// `ff`: `true`
  ///
  /// `pf`: `false`
  pub ff_pf: bool,
  pub val: NotNan<f64>,
}

impl CapacitiveLoadUnit {
  #[inline]
  pub fn value(&self) -> NotNan<f64> {
    if self.ff_pf {
      self.val * 1e-15
    } else {
      self.val * 1e-12
    }
  }
}

impl ComplexAttri for CapacitiveLoadUnit {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a Vec<&'a str>>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter.flat_map(IntoIterator::into_iter);
    let val = match i.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let ff_pf = match i.next() {
      Some(&s) => match s {
        "ff" => true,
        "pf" => false,
        _ => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { ff_pf, val })
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let mut buffer = ryu::Buffer::new();
    if self.ff_pf {
      write!(f, "{}, ff", buffer.format(self.val.into_inner()))
    } else {
      write!(f, "{}, pf", buffer.format(self.val.into_inner()))
    }
  }
}

/// This attribute indicates the units of the power values
/// in the library. If this attribute is missing, the
/// leakage-power values are expressed without units.
///
/// Valid values are 1W, 100mW, 10mW, 1mW, 100nW, 10nW, 1nW, 100pW, 10pW, and 1pW.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.22&end=44.31
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LeakagePowerUnit {
  /// 1pW, 1e-12
  #[strum(serialize = "1pW")]
  _1pW,
  /// 10pW, 1e-11
  #[strum(serialize = "10pW")]
  _10pW,
  /// 100pW, 1e-10
  #[strum(serialize = "100pW")]
  _100pW,
  /// 1nW, 1e-9
  #[strum(serialize = "1nW")]
  _1nW,
  /// 10nW, 1e-8
  #[strum(serialize = "10nW")]
  _10nW,
  /// 100nW, 1e-7
  #[strum(serialize = "100nW")]
  _100nW,
  /// 1uW, 1e-6
  #[strum(serialize = "1uW")]
  _1uW,
  /// 10uW, 1e-5
  #[strum(serialize = "10uW")]
  _10uW,
  /// 100uW, 1e-4
  #[strum(serialize = "100uW")]
  _100uW,
  /// 1mW, 1e-3
  #[strum(serialize = "1mW")]
  _1mW,
  /// 10mW, 1e-2
  #[strum(serialize = "10mW")]
  _10mW,
  /// 100mW, 1e-1
  #[strum(serialize = "100mW")]
  _100mW,
  /// 1W, 1e0
  #[strum(serialize = "1W")]
  _1W,
}

impl LeakagePowerUnit {
  #[inline]
  pub fn value(&self) -> NotNan<f64> {
    match self {
      Self::_1pW => unsafe { NotNan::new_unchecked(1E-12_f64) },
      Self::_10pW => unsafe { NotNan::new_unchecked(1E-11_f64) },
      Self::_100pW => unsafe { NotNan::new_unchecked(1E-10_f64) },
      Self::_1nW => unsafe { NotNan::new_unchecked(1E-9_f64) },
      Self::_10nW => unsafe { NotNan::new_unchecked(1E-8_f64) },
      Self::_100nW => unsafe { NotNan::new_unchecked(1E-7_f64) },
      Self::_1uW => unsafe { NotNan::new_unchecked(1E-6_f64) },
      Self::_10uW => unsafe { NotNan::new_unchecked(1E-5_f64) },
      Self::_100uW => unsafe { NotNan::new_unchecked(1E-4_f64) },
      Self::_1mW => unsafe { NotNan::new_unchecked(1E-3_f64) },
      Self::_10mW => unsafe { NotNan::new_unchecked(1E-2_f64) },
      Self::_100mW => unsafe { NotNan::new_unchecked(1E-1_f64) },
      Self::_1W => unsafe { NotNan::new_unchecked(1E-0_f64) },
    }
  }
}
impl SimpleAttri for LeakagePowerUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}
