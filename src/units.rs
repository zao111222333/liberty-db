#![allow(clippy::unsafe_derive_deserialize, clippy::undocumented_unsafe_blocks)]
//! The unit system

use crate::{
  Ctx,
  ast::{
    CodeFormatter, ComplexAttri, ComplexParseError, Indentation, ParseScope, SimpleAttri,
  },
  common::{f64_into_hash_ord_fn, parse_f64},
};
use core::{
  cmp::Ordering,
  fmt::{self, Write},
};

/// Valid values are 1ps, 10ps, 100ps, and 1ns. The default is 1ns.
///
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=42.25&end=42.30
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIter)]
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
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1ps => 1E-12_f64,
      Self::_10ps => 1E-11_f64,
      Self::_100ps => 1E-10_f64,
      Self::_1ns => 1E-9_f64,
    }
  }
}
crate::ast::impl_self_builder!(TimeUnit);
impl<C: Ctx> SimpleAttri<C> for TimeUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIter)]
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
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1mV => 1E-3_f64,
      Self::_10mV => 1E-2_f64,
      Self::_100mV => 1E-1_f64,
      Self::_1V => 1E0_f64,
    }
  }
}
crate::ast::impl_self_builder!(VoltageUnit);
impl<C: Ctx> SimpleAttri<C> for VoltageUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIter)]
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
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1uA => 1E-6_f64,
      Self::_10uA => 1E-5_f64,
      Self::_100uA => 1E-4_f64,
      Self::_1mA => 1E-3_f64,
      Self::_10mA => 1E-2_f64,
      Self::_100mA => 1E-1_f64,
      Self::_1A => 1E0_f64,
    }
  }
}
crate::ast::impl_self_builder!(CurrentUnit);
impl<C: Ctx> SimpleAttri<C> for CurrentUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIter)]
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
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1ohm => 1E0_f64,
      Self::_10ohm => 1E1_f64,
      Self::_100ohm => 1E2_f64,
      Self::_1kohm => 1E3_f64,
    }
  }
}
crate::ast::impl_self_builder!(PullingResistanceUnit);
impl<C: Ctx> SimpleAttri<C> for PullingResistanceUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
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
#[derive(Debug, Clone, Copy)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CapacitiveLoadUnit {
  /// `ff`: `true`
  ///
  /// `pf`: `false`
  pub ff_pf: bool,
  pub val: f64,
}

impl Default for CapacitiveLoadUnit {
  #[inline]
  fn default() -> Self {
    Self::_1pf
  }
}

impl PartialEq for CapacitiveLoadUnit {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    f64_into_hash_ord_fn(&self.value()) == f64_into_hash_ord_fn(&other.value())
  }
}

impl fmt::Display for CapacitiveLoadUnit {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("{}", self.val))?;
    if self.ff_pf { f.write_str(" ff") } else { f.write_str(" pf") }
  }
}

impl Eq for CapacitiveLoadUnit {}

impl PartialOrd for CapacitiveLoadUnit {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for CapacitiveLoadUnit {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    f64_into_hash_ord_fn(&self.value()).cmp(&f64_into_hash_ord_fn(&other.value()))
  }
}

impl CapacitiveLoadUnit {
  #[expect(non_upper_case_globals)]
  pub const _1pf: Self = Self { ff_pf: false, val: 1.0 };
  #[inline]
  #[must_use]
  #[expect(clippy::float_arithmetic)]
  pub fn value(&self) -> f64 {
    if self.ff_pf { self.val * 1e-15 } else { self.val * 1e-12 }
  }
}
crate::ast::impl_self_builder!(CapacitiveLoadUnit);
impl<C: Ctx> ComplexAttri<C> for CapacitiveLoadUnit {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let val = match iter.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let ff_pf = match iter.next() {
      Some(&s) => match s {
        "ff" => true,
        "pf" => false,
        _ => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { ff_pf, val })
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(self.val)?;
    if self.ff_pf { f.write_str(", ff") } else { f.write_str(", pf") }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(strum::EnumString, strum::Display, strum::AsRefStr, strum::EnumIter)]
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
  #[must_use]
  pub const fn value(&self) -> f64 {
    match self {
      Self::_1pW => 1E-12_f64,
      Self::_10pW => 1E-11_f64,
      Self::_100pW => 1E-10_f64,
      Self::_1nW => 1E-9_f64,
      Self::_10nW => 1E-8_f64,
      Self::_100nW => 1E-7_f64,
      Self::_1uW => 1E-6_f64,
      Self::_10uW => 1E-5_f64,
      Self::_100uW => 1E-4_f64,
      Self::_1mW => 1E-3_f64,
      Self::_10mW => 1E-2_f64,
      Self::_100mW => 1E-1_f64,
      Self::_1W => 1E-0_f64,
    }
  }
}
crate::ast::impl_self_builder!(LeakagePowerUnit);
impl<C: Ctx> SimpleAttri<C> for LeakagePowerUnit {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
  }
}
