//!
//! implement basic types
//!
use core::fmt::{self, Write};

use crate::{
  ast::{
    is_word, CodeFormatter, ComplexAttri, ComplexParseError, IdError, Indentation,
    NameAttri, SimpleAttri,
  },
  ArcStr, NotNan,
};
use itertools::Itertools;
impl SimpleAttri for f64 {
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
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    let float: f64 = (*self).into();
    let mut buffer = ryu::Buffer::new();
    f.write_str(buffer.format(float))
  }
}

impl SimpleAttri for bool {}
impl SimpleAttri for usize {
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
  fn parse(mut v: Vec<ArcStr>) -> Result<Self, IdError> {
    Ok(v.pop())
  }
  #[inline]
  fn to_vec(self) -> Vec<ArcStr> {
    self.map_or(vec![], |s| vec![s])
  }
}

impl NameAttri for ArcStr {
  #[inline]
  fn parse(mut v: Vec<ArcStr>) -> Result<Self, IdError> {
    let l = v.len();
    if l != 1 {
      return Err(IdError::LengthDismatch(1, l, v));
    }
    v.pop().ok_or(IdError::Other("Unkown pop error".into()))
  }
  #[inline]
  fn to_vec(self) -> Vec<ArcStr> {
    vec![self]
  }
}

impl NameAttri for Vec<ArcStr> {
  #[inline]
  fn parse(v: Vec<ArcStr>) -> Result<Self, IdError> {
    Ok(v)
  }
  #[inline]
  fn to_vec(self) -> Vec<ArcStr> {
    self
  }
}
impl NameAttri for (ArcStr, ArcStr, usize) {
  #[inline]
  fn parse(mut v: Vec<ArcStr>) -> Result<Self, IdError> {
    let l = v.len();
    if l != 3 {
      return Err(IdError::LengthDismatch(3, l, v));
    }
    v.pop().map_or(Err(IdError::Other("Unkown pop error".into())), |s3| {
      match s3.parse::<usize>() {
        Ok(s3_i) => {
          v.pop().map_or(Err(IdError::Other("Unkown pop error".into())), |s2| {
            v.pop().map_or(Err(IdError::Other("Unkown pop error".into())), |s1| {
              Ok((s1, s2, s3_i))
            })
          })
        }
        Err(e) => Err(IdError::Int(e)),
      }
    })
  }
  #[inline]
  fn to_vec(self) -> Vec<ArcStr> {
    vec![self.0, self.1, self.2.to_string().into()]
  }
}
impl<const N: usize> NameAttri for [ArcStr; N] {
  #[inline]
  fn parse(v: Vec<ArcStr>) -> Result<Self, IdError> {
    let l = v.len();
    if l != N {
      return Err(IdError::LengthDismatch(N, l, v));
    }
    match TryInto::<[ArcStr; N]>::try_into(v) {
      Ok(name) => Ok(name),
      Err(e) => Err(IdError::Other(format!("try_into error: {e:?}"))),
    }
  }
  #[inline]
  fn to_vec(self) -> Vec<ArcStr> {
    self.into_iter().collect_vec()
  }
}

impl SimpleAttri for ArcStr {
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
      |float, ff| {
        let float64: f64 = (*float).into();
        write!(ff, "{}", buffer.format(float64))
      },
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
    crate::ast::join_fmt(self.iter(), f, |s, ff| write!(ff, "{s}"), ", ")
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
    crate::ast::join_fmt(
      self.iter(),
      f,
      |i, ff| write!(ff, "{}", buffer.format(*i)),
      ", ",
    )
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
impl ComplexAttri for (f64, f64) {
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
    write!(f, "{}, ", buffer.format(self.0))?;
    write!(f, "{}", buffer.format(self.1))
  }
}
