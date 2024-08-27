//!
//! implement basic types
//!
use core::fmt::{self, Write};

use itertools::Itertools;

use crate::{
  ast::{
    is_word, join_fmt_no_quote, parser::simple_custom, CodeFormatter, ComplexAttri,
    ComplexParseError, IdError, Indentation, NameAttri, SimpleAttri,
  },
  ArcStr, NotNan,
};

use super::items::{Formula, NameList, WordSet};
impl SimpleAttri for f64 {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    simple_custom(i, line_num, nom::number::complete::double)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let mut buffer = ryu::Buffer::new();
    f.write_str(buffer.format(*self))
  }
}

impl SimpleAttri for NotNan<f64> {
  #[inline]
  #[allow(clippy::undocumented_unsafe_blocks)]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    #[inline]
    fn f(i: &str) -> nom::IResult<&str, NotNan<f64>, nom::error::Error<&str>> {
      nom::combinator::map(nom::number::complete::double, |f| unsafe {
        NotNan::<f64>::new_unchecked(f)
      })(i)
    }
    simple_custom(i, line_num, f)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let float: f64 = (*self).into();
    let mut buffer = ryu::Buffer::new();
    f.write_str(buffer.format(float))
  }
}

impl SimpleAttri for bool {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
}
impl SimpleAttri for usize {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let mut buffer = itoa::Buffer::new();
    f.write_str(buffer.format(*self))
  }
}

impl SimpleAttri for isize {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let mut buffer = itoa::Buffer::new();
    f.write_str(buffer.format(*self))
  }
}

impl NameAttri for Option<ArcStr> {
  #[inline]
  fn parse(mut v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l > 1 {
      Err(IdError::length_dismatch(1, l, v))
    } else {
      Ok(v.pop().map(ArcStr::from))
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

impl NameAttri for ArcStr {
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
    let l = v.len();
    match l {
      0 => Err(IdError::length_dismatch(1, 0, v)),
      #[allow(clippy::indexing_slicing)]
      1 => Ok(Self::Name(v[0].into())),
      _ => Ok(Self::List(WordSet { inner: v.into_iter().map(ArcStr::from).collect() })),
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

impl NameAttri for Vec<ArcStr> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    Ok(v.into_iter().map(ArcStr::from).collect())
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
impl<const N: usize> NameAttri for [ArcStr; N] {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, IdError> {
    let l = v.len();
    if l == N {
      match TryInto::<[ArcStr; N]>::try_into(
        v.into_iter().map(ArcStr::from).collect::<Vec<ArcStr>>(),
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

impl SimpleAttri for ArcStr {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    crate::ast::nom_parse_from_str(i, line_num)
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

impl<const N: usize> ComplexAttri for [ArcStr; N] {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let l = v.len();
    if l != N {
      return Err(ComplexParseError::LengthDismatch);
    }
    TryInto::<[ArcStr; N]>::try_into(
      v.iter().map(|&s| ArcStr::from(s)).collect::<Vec<ArcStr>>(),
    )
    .map_or(Err(ComplexParseError::Other), Ok)
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
    crate::ast::join_fmt(self.iter(), f, |s, ff| write!(ff, "{s}"), ", ")
  }
}
impl<const N: usize> ComplexAttri for [NotNan<f64>; N] {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let l = v.len();
    if l != N {
      return Err(ComplexParseError::LengthDismatch);
    }
    v.iter()
      .map(|&s| s.parse::<NotNan<f64>>())
      .collect::<Result<Vec<NotNan<f64>>, _>>()
      .map_or_else(
        |e| Err(ComplexParseError::Float(e)),
        |_v| {
          TryInto::<[NotNan<f64>; N]>::try_into(_v)
            .map_or(Err(ComplexParseError::Other), Ok)
        },
      )
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
    let mut buffer = ryu::Buffer::new();
    crate::ast::join_fmt(
      self.iter(),
      f,
      |float, ff| write!(ff, "{}", buffer.format(Into::<f64>::into(*float))),
      ", ",
    )
  }
}

impl ComplexAttri for Vec<f64> {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    match v.iter().map(|&s| s.parse()).collect() {
      Ok(r) => Ok(r),
      Err(e) => {
        Err(ComplexParseError::Float(ordered_float::ParseNotNanError::ParseFloatError(e)))
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
    let mut buffer = ryu::Buffer::new();
    crate::ast::join_fmt(
      self.iter(),
      f,
      |float, ff| write!(ff, "{}", buffer.format(*float)),
      ", ",
    )
  }
}

impl ComplexAttri for super::items::IdVector {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut iter = v.iter();
    let id = if let Some(&id_str) = iter.next() {
      id_str.parse()?
    } else {
      return Err(ComplexParseError::LengthDismatch);
    };
    let vec = match iter.map(|&s| s.parse()).collect() {
      Ok(r) => r,
      Err(e) => return Err(ComplexParseError::Float(e)),
    };
    Ok(Self { id, vec })
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{}, \\\n{}", itoa::Buffer::new().format(self.id), f.indentation())?;
    let mut buffer = ryu::Buffer::new();
    crate::ast::join_fmt(
      self.vec.iter(),
      f,
      |float, ff| write!(ff, "{}", buffer.format(Into::<f64>::into(*float))),
      ", ",
    )
  }
}

impl ComplexAttri for Vec<NotNan<f64>> {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    match v.iter().map(|&s| s.parse()).collect() {
      Ok(r) => Ok(r),
      Err(e) => Err(ComplexParseError::Float(e)),
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
    let mut buffer = ryu::Buffer::new();
    crate::ast::join_fmt(
      self.iter(),
      f,
      |float, ff| write!(ff, "{}", buffer.format(Into::<f64>::into(*float))),
      ", ",
    )
  }
}

impl ComplexAttri for ArcStr {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
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
impl ComplexAttri for NotNan<f64> {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let v1: Self = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Float(e)),
      },
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
    let mut buffer = ryu::Buffer::new();
    let float: f64 = (*self).into();
    write!(f, "{}", buffer.format(float))
  }
}
impl ComplexAttri for Vec<ArcStr> {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    Ok(v.iter().map(|&s| ArcStr::from(s)).collect())
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
impl ComplexAttri for Vec<usize> {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    match v.iter().map(|&s| s.parse()).collect() {
      Ok(r) => Ok(r),
      Err(e) => Err(ComplexParseError::Int(e)),
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
    let mut buffer = itoa::Buffer::new();
    join_fmt_no_quote(self.iter(), f, |i, ff| write!(ff, "{}", buffer.format(*i)), ", ")
  }
}

impl ComplexAttri for (f64, f64, ArcStr) {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let v1: f64 = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => {
          return Err(ComplexParseError::Float(
            ordered_float::ParseNotNanError::ParseFloatError(e),
          ))
        }
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2: f64 = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => {
          return Err(ComplexParseError::Float(
            ordered_float::ParseNotNanError::ParseFloatError(e),
          ))
        }
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v3: ArcStr = match i.next() {
      Some(&s) => ArcStr::from(s),
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
    let mut buffer = ryu::Buffer::new();
    write!(f, "{}, ", buffer.format(self.0))?;
    write!(f, "{}, {}", buffer.format(self.1), self.2)
  }
}
impl ComplexAttri for (NotNan<f64>, NotNan<f64>) {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let v1: NotNan<f64> = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Float(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2: NotNan<f64> = match i.next() {
      Some(&s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Float(e)),
      },
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
    let mut buffer = ryu::Buffer::new();
    write!(f, "{}, ", buffer.format(Into::<f64>::into(self.0)))?;
    write!(f, "{}", buffer.format(Into::<f64>::into(self.1)))
  }
}

impl fmt::Display for Formula {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}
impl SimpleAttri for Formula {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> crate::ast::SimpleParseErr<'a, Self> {
    #[inline]
    fn f(i: &str) -> nom::IResult<&str, Formula, nom::error::Error<&str>> {
      nom::combinator::map(crate::ast::parser::formula, |s| Formula(ArcStr::from(s)))(i)
    }
    simple_custom(i, line_num, f)
  }
}
