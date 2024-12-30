use itertools::Itertools as _;

// use super::BooleanExpression;
// use super::Port;
use crate::types::*;
use crate::units;
// use crate::util;
// use std::collections::HashMap;
// use std::collections::HashSet;
use core::hash::Hash;
use std::ops::{Deref, DerefMut};

/// LogicLike
pub trait LogicLike: std::fmt::Display + std::fmt::Debug {
  /// inverse
  ///
  /// 0->1, 1->0
  fn inverse(&self) -> Self;
  /// Fall(_) == Fall(_)
  fn variant_eq(&self, other: &Self) -> bool;
}

/// ``` text
/// High:          _______
///               /│
///              / │
///             /  │
/// Low: ______/   │
///     │<-  ->│<->│
///      settle transition
/// ```
#[derive(Default)]
#[derive(Debug, Clone, Copy)]
#[derive(PartialOrd)]
// #[derive(PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChangePattern {
  /// settle down time
  pub settle_down_time: units::Time,
  /// transition time
  pub transition_time: units::Time,
}

impl Ord for ChangePattern {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match (
      self.settle_down_time.partial_cmp(&other.settle_down_time),
      self.transition_time.partial_cmp(&other.transition_time),
    ) {
      (Some(c), _) => c,
      (None, Some(c)) => c,
      (None, None) => std::cmp::Ordering::Equal,
    }
  }
}
impl Hash for ChangePattern {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    float_hash(state, self.settle_down_time.value);
    float_hash(state, self.transition_time.value);
  }
}

impl PartialEq for ChangePattern {
  fn eq(&self, other: &Self) -> bool {
    float_eq(self.settle_down_time.value, other.settle_down_time.value)
      && float_eq(self.transition_time.value, other.transition_time.value)
  }
}
impl Eq for ChangePattern {}
impl std::fmt::Display for ChangePattern {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut buffer1 = ryu::Buffer::new();
    let mut buffer2 = ryu::Buffer::new();
    write!(
      f,
      "({}|{})",
      buffer1.format(self.settle_down_time.value),
      buffer2.format(self.transition_time.value),
    )
  }
}

impl ChangePattern {
  #[inline]
  /// new ChangePattern
  pub fn new(settle_down_time: units::Time, transition_time: units::Time) -> Self {
    Self { settle_down_time, transition_time }
  }
  /// combine change pattern
  #[inline]
  pub fn combine(a: &Option<Self>, b: &Option<Self>) -> Option<Self> {
    match (a, b) {
      (None, None) => None,
      (None, Some(b)) => Some(*b),
      (Some(a), None) => Some(*a),
      // FIXME:
      (Some(a), Some(b)) => Some(*a),
    }
  }
}

/// Level
#[derive(Ord, PartialOrd)]
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Level {
  /// High
  #[strum(serialize = "h", serialize = "H", serialize = "1")]
  High,
  /// Low
  #[strum(serialize = "l", serialize = "L", serialize = "0")]
  Low,
}

impl Level {
  /// High
  pub const H: Self = Self::High;
  /// Low
  pub const L: Self = Self::Low;
}

impl LogicLike for Level {
  #[inline]
  fn inverse(&self) -> Self {
    match self {
      Self::Low => Self::High,
      Self::High => Self::Low,
    }
  }
  #[inline]
  fn variant_eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Level::High, Level::High) => true,
      (Level::Low, Level::Low) => true,
      _ => false,
    }
  }
}

/// Edge
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Edge {
  /// Fall
  #[strum(serialize = "f", serialize = "F")]
  Fall(Option<ChangePattern>),
  /// Rise
  #[strum(serialize = "r", serialize = "R")]
  Rise(Option<ChangePattern>),
}

impl Edge {
  /// Fall
  pub const F: Self = Self::Fall(None);
  /// Rise
  pub const R: Self = Self::Rise(None);
}

impl std::fmt::Display for Edge {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Edge::Fall(c) => match c {
        Some(c) => write!(f, "F{c}"),
        None => write!(f, "F"),
      },
      Edge::Rise(c) => match c {
        Some(c) => write!(f, "R{c}"),
        None => write!(f, "R"),
      },
    }
  }
}

impl LogicLike for Edge {
  #[inline]
  fn inverse(&self) -> Self {
    match self {
      Self::Fall(c) => Self::Rise(*c),
      Self::Rise(c) => Self::Fall(*c),
    }
  }
  #[inline]
  fn variant_eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Edge::Fall(_), Edge::Fall(_)) => true,
      (Edge::Rise(_), Edge::Rise(_)) => true,
      _ => false,
    }
  }
}

/// State
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum IllegalType {
  HighImpedanceInput,
  NoIdea,
  RiseFallAtLevel,
}
impl IllegalType {
  #[inline]
  pub fn combine(a: &Option<Self>, b: &Option<Self>) -> Option<Self> {
    match (a, b) {
      (None, None) => None,
      (None, Some(b_t)) => Some(*b_t),
      (Some(a_t), None) => Some(*a_t),
      (Some(a_t), Some(b_t)) => {
        match (a_t, b_t) {
          // FIXME:
          (a_vaild, b_vaild) => Some(*a_vaild),
        }
      }
    }
  }
}

/// UnInit
#[derive(Debug, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(derivative::Derivative)]
#[derivative(PartialEq, Hash, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UnInit {
  /// Unknown
  #[strum(serialize = "x", serialize = "X")]
  Unknown(
    #[derivative(Hash = "ignore")]
    #[derivative(PartialEq = "ignore")]
    Option<IllegalType>,
  ),
  /// HighImpedance
  #[strum(serialize = "z", serialize = "Z")]
  HighImpedance,
}

impl UnInit {
  /// Unknown
  pub const X: Self = Self::Unknown(None);
  /// Unknown
  pub const Z: Self = Self::HighImpedance;
}

impl PartialOrd for UnInit {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for UnInit {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    match (self, other) {
      (UnInit::Unknown(_), UnInit::Unknown(_)) => std::cmp::Ordering::Equal,
      (UnInit::Unknown(_), UnInit::HighImpedance) => std::cmp::Ordering::Greater,
      (UnInit::HighImpedance, UnInit::Unknown(_)) => std::cmp::Ordering::Less,
      (UnInit::HighImpedance, UnInit::HighImpedance) => std::cmp::Ordering::Equal,
    }
  }
}

impl std::fmt::Display for UnInit {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UnInit::Unknown(c) => match c {
        Some(c) => write!(f, "X({})", c),
        None => write!(f, "X"),
      },
      UnInit::HighImpedance => write!(f, "Z"),
    }
  }
}

impl Default for UnInit {
  fn default() -> Self {
    Self::X
  }
}
impl LogicLike for UnInit {
  #[inline]
  fn inverse(&self) -> Self {
    *self
  }
  #[inline]
  fn variant_eq(&self, other: &Self) -> bool {
    match (self, other) {
      (UnInit::Unknown(_), UnInit::Unknown(_)) => true,
      (UnInit::HighImpedance, UnInit::HighImpedance) => true,
      _ => false,
    }
  }
}
/// H L R F
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Normal {
  /// R F
  Edge(Edge),
  /// H L
  Level(Level),
}

impl Normal {
  /// Rise
  pub const R: Self = Self::Edge(Edge::R);
  /// Fall
  pub const F: Self = Self::Edge(Edge::F);
  /// High
  pub const H: Self = Self::Level(Level::H);
  /// Low
  pub const L: Self = Self::Level(Level::L);
}

/// H L R F
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Static {
  /// R F
  Edge(Edge),
  /// H L
  Level(Level),
}

impl Static {
  /// Rise
  pub const R: Self = Self::Edge(Edge::R);
  /// Fall
  pub const F: Self = Self::Edge(Edge::F);
  /// High
  pub const H: Self = Self::Level(Level::H);
  /// Low
  pub const L: Self = Self::Level(Level::L);
}

impl Into<State> for Normal {
  fn into(self) -> State {
    match self {
      Normal::Edge(s) => State::Edge(s),
      Normal::Level(s) => State::Level(s),
    }
  }
}

/// State
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum State {
  /// X Z
  UnInit(UnInit),
  /// R F
  Edge(Edge),
  /// H L
  Level(Level),
}
impl std::fmt::Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      State::UnInit(s) => s.fmt(f),
      State::Edge(s) => s.fmt(f),
      State::Level(s) => s.fmt(f),
    }
  }
}

impl Default for State {
  fn default() -> Self {
    return Self::X;
  }
}

impl LogicLike for State {
  #[inline]
  fn inverse(&self) -> Self {
    match self {
      Self::UnInit(s) => Self::UnInit(s.inverse()),
      Self::Edge(s) => Self::Edge(s.inverse()),
      Self::Level(s) => Self::Level(s.inverse()),
    }
  }
  #[inline]
  fn variant_eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::UnInit(a), Self::UnInit(b)) => a.variant_eq(b),
      (Self::Edge(a), Self::Edge(b)) => a.variant_eq(b),
      (Self::Level(a), Self::Level(b)) => a.variant_eq(b),
      _ => false,
    }
  }
}

impl State {
  /// Unknown
  pub const X: Self = Self::UnInit(UnInit::X);
  /// HighImpedance
  pub const Z: Self = Self::UnInit(UnInit::Z);
  /// Fall
  pub const F: Self = Self::Edge(Edge::F);
  /// Rise
  pub const R: Self = Self::Edge(Edge::R);
  /// High
  pub const H: Self = Self::Level(Level::H);
  /// Low
  pub const L: Self = Self::Level(Level::L);
  const LIST: [Self; 6] = [Self::X, Self::Z, Self::F, Self::R, Self::H, Self::L];
  /// iter
  #[inline]
  pub fn iter() -> impl Iterator<Item = Self> {
    Self::LIST.iter().copied()
  }
  /// get_change_pattern
  #[inline]
  pub fn get_change_pattern(&self) -> Option<ChangePattern> {
    match self {
      State::Edge(s) => match s {
        Edge::Fall(c) => *c,
        Edge::Rise(c) => *c,
      },
      _ => None,
    }
  }
  /// set_change_pattern
  #[inline]
  pub fn set_change_pattern(&self, c: &Option<ChangePattern>) -> Self {
    match self {
      State::Edge(s) => match s {
        Edge::Fall(_) => Self::Edge(Edge::Fall(*c)),
        Edge::Rise(_) => Self::Edge(Edge::Rise(*c)),
      },
      _ => *self,
    }
  }
  /// get_illegal_type
  #[inline]
  pub fn get_illegal_type(&self) -> Option<IllegalType> {
    match self {
      Self::UnInit(uninit) => match uninit {
        UnInit::Unknown(t) => *t,
        _ => None,
      },
      _ => None,
    }
  }
  /// set_illegal_type
  #[inline]
  pub fn set_illegal_type(&self, t: &Option<IllegalType>) -> Self {
    match (self, t) {
      (Self::UnInit(uninit), _) => match uninit {
        UnInit::Unknown(_) => Self::UnInit(UnInit::Unknown(*t)),
        _ => *self,
      },
      _ => *self,
    }
  }
  /// get_bgn state
  ///
  /// R -> 0, F -> 1, otherwise not change
  #[inline]
  pub fn get_bgn(&self) -> Self {
    match self {
      State::Edge(s) => match s {
        Edge::Fall(_) => Self::Level(Level::High),
        Edge::Rise(_) => Self::Level(Level::Low),
      },
      _ => *self,
    }
  }
  /// get_end state
  ///
  /// R -> 1, F -> 1, otherwise not change
  #[inline]
  pub fn get_end(&self) -> Self {
    match self {
      State::Edge(s) => match s {
        Edge::Fall(_) => Self::Level(Level::Low),
        Edge::Rise(_) => Self::Level(Level::High),
      },
      _ => *self,
    }
  }

  /// | BGN(self) | END  | Combined|
  /// | :-------: | :--: | :-----: |
  /// | 1         | 0    | F       |
  /// | 1         | 1    | 1       |
  /// | 1         | X    | X       |
  /// | X         | 1    | 1       |
  /// | 1         | Z    | Z       |
  /// | Z         | 1    | 1       |
  /// | Any       | F/R  | Illegal |
  /// | F/R       | Any  | Illegal |
  #[inline]
  pub fn combine_bgn_end(bgn: &Self, end: &Self) -> Self {
    match (bgn, end) {
      (_, Self::Edge(_)) => {
        Self::UnInit(UnInit::Unknown(Some(IllegalType::RiseFallAtLevel)))
      }
      (Self::Edge(_), _) => {
        Self::UnInit(UnInit::Unknown(Some(IllegalType::RiseFallAtLevel)))
      }
      (Self::UnInit(_), Self::UnInit(_)) => *end,
      (Self::UnInit(_), Self::Level(_)) => *end,
      (Self::Level(_), Self::UnInit(_)) => *end,
      (Self::Level(bgn), Self::Level(end)) => match (bgn, end) {
        (Level::High, Level::High) => Self::Level(Level::High),
        (Level::High, Level::Low) => Self::Edge(Edge::Fall(None)),
        (Level::Low, Level::High) => Self::Edge(Edge::Rise(None)),
        (Level::Low, Level::Low) => Self::Level(Level::Low),
      },
    }
  }
}

/// Vector
#[derive(Default)]
#[derive(Debug, Clone)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Vector {
  value: Vec<State>,
}

impl DerefMut for Vector {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.value
  }
}
impl Deref for Vector {
  type Target = Vec<State>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl From<Vec<State>> for Vector {
  fn from(value: Vec<State>) -> Self {
    Self { value }
  }
}

impl Into<Vec<State>> for Vector {
  fn into(self) -> Vec<State> {
    self.value
  }
}

// impl Vector {
//     #[inline]
//     pub fn new(value: Vec<State>) -> Self{
//         Self { value }
//     }
// }

impl std::fmt::Display for Vector {
  #[inline]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self
      .iter()
      .fold(Ok(()), |result, state| match state.get_change_pattern() {
        Some(c) => result.and_then(|_| write!(f, "{}{}", state, c)),
        None => result.and_then(|_| write!(f, "{}", state)),
      })
  }
}

impl LogicLike for Vector {
  #[inline]
  fn inverse(&self) -> Self {
    self
      .iter()
      .map(|v_state| v_state.inverse())
      .collect::<Vec<State>>()
      .into()
  }
  #[inline]
  fn variant_eq(&self, other: &Self) -> bool {
    if self.len() != other.len() {
      return false;
    }
    for (idx, a) in self.iter().enumerate() {
      if !a.variant_eq(&other[idx]) {
        return false;
      }
    }
    return true;
  }
}

// /// Operator1
// #[derive(Debug, Clone, Copy, PartialEq)]
// // #[derive(strum_macros::Display, strum_macros::EnumString)]
// pub enum Operator1 {
//   /// invert previous expression & invert following expression
//   Not,
//   /// signal tied to logic 1
//   Logic1,
//   /// signal tied to logic 0
//   Logic0,
// }

// impl std::fmt::Display for Operator1 {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self {
//       Operator1::Not => write!(f, "{}", Self::NOT_LIST[0]),
//       Operator1::Logic1 => write!(f, "{}", Self::LOGIC1_LIST[0]),
//       Operator1::Logic0 => write!(f, "{}", Self::LOGIC0_LIST[0]),
//     }
//   }
// }

// impl Operator1 {
//   const NOT_LIST: [char; 2] = ['!', '\''];
//   const LOGIC1_LIST: [char; 1] = ['1'];
//   const LOGIC0_LIST: [char; 1] = ['0'];
//   /// compute one logic state with logic operation, e.g.
//   ///
//   /// `Not` `High` = `Low`
//   ///
//   /// `Logic1` `Any` = `High`
//   #[inline]
//   pub fn compute(&self, a: &State) -> State {
//     match self {
//       Operator1::Not => a.inverse(),
//       Operator1::Logic1 => State::H,
//       Operator1::Logic0 => State::L,
//     }
//   }
//   /// compute_table
//   #[inline]
//   pub fn compute_table(&self, a: &Table) -> Table {
//     Table::new(
//       &a.self_node,
//       a.table
//         .iter()
//         .map(|(k_vec, v_state)| (k_vec.clone(), self.compute(v_state)))
//         .collect(),
//       a.port_idx.clone(),
//     )
//   }
// }

// /// Operator2
// /// <a name ="reference_link" href="
// /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
// /// ?field=test
// /// &bgn
// /// =132.42
// /// &end
// /// =133.11
// /// ">Reference</a>
// #[derive(Debug, Clone, Copy, PartialEq)]
// // #[derive(strum_macros::Display, strum_macros::EnumString)]
// pub enum Operator2 {
//   /// FIXME: only sapce `" "` between two expression means `AND`
//   // #[strum(serialize = "*",serialize = " ",serialize = "&")]
//   And,
//   /// Or
//   // #[strum(serialize = "+",serialize = "|")]
//   Or,
//   /// Xor
//   // #[strum(serialize = "^")]
//   Xor,
// }

// impl std::fmt::Display for Operator2 {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self {
//       Operator2::And => write!(f, "{}", Self::AND_LIST[0]),
//       Operator2::Or => write!(f, "{}", Self::OR_LIST[0]),
//       Operator2::Xor => write!(f, "{}", Self::XOR_LIST[0]),
//     }
//   }
// }
// impl Operator2 {
//   const AND_LIST: [char; 3] = ['*', ' ', '&'];
//   const OR_LIST: [char; 2] = ['+', '|'];
//   const XOR_LIST: [char; 1] = ['^'];
//   /// compute two logic state with logic operation
//   ///
//   /// e.g. `High` `or` `Low` = `High`
//   pub fn compute(&self, a: &State, b: &State) -> State {
//     let compute_edge_logic = || -> State {
//       let bgn_state = self.compute(&a.get_bgn(), &b.get_bgn());
//       let end_state = self.compute(&a.get_end(), &b.get_end());
//       let a_pattern = a.get_change_pattern();
//       let b_pattern = b.get_change_pattern();
//       State::combine_bgn_end(&bgn_state, &end_state)
//         .set_change_pattern(&ChangePattern::combine(&a_pattern, &b_pattern))
//     };
//     let combine_illegal = || -> State {
//       let a_illegal = a.get_illegal_type();
//       let b_illegal = b.get_illegal_type();
//       State::UnInit(UnInit::Unknown(IllegalType::combine(&a_illegal, &b_illegal)))
//     };
//     match (self, a, b) {
//       (_, _, State::Edge(_)) => compute_edge_logic(),
//       (_, State::Edge(_), _) => compute_edge_logic(),
//       (_, State::UnInit(_a), State::UnInit(_b)) => combine_illegal(),
//       (Operator2::And, State::UnInit(_a), State::Level(_b)) => match (_a, _b) {
//         (UnInit::Unknown(_), Level::High) => *a,
//         (UnInit::Unknown(_), Level::Low) => State::Level(Level::Low),
//         (UnInit::HighImpedance, Level::High) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//         (UnInit::HighImpedance, Level::Low) => State::Level(Level::Low),
//       },
//       (Operator2::And, State::Level(_a), State::UnInit(_b)) => match (_a, _b) {
//         (Level::High, UnInit::Unknown(_)) => *b,
//         (Level::High, UnInit::HighImpedance) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//         (Level::Low, UnInit::Unknown(_)) => State::Level(Level::Low),
//         (Level::Low, UnInit::HighImpedance) => State::Level(Level::Low),
//       },
//       (Operator2::And, State::Level(_a), State::Level(_b)) => match (_a, _b) {
//         (Level::High, Level::High) => State::Level(Level::High),
//         (Level::High, Level::Low) => State::Level(Level::Low),
//         (Level::Low, Level::High) => State::Level(Level::Low),
//         (Level::Low, Level::Low) => State::Level(Level::Low),
//       },
//       (Operator2::Or, State::UnInit(_a), State::Level(_b)) => match (_a, _b) {
//         (UnInit::Unknown(_), Level::High) => State::Level(Level::High),
//         (UnInit::Unknown(_), Level::Low) => *a,
//         (UnInit::HighImpedance, Level::High) => State::Level(Level::High),
//         (UnInit::HighImpedance, Level::Low) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//       },
//       (Operator2::Or, State::Level(_a), State::UnInit(_b)) => match (_a, _b) {
//         (Level::High, UnInit::Unknown(_)) => State::Level(Level::High),
//         (Level::High, UnInit::HighImpedance) => State::Level(Level::High),
//         (Level::Low, UnInit::Unknown(_)) => *b,
//         (Level::Low, UnInit::HighImpedance) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//       },
//       (Operator2::Or, State::Level(_a), State::Level(_b)) => match (_a, _b) {
//         (Level::High, Level::High) => State::Level(Level::High),
//         (Level::High, Level::Low) => State::Level(Level::High),
//         (Level::Low, Level::High) => State::Level(Level::High),
//         (Level::Low, Level::Low) => State::Level(Level::Low),
//       },
//       (Operator2::Xor, State::UnInit(_a), State::Level(_b)) => match (_a, _b) {
//         (UnInit::Unknown(_), Level::High) => *a,
//         (UnInit::Unknown(_), Level::Low) => *a,
//         (UnInit::HighImpedance, Level::High) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//         (UnInit::HighImpedance, Level::Low) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//       },
//       (Operator2::Xor, State::Level(_a), State::UnInit(_b)) => match (_a, _b) {
//         (Level::High, UnInit::Unknown(_)) => *b,
//         (Level::High, UnInit::HighImpedance) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//         (Level::Low, UnInit::Unknown(_)) => *b,
//         (Level::Low, UnInit::HighImpedance) => {
//           State::UnInit(UnInit::Unknown(Some(IllegalType::HighImpedanceInput)))
//         }
//       },
//       (Operator2::Xor, State::Level(_a), State::Level(_b)) => match (_a, _b) {
//         (Level::High, Level::High) => State::Level(Level::Low),
//         (Level::High, Level::Low) => State::Level(Level::High),
//         (Level::Low, Level::High) => State::Level(Level::High),
//         (Level::Low, Level::Low) => State::Level(Level::Low),
//       },
//     }
//   }
//   /// compute_table
//   pub fn compute_table(&self, a: &Table, b: &Table) -> Table {
//     use itertools::iproduct;
//     let mut combine = a.clone();
//     let vec_a_len = a.port_idx.len();
//     let vec_combine_to_a =
//       |vec_combine: &Vector| -> Vector { vec_combine[..vec_a_len].to_vec.into() };
//     let idx_vec_combine_to_b: Vec<usize> = b
//       .port_idx
//       .iter()
//       .map(|port_b| match combine.port_idx.iter().position(|v| v == port_b) {
//         Some(idx_combine) => {
//           // mapping
//           idx_combine
//         }
//         None => {
//           // change table
//           combine.port_idx.push(port_b.clone());
//           combine.table = iproduct!(State::iter(), combine.table.iter())
//             .map(|(state, (vec, _))| {
//               (
//                 {
//                   let mut new_key = vec.clone();
//                   new_key.push(state);
//                   new_key
//                 },
//                 State::default(),
//               )
//             })
//             .collect();
//           // mapping
//           combine.port_idx.len() - 1
//         }
//       })
//       .collect();
//     let vec_combine_to_b = |vec_combine: &Vector| -> Vector {
//       idx_vec_combine_to_b
//         .iter()
//         .map(|&idx_combine| vec_combine[idx_combine])
//         .collect::<Vec<State>>()
//         .into()
//     };
//     Table::new(
//       &format!("{}{}{}", a.self_node, self, b.self_node),
//       combine
//         .table
//         .iter()
//         .map(|(vec_in, _)| {
//           (
//             vec_in.clone(),
//             self.compute(
//               &a.table[&vec_combine_to_a(vec_in)],
//               &b.table[&vec_combine_to_b(vec_in)],
//             ),
//           )
//         })
//         .collect::<HashMap<Vector, State>>(),
//       combine.port_idx,
//     )
//   }
// }

// /// Table
// // #[derive(Clone, Debug)]
// // #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Table {
// //   /// self_node
// //   pub self_node: String,
// //   /// table
// //   pub table: HashMap<Vector, State>,
// //   /// port_idx
// //   pub port_idx: Vec<Port>,
// // }
// // #[derive(Clone, Debug)]
// // #[derive(serde::Serialize, serde::Deserialize)]
// pub struct NewTable {
// //   /// self_node
// //   pub self_node: String,
// //   /// port_idx
// //   pub port_idx: Vec<Port>,
// //   /// table
// //   pub table: HashMap<Vec<Normal>, HashMap<Vec<Static>, Vec<Static>>>,
// // }

// // impl PartialEq for Table {
// //   #[inline]
// //   fn eq(&self, other: &Self) -> bool {
// //     if self.port_idx.len() != other.port_idx.len() {
// //       return false;
// //     }
// //     let mut other_mapping_self = vec![];
// //     for port in other.port_idx.iter() {
// //       match self.port_idx.iter().position(|v| v == port) {
// //         Some(self_idx) => other_mapping_self.push(self_idx),
// //         None => return false,
// //       }
// //     }
// //     for (other_vec, other_state) in other.table.iter() {
// //       let self_vec: Vector = other_mapping_self
// //         .iter()
// //         .map(|self_idx| other_vec[*self_idx])
// //         .collect::<Vec<State>>()
// //         .into();
// //       match self.table.get(&self_vec) {
// //         Some(self_state) => {
// //           if !self_state.variant_eq(other_state) {
// //             return false;
// //           }
// //         }
// //         None => return false,
// //       }
// //     }
// //     return true;
// //   }
// // }

// // impl Hash for Table {
// //   fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
// //     let sorted_table = self.sort();
// //     _ = sorted_table.table.iter().map(|xy| xy.hash(state));
// //     sorted_table.port_idx.hash(state);
// //   }
// // }

// // impl Table {
// //   /// new `Table`
// //   #[inline]
// //   pub fn new(
// //     self_node: &str,
// //     table: HashMap<Vector, State>,
// //     port_idx: Vec<Port>,
// //   ) -> Self {
// //     Self { self_node: self_node.to_string(), table, port_idx }
// //   }
// //   /// sort
// //   pub fn sort(&self) -> Self {
// //     let idx_map = util::misc::argsort(&self.port_idx);
// //     Self::new(
// //       &self.self_node,
// //       self
// //         .table
// //         .iter()
// //         .map(|(vec,s)|
// //                 // self.port_idx[idx_map[idx]]
// //                 (vec.iter().enumerate().map(|(idx,_)|
// //                 vec[idx_map[idx]].clone()
// //             ).collect::<Vec<State>>().into(),
// //                     s.clone()))
// //         .collect(),
// //       self
// //         .port_idx
// //         .iter()
// //         .enumerate()
// //         .map(|(idx, _)| self.port_idx[idx_map[idx]].clone())
// //         .collect(),
// //     )
// //   }
// //   /// TODO: simplify
// //   pub fn simplify(&self) -> Self {
// //     todo!()
// //   }
// //   /// TODO: to_expression
// //   pub fn to_expression(&self) -> BooleanExpression {
// //     let table = self.simplify();
// //      _ = table;
// //     todo!()
// //   }
// // }

// // impl std::fmt::Display for Table {
// //   #[inline]
// //   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //     use prettytable::{Row, Table};
// //     let mut table = Table::new();
// //     table.set_format(*util::format::FORMAT_NO_BORDER_BOX_CHARS);
// //     table.set_titles(Row::from({
// //       let mut v = self.port_idx.clone();
// //       v.push(Port::new(&self.self_node));
// //       v
// //     }));
// //     for (vec_in, state_out) in self.table.iter().sorted() {
// //        _ = table.add_row(Row::from({
// //         let mut v: Vec<State> = vec_in.to_vec;
// //         v.push(state_out.clone());
// //         v
// //       }));
// //     }
// //     table.fmt(f)
// //   }
// // }

// // impl LogicLike for Table {
// //   #[inline]
// //   fn inverse(&self) -> Self {
// //     Self::new(
// //       &self.self_node,
// //       self
// //         .table
// //         .iter()
// //         .map(|(k_vec, v_state)| (k_vec.clone(), v_state.inverse()))
// //         .collect(),
// //       self.port_idx.clone(),
// //     )
// //   }
// //   #[inline]
// //   fn variant_eq(&self, other: &Self) -> bool {
// //     todo!()
// //   }
// // }

// // /// Logic Searcher
// // #[derive(Debug, Clone)]
// // #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Searcher {
// //   include_port_state: HashMap<Port, HashSet<State>>,
// //   include_out_state: Option<HashSet<State>>,
// //   exclude_port_state: HashMap<Port, HashSet<State>>,
// //   exclude_out_state: Option<HashSet<State>>,
// // }

// // impl std::fmt::Display for Searcher {
// //   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //     let print_hash_set = |name: &str, set: &HashSet<State>| -> String {
// //       let s = set
// //         .iter()
// //         .fold("".to_string(), |result, pair| format!("{result}/{pair}"));
// //       if s != "" {
// //         format!("{name}=({})", s.chars().skip(1).collect::<String>())
// //       } else {
// //         s
// //       }
// //     };
// //     write!(
// //       f,
// //       "searcher-[include:{}]-[exclude:{}]",
// //       self.include_port_state.iter().fold(
// //         {
// //           match &self.include_out_state {
// //             Some(s) => print_hash_set("Out", s),
// //             None => format!("Out=All"),
// //           }
// //         },
// //         |result, pair| {
// //           format!("{},{}", result, print_hash_set(&pair.0.to_string(), &pair.1))
// //         }
// //       ),
// //       self.exclude_port_state.iter().fold(
// //         {
// //           match &self.exclude_out_state {
// //             Some(s) => print_hash_set("Out", s),
// //             None => format!("Out=None"),
// //           }
// //         },
// //         |result, pair| {
// //           format!("{},{}", result, print_hash_set(&pair.0.to_string(), &pair.1))
// //         }
// //       )
// //     )
// //   }
// // }

// // impl Searcher {
// //   /// New Searcher
// //   pub fn new(
// //     include_port_state: Vec<(Port, Vec<State>)>,
// //     include_out_state: Option<Vec<State>>,
// //     exclude_port_state: Vec<(Port, Vec<State>)>,
// //     exclude_out_state: Option<Vec<State>>,
// //   ) -> Self {
// //     let port_state2map =
// //       |port_state: Vec<(Port, Vec<State>)>| -> HashMap<Port, HashSet<State>> {
// //         let mut map: HashMap<Port, HashSet<State>> = HashMap::new();
// //         for (p, v) in port_state.iter() {
// //           match map.get(p) {
// //             Some(set) => {
// //               let mut _set = set.clone();
// //               _set.extend(v.iter());
// //                _ = map.insert(p.clone(), _set);
// //             }
// //             None => {
// //                _ = map.insert(p.clone(), v.iter().copied().collect());
// //             }
// //           }
// //         }
// //         map
// //       };
// //     Self {
// //       include_port_state: port_state2map(include_port_state),
// //       include_out_state: match include_out_state {
// //         Some(v) => Some(v.iter().copied().collect()),
// //         None => None,
// //       },
// //       exclude_port_state: port_state2map(exclude_port_state),
// //       exclude_out_state: match exclude_out_state {
// //         Some(v) => Some(v.iter().copied().collect()),
// //         None => None,
// //       },
// //     }
// //   }
// //   /// search `Table` by port-state-pair
// //   pub fn search(&self, table: &Table) -> Table {
// //     let get_port_idx =
// //       |port: &Port| -> Option<usize> { table.port_idx.iter().position(|v| v == port) };
// //     let include_state_idx = self
// //       .include_port_state
// //       .iter()
// //       .filter_map(|(port, state_want)| match get_port_idx(port) {
// //         Some(u) => Some((u, state_want)),
// //         None => {
// //           error!("Can Not Find {}, auto skip it.", port);
// //           None
// //         }
// //       })
// //       .collect::<Vec<(usize, &HashSet<State>)>>();
// //     let exclude_state_idx = self
// //       .exclude_port_state
// //       .iter()
// //       .filter_map(|(port, state_want)| match get_port_idx(port) {
// //         Some(u) => Some((u, state_want)),
// //         None => {
// //           error!("Can Not Find {}, auto skip it.", port);
// //           None
// //         }
// //       })
// //       .collect::<Vec<(usize, &HashSet<State>)>>();
// //     Table::new(
// //       &format!("[{}]-[{self}]", table.self_node),
// //       table
// //         .table
// //         .iter()
// //         .filter_map(|(k_vec, v_state)| {
// //           if let Some(_include_out_state) = &self.include_out_state {
// //             if !_include_out_state.contains(v_state) {
// //               return None;
// //             }
// //           }

// //           if let Some(_exclude_out_state) = &self.exclude_out_state {
// //             if _exclude_out_state.contains(v_state) {
// //               return None;
// //             }
// //           }
// //           for (port_idx, state) in include_state_idx.iter() {
// //             if !state.contains(&k_vec[*port_idx]) {
// //               return None;
// //             }
// //           }
// //           for (port_idx, state) in exclude_state_idx.iter() {
// //             if state.contains(&k_vec[*port_idx]) {
// //               return None;
// //             }
// //           }
// //           return Some((k_vec.clone(), v_state.clone()));
// //         })
// //         .collect::<HashMap<Vector, State>>(),
// //       table.port_idx.clone(),
// //     )
// //   }
// // }
