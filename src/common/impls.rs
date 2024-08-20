//!
//! implement basic types
//!
use core::fmt::{self, Write};

use crate::{
  ast::{
    is_word, CodeFormatter, ComplexAttri, ComplexParseError, ComplexWrapper, IdError,
    Indentation, NameAttri, SimpleAttri,
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
      f.write_fmt(format_args!("{self}"))
    } else {
      f.write_fmt(format_args!("\"{self}\""))
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
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![self.clone().into_iter().collect_vec()]
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
  fn to_wrapper(&self) -> ComplexWrapper {
    if self.is_empty() {
      vec![vec![]]
    } else {
      let mut buffer = ryu::Buffer::new();
      vec![vec![self.iter().map(|f| buffer.format(*f).to_owned()).join(",").into()]]
    }
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
  fn to_wrapper(&self) -> ComplexWrapper {
    if self.is_empty() {
      vec![vec![]]
    } else {
      let mut buffer = ryu::Buffer::new();
      vec![vec![self
        .iter()
        .map(|f| buffer.format(f.into_inner()).to_owned())
        .join(",")
        .into()]]
    }
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
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![vec![self.clone()]]
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
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![ArcStr::from(buffer.format(self.into_inner()))]]
  }
}
impl ComplexAttri for Vec<ArcStr> {
  #[inline]
  fn parse(v: &[&str]) -> Result<Self, ComplexParseError> {
    Ok(v.iter().map(|&s| ArcStr::from(s)).collect())
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![self.clone()]
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
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = itoa::Buffer::new();
    vec![self.iter().map(|i| ArcStr::from(buffer.format(*i))).collect()]
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
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![
      ArcStr::from(buffer.format(self.0)),
      ArcStr::from(buffer.format(self.1)),
      self.2.clone(),
    ]]
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
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![ArcStr::from(buffer.format(self.0)), ArcStr::from(buffer.format(self.1))]]
  }
}
