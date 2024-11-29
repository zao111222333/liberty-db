#![allow(clippy::unsafe_derive_deserialize, clippy::undocumented_unsafe_blocks)]
//! <script>
//! IFRAME('https://en.wikipedia.org/wiki/International_System_of_Units');
//! </script>

use crate::{
  ast::{self, CodeFormatter, ComplexAttri, Indentation, ParseScope, SimpleAttri},
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
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum TimeUnit {
  /// 1ps, 1e-12
  #[token("1ps")]
  _1ps,
  /// 10ps, 1e-11
  #[token("10ps")]
  _10ps,
  /// 100ps, 1e-10
  #[token("100ps")]
  _100ps,
  /// 1ns, 1e-9
  #[default]
  #[token("1ns")]
  _1ns,
}

impl TimeUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> NotNan<f64> {
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
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <TimeUnit as ast::NomParseTerm>::nom_parse,
    )
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
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum VoltageUnit {
  /// 1mV, 1e-3
  #[token("1mV")]
  _1mV,
  /// 10mV, 1e-2
  #[token("10mV")]
  _10mV,
  /// 100mV, 1e-1
  #[token("100mV")]
  _100mV,
  /// 1V, 1e0
  #[default]
  #[token("1V")]
  _1V,
}

impl VoltageUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> NotNan<f64> {
    match self {
      Self::_1mV => unsafe { NotNan::new_unchecked(1E-3_f64) },
      Self::_10mV => unsafe { NotNan::new_unchecked(1E-2_f64) },
      Self::_100mV => unsafe { NotNan::new_unchecked(1E-1_f64) },
      Self::_1V => unsafe { NotNan::new_unchecked(1E0_f64) },
    }
  }
}

impl SimpleAttri for VoltageUnit {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
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
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CurrentUnit {
  /// 1uA, 1e-6
  #[token("1uA")]
  _1uA,
  /// 10uA, 1e-5
  #[token("10uA")]
  _10uA,
  /// 100uA, 1e-4
  #[token("100uA")]
  _100uA,
  /// 1mA, 1e-3
  #[token("1mA")]
  _1mA,
  /// 10mA, 1e-2
  #[token("10mA")]
  _10mA,
  /// 100mA, 1e-1
  #[token("100mA")]
  _100mA,
  /// 1A, 1e0
  #[token("1A")]
  _1A,
}

impl CurrentUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> NotNan<f64> {
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
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
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
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PullingResistanceUnit {
  /// 1ohm, 1
  #[token("1ohm")]
  _1ohm,
  /// 10ohm, 10
  #[token("10ohm")]
  _10ohm,
  /// 100ohm, 100
  #[token("100ohm")]
  _100ohm,
  /// 1kohm, 1000
  #[token("1kohm")]
  _1kohm,
}

impl PullingResistanceUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> NotNan<f64> {
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
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
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
  #[must_use]
  #[expect(clippy::arithmetic_side_effects, clippy::float_arithmetic)]
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
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::ComplexParseRes<'a, Self> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map};
    ast::parser::complex2(
      i,
      &mut scope.line_num,
      ast::parser::parse_float,
      alt((map(tag("ff"), |_| true), map(tag("pf"), |_| false))),
      |val, ff_pf| Self { ff_pf, val },
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self.val.into_inner())?;
    if self.ff_pf {
      f.write_str(", ff")
    } else {
      f.write_str(", pf")
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
#[derive(liberty_macros::EnumToken)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum LeakagePowerUnit {
  /// 1pW, 1e-12
  #[token("1pW")]
  _1pW,
  /// 10pW, 1e-11
  #[token("10pW")]
  _10pW,
  /// 100pW, 1e-10
  #[token("100pW")]
  _100pW,
  /// 1nW, 1e-9
  #[token("1nW")]
  _1nW,
  /// 10nW, 1e-8
  #[token("10nW")]
  _10nW,
  /// 100nW, 1e-7
  #[token("100nW")]
  _100nW,
  /// 1uW, 1e-6
  #[token("1uW")]
  _1uW,
  /// 10uW, 1e-5
  #[token("10uW")]
  _10uW,
  /// 100uW, 1e-4
  #[token("100uW")]
  _100uW,
  /// 1mW, 1e-3
  #[token("1mW")]
  _1mW,
  /// 10mW, 1e-2
  #[token("10mW")]
  _10mW,
  /// 100mW, 1e-1
  #[token("100mW")]
  _100mW,
  /// 1W, 1e0
  #[token("1W")]
  _1W,
}

impl LeakagePowerUnit {
  #[inline]
  #[must_use]
  pub const fn value(&self) -> NotNan<f64> {
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
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_basic(
      i,
      &mut scope.line_num,
      <Self as ast::NomParseTerm>::nom_parse,
    )
  }
}
