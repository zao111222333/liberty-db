use crate::{
  ast::{
    join_fmt, CodeFormatter, GroupComments, GroupFn, Indentation, ParseScope, SimpleAttri,
  },
  ArcStr, Ctx, NotNan,
};
use core::{
  cmp::Ordering,
  fmt::{self, Write},
  hash,
  str::FromStr,
};
use itertools::Itertools as _;
use std::collections::HashSet;
use strum_macros::{Display, EnumString};

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
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =210.31
/// &end
/// =211.2
/// ">Reference-Definition</a>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =203.47
/// &end
/// =203.47
/// ">Reference-Instance</a>
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
impl SimpleAttri for SdfEdgeType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

#[mut_set::derive::item(sort)]
#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct IdVector {
  #[id]
  pub id: usize,
  pub vec: Vec<NotNan<f64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Display, EnumString)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum VariableType {
  #[strum(serialize = "input_net_transition")]
  InputNetTransition,
  #[strum(serialize = "normalized_voltage")]
  NormalizedVoltage,
  #[strum(serialize = "total_output_net_capacitance")]
  TotalOutputNetCapacitance,
  #[strum(serialize = "related_out_total_output_net_capacitance")]
  RelatedOutTotalOutputNetCapacitance,
  #[strum(serialize = "constrained_pin_transition")]
  ConstrainedPinTransition,
  #[strum(serialize = "fanout_number")]
  FanoutNumber,
  #[strum(serialize = "fanout_pin_capacitance")]
  FanoutPinCapacitance,
  #[strum(serialize = "driver_slew")]
  DriverSlew,
  #[strum(serialize = "input_transition_time")]
  InputTransitionTime,
}
crate::ast::impl_self_builder!(VariableType);
impl SimpleAttri for VariableType {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =38.48
/// &end
/// =39.24
/// ">Reference-Definition</a>
#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Dummy: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Domain<C: Ctx> {
  #[size = 8]
  #[liberty(name)]
  #[id(borrow = "&str", with_ref = false)]
  pub name: ArcStr,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  extra_ctx: C::Dummy,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
  #[size = 8]
  #[liberty(simple(type = Option))]
  pub calc_mode: Option<ArcStr>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub variable_1: Option<VariableType>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub variable_2: Option<VariableType>,
  #[size = 1]
  #[liberty(simple(type = Option))]
  pub variable_3: Option<VariableType>,
  #[size = 24]
  #[liberty(complex)]
  pub index_1: Vec<NotNan<f64>>,
  #[size = 24]
  #[liberty(complex)]
  pub index_2: Vec<NotNan<f64>>,
  #[size = 24]
  #[liberty(complex)]
  pub index_3: Vec<NotNan<f64>>,
}
impl<C: Ctx> GroupFn for Domain<C> {}
/// sth. like "A B C" will save as set{A B C}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct WordSet {
  pub inner: HashSet<ArcStr, crate::ast::RandomState>,
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
    if self.inner.is_subset(&other.inner) {
      Ordering::Less
    } else if self.inner.is_superset(&other.inner) {
      Ordering::Greater
    } else {
      Ordering::Equal
    }
  }
}

#[expect(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for WordSet {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.inner.len() == other.inner.len() {
      self.inner.iter().sorted().partial_cmp(other.inner.iter().sorted())
    } else if self.inner.is_subset(&other.inner) {
      Some(Ordering::Less)
    } else if self.inner.is_superset(&other.inner) {
      Some(Ordering::Greater)
    } else {
      None
    }
  }
}
crate::ast::impl_self_builder!(WordSet);
impl SimpleAttri for WordSet {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
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
        .filter_map(|_s| if _s.is_empty() { None } else { Some(ArcStr::from(_s)) })
        .collect(),
    })
  }
}

#[mut_set::derive::item(sort)]
#[derive(Debug, Clone)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Dummy: serde::Serialize + serde::de::DeserializeOwned")]
pub struct DummyGroup<C: Ctx> {
  #[size = 8]
  #[liberty(name)]
  #[id(borrow = "Option<&str>", check_fn = "mut_set::borrow_option!", with_ref = false)]
  name: Option<ArcStr>,
  /// group comments
  #[size = 32]
  #[liberty(comments)]
  comments: GroupComments,
  #[size = 0]
  #[liberty(extra_ctx)]
  extra_ctx: C::Dummy,
  /// group undefined attributes
  #[size = 40]
  #[liberty(attributes)]
  pub attributes: crate::ast::Attributes,
}
impl<C: Ctx> GroupFn for DummyGroup<C> {}

#[derive(Debug, Clone, Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Formula(pub ArcStr);
// type Aaa = mexprp::Expression<f64>;

// /// Recursive type for boolean expression tree.
// #[derive(serde::Serialize, serde::Deserialize)]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum _Formula {
//   Float(NotNan<f64>),
//   Variable(ArcStr),
//   Neg(Box<_Formula>),
//   Add(Box<_Formula>, Box<_Formula>),
//   Sub(Box<_Formula>, Box<_Formula>),
//   Mul(Box<_Formula>, Box<_Formula>),
//   Div(Box<_Formula>, Box<_Formula>),
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NameList {
  Name(ArcStr),
  List(WordSet),
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RefNameList<'a> {
  Name(&'a str),
  List(&'a WordSet),
}
impl<'a> From<&'a str> for RefNameList<'a> {
  #[inline]
  fn from(value: &'a str) -> Self {
    Self::Name(value)
  }
}
#[inline]
pub(crate) fn namelist_borrow(id: &NameList) -> RefNameList<'_> {
  match id {
    NameList::Name(s) => RefNameList::Name(s.as_str()),
    NameList::List(word_set) => RefNameList::List(word_set),
  }
}
impl Default for NameList {
  #[inline]
  fn default() -> Self {
    Self::Name(ArcStr::new())
  }
}
