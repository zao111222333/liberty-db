use crate::{
  Ctx,
  ast::{
    CodeFormatter, GroupComments, GroupFn, Indentation, ParseScope, SimpleAttri, join_fmt,
  },
};
use core::{
  cmp::Ordering,
  fmt::{self, Write},
  hash,
  str::FromStr,
};
use itertools::Itertools as _;
use std::collections::HashSet;
use strum::{Display, EnumString};

/// The `sdf_edges` attribute defines the edge specification on both
/// the start pin and the end pin. The default is noedge.
///
/// #### Syntax
/// `sdf_edges : sdf_edge_type;`
///
/// `sdf_edge_type`: One of these four edge types: `noedge`, `start_edge`,
/// `end_edge`, or `both_edges`. The default is `noedge`.
///
/// #### Example
/// ``` liberty
/// sdf_edges : both_edges;
/// sdf_edges : start_edge ; /* edge specification on starting pin */
/// sdf_edges : end_edge ; /* edge specification on end pin */
/// ```
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=328.7&end=328.17
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default, EnumString, Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum SdfEdgeType {
  #[default]
  #[strum(serialize = "noedge")]
  Noedge,
  #[strum(serialize = "start_edge")]
  StartEdge,
  #[strum(serialize = "end_edge")]
  EndEdge,
  #[strum(serialize = "both_edges")]
  BothEdges,
}
crate::ast::impl_self_builder!(SdfEdgeType);
crate::ast::impl_simple!(SdfEdgeType);

#[mut_set::derive::item]
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IdVector {
  #[id]
  pub id: usize,
  pub vec: Vec<f64>,
}

/// sth. like "A B C" will save as set{A B C}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WordSet {
  pub inner: HashSet<String, crate::ast::RandomState>,
}
impl fmt::Display for WordSet {
  #[expect(clippy::unwrap_in_result)]
  #[expect(clippy::unwrap_used)]
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.inner.len() {
      0 => Ok(()),
      1 => write!(f, "{}", self.inner.iter().next().unwrap()),
      _ => join_fmt(self.inner.iter().sorted(), f, |s, ff| ff.write_str(s.as_str()), " "),
    }
  }
}

impl Ord for WordSet {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    match self.inner.len().cmp(&other.inner.len()) {
      Ordering::Less => Ordering::Less,
      Ordering::Greater => Ordering::Greater,
      Ordering::Equal => self.inner.iter().sorted().cmp(other.inner.iter().sorted()),
    }
  }
}

#[expect(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for WordSet {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.inner.len().cmp(&other.inner.len()) {
      Ordering::Less => self.inner.is_subset(&other.inner).then_some(Ordering::Less),
      Ordering::Greater => {
        self.inner.is_superset(&other.inner).then_some(Ordering::Greater)
      }
      Ordering::Equal => {
        self.inner.iter().sorted().partial_cmp(other.inner.iter().sorted())
      }
    }
  }
}
crate::ast::impl_self_builder!(WordSet);
impl<C: Ctx> SimpleAttri<C> for WordSet {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str::<C, _>(i, scope)
  }
  #[inline]
  fn is_set(&self) -> bool {
    !self.inner.is_empty()
  }
  #[inline]
  fn fmt_self<T: Write, I: Indentation>(
    &self,
    f: &mut CodeFormatter<'_, T, I>,
  ) -> fmt::Result {
    write!(f, "{self}")
  }
}
impl hash::Hash for WordSet {
  #[inline]
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    let mut sum = 0_u64;
    for item in &self.inner {
      let mut hasher = std::hash::DefaultHasher::new();
      item.hash(&mut hasher);
      sum = sum.wrapping_add(hash::Hasher::finish(&hasher));
    }
    state.write_u64(sum);
  }
}

impl FromStr for WordSet {
  type Err = fmt::Error;

  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      inner: s
        .split(' ')
        .filter_map(|_s| if _s.is_empty() { None } else { Some(String::from(_s)) })
        .collect(),
    })
  }
}

#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct DummyGroup<C: Ctx> {
  #[liberty(name)]
  #[id]
  name: Option<String>,
  /// group comments
  #[liberty(comments)]
  comments: GroupComments,
  #[liberty(extra_ctx)]
  pub extra_ctx: C::Other,
  /// group undefined attributes
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
}
impl<C: Ctx> GroupFn<C> for DummyGroup<C> {}

// /// Recursive type for boolean expression tree.
// #[derive(serde::Serialize, serde::Deserialize)]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum _Formula {
//   Float(f64),
//   Variable(String),
//   Neg(Box<_Formula>),
//   Add(Box<_Formula>, Box<_Formula>),
//   Sub(Box<_Formula>, Box<_Formula>),
//   Mul(Box<_Formula>, Box<_Formula>),
//   Div(Box<_Formula>, Box<_Formula>),
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NameList {
  Name(String),
  List(WordSet),
}
impl From<String> for NameList {
  #[inline]
  fn from(value: String) -> Self {
    Self::Name(value)
  }
}
impl From<&str> for NameList {
  #[inline]
  fn from(value: &str) -> Self {
    Self::Name(value.into())
  }
}
impl NameList {
  #[inline]
  #[must_use]
  pub fn contains(&self, name: &str) -> bool {
    match self {
      Self::Name(s) => s.as_str() == name,
      Self::List(word_set) => word_set.inner.contains(name),
    }
  }
}
impl Default for NameList {
  #[inline]
  fn default() -> Self {
    Self::Name(String::new())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MaxMin {
  Max,
  Min,
}
