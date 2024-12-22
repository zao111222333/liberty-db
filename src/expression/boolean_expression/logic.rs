use crate::ast::{ComplexAttri, ComplexParseError, ParseScope, SimpleAttri};
use core::{cmp::Ordering, hash::Hash};

/// Level
#[derive(Ord, PartialOrd)]
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Level {
  /// High
  #[strum(serialize = "h", serialize = "1", serialize = "H")]
  H,
  /// Low
  #[strum(serialize = "l", serialize = "0", serialize = "L")]
  L,
}

/// Edge
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter, strum_macros::Display)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Edge {
  /// Fall
  #[strum(serialize = "falling")]
  F,
  /// Rise
  #[strum(serialize = "rising")]
  R,
}
crate::ast::impl_self_builder!(Edge);
impl SimpleAttri for Edge {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}

/// Use the `output_switching_condition` attribute to specify the sense of the toggling
/// output. If there is more than one `switching_group` group specified within the
/// `dynamic_current` group, you can place the attribute in any order. The order in the list of
/// the `output_switching_condition` attribute is mapped to the same order of output pins in
/// the `related_outputs` attribute.
/// The valid values are rise and fall. rise represents a rising pin and fall represents a
/// falling pin.
/// Syntax
/// `output_switching_condition (enum(rise, fall));`
///
/// `enum(rise, fall)`
/// Enumerated type specifying the rise or fall condition.
///
/// Example
/// `output_switching_condition (rise, fall);`
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=151.17&end=151.29
/// ">Reference</a>
impl ComplexAttri for Edge {
  #[inline]
  fn parse<'a, I: Iterator<Item = &'a &'a str>>(
    mut iter: I,
    _scope: &mut ParseScope,
  ) -> Result<Self, ComplexParseError> {
    let res = match iter.next() {
      Some(&s) => match s {
        "rise" => Self::R,
        "fall" => Self::F,
        _ => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if iter.next().is_some() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(res)
  }
  #[inline]
  fn fmt_self<T: core::fmt::Write, I: crate::ast::Indentation>(
    &self,
    f: &mut crate::ast::CodeFormatter<'_, T, I>,
  ) -> core::fmt::Result {
    use core::fmt::Write;
    f.write_str(match self {
      Self::F => "rise",
      Self::R => "fall",
    })
  }
}

impl<'a> core::ops::Not for &'a Edge {
  type Output = Edge;
  #[inline]
  fn not(self) -> Self::Output {
    match self {
      Edge::F => Edge::R,
      Edge::R => Edge::F,
    }
  }
}

/// `UnInit`
#[derive(Debug, Clone, Copy)]
#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(PartialEq, Hash, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UnInit {
  /// `Unknown`
  #[strum(serialize = "x", serialize = "X")]
  X,
  /// `HighImpedance`
  #[strum(serialize = "z", serialize = "Z")]
  Z,
}

impl PartialOrd for UnInit {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl Ord for UnInit {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::X, Self::Z) => Ordering::Greater,
      (Self::Z, Self::X) => Ordering::Less,
      (Self::X, Self::X) | (Self::Z, Self::Z) => Ordering::Equal,
    }
  }
}
/// H L R F
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
pub enum Normal {
  /// R
  #[strum(serialize = "R")]
  R,
  /// F
  #[strum(serialize = "F")]
  F,
  /// H
  #[strum(serialize = "H")]
  H,
  /// L
  #[strum(serialize = "L")]
  L,
}
crate::ast::impl_self_builder!(Normal);
impl SimpleAttri for Normal {
  #[inline]
  fn nom_parse<'a>(
    i: &'a str,
    scope: &mut ParseScope,
  ) -> crate::ast::SimpleParseRes<'a, Self> {
    crate::ast::nom_parse_from_str(i, scope)
  }
}
/// H L R F
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
pub enum Static {
  /// X
  #[strum(serialize = "X")]
  X,
  /// Z
  #[strum(serialize = "Z")]
  Z,
  /// H
  #[strum(serialize = "H")]
  H,
  /// L
  #[strum(serialize = "L")]
  L,
}

/// State
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumIter)]
pub enum State {
  /// L
  #[strum(serialize = "L")]
  L,
  /// H
  #[strum(serialize = "H")]
  H,
  /// X
  #[strum(serialize = "X")]
  X,
  /// Z
  #[strum(serialize = "Z")]
  Z,
  /// LH
  #[strum(serialize = "LH")]
  LH,
  /// HL
  #[strum(serialize = "HL")]
  HL,
  /// LX
  #[strum(serialize = "LX")]
  LX,
  /// LZ
  #[strum(serialize = "LZ")]
  LZ,
  /// HX
  #[strum(serialize = "HX")]
  HX,
  /// HZ
  #[strum(serialize = "HZ")]
  HZ,
  /// XL
  #[strum(serialize = "XL")]
  XL,
  /// ZL
  #[strum(serialize = "ZL")]
  ZL,
  /// XH
  #[strum(serialize = "XH")]
  XH,
  /// ZH
  #[strum(serialize = "ZH")]
  ZH,
}

impl State {
  /// `bgn` state
  ///
  /// R -> 0, F -> 1, otherwise not change
  #[must_use]
  #[inline]
  pub const fn bgn(&self) -> Static {
    #[expect(clippy::match_same_arms)]
    match self {
      Self::L => Static::L,
      Self::H => Static::H,
      Self::X => Static::X,
      Self::Z => Static::Z,
      Self::LH => Static::L,
      Self::HL => Static::H,
      Self::LX => Static::L,
      Self::LZ => Static::L,
      Self::HX => Static::H,
      Self::HZ => Static::H,
      Self::XL => Static::X,
      Self::ZL => Static::Z,
      Self::XH => Static::X,
      Self::ZH => Static::Z,
    }
  }
  /// `end` state
  ///
  /// R -> 1, F -> 1, otherwise not change
  #[must_use]
  #[inline]
  pub const fn end(&self) -> Static {
    #[expect(clippy::match_same_arms)]
    match self {
      Self::L => Static::L,
      Self::H => Static::H,
      Self::X => Static::X,
      Self::Z => Static::Z,
      Self::LH => Static::H,
      Self::HL => Static::L,
      Self::LX => Static::X,
      Self::LZ => Static::Z,
      Self::HX => Static::X,
      Self::HZ => Static::Z,
      Self::XL => Static::L,
      Self::ZL => Static::L,
      Self::XH => Static::H,
      Self::ZH => Static::H,
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
  #[must_use]
  #[inline]
  pub const fn combine_bgn_end(bgn: Static, end: Static) -> Self {
    #[expect(clippy::match_same_arms)]
    match (bgn, end) {
      (Static::X, Static::X) => Self::X,
      (Static::X, Static::Z) => Self::X,
      (Static::X, Static::H) => Self::XH,
      (Static::X, Static::L) => Self::XL,
      (Static::Z, Static::X) => Self::X,
      (Static::Z, Static::Z) => Self::X,
      (Static::Z, Static::H) => Self::ZH,
      (Static::Z, Static::L) => Self::ZL,
      (Static::H, Static::X) => Self::HX,
      (Static::H, Static::Z) => Self::HZ,
      (Static::H, Static::H) => Self::H,
      (Static::H, Static::L) => Self::HL,
      (Static::L, Static::X) => Self::LX,
      (Static::L, Static::Z) => Self::LZ,
      (Static::L, Static::H) => Self::LH,
      (Static::L, Static::L) => Self::L,
    }
  }
}

// /// Vector
// #[derive(Default)]
// #[derive(Debug, Clone)]
// #[derive(Hash, PartialEq, Eq)]
// #[derive(Ord, PartialOrd)]
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Vector {
//   value: Vec<State>,
// }

// impl DerefMut for Vector {
//   #[inline]
//   fn deref_mut(&mut self) -> &mut Self::Target {
//     &mut self.value
//   }
// }
// impl Deref for Vector {
//   type Target = Vec<State>;
//   #[inline]
//   fn deref(&self) -> &Self::Target {
//     &self.value
//   }
// }

// impl From<Vec<State>> for Vector {
//   #[inline]
//   fn from(value: Vec<State>) -> Self {
//     Self { value }
//   }
// }

// impl From<Vector> for Vec<State> {
//   #[inline]
//   fn from(value: Vector) -> Self {
//     value.value
//   }
// }

// impl fmt::Display for Vector {
//   #[inline]
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     self.iter().fold(
//       Ok(()),
//       |result, state| result.and_then(|_| state.fmt(f)), // match state.get_change_pattern() {
//                                                          // Some(c) => result.and_then(|_| write!(f, "{}{}", state, c)),
//                                                          // None => result.and_then(|_| write!(f, "{}", state)),
//                                                          // }
//     )
//   }
// }

// impl LogicLike for Vector {
//   #[inline]
//   fn inverse(&self) -> Self {
//     self.iter().map(State::inverse).collect::<Vec<State>>().into()
//   }
//   #[inline]
//   #[expect(clippy::indexing_slicing)]
//   fn variant_eq(&self, other: &Self) -> bool {
//     if self.len() != other.len() {
//       return false;
//     }
//     for (idx, a) in self.iter().enumerate() {
//       if !a.variant_eq(&other[idx]) {
//         return false;
//       }
//     }
//     true
//   }
// }

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

// impl fmt::Display for Operator1 {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

// impl fmt::Display for Operator2 {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
//       let bgn_state = self.compute(&a.bgn(), &b.bgn());
//       let end_state = self.compute(&a.end(), &b.end());
//       let a_pattern = a.get_change_pattern();
//       let b_pattern = b.get_change_pattern();
//       State::combine_bgn_end(&bgn_state, &end_state)
//         .set_change_pattern(&ChangePattern::combine(&a_pattern, &b_pattern))
//     };
//     let combine_illegal = || -> State {
//       let a_illegal = a.get_illegal_type();
//       let b_illegal = b.get_illegal_type();
//       State::UnInit(UnInit::X(IllegalType::combine(&a_illegal, &b_illegal)))
//     };
//     match (self, a, b) {
//       (_, _, State::Edge(_)) => compute_edge_logic(),
//       (_, State::Edge(_), _) => compute_edge_logic(),
//       (_, State::UnInit(_a), State::UnInit(_b)) => combine_illegal(),
//       (Operator2::And, State::UnInit(_a), State::Level(_b)) => match (_a, _b) {
//         (UnInit::X, Level::High) => *a,
//         (UnInit::X, Level::Low) => State::Level(Level::Low),
//         (UnInit::Z, Level::High) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
//         }
//         (UnInit::Z, Level::Low) => State::Level(Level::Low),
//       },
//       (Operator2::And, State::Level(_a), State::UnInit(_b)) => match (_a, _b) {
//         (Level::High, UnInit::X) => *b,
//         (Level::High, UnInit::Z) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
//         }
//         (Level::Low, UnInit::X) => State::Level(Level::Low),
//         (Level::Low, UnInit::Z) => State::Level(Level::Low),
//       },
//       (Operator2::And, State::Level(_a), State::Level(_b)) => match (_a, _b) {
//         (Level::High, Level::High) => State::Level(Level::High),
//         (Level::High, Level::Low) => State::Level(Level::Low),
//         (Level::Low, Level::High) => State::Level(Level::Low),
//         (Level::Low, Level::Low) => State::Level(Level::Low),
//       },
//       (Operator2::Or, State::UnInit(_a), State::Level(_b)) => match (_a, _b) {
//         (UnInit::X, Level::High) => State::Level(Level::High),
//         (UnInit::X, Level::Low) => *a,
//         (UnInit::Z, Level::High) => State::Level(Level::High),
//         (UnInit::Z, Level::Low) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
//         }
//       },
//       (Operator2::Or, State::Level(_a), State::UnInit(_b)) => match (_a, _b) {
//         (Level::High, UnInit::X) => State::Level(Level::High),
//         (Level::High, UnInit::Z) => State::Level(Level::High),
//         (Level::Low, UnInit::X) => *b,
//         (Level::Low, UnInit::Z) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
//         }
//       },
//       (Operator2::Or, State::Level(_a), State::Level(_b)) => match (_a, _b) {
//         (Level::High, Level::High) => State::Level(Level::High),
//         (Level::High, Level::Low) => State::Level(Level::High),
//         (Level::Low, Level::High) => State::Level(Level::High),
//         (Level::Low, Level::Low) => State::Level(Level::Low),
//       },
//       (Operator2::Xor, State::UnInit(_a), State::Level(_b)) => match (_a, _b) {
//         (UnInit::X, Level::High) => *a,
//         (UnInit::X, Level::Low) => *a,
//         (UnInit::Z, Level::High) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
//         }
//         (UnInit::Z, Level::Low) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
//         }
//       },
//       (Operator2::Xor, State::Level(_a), State::UnInit(_b)) => match (_a, _b) {
//         (Level::High, UnInit::X) => *b,
//         (Level::High, UnInit::Z) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
//         }
//         (Level::Low, UnInit::X) => *b,
//         (Level::Low, UnInit::Z) => {
//           State::UnInit(UnInit::X(Some(IllegalType::HighImpedanceInput)))
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
// #[derive(Clone, Debug)]
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Table {
//   /// self_node
//   pub self_node: String,
//   /// table
//   pub table: HashMap<Vector, State>,
//   /// port_idx
//   pub port_idx: Vec<Port>,
// }
// #[derive(Clone, Debug)]
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct NewTable {
//   /// self_node
//   pub self_node: String,
//   /// port_idx
//   pub port_idx: Vec<Port>,
//   /// table
//   pub table: HashMap<Vec<Normal>, HashMap<Vec<Static>, Vec<Static>>>,
// }

// impl PartialEq for Table {
//   #[inline]
//   fn eq(&self, other: &Self) -> bool {
//     if self.port_idx.len() != other.port_idx.len() {
//       return false;
//     }
//     let mut other_mapping_self = vec![];
//     for port in other.port_idx.iter() {
//       match self.port_idx.iter().position(|v| v == port) {
//         Some(self_idx) => other_mapping_self.push(self_idx),
//         None => return false,
//       }
//     }
//     for (other_vec, other_state) in other.table.iter() {
//       let self_vec: Vector = other_mapping_self
//         .iter()
//         .map(|self_idx| other_vec[*self_idx])
//         .collect::<Vec<State>>()
//         .into();
//       match self.table.get(&self_vec) {
//         Some(self_state) => {
//           if !self_state.variant_eq(other_state) {
//             return false;
//           }
//         }
//         None => return false,
//       }
//     }
//     return true;
//   }
// }

// impl Hash for Table {
//   fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
//     let sorted_table = self.sort();
//     _ = sorted_table.table.iter().map(|xy| xy.hash(state));
//     sorted_table.port_idx.hash(state);
//   }
// }

// impl Table {
//   /// new `Table`
//   #[inline]
//   pub fn new(
//     self_node: &str,
//     table: HashMap<Vector, State>,
//     port_idx: Vec<Port>,
//   ) -> Self {
//     Self { self_node: self_node.to_string(), table, port_idx }
//   }
//   /// sort
//   pub fn sort(&self) -> Self {
//     let idx_map = util::misc::argsort(&self.port_idx);
//     Self::new(
//       &self.self_node,
//       self
//         .table
//         .iter()
//         .map(|(vec,s)|
//                 // self.port_idx[idx_map[idx]]
//                 (vec.iter().enumerate().map(|(idx,_)|
//                 vec[idx_map[idx]].clone()
//             ).collect::<Vec<State>>().into(),
//                     s.clone()))
//         .collect(),
//       self
//         .port_idx
//         .iter()
//         .enumerate()
//         .map(|(idx, _)| self.port_idx[idx_map[idx]].clone())
//         .collect(),
//     )
//   }
//   /// TODO: simplify
//   pub fn simplify(&self) -> Self {
//     todo!()
//   }
//   /// TODO: to_expression
//   pub fn to_expression(&self) -> BooleanExpression {
//     let table = self.simplify();
//      _ = table;
//     todo!()
//   }
// }

// impl fmt::Display for Table {
//   #[inline]
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     use prettytable::{Row, Table};
//     let mut table = Table::new();
//     table.set_format(*util::format::FORMAT_NO_BORDER_BOX_CHARS);
//     table.set_titles(Row::from({
//       let mut v = self.port_idx.clone();
//       v.push(Port::new(&self.self_node));
//       v
//     }));
//     for (vec_in, state_out) in self.table.iter().sorted() {
//        _ = table.add_row(Row::from({
//         let mut v: Vec<State> = vec_in.to_vec;
//         v.push(state_out.clone());
//         v
//       }));
//     }
//     table.fmt(f)
//   }
// }

// impl LogicLike for Table {
//   #[inline]
//   fn inverse(&self) -> Self {
//     Self::new(
//       &self.self_node,
//       self
//         .table
//         .iter()
//         .map(|(k_vec, v_state)| (k_vec.clone(), v_state.inverse()))
//         .collect(),
//       self.port_idx.clone(),
//     )
//   }
//   #[inline]
//   fn variant_eq(&self, other: &Self) -> bool {
//     todo!()
//   }
// }

// /// Logic Searcher
// #[derive(Debug, Clone)]
// #[derive(serde::Serialize, serde::Deserialize)]
// pub struct Searcher {
//   include_port_state: HashMap<Port, HashSet<State>>,
//   include_out_state: Option<HashSet<State>>,
//   exclude_port_state: HashMap<Port, HashSet<State>>,
//   exclude_out_state: Option<HashSet<State>>,
// }

// impl fmt::Display for Searcher {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     let print_hash_set = |name: &str, set: &HashSet<State>| -> String {
//       let s = set
//         .iter()
//         .fold("".to_string(), |result, pair| format!("{result}/{pair}"));
//       if s != "" {
//         format!("{name}=({})", s.chars().skip(1).collect::<String>())
//       } else {
//         s
//       }
//     };
//     write!(
//       f,
//       "searcher-[include:{}]-[exclude:{}]",
//       self.include_port_state.iter().fold(
//         {
//           match &self.include_out_state {
//             Some(s) => print_hash_set("Out", s),
//             None => format!("Out=All"),
//           }
//         },
//         |result, pair| {
//           format!("{},{}", result, print_hash_set(&pair.0.to_string(), &pair.1))
//         }
//       ),
//       self.exclude_port_state.iter().fold(
//         {
//           match &self.exclude_out_state {
//             Some(s) => print_hash_set("Out", s),
//             None => format!("Out=None"),
//           }
//         },
//         |result, pair| {
//           format!("{},{}", result, print_hash_set(&pair.0.to_string(), &pair.1))
//         }
//       )
//     )
//   }
// }

// impl Searcher {
//   /// New Searcher
//   pub fn new(
//     include_port_state: Vec<(Port, Vec<State>)>,
//     include_out_state: Option<Vec<State>>,
//     exclude_port_state: Vec<(Port, Vec<State>)>,
//     exclude_out_state: Option<Vec<State>>,
//   ) -> Self {
//     let port_state2map =
//       |port_state: Vec<(Port, Vec<State>)>| -> HashMap<Port, HashSet<State>> {
//         let mut map: HashMap<Port, HashSet<State>> = HashMap::new();
//         for (p, v) in port_state.iter() {
//           match map.get(p) {
//             Some(set) => {
//               let mut _set = set.clone();
//               _set.extend(v.iter());
//                _ = map.insert(p.clone(), _set);
//             }
//             None => {
//                _ = map.insert(p.clone(), v.iter().copied().collect());
//             }
//           }
//         }
//         map
//       };
//     Self {
//       include_port_state: port_state2map(include_port_state),
//       include_out_state: match include_out_state {
//         Some(v) => Some(v.iter().copied().collect()),
//         None => None,
//       },
//       exclude_port_state: port_state2map(exclude_port_state),
//       exclude_out_state: match exclude_out_state {
//         Some(v) => Some(v.iter().copied().collect()),
//         None => None,
//       },
//     }
//   }
//   /// search `Table` by port-state-pair
//   pub fn search(&self, table: &Table) -> Table {
//     let get_port_idx =
//       |port: &Port| -> Option<usize> { table.port_idx.iter().position(|v| v == port) };
//     let include_state_idx = self
//       .include_port_state
//       .iter()
//       .filter_map(|(port, state_want)| match get_port_idx(port) {
//         Some(u) => Some((u, state_want)),
//         None => {
//           error!("Can Not Find {}, auto skip it.", port);
//           None
//         }
//       })
//       .collect::<Vec<(usize, &HashSet<State>)>>();
//     let exclude_state_idx = self
//       .exclude_port_state
//       .iter()
//       .filter_map(|(port, state_want)| match get_port_idx(port) {
//         Some(u) => Some((u, state_want)),
//         None => {
//           error!("Can Not Find {}, auto skip it.", port);
//           None
//         }
//       })
//       .collect::<Vec<(usize, &HashSet<State>)>>();
//     Table::new(
//       &format!("[{}]-[{self}]", table.self_node),
//       table
//         .table
//         .iter()
//         .filter_map(|(k_vec, v_state)| {
//           if let Some(_include_out_state) = &self.include_out_state {
//             if !_include_out_state.contains(v_state) {
//               return None;
//             }
//           }

//           if let Some(_exclude_out_state) = &self.exclude_out_state {
//             if _exclude_out_state.contains(v_state) {
//               return None;
//             }
//           }
//           for (port_idx, state) in include_state_idx.iter() {
//             if !state.contains(&k_vec[*port_idx]) {
//               return None;
//             }
//           }
//           for (port_idx, state) in exclude_state_idx.iter() {
//             if state.contains(&k_vec[*port_idx]) {
//               return None;
//             }
//           }
//           return Some((k_vec.clone(), v_state.clone()));
//         })
//         .collect::<HashMap<Vector, State>>(),
//       table.port_idx.clone(),
//     )
//   }
// }
