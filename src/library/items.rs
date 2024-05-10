//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
//! </script>
// use std::ops::DerefMut;

use crate::{
  ast::{AttributeList, ComplexAttri, ComplexParseError, GroupComments, GroupFn},
  pin::Pin,
};

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =66.4
/// &end
/// =66.21
/// ">Reference-Definition</a>
#[derive(Debug, Clone, Default)]
#[derive(liberty_macros::Group)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct Sensitization {
  #[id]
  #[liberty(name)]
  pub name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  /// TODO
  pub pin_names: Vec<<Pin as mut_set::Item>::Id>,
  /// TODO
  #[liberty(complex)]
  pub vector: (usize, String),
}

impl GroupFn for Sensitization {}

#[derive(Debug, Clone, Default)]
#[mut_set_derive::item(
  sort,
  macro(derive(Debug, Clone,Default);)
)]
pub struct VoltageMap {
  #[id]
  pub name: String,
  pub voltage: f64,
}
impl ComplexAttri for VoltageMap {
  #[inline]
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let v1: String = match i.next() {
      Some(s) => s.to_owned(),
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
    Ok(Self { name: v1, voltage: v2 })
  }
  #[inline]
  fn to_wrapper(&self) -> crate::ast::ComplexWrapper {
    let mut buffer = ryu::Buffer::new();
    vec![vec![self.name.clone(), buffer.format(self.voltage).to_string()]]
  }
}
