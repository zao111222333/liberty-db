//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
//! </script>
// use std::ops::DerefMut;

use crate::{
  ast::{AttributeList, ComplexAttri, GroupComments},
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
#[mut_set_derive::item(derive(liberty_macros::Nothing, Debug, Clone))]
pub struct Sensitization {
  #[id]
  #[liberty(name)]
  pub name: String,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  pub pin_names: Vec<<Pin as mut_set::Item>::Id>,
  #[liberty(complex)]
  pub vector: (usize, String),
}
