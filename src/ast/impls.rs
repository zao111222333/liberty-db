//!
//! implement basic types
//!

impl super::SimpleAttri for f64 {
  type Error=std::num::ParseFloatError;
  fn parse(s: &str)->Result<Self,Self::Error> {
    s.parse()
  }
  fn to_wrapper(&self) -> super::SimpleWrapper {
    let mut buffer = ryu::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl super::SimpleAttri for bool {
  type Error=std::str::ParseBoolError;
  fn parse(s: &str)->Result<Self,Self::Error> {
    s.parse()
  }
}

impl super::SimpleAttri for usize {
  type Error=std::num::ParseIntError;
  fn parse(s: &str)->Result<Self,Self::Error> {
    s.parse()
  }
  fn to_wrapper(&self) -> super::SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl super::SimpleAttri for isize {
  type Error=std::num::ParseIntError;
  fn parse(s: &str)->Result<Self,Self::Error> {
    s.parse()
  }
  fn to_wrapper(&self) -> super::SimpleWrapper {
    let mut buffer = itoa::Buffer::new();
    buffer.format(*self).to_string()
  }
}

impl super::SimpleAttri for String {
  type Error=std::fmt::Error;
  fn parse(s: &str)->Result<Self,Self::Error> {
    Ok(s.to_string())
  }
  fn to_wrapper(&self) -> super::SimpleWrapper {
    self.to_string()
  }
}

impl super::ComplexAttri for Vec<f64> {
  type Error=std::num::ParseFloatError;
  fn parse<'a>(v: &'a Vec<Vec<&'a str>>)->Result<Self,Self::Error> {
    v.iter().flatten()
      .map(|s| s.parse())
      .collect()
  }

  fn to_wrapper(&self) -> Option<super::ComplexWrapper> {
    if self.is_empty(){
      None
    }else{
      let mut buffer = ryu::Buffer::new();
      Some(vec![self.iter().map(|f|buffer.format(*f).to_string()).collect()])
    }
  }
}

impl super::ComplexAttri for Vec<usize> {
  type Error=std::num::ParseIntError;
  fn parse<'a>(v: &'a Vec<Vec<&'a str>>)->Result<Self,Self::Error> {
    v.iter().flatten()
      .map(|s| s.parse())
      .collect()
  }

  fn to_wrapper(&self) -> Option<super::ComplexWrapper> {
    if self.is_empty(){
      None
    }else{
      let mut buffer = itoa::Buffer::new();
      Some(vec![self.iter().map(|i|buffer.format(*i).to_string()).collect()])
    }
  }
}
impl super::ComplexAttri for Option<(f64,f64)> {
  type Error=crate::ast::ComplexParseError;

  fn parse(v: &Vec<Vec<&str>>)->Result<Self,Self::Error> {
    let mut i = v.iter().flatten();
    let v1: f64 = match i.next(){
      Some(s) => match s.parse(){
        Ok(f) => f,
        Err(e) => return Err(Self::Error::Float(e)),
      },
      None => return Err(Self::Error::LengthDismatch),
    };
    let v2: f64 = match i.next(){
      Some(s) => match s.parse(){
        Ok(f) => f,
        Err(e) => return Err(Self::Error::Float(e)),
      },
      None => return Err(Self::Error::LengthDismatch),
    };
    if let Some(_) = i.next(){
      return Err(Self::Error::LengthDismatch);
    }
    Ok(Some((v1,v2)))
  }

  fn to_wrapper(&self) -> Option<super::ComplexWrapper> {
    if let Some((v1,v2)) = self {
      let mut buffer = ryu::Buffer::new();
      Some(vec![vec![buffer.format(*v1).to_string(),buffer.format(*v2).to_string()]])
    }else{
      None
    }
  }
}