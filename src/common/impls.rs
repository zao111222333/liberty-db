//!
//! implement basic types
//!
use super::parse_f64;
use crate::{
  Ctx,
  ast::{
    self, CodeFormatter, ComplexAttri, ComplexParseError, ComplexParseRes,
    FlattenNameAttri, IdError, Indentation, NameAttri, ParseScope, SimpleAttri, is_word,
    join_fmt, join_fmt_no_quote,
  },
  expression,
};
use core::fmt::{self, Write};

crate::ast::impl_self_builder!(f64);
impl<C: Ctx> SimpleAttri<C> for f64 {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> ast::SimpleParseRes<'a, Self> {
    let (new_i, expr_res) = ast::parser::simple_custom(
      i,
      &mut scope.loc.line_num,
      expression::FormulaExpr::parse,
      ast::parser::unquote,
    )?;
    let value_res = expr_res.and_then(|expr| {
      expr
        .eval(&expr, |k: &str| scope.variables.get(k).and_then(|f| f.value))
        .ok_or(expr.to_string())
    });
    Ok((new_i, value_res))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(*self)
  }
}
crate::ast::impl_self_builder!(bool);
crate::ast::impl_simple!(bool);

crate::ast::impl_self_builder!(usize);
impl<C: Ctx> SimpleAttri<C> for usize {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_custom(
      i,
      &mut scope.loc.line_num,
      ast::parser::int_usize,
      ast::parser::unquote,
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(*self)
  }
}
crate::ast::impl_self_builder!(isize);
impl<C: Ctx> SimpleAttri<C> for isize {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> ast::SimpleParseRes<'a, Self> {
    ast::parser::simple_custom(
      i,
      &mut scope.loc.line_num,
      ast::parser::int_isize,
      ast::parser::unquote,
    )
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(*self)
  }
}

impl NameAttri for Option<String> {
  #[inline]
  fn parse(mut v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l > 1 {
      Err(IdError::length_dismatch(1, l, v))
    } else {
      Ok(v.pop().map(String::from))
    }
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    self.as_ref().map_or(Ok(()), |s| {
      if is_word(s) { write!(f, "{s}") } else { write!(f, "\"{s}\"") }
    })
  }
}

impl NameAttri for String {
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
    if is_word(self) { write!(f, "{self}") } else { write!(f, "\"{self}\"") }
  }
}

impl FlattenNameAttri for String {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Vec<Self>, IdError> {
    Ok(v.into_iter().map(Self::from).collect())
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{self}")
  }
  #[inline]
  fn pretend_group(parsed: Vec<Self>) -> Self {
    parsed.join(";")
  }
  #[inline]
  fn ungroup(&self) -> Option<impl Iterator<Item = Self>> {
    self.contains(';').then(|| self.split(';').map(Self::from))
  }
}

impl NameAttri for Vec<String> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    Ok(v.into_iter().map(String::from).collect())
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
      |ff| write!(ff, ", "),
    )
  }
}
impl<const N: usize> NameAttri for [String; N] {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l == N {
      match TryInto::<[String; N]>::try_into(
        v.into_iter().map(String::from).collect::<Vec<String>>(),
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
      |ff| write!(ff, ", "),
    )
  }
}
crate::ast::impl_self_builder!(String);
impl<C: Ctx> SimpleAttri<C> for String {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
  ) -> ast::SimpleParseRes<'a, Self> {
    ast::nom_parse_from_str::<C, _>(i, scope)
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
    if is_word(self) { write!(f, "{self}") } else { write!(f, "\"{self}\"") }
  }
}
impl<const N: usize, C: Ctx> ast::ParsingBuilder<C> for [String; N] {
  type Builder = Self;
  #[inline]
  fn build(builder: Self::Builder, _scope: &mut ast::BuilderScope<C>) -> Self {
    builder
  }
}
impl<const N: usize, C: Ctx> ComplexAttri<C> for [String; N] {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let v = iter.map(|&s| String::from(s)).collect::<Vec<String>>();
    if v.len() == N {
      TryInto::<[String; N]>::try_into(v).map_or(Err(ComplexParseError::Other), Ok)
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
    join_fmt(self.iter(), f, |s, ff| write!(ff, "{s}"), |ff| write!(ff, ", "))
  }
}

impl<const N: usize, C: Ctx> ast::ParsingBuilder<C> for [f64; N] {
  type Builder = Self;
  #[inline]
  fn build(builder: Self::Builder, _scope: &mut ast::BuilderScope<C>) -> Self {
    builder
  }
}

impl<const N: usize, C: Ctx> ComplexAttri<C> for [f64; N] {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope<'_>,
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
    join_fmt(self.iter(), f, |float, ff| ff.write_num(*float), |ff| write!(ff, ", "))
  }
}
crate::ast::impl_self_builder!((String, f64));
impl<C: Ctx> ComplexAttri<C> for (String, f64) {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let v1 = match iter.next() {
      Some(&s) => s.to_owned(),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2 = match iter.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_str(&self.0)?;
    f.write_str(", ")?;
    f.write_num(self.1)
  }
}

crate::ast::impl_self_builder!(super::items::IdVector);
impl<C: Ctx> ComplexAttri<C> for super::items::IdVector {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    _iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    unreachable!()
  }
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> ComplexParseRes<'a, Self> {
    match ast::parser::complex_id_vector(i, &mut scope.loc.line_num) {
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
    f.write_num(self.id)?;
    write!(f, ", \\")?;
    f.write_new_line_indentation()?;
    join_fmt(self.vec.iter(), f, |float, ff| ff.write_num(*float), |ff| write!(ff, ", "))
  }
}
crate::ast::impl_self_builder!(Vec<f64>);
impl<C: Ctx> ComplexAttri<C> for Vec<f64> {
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    _iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    unreachable!()
  }
  #[inline]
  fn nom_parse<'a>(i: &'a str, scope: &mut ParseScope<'_>) -> ComplexParseRes<'a, Self> {
    match ast::parser::complex_float_vec(i, &mut scope.loc.line_num) {
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
  #[expect(clippy::indexing_slicing)]
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if self.len() == 1 {
      f.write_num(self[0])
    } else {
      join_fmt(self.iter(), f, |float, ff| ff.write_num(*float), |ff| write!(ff, ", "))
    }
  }
}
impl<C: Ctx> ComplexAttri<C> for String {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let v1 = match iter.next() {
      Some(&s) => Self::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(v1)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    if is_word(self) { write!(f, "{self}") } else { write!(f, "\"{self}\"") }
  }
}
impl<C: Ctx> ComplexAttri<C> for f64 {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let v1: Self = match iter.next() {
      Some(&s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(v1)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(*self)
  }
}
crate::ast::impl_self_builder!(Vec<String>);
impl<C: Ctx> ComplexAttri<C> for Vec<String> {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    Ok(iter.map(|&s| String::from(s)).collect())
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
      |ff| write!(ff, ", "),
    )
  }
}
crate::ast::impl_self_builder!(Vec<usize>);
impl<C: Ctx> ComplexAttri<C> for Vec<usize> {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    iter
      .map(|&s| lexical_core::parse(s.as_bytes()))
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
    join_fmt_no_quote(self.iter(), f, |i, ff| ff.write_num(*i), |ff| write!(ff, ", "))
  }
}
crate::ast::impl_self_builder!((f64, f64, String));
impl<C: Ctx> ComplexAttri<C> for (f64, f64, String) {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let v1 = match iter.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2 = match iter.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v3: String = match iter.next() {
      Some(&s) => String::from(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2, v3))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(self.0)?;
    f.write_str(", ")?;
    f.write_num(self.1)?;
    f.write_str(", ")?;
    f.write_str(&self.2)
  }
}
crate::ast::impl_self_builder!((i64, f64));
impl<C: Ctx> ComplexAttri<C> for (i64, f64) {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let v1 = match iter.next() {
      Some(s) => lexical_core::parse(s.as_bytes())?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2 = match iter.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(self.0)?;
    f.write_str(", ")?;
    f.write_num(self.1)
  }
}
crate::ast::impl_self_builder!((f64, f64));
impl<C: Ctx> ComplexAttri<C> for (f64, f64) {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope<'_>,
  ) -> Result<Self, ComplexParseError> {
    let v1 = match iter.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2 = match iter.next() {
      Some(s) => parse_f64(s)?,
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    f.write_num(self.0)?;
    f.write_str(", ")?;
    f.write_num(self.1)
  }
}
