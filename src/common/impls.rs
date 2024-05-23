//!
//! implement basic types
//!
use crate::{
  ast::{
    ComplexAttri, ComplexParseError, ComplexWrapper, IdError, NameAttri, SimpleAttri,
    SimpleWrapper,
  },
  FastStr,
};
use itertools::Itertools;
use ordered_float::NotNan;
impl SimpleAttri for f64 {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = ryu::Buffer::new();
    FastStr::new(buffer.format(*self))
  }
}

impl SimpleAttri for NotNan<f64> {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let f: f64 = (*self).into();
    let mut buffer = ryu::Buffer::new();
    FastStr::new(buffer.format(f))
  }
}

impl SimpleAttri for bool {}
impl SimpleAttri for usize {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    FastStr::new(buffer.format(*self))
  }
}

impl SimpleAttri for isize {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    FastStr::new(buffer.format(*self))
  }
}

impl NameAttri for Option<FastStr> {
  #[inline]
  fn parse(mut v: Vec<FastStr>) -> Result<Self, IdError> {
    Ok(v.pop())
  }
  #[inline]
  fn to_vec(self) -> Vec<FastStr> {
    match self {
      Some(s) => vec![s],
      None => vec![],
    }
  }
}

impl NameAttri for FastStr {
  #[inline]
  fn parse(mut v: Vec<FastStr>) -> Result<Self, IdError> {
    let l = v.len();
    if l != 1 {
      return Err(IdError::LengthDismatch(1, l, v));
    }
    if let Some(name) = v.pop() {
      Ok(name)
    } else {
      return Err(IdError::Other("Unkown pop error".into()));
    }
  }
  #[inline]
  fn to_vec(self) -> Vec<FastStr> {
    vec![self]
  }
}

impl NameAttri for Vec<FastStr> {
  #[inline]
  fn parse(v: Vec<FastStr>) -> Result<Self, IdError> {
    Ok(v)
  }
  #[inline]
  fn to_vec(self) -> Vec<FastStr> {
    self
  }
}
impl NameAttri for (FastStr, FastStr, usize) {
  #[inline]
  fn parse(mut v: Vec<FastStr>) -> Result<Self, IdError> {
    let l = v.len();
    if l != 3 {
      return Err(crate::ast::IdError::LengthDismatch(3, l, v));
    }
    if let Some(s3) = v.pop() {
      match s3.parse::<usize>() {
        Ok(s3) => {
          if let Some(s2) = v.pop() {
            if let Some(s1) = v.pop() {
              Ok((s1, s2, s3))
            } else {
              Err(IdError::Other("Unkown pop error".into()))
            }
          } else {
            Err(IdError::Other("Unkown pop error".into()))
          }
        }
        Err(e) => Err(IdError::Int(e)),
      }
    } else {
      Err(IdError::Other("Unkown pop error".into()))
    }
  }
  #[inline]
  fn to_vec(self) -> Vec<FastStr> {
    vec![self.0, self.1, self.2.to_string().into()]
  }
}
impl<const N: usize> NameAttri for [FastStr; N] {
  #[inline]
  fn parse(v: Vec<FastStr>) -> Result<Self, IdError> {
    let l = v.len();
    if l != N {
      return Err(crate::ast::IdError::LengthDismatch(N, l, v));
    }
    match TryInto::<[FastStr; N]>::try_into(v) {
      Ok(name) => Ok(name),
      Err(e) => Err(crate::ast::IdError::Other(format!("try_into error: {:?}", e))),
    }
  }
  #[inline]
  fn to_vec(self) -> Vec<FastStr> {
    self.into_iter().collect_vec()
  }
}

impl SimpleAttri for FastStr {}

impl<const N: usize> ComplexAttri for [FastStr; N] {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let l = v.len();
    if l != N {
      return Err(ComplexParseError::LengthDismatch);
    }
    match TryInto::<[FastStr; N]>::try_into(
      v.into_iter().map(FastStr::new).collect::<Vec<FastStr>>(),
    ) {
      Ok(name) => Ok(name),
      Err(_) => Err(ComplexParseError::Other),
    }
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![self.clone().into_iter().collect_vec()]
  }
}

impl ComplexAttri for Vec<f64> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    match v.into_iter().map(|s| s.parse()).collect() {
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
      vec![vec![format!(
        "{}",
        self.iter().map(|f| buffer.format(*f).to_string()).join(",")
      )
      .into()]]
    }
  }
}

impl ComplexAttri for Vec<NotNan<f64>> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    match v.into_iter().map(|s| s.parse()).collect() {
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
      vec![vec![format!(
        "{}",
        self
          .iter()
          .map(|f| buffer.format(f.into_inner()).to_string())
          .join(",")
      )
      .into()]]
    }
  }
}

impl ComplexAttri for FastStr {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: FastStr = match i.next() {
      Some(s) => FastStr::new(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if let Some(_) = i.next() {
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
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: NotNan<f64> = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Float(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if let Some(_) = i.next() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(v1)
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![FastStr::new(buffer.format(self.into_inner()))]]
  }
}
impl ComplexAttri for Vec<FastStr> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    Ok(v.into_iter().map(FastStr::new).collect())
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![self.clone()]
  }
}
impl ComplexAttri for Vec<usize> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    match v.into_iter().map(|s| s.parse()).collect() {
      Ok(r) => Ok(r),
      Err(e) => Err(ComplexParseError::Int(e)),
    }
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = itoa::Buffer::new();
    vec![self.iter().map(|i| FastStr::new(buffer.format(*i))).collect()]
  }
}

impl ComplexAttri for (f64, f64, FastStr) {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: f64 = match i.next() {
      Some(s) => match s.parse() {
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
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => {
          return Err(ComplexParseError::Float(
            ordered_float::ParseNotNanError::ParseFloatError(e),
          ))
        }
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v3: FastStr = match i.next() {
      Some(s) => FastStr::new(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if let Some(_) = i.next() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2, v3))
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![
      FastStr::new(buffer.format(self.0)),
      FastStr::new(buffer.format(self.1)),
      self.2.clone(),
    ]]
  }
}
impl ComplexAttri for (usize, FastStr) {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: usize = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Int(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2: FastStr = match i.next() {
      Some(s) => FastStr::new(s),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if let Some(_) = i.next() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = itoa::Buffer::new();
    vec![vec![FastStr::new(buffer.format(self.0)), self.1.clone()]]
  }
}
impl ComplexAttri for (f64, f64) {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: f64 = match i.next() {
      Some(s) => match s.parse() {
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
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => {
          return Err(ComplexParseError::Float(
            ordered_float::ParseNotNanError::ParseFloatError(e),
          ))
        }
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if let Some(_) = i.next() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![FastStr::new(buffer.format(self.0)), FastStr::new(buffer.format(self.1))]]
  }
}
