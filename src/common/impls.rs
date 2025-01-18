//!
//! implement basic types
//!
use crate::{
  ast::{
    self, is_word, join_fmt_no_quote, CodeFormatter, ComplexAttri, ComplexParseError,
    ComplexParseRes, IdError, Indentation, NameAttri, ParseScope, SimpleAttri,
  },
  LibertyStr,
};
use core::{
  fmt::{self, Write},
  str::FromStr,
};
use itertools::Itertools as _;

use super::{
  items::{Formula, NameList, WordSet},
  parse_f64,
};
crate::ast::impl_self_builder!(f64);
impl SimpleAttri for f64 {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_custom(i, &mut scope.line_num, ast::parser::float_one)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(*self)
  }
}
crate::ast::impl_self_builder!(bool);
impl SimpleAttri for bool {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str(i, scope)
  }
}
crate::ast::impl_self_builder!(usize);
impl SimpleAttri for usize {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str(i, scope)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(*self)
  }
}
crate::ast::impl_self_builder!(isize);
impl SimpleAttri for isize {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str(i, scope)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(*self)
  }
}

impl NameAttri for Option<LibertyStr> {
  #[inline]
  fn parse(mut v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l > 1 {
      Err(IdError::length_dismatch(1, l, v))
    } else {
      Ok(v.pop().map(LibertyStr::from))
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    self.as_ref().map_or(Ok(()), |s| {
      if is_word(s) {
        write!(f, "{s}")
      } else {
        write!(f, "\"{s}\"")
      }
    })
  }
}

impl NameAttri for LibertyStr {
  #[inline]
  fn parse(mut v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l != 1 {
      return Err(IdError::length_dismatch(1, l, v));
    }
    v.pop()
      .map_or(Err(IdError::Other("Unkown pop error".into())), |s| Ok(s.into()))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if is_word(self) {
      write!(f, "{self}")
    } else {
      write!(f, "\"{self}\"")
    }
  }
}

impl NameAttri for NameList {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    match v.len() {
      0 => Err(IdError::length_dismatch(1, 0, v)),
      #[expect(clippy::indexing_slicing)]
      1 => Ok(Self::Name(v[0].into())),
      _ => Ok(Self::List(WordSet { inner: v.into_iter().map(LibertyStr::from).collect() })),
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{self}")
  }
}
impl FromStr for NameList {
  type Err = ();
  #[inline]
  #[expect(clippy::unwrap_in_result, clippy::unwrap_used)]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut v: Vec<_> = s
      .split(' ')
      .filter_map(|_s| if _s.is_empty() { None } else { Some(LibertyStr::from(_s)) })
      .collect();
    match v.len() {
      0 => Err(()),
      1 => Ok(Self::Name(v.pop().unwrap())),
      _ => Ok(Self::List(WordSet { inner: v.into_iter().map(LibertyStr::from).collect() })),
    }
  }
}
crate::ast::impl_self_builder!(NameList);
impl SimpleAttri for NameList {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> ast::SimpleParseRes<'a, Self::Builder> {
    ast::nom_parse_from_str(i, scope)
  }
  #[inline]
  fn is_set(&self) -> bool {
    match self {
      Self::Name(s) => !s.is_empty(),
      Self::List(word_set) => word_set.is_set(),
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    match self {
      Self::Name(s) => {
        if is_word(s) {
          write!(f, "{s}")
        } else {
          write!(f, "\"{s}\"")
        }
      }
      Self::List(set) => join_fmt_no_quote(
        set.inner.iter().sorted(),
        f,
        |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
        " ",
      ),
    }
  }
}

impl fmt::Display for NameList {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Name(s) => {
        if is_word(s) {
          write!(f, "{s}")
        } else {
          write!(f, "\"{s}\"")
        }
      }
      Self::List(set) => join_fmt_no_quote(
        set.inner.iter().sorted(),
        f,
        |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
        ", ",
      ),
    }
  }
}

impl NameAttri for Vec<LibertyStr> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    Ok(v.into_iter().map(LibertyStr::from).collect())
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    join_fmt_no_quote(
      self.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      ", ",
    )
  }
}
impl<const N: usize> NameAttri for [LibertyStr; N] {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l == N {
      match TryInto::<[LibertyStr; N]>::try_into(
        v.into_iter().map(LibertyStr::from).collect::<Vec<LibertyStr>>(),
      ) {
        Ok(name) => Ok(name),
        Err(e) => Err(IdError::Other(format!("try_into error: {e:?}"))),
      }
    } else {
      Err(IdError::length_dismatch(N, l, v))
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    join_fmt_no_quote(
      self.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      ", ",
    )
  }
}
crate::ast::impl_self_builder!(LibertyStr);
impl SimpleAttri for LibertyStr {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str(i, scope)
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if is_word(self) {
      write!(f, "{self}")
    } else {
      write!(f, "\"{self}\"")
    }
  }
}
impl<const N: usize> ast::ParsingBuilder for [LibertyStr; N] {
  type Builder = Self;
  #[inline]
  fn build(builder: Self::Builder, _scope: &mut ast::BuilderScope) -> Self {
    builder
  }
}
impl<const N: usize> ComplexAttri for [LibertyStr; N] {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let v = iter.map(|&s| LibertyStr::from(s)).collect::<Vec<LibertyStr>>();
    if v.len() == N {
      TryInto::<[LibertyStr; N]>::try_into(v).map_or(Err(ComplexParseError::Other), Ok)
    } else {
      Err(ComplexParseError::LengthDismatch)
    }
  }
  #[inline]
  fn is_set(&self) -> bool {
    N == 0
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    ast::join_fmt(self.iter(), f, |s, ff| write!(ff, "{s}"), ", ")
  }
}

impl<const N: usize> ast::ParsingBuilder for [f64; N] {
  type Builder = Self;
  #[inline]
  fn build(builder: Self::Builder, _scope: &mut ast::BuilderScope) -> Self {
    builder
  }
}

impl<const N: usize> ComplexAttri for [f64; N] {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let v = iter.map(parse_f64).collect::<Result<Vec<f64>, _>>()?;
    if v.len() == N {
      TryInto::<[f64; N]>::try_into(v).map_or(Err(ComplexParseError::Other), Ok)
    } else {
      Err(ComplexParseError::LengthDismatch)
    }
  }
  #[inline]
  fn is_set(&self) -> bool {
    N == 0
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    ast::join_fmt(self.iter(), f, |float, ff| ff.write_float(*float), ", ")
  }
}
crate::ast::impl_self_builder!(super::items::IdVector);
impl ComplexAttri for super::items::IdVector {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    _iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    unreachable!()
  }
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    match ast::parser::complex_id_vector(i, &mut scope.line_num) {
      Ok((_i, (id, vec))) => Ok((_i, Ok(Self { id, vec }))),
      Err(_) => {
        Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Many0)))
      }
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(self.id)?;
    write!(f, ", \\\n{}", f.indentation())?;
    ast::join_fmt(self.vec.iter(), f, |float, ff| ff.write_float(*float), ", ")
  }
}
crate::ast::impl_self_builder!(Vec<f64>);
impl ComplexAttri for Vec<f64> {
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    _iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    unreachable!()
  }
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ComplexParseRes<'a, Self> {
    match ast::parser::complex_float_vec(i, &mut scope.line_num) {
      Ok((_i, v)) => Ok((_i, Ok(v))),
      Err(_) => {
        Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Many0)))
      }
    }
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    ast::join_fmt(self.iter(), f, |float, ff| ff.write_float(*float), ", ")
  }
}
impl ComplexAttri for LibertyStr {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter;
    let v1 = match i.next() {
      Some(&s) => Self::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(v1)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if is_word(self) {
      write!(f, "{self}")
    } else {
      write!(f, "\"{self}\"")
    }
  }
}
impl ComplexAttri for f64 {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter;
    let v1: Self = match i.next() {
      Some(&s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(v1)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(*self)
  }
}
crate::ast::impl_self_builder!(Vec<LibertyStr>);
impl ComplexAttri for Vec<LibertyStr> {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    Ok(iter.map(|&s| LibertyStr::from(s)).collect())
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    join_fmt_no_quote(
      self.iter(),
      f,
      |s, ff| if is_word(s) { write!(ff, "{s}") } else { write!(ff, "\"{s}\"") },
      ", ",
    )
  }
}
crate::ast::impl_self_builder!(Vec<usize>);
impl ComplexAttri for Vec<usize> {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    iter
      .map(|&s| s.parse())
      .collect::<Result<Self, _>>()
      .map_err(ComplexParseError::Int)
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    join_fmt_no_quote(self.iter(), f, |i, ff| ff.write_int(*i), ", ")
  }
}
crate::ast::impl_self_builder!((f64, f64, LibertyStr));
impl ComplexAttri for (f64, f64, LibertyStr) {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter;
    let v1 = match i.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2 = match i.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v3: LibertyStr = match i.next() {
      Some(&s) => LibertyStr::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2, v3))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self.0)?;
    f.write_str(", ")?;
    f.write_float(self.1)?;
    f.write_str(", ")?;
    f.write_str(&self.2)
  }
}
crate::ast::impl_self_builder!((i64, f64));
impl ComplexAttri for (i64, f64) {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter;
    let v1 = match i.next() {
      Some(s) => s.parse::<i64>()?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2 = match i.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_int(self.0)?;
    f.write_str(", ")?;
    f.write_float(self.1)
  }
}
crate::ast::impl_self_builder!((f64, f64));
impl ComplexAttri for (f64, f64) {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let mut i = iter;
    let v1 = match i.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2 = match i.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if i.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_float(self.0)?;
    f.write_str(", ")?;
    f.write_float(self.1)
  }
}

impl fmt::Display for Formula {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}
crate::ast::impl_self_builder!(Formula);
impl SimpleAttri for Formula {
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope) -> ast::SimpleParseRes<'a, Self> {
    #[inline]
    fn f(i: &str) -> nom::IResult<&str, Formula, nom::error::Error<&str>> {
      nom::combinator::map(ast::parser::formula, |s| Formula(LibertyStr::from(s)))(i)
    }
    ast::parser::simple_custom(i, &mut scope.line_num, f)
  }
}
