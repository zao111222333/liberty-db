use crate::{
  Ctx,
  ast::{ParseScope, impl_self_builder},
};
use core::{cmp::Ordering, fmt::Write, hash::Hash};

/// Level
#[derive(Ord, PartialOrd)]
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(strum::Display, strum::EnumString, strum::EnumIter, strum::IntoStaticStr)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Level {
  /// High
  #[strum(serialize = "h", serialize = "1", to_string = "H")]
  H,
  /// Low
  #[strum(serialize = "l", serialize = "0", to_string = "L")]
  L,
}
impl From<Level> for bool {
  #[inline]
  fn from(value: Level) -> Self {
    match value {
      Level::H => true,
      Level::L => false,
    }
  }
}
impl From<bool> for Level {
  #[inline]
  fn from(value: bool) -> Self {
    if value { Self::H } else { Self::L }
  }
}
impl From<Level> for Normal {
  #[inline]
  fn from(value: Level) -> Self {
    match value {
      Level::H => Self::H,
      Level::L => Self::L,
    }
  }
}
impl Level {
  #[must_use]
  #[inline]
  pub const fn inverse(self) -> Self {
    match self {
      Self::H => Self::L,
      Self::L => Self::H,
    }
  }
  #[must_use]
  #[inline]
  pub const fn toggle_self(&mut self) -> Edge {
    match self {
      Self::H => {
        *self = Self::L;
        Edge::F
      }
      Self::L => {
        *self = Self::H;
        Edge::R
      }
    }
  }
  #[must_use]
  #[inline]
  pub const fn edge(bgn: Self, end: Self) -> Option<Edge> {
    match (bgn, end) {
      (Self::H, Self::H) | (Self::L, Self::L) => None,
      (Self::H, Self::L) => Some(Edge::F),
      (Self::L, Self::H) => Some(Edge::R),
    }
  }
}
impl From<Level> for State {
  #[inline]
  fn from(value: Level) -> Self {
    match value {
      Level::H => Self::H,
      Level::L => Self::L,
    }
  }
}

/// Edge
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum::EnumString, strum::EnumIter, strum::Display, strum::IntoStaticStr)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Edge {
  /// Rise
  #[strum(serialize = "R")]
  R,
  /// Fall
  #[strum(serialize = "F")]
  F,
}
impl From<Edge> for State {
  #[inline]
  fn from(value: Edge) -> Self {
    match value {
      Edge::F => Self::HL,
      Edge::R => Self::LH,
    }
  }
}
impl From<Edge> for Normal {
  #[inline]
  fn from(value: Edge) -> Self {
    match value {
      Edge::F => Self::F,
      Edge::R => Self::R,
    }
  }
}
impl Edge {
  #[must_use]
  #[inline]
  pub const fn bgn(&self) -> Level {
    match self {
      Self::R => Level::L,
      Self::F => Level::H,
    }
  }
  #[must_use]
  #[inline]
  pub const fn end(&self) -> Level {
    match self {
      Self::R => Level::H,
      Self::F => Level::L,
    }
  }
  #[must_use]
  #[inline]
  pub const fn inverse(&self) -> Self {
    match self {
      Self::R => Self::F,
      Self::F => Self::R,
    }
  }
  #[must_use]
  #[inline]
  pub const fn full_name(&self) -> &'static str {
    match self {
      Self::R => "rise",
      Self::F => "fall",
    }
  }
}
crate::ast::impl_self_builder!(Edge);
impl<C: 'static + Ctx> crate::ast::SimpleAttri<C> for Edge {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from::<C, _, _, _>(i, scope, |s| match s {
      "fall" => Ok(Self::F),
      "rise" => Ok(Self::R),
      _ => Err(()),
    })
  }
  #[inline]
  fn fmt_self<T: Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    use core::fmt::Write as _;
    f.write_str(self.full_name())
  }
}
impl<C: 'static + Ctx> crate::ast::ComplexAttri<C> for Edge {
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _: &mut ParseScope<'_>,
  ) -> Result<Self::Builder, crate::ast::ComplexParseError> {
    match iter.next().copied() {
      Some("rise") => {
        if iter.next().is_some() {
          Err(crate::ast::ComplexParseError::LengthDismatch)
        } else {
          Ok(Self::R)
        }
      }
      Some("fall") => {
        if iter.next().is_some() {
          Err(crate::ast::ComplexParseError::LengthDismatch)
        } else {
          Ok(Self::F)
        }
      }
      Some(_) => Err(crate::ast::ComplexParseError::UnsupportedWord),
      None => Err(crate::ast::ComplexParseError::LengthDismatch),
    }
  }

  fn fmt_self<T: Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    use core::fmt::Write as _;
    f.write_str(self.full_name())
  }
}
impl_self_builder!(Vec<Edge>);
impl<C: 'static + Ctx> crate::ast::ComplexAttri<C> for Vec<Edge> {
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _: &mut ParseScope<'_>,
  ) -> Result<Self::Builder, crate::ast::ComplexParseError> {
    iter
      .into_iter()
      .map(|s| match *s {
        "rise" => Ok(Edge::R),
        "fall" => Ok(Edge::F),
        _ => Err(crate::ast::ComplexParseError::UnsupportedWord),
      })
      .collect()
  }

  fn fmt_self<T: Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    crate::ast::join_fmt_no_quote(
      self.iter(),
      f,
      |edge, ff| ff.write_str(edge.full_name()),
      |ff| ff.write_str(", "),
    )
  }
}

impl core::ops::Not for &Edge {
  type Output = Edge;
  #[inline]
  fn not(self) -> Self::Output {
    match self {
      Edge::F => Edge::R,
      Edge::R => Edge::F,
    }
  }
}

/// `UnInit`
#[derive(Debug, Clone, Copy)]
#[derive(strum::Display, strum::EnumString, strum::EnumIter)]
#[derive(PartialEq, Hash, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UnInit {
  /// `Unknown`
  #[strum(serialize = "x", to_string = "X")]
  X,
  /// `HighImpedance`
  #[strum(serialize = "z", to_string = "Z")]
  Z,
}

impl From<UnInit> for State {
  #[inline]
  fn from(value: UnInit) -> Self {
    match value {
      UnInit::X => Self::X,
      UnInit::Z => Self::Z,
    }
  }
}

impl PartialOrd for UnInit {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for UnInit {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::X, Self::Z) => Ordering::Greater,
      (Self::Z, Self::X) => Ordering::Less,
      (Self::X, Self::X) | (Self::Z, Self::Z) => Ordering::Equal,
    }
  }
}
/// H L R F
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(strum::Display, strum::EnumString, strum::EnumIter)]
pub enum Normal {
  /// R
  #[strum(serialize = "R")]
  R,
  /// F
  #[strum(serialize = "F")]
  F,
  /// H
  #[strum(serialize = "H")]
  H,
  /// L
  #[strum(serialize = "L")]
  L,
}
crate::ast::impl_self_builder!(Normal);
crate::ast::impl_simple!(Normal);

impl From<Normal> for State {
  #[inline]
  fn from(value: Normal) -> Self {
    match value {
      Normal::R => Self::LH,
      Normal::F => Self::HL,
      Normal::H => Self::H,
      Normal::L => Self::L,
    }
  }
}

/// H L R F
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(strum::Display, strum::EnumString, strum::EnumIter)]
pub enum Static {
  /// X
  #[strum(serialize = "X")]
  X,
  /// Z
  #[strum(serialize = "Z")]
  Z,
  /// H
  #[strum(serialize = "H")]
  H,
  /// L
  #[strum(serialize = "L")]
  L,
}

impl From<Static> for State {
  #[inline]
  fn from(value: Static) -> Self {
    match value {
      Static::X => Self::X,
      Static::Z => Self::Z,
      Static::H => Self::H,
      Static::L => Self::L,
    }
  }
}

/// State
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(strum::Display, strum::EnumString, strum::EnumIter)]
pub enum State {
  /// L
  #[strum(serialize = "L")]
  L,
  /// H
  #[strum(serialize = "H")]
  H,
  /// X
  #[strum(serialize = "X")]
  X,
  /// Z
  #[strum(serialize = "Z")]
  Z,
  /// LH
  #[strum(serialize = "LH")]
  LH,
  /// HL
  #[strum(serialize = "HL")]
  HL,
  /// LX
  #[strum(serialize = "LX")]
  LX,
  /// LZ
  #[strum(serialize = "LZ")]
  LZ,
  /// HX
  #[strum(serialize = "HX")]
  HX,
  /// HZ
  #[strum(serialize = "HZ")]
  HZ,
  /// XL
  #[strum(serialize = "XL")]
  XL,
  /// ZL
  #[strum(serialize = "ZL")]
  ZL,
  /// XH
  #[strum(serialize = "XH")]
  XH,
  /// ZH
  #[strum(serialize = "ZH")]
  ZH,
}

impl State {
  /// `bgn` state
  ///
  /// R -> 0, F -> 1, otherwise not change
  #[must_use]
  #[inline]
  pub const fn bgn(&self) -> Static {
    #[expect(clippy::match_same_arms)]
    match self {
      Self::L => Static::L,
      Self::H => Static::H,
      Self::X => Static::X,
      Self::Z => Static::Z,
      Self::LH => Static::L,
      Self::HL => Static::H,
      Self::LX => Static::L,
      Self::LZ => Static::L,
      Self::HX => Static::H,
      Self::HZ => Static::H,
      Self::XL => Static::X,
      Self::ZL => Static::Z,
      Self::XH => Static::X,
      Self::ZH => Static::Z,
    }
  }
  /// `end` state
  ///
  /// R -> 1, F -> 1, otherwise not change
  #[must_use]
  #[inline]
  pub const fn end(&self) -> Static {
    #[expect(clippy::match_same_arms)]
    match self {
      Self::L => Static::L,
      Self::H => Static::H,
      Self::X => Static::X,
      Self::Z => Static::Z,
      Self::LH => Static::H,
      Self::HL => Static::L,
      Self::LX => Static::X,
      Self::LZ => Static::Z,
      Self::HX => Static::X,
      Self::HZ => Static::Z,
      Self::XL => Static::L,
      Self::ZL => Static::L,
      Self::XH => Static::H,
      Self::ZH => Static::H,
    }
  }
  /// | BGN(self) | END  | Combined|
  /// | :-------: | :--: | :-----: |
  /// | 1         | 0    | F       |
  /// | 1         | 1    | 1       |
  /// | 1         | X    | X       |
  /// | X         | 1    | 1       |
  /// | 1         | Z    | Z       |
  /// | Z         | 1    | 1       |
  /// | Any       | F/R  | Illegal |
  /// | F/R       | Any  | Illegal |
  #[must_use]
  #[inline]
  pub const fn combine_bgn_end(bgn: Static, end: Static) -> Self {
    #[expect(clippy::match_same_arms)]
    match (bgn, end) {
      (Static::X, Static::X) => Self::X,
      (Static::X, Static::Z) => Self::X,
      (Static::X, Static::H) => Self::XH,
      (Static::X, Static::L) => Self::XL,
      (Static::Z, Static::X) => Self::X,
      (Static::Z, Static::Z) => Self::X,
      (Static::Z, Static::H) => Self::ZH,
      (Static::Z, Static::L) => Self::ZL,
      (Static::H, Static::X) => Self::HX,
      (Static::H, Static::Z) => Self::HZ,
      (Static::H, Static::H) => Self::H,
      (Static::H, Static::L) => Self::HL,
      (Static::L, Static::X) => Self::LX,
      (Static::L, Static::Z) => Self::LZ,
      (Static::L, Static::H) => Self::LH,
      (Static::L, Static::L) => Self::L,
    }
  }
}
