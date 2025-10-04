use crate::{
  Ctx,
  ast::{
    CodeFormatter, GroupComments, GroupFn, Indentation, ParseScope, RandomState,
    SimpleAttri, join_fmt,
  },
};
use core::{
  cmp::Ordering,
  convert::Infallible,
  fmt::{self, Write},
  hash,
  str::FromStr,
};
use indexmap::IndexSet;
use itertools::Itertools as _;
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
  pub inner: IndexSet<String, RandomState>,
}
impl fmt::Display for WordSet {
  #[expect(clippy::unwrap_in_result)]
  #[expect(clippy::unwrap_used)]
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.inner.len() {
      0 => Ok(()),
      1 => write!(f, "{}", self.inner.iter().next().unwrap()),
      _ => join_fmt(
        self.inner.iter().sorted(),
        f,
        |s, ff| ff.write_str(s.as_str()),
        |ff| write!(ff, " "),
      ),
    }
  }
}

impl Ord for WordSet {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.inner.as_slice().cmp(other.inner.as_slice())
  }
}
impl PartialOrd for WordSet {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
crate::ast::impl_self_builder!(WordSet);
impl<C: 'static + Ctx> SimpleAttri<C> for WordSet {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope<'_>,
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
    for item in &self.inner {
      item.hash(state);
    }
  }
}

impl FromStr for WordSet {
  type Err = Infallible;
  #[inline]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut inner: IndexSet<_, _> = s
      .split(' ')
      .filter_map(|_s| if _s.is_empty() { None } else { Some(String::from(_s)) })
      .collect();
    inner.sort_unstable();
    Ok(Self { inner })
  }
}

#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[mut_set::derive::item]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct DummyGroup<C: 'static + Ctx> {
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
impl<C: 'static + Ctx> GroupFn<C> for DummyGroup<C> {}

impl From<String> for WordSet {
  #[inline]
  fn from(value: String) -> Self {
    let mut inner = IndexSet::with_capacity_and_hasher(1, RandomState::default());
    _ = inner.insert(value);
    Self { inner }
  }
}

impl From<&str> for WordSet {
  #[inline]
  fn from(value: &str) -> Self {
    let mut inner = IndexSet::with_capacity_and_hasher(1, RandomState::default());
    _ = inner.insert(value.to_owned());
    Self { inner }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MaxMin {
  Max,
  Min,
}
