//!
//! implement basic types
//!

impl super::SimpleAttri for f64 {
  #[inline]
  fn to_wrapper(&self) -> super::SimpleWrapper {
    let mut buffer = ryu::Buffer::new();
    buffer.format(*self).to_string()
  }
}

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
  type Error = std::num::ParseFloatError;
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, Self::Error> {
    v.into_iter().map(|s| s.parse()).collect()
  }
  #[inline]
  fn to_wrapper(&self) -> Option<super::ComplexWrapper> {
    if self.is_empty() {
      None
    } else {
      let mut buffer = ryu::Buffer::new();
      Some(vec![self.iter().map(|f| buffer.format(*f).to_string()).collect()])
    }
  }
}

// impl super::ComplexAttri for Vec<Vec<f64>> {
//   type Error=std::num::ParseFloatError;
//   fn parse<'a>(v: &'a Vec<Vec<&'a str>>)->Result<Self,Self::Error> {
//     println!("{:?}",v);
//     v.iter()
//       .map(|inner_vec| {
//           inner_vec
//               .iter()
//               .map(|&s| s.parse::<f64>())
//               .collect()
//       })
//       .collect()
//   }

//   fn to_wrapper(&self) -> Option<super::ComplexWrapper> {
//     if self.is_empty(){
//       None
//     }else{
//       if self[0].is_empty(){
//         None
//       }else{
//         let mut buffer = ryu::Buffer::new();
//         Some(self.iter().map(|inner_vec|
//           inner_vec.iter().map(|f|buffer.format(*f).to_string()).collect::<Vec<String>>()
//         ).collect())
//       }
//     }
//   }
// }

impl super::ComplexAttri for Vec<usize> {
  type Error = std::num::ParseIntError;
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, Self::Error> {
    v.into_iter().map(|s| s.parse()).collect()
  }
  #[inline]
  fn to_wrapper(&self) -> Option<super::ComplexWrapper> {
    if self.is_empty() {
      None
    } else {
      let mut buffer = itoa::Buffer::new();
      Some(vec![self.iter().map(|i| buffer.format(*i).to_string()).collect()])
    }
  }
}
impl super::ComplexAttri for Option<(f64, f64)> {
  type Error = crate::ast::ComplexParseError;
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, Self::Error> {
    let mut i = v.into_iter();
    let v1: f64 = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(Self::Error::Float(e)),
      },
      None => return Err(Self::Error::LengthDismatch),
    };
    let v2: f64 = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(Self::Error::Float(e)),
      },
      None => return Err(Self::Error::LengthDismatch),
    };
    if let Some(_) = i.next() {
      return Err(Self::Error::LengthDismatch);
    }
    Ok(Some((v1, v2)))
  }
  #[inline]
  fn to_wrapper(&self) -> Option<super::ComplexWrapper> {
    if let Some((v1, v2)) = self {
      let mut buffer = ryu::Buffer::new();
      Some(vec![vec![buffer.format(*v1).to_string(), buffer.format(*v2).to_string()]])
    } else {
      None
    }
  }
}
