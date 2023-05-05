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
  fn is_empty(&self) -> bool {
    self.is_empty()
  }
  fn parse<'a>(v: &'a Vec<Vec<&'a str>>)->Result<Self,Self::Error> {
    v.iter().flatten()
      .map(|s| s.parse())
      .collect()
  }

  fn to_wrapper(&self) -> super::ComplexWrapper {
    vec![self.iter().map(|f|format!("{:.10E}",f)).collect()]
  }
}


impl super::ComplexAttri for crate::units::Capacitance {
  type Error=std::fmt::Error;

  fn parse(v: &Vec<Vec<&str>>)->Result<Self,Self::Error> {
      todo!()
  }

  fn is_empty(&self) -> bool {
      let zero = <Self as Default>::default();
      self.eq(&zero)
  }

  fn to_wrapper(&self) -> crate::ast::ComplexWrapper {
      todo!()
  }
}