//!
//! implement basic types
//!

use crate::{common::items::WordSet, expression::BooleanExpression};

use super::ComplexParseError;

impl super::SimpleAttri for f64 {
  #[inline]
  fn to_wrapper(&self) -> super::SimpleWrapper {
    let mut buffer = ryu::Buffer::new();
    buffer.format(*self).to_string()
  }
}

// impl super::SimpleAttri for BooleanExpression {}

impl super::SimpleAttri for WordSet {}
impl super::SimpleAttri for BooleanExpression {}
impl super::SimpleAttri for bool {}

impl super::SimpleAttri for usize {
  #[inline]
  fn to_wrapper(&self) -> super::SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl super::SimpleAttri for isize {
  #[inline]
  fn to_wrapper(&self) -> super::SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl super::SimpleAttri for String {}

impl super::ComplexAttri for Vec<f64> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    match v.into_iter().map(|s| s.parse()).collect() {
      Ok(r) => Ok(r),
      Err(e) => Err(ComplexParseError::Float(e)),
    }
  }
  #[inline]
  fn to_wrapper(&self) -> super::ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![self.iter().map(|f| buffer.format(*f).to_string()).collect()]
  }
}

impl super::ComplexAttri for Vec<usize> {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    match v.into_iter().map(|s| s.parse()).collect() {
      Ok(r) => Ok(r),
      Err(e) => Err(ComplexParseError::Int(e)),
    }
  }
  #[inline]
  fn to_wrapper(&self) -> super::ComplexWrapper {
    let mut buffer = itoa::Buffer::new();
    vec![self.iter().map(|i| buffer.format(*i).to_string()).collect()]
  }
}

impl super::ComplexAttri for (usize, String) {
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
impl super::ComplexAttri for (f64, f64) {
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
  fn to_wrapper(&self) -> super::ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![buffer.format(self.0).to_string(), buffer.format(self.1).to_string()]]
  }
}
