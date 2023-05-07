//!
//! implement basic types
//!

impl super::SimpleAttri for f64 {
  type Error=std::num::ParseFloatError;
  fn parse(s: &str)->Result<Self,Self::Error> {
    s.parse()
  }
}

impl super::SimpleAttri for String {
  type Error=std::num::ParseFloatError;
  fn parse(s: &str)->Result<Self,Self::Error> {
    Ok(s.to_string())
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