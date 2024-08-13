//!
//! implement basic types
//!
use crate::{
  ast::{
    ComplexAttri, ComplexParseError, ComplexWrapper, IdError, NameAttri, SimpleAttri,
    SimpleWrapper,
  },
  ArcStr, NotNan,
};
use itertools::Itertools;
impl SimpleAttri for f64 {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = ryu::Buffer::new();
    ArcStr::from(buffer.format(*self))
  }
}

impl SimpleAttri for NotNan<f64> {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let f: f64 = (*self).into();
    let mut buffer = ryu::Buffer::new();
    ArcStr::from(buffer.format(f))
  }
}

impl SimpleAttri for bool {}
impl SimpleAttri for usize {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    ArcStr::from(buffer.format(*self))
  }
}

impl SimpleAttri for isize {
  #[inline]
  fn to_wrapper(&self) -> SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    ArcStr::from(buffer.format(*self))
  }
}

impl NameAttri for Option<ArcStr> {
  #[inline]
  fn parse(mut v: Vec<ArcStr>) -> Result<Self, IdError> {
    Ok(v.pop())
  }
  #[inline]
  fn to_vec(self) -> Vec<ArcStr> {
    match self {
      Some(s) => vec![s],
      None => vec![],
    }
  }
}

impl NameAttri for ArcStr {
  #[inline]
  fn parse(mut v: Vec<ArcStr>) -> Result<Self, IdError> {
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
      Err(e) => Err(IdError::Other(format!("try_into error: {:?}", e))),
    }
  }
  #[inline]
  fn to_vec(self) -> Vec<ArcStr> {
    self.into_iter().collect_vec()
  }
}

impl SimpleAttri for ArcStr {}

impl<const N: usize> ComplexAttri for [ArcStr; N] {
  #[inline]
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
    let l = v.len();
    if l != N {
      return Err(ComplexParseError::LengthDismatch);
    }
    match TryInto::<[ArcStr; N]>::try_into(
      v.iter().map(|&s| ArcStr::from(s)).collect::<Vec<ArcStr>>(),
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
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
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
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
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

impl ComplexAttri for ArcStr {
  #[inline]
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let v1: ArcStr = match i.next() {
      Some(&s) => ArcStr::from(s),
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
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.iter();
    let v1: NotNan<f64> = match i.next() {
      Some(&s) => match s.parse() {
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
    vec![vec![ArcStr::from(buffer.format(self.into_inner()))]]
  }
}
impl ComplexAttri for Vec<ArcStr> {
  #[inline]
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
    Ok(v.iter().map(|&s| ArcStr::from(s)).collect())
  }
  #[inline]
  fn to_wrapper(&self) -> ComplexWrapper {
    vec![self.clone()]
  }
}
impl ComplexAttri for Vec<usize> {
  #[inline]
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
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
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
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
    if let Some(_) = i.next() {
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
  fn parse(v: &Vec<&str>) -> Result<Self, ComplexParseError> {
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
    if let Some(_) = i.next() {
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
