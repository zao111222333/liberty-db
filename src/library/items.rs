//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
//! </script>
// use std::ops::DerefMut;

use crate::{
  ast::{AttributeList, ComplexAttri, GroupComments, GroupId},
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
pub struct Sensitization {
  #[liberty(id(auto_impl_len = 1))]
  _id: GroupId<Self>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  pub pin_names: Vec<GroupId<Pin>>,
  #[liberty(complex)]
  pub vector: (usize, String),
}
