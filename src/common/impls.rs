//!
//! implement basic types
//!

use crate::ast::{
  ComplexAttri, ComplexParseError, ComplexWrapper, IdError, NameAttri, SimpleAttri,
  SimpleWrapper,
};

impl SimpleAttri for f64 {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = ryu::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl SimpleAttri for bool {}
impl SimpleAttri for usize {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl SimpleAttri for isize {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl NameAttri for Option<String> {
  #[inline]
  fn parse(mut v: Vec<String>) -> Result<Self, IdError> {
    Ok(v.pop())
  }
  #[inline]
  fn into_vec(&self) -> Vec<String> {
    match self {
      Some(s) => vec![s.clone()],
      None => vec![],
    }
  }
}

impl NameAttri for String {
  #[inline]
  fn parse(mut v: Vec<String>) -> Result<Self, IdError> {
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
  fn into_vec(&self) -> Vec<String> {
    vec![self.clone()]
  }
}

impl NameAttri for Vec<String> {
  fn parse(v: Vec<String>) -> Result<Self, IdError> {
    Ok(v)
  }

  fn into_vec(&self) -> Vec<String> {
    self.clone()
  }
}
impl NameAttri for (String, String, usize) {
  fn parse(mut v: Vec<String>) -> Result<Self, IdError> {
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

    // match TryInto::<[String; N]>::try_into(v) {
    //   Ok(name) => Ok(name),
    //   Err(e) => Err(crate::ast::IdError::Other(format!("try_into error: {:?}", e))),
    // }
  }

  fn into_vec(&self) -> Vec<String> {
    vec![self.0.clone(), self.1.clone(), self.2.to_string()]
  }
}
impl<const N: usize> NameAttri for [String; N] {
  fn parse(v: Vec<String>) -> Result<Self, IdError> {
    let l = v.len();
    if l != N {
      return Err(crate::ast::IdError::LengthDismatch(N, l, v));
    }
    match TryInto::<[String; N]>::try_into(v) {
      Ok(name) => Ok(name),
      Err(e) => Err(crate::ast::IdError::Other(format!("try_into error: {:?}", e))),
    }
  }

  fn into_vec(&self) -> Vec<String> {
    self.to_vec()
  }
}

impl SimpleAttri for String {}

impl ComplexAttri for Vec<f64> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    match v.into_iter().map(|s| s.parse()).collect() {
      Ok(r) => Ok(r),
      Err(e) => Err(ComplexParseError::Float(e)),
    }
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![self.iter().map(|f| buffer.format(*f).to_string()).collect()]
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
    vec![self.iter().map(|i| buffer.format(*i).to_string()).collect()]
  }
}

impl ComplexAttri for (usize, String) {
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: usize = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Int(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2: String = match i.next() {
      Some(s) => s.to_owned(),
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if let Some(_) = i.next() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok((v1, v2))
  }

  fn to_wrapper(&self) -> crate::ast::ComplexWrapper {
    let mut buffer = itoa::Buffer::new();
    vec![vec![buffer.format(self.0).to_string(), self.1.clone()]]
  }
}
impl ComplexAttri for (f64, f64) {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: f64 = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Float(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let v2: f64 = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Float(e)),
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
    vec![vec![buffer.format(self.0).to_string(), buffer.format(self.1).to_string()]]
  }
}
