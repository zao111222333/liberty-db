use itertools::Itertools;

use crate::types::*;
use crate::units;
use crate::util;
use super::BooleanExpression;
use super::Port;
use std::hash::Hash;
use std::ops::{Deref,DerefMut};

/// LogicLike
pub trait LogicLike: std::fmt::Display + std::fmt::Debug{
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
pub struct ChangePattern{
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
impl std::hash::Hash for ChangePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        float_hash(state, self.settle_down_time.value);
        float_hash(state, self.transition_time.value);
    }
}

impl PartialEq for ChangePattern {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.settle_down_time.value,other.settle_down_time.value) && 
        float_eq(self.transition_time.value,other.transition_time.value)
    }
}
impl Eq for ChangePattern {
}
impl std::fmt::Display for ChangePattern {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use units::Unit;
        write!(f, "({:.10E}{}|{:.10E}{})", 
                self.settle_down_time.get::<units::time::nanosecond>(),units::time::nanosecond::abbreviation(),
                self.transition_time.get::<units::time::nanosecond>(),units::time::nanosecond::abbreviation())
    }
}

impl ChangePattern {
    #[inline]
    /// new ChangePattern
    pub fn new(
        settle_down_time: units::Time,
        transition_time: units::Time,
    )->Self{
        Self{
            settle_down_time,
            transition_time,
        }
    }
    /// combine change pattern
    #[inline]
    pub fn combine(a: &Option<Self>, b: &Option<Self>) -> Option<Self>{
        match (a,b) {
            (None, None) => None,
            (None, Some(b)) => Some(*b),
            (Some(a), None) => Some(*a),
            // FIXME: 
            (Some(a), Some(b)) => Some(*a),
        }
    }
}


/// StaticState
#[derive(Ord,PartialOrd)]
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(
    strum_macros::Display, 
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
pub enum StaticState {
    /// High
    #[strum(serialize = "h", serialize = "H", serialize = "1")]
    High,
    /// Low
    #[strum(serialize = "l", serialize = "L", serialize = "0")]
    Low,
}

impl StaticState {
    /// High
    pub const H: Self = Self::High;
    /// Low
    pub const L: Self = Self::Low;
}

impl LogicLike for StaticState {
    #[inline]
    fn inverse(&self) -> Self{
        match self{
            Self::Low  => Self::High,
            Self::High => Self::Low,
        }
    }
    #[inline]
    fn variant_eq(&self, other: &Self) -> bool{
        match (self,other) {
            (StaticState::High, StaticState::High) => true,
            (StaticState::Low, StaticState::Low) => true,
            _ => false,
        }
    }
}

/// EdgeState
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord, PartialOrd)]
#[derive(
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
pub enum EdgeState {
    /// Fall
    #[strum(serialize = "f", serialize = "F")]
    Fall(Option<ChangePattern>),
    /// Rise
    #[strum(serialize = "r", serialize = "R")]
    Rise(Option<ChangePattern>),
}

impl EdgeState {
    /// Fall
    pub const F: Self = Self::Fall(None);
    /// Rise
    pub const R: Self = Self::Rise(None);
}

impl std::fmt::Display for EdgeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeState::Fall(c) => match c {
                Some(c) => write!(f,"F{c}"),
                None => write!(f,"F"),
            },
            EdgeState::Rise(c) => match c {
                Some(c) =>  write!(f,"R{c}"),
                None => write!(f,"R"),
            },
        }
    }
}

impl LogicLike for EdgeState {
    #[inline]
    fn inverse(&self) -> Self{
        match self{
            Self::Fall(c)  => Self::Rise(*c),
            Self::Rise(c)  => Self::Fall(*c),
        }
    }
    #[inline]
    fn variant_eq(&self, other: &Self) -> bool{
        match (self,other) {
            (EdgeState::Fall(_), EdgeState::Fall(_)) => true,
            (EdgeState::Rise(_), EdgeState::Rise(_)) => true,
            _ => false,
        }
    }
}

/// LogicState
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(strum_macros::Display)]
pub enum IllegalType {
    HighImpedanceInput,
    NoIdea,
    RiseFallAtStatic,
}
impl IllegalType {
    #[inline]
    pub fn combine(a: &Option<Self>, b: &Option<Self>)->Option<Self>{
        match (a,b) {
            (None, None) => None,
            (None, Some(b_t)) => Some(*b_t),
            (Some(a_t), None) => Some(*a_t),
            (Some(a_t), Some(b_t)) => {
                match (a_t,b_t) {
                    // FIXME:
                    (a_vaild, b_vaild) => Some(*a_vaild),
                }
            },
        }
    }
}

/// UninitState
#[derive(Debug, Clone, Copy)]
#[derive(
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
#[derive(derivative::Derivative)]
#[derivative(PartialEq, Hash, Eq)]
pub enum UninitState {
    /// Unknown
    #[strum(serialize = "x", serialize = "X")]
    Unknown(
        #[derivative(Hash="ignore")]
        #[derivative(PartialEq="ignore")]
        Option<IllegalType>,
    ),
    /// HighImpedance
    #[strum(serialize = "z", serialize = "Z")]
    HighImpedance,
}

impl UninitState {
    /// Unknown
    pub const X: Self = Self::Unknown(None);
    /// Unknown
    pub const Z: Self = Self::HighImpedance;
}

impl PartialOrd for UninitState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for UninitState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self,other) {
            (UninitState::Unknown(_), UninitState::Unknown(_)) => std::cmp::Ordering::Equal,
            (UninitState::Unknown(_), UninitState::HighImpedance) => std::cmp::Ordering::Greater,
            (UninitState::HighImpedance, UninitState::Unknown(_)) => std::cmp::Ordering::Less,
            (UninitState::HighImpedance, UninitState::HighImpedance) => std::cmp::Ordering::Equal,
        }
    }
}

impl std::fmt::Display for UninitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UninitState::Unknown(c) => match c {
                Some(c) =>  write!(f,"X({})",c),
                None => write!(f,"X"),
            },
            UninitState::HighImpedance => write!(f,"Z"),
        }
    }
}

impl Default for UninitState {
    fn default() -> Self {
        Self::X
    }
}
impl LogicLike for UninitState {
    #[inline]
    fn inverse(&self) -> Self{
        *self
    }
    #[inline]
    fn variant_eq(&self, other: &Self) -> bool{
        match (self,other) {
            (UninitState::Unknown(_), UninitState::Unknown(_)) => true,
            (UninitState::HighImpedance, UninitState::HighImpedance) => true,
            _ => false,
        }
    }
}
/// H L R F
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum CommonState {
    /// R F
    Edge(EdgeState),
    /// H L
    Static(StaticState),
}

impl CommonState {
    /// Rise
    pub const R: Self = Self::Edge(EdgeState::R);
    /// Fall
    pub const F: Self = Self::Edge(EdgeState::F);
    /// High
    pub const H: Self = Self::Static(StaticState::H);
    /// Low
    pub const L: Self = Self::Static(StaticState::L);
}

impl Into<LogicState> for CommonState {
    fn into(self) -> LogicState {
        match self {
            CommonState::Edge(s) => LogicState::Edge(s),
            CommonState::Static(s) => LogicState::Static(s),
        }
    }
}

/// LogicState
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord,PartialOrd)]
pub enum LogicState {
    /// X Z
    Uninit(UninitState),
    /// R F
    Edge(EdgeState),
    /// H L
    Static(StaticState),
}
impl std::fmt::Display for LogicState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicState::Uninit(s) => s.fmt(f),
            LogicState::Edge(s) => s.fmt(f),
            LogicState::Static(s) => s.fmt(f),
        }
    }
}

impl Default for LogicState {
    fn default() -> Self {
        return Self::X;
    }
}

impl LogicLike for LogicState {
    #[inline]
    fn inverse(&self) -> Self{
        match self{
            Self::Uninit(s) => Self::Uninit(s.inverse()),
            Self::Edge(s) => Self::Edge(s.inverse()),
            Self::Static(s) => Self::Static(s.inverse()),
        }
    }
    #[inline]
    fn variant_eq(&self, other: &Self) -> bool{
        match (self,other) {
            (Self::Uninit(a), Self::Uninit(b)) => a.variant_eq(b),
            (Self::Edge(a), Self::Edge(b)) => a.variant_eq(b),
            (Self::Static(a), Self::Static(b)) => a.variant_eq(b),
            _ => false,
        }
    }
}

impl LogicState {
    /// Unknown
    pub const X: Self = Self::Uninit(UninitState::X);
    /// HighImpedance
    pub const Z: Self = Self::Uninit(UninitState::Z);
    /// Fall
    pub const F: Self = Self::Edge(EdgeState::F);
    /// Rise
    pub const R: Self = Self::Edge(EdgeState::R);
    /// High
    pub const H: Self = Self::Static(StaticState::L);
    /// Low
    pub const L: Self = Self::Static(StaticState::H);
    const LIST: [Self;6] = [
        Self::X,
        Self::Z,
        Self::F,
        Self::R,
        Self::H,
        Self::L,
    ];
    /// iter
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::LIST.iter().copied()
    }
    /// get_change_pattern
    #[inline]
    pub fn get_change_pattern(&self) -> Option<ChangePattern>{
        match self {
            LogicState::Edge(s) => match s {
                EdgeState::Fall(c) => *c,
                EdgeState::Rise(c) => *c,
            },
            _ => None,
        }
    }
    /// set_change_pattern
    #[inline]
    pub fn set_change_pattern(&self,c: &Option<ChangePattern>) -> Self{
        match self {
            LogicState::Edge(s) => match s {
                EdgeState::Fall(_) => Self::Edge(EdgeState::Fall(*c)),
                EdgeState::Rise(_) => Self::Edge(EdgeState::Rise(*c)),
            },
            _ => *self,
        }
    }
    /// get_illegal_type
    #[inline]
    pub fn get_illegal_type(&self) -> Option<IllegalType>{
        match self {
            Self::Uninit(uninit) => match uninit {
                UninitState::Unknown(t) => *t,
                _ => None,
            },
            _ => None,
        }
    }
    /// set_illegal_type
    #[inline]
    pub fn set_illegal_type(&self,t: &Option<IllegalType>) -> Self{
        match (self,t) {
            (Self::Uninit(uninit), _) => match uninit {
                UninitState::Unknown(_) => Self::Uninit(UninitState::Unknown(*t)),
                _ => *self,
            },
            _ => *self,
        }
    }
    /// get_bgn state
    /// 
    /// R -> 0, F -> 1, otherwise not change
    #[inline]
    pub fn get_bgn(&self) -> Self{
        match self{
            LogicState::Edge(s) => match s {
                EdgeState::Fall(_) => Self::Static(StaticState::High),
                EdgeState::Rise(_) => Self::Static(StaticState::Low),
            },
            _ => *self,
        }
    }
    /// get_end state
    /// 
    /// R -> 1, F -> 1, otherwise not change
    #[inline]
    pub fn get_end(&self) -> Self{
        match self{
            LogicState::Edge(s) => match s {
                EdgeState::Fall(_) => Self::Static(StaticState::Low),
                EdgeState::Rise(_) => Self::Static(StaticState::High),
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
    pub fn combine_bgn_end(bgn: &Self, end: &Self) -> Self{
        match (bgn,end) {
            (_, Self::Edge(_)) => Self::Uninit(UninitState::Unknown(Some(IllegalType::RiseFallAtStatic))),
            (Self::Edge(_), _) => Self::Uninit(UninitState::Unknown(Some(IllegalType::RiseFallAtStatic))),
            (Self::Uninit(_), Self::Uninit(_)) => *end,
            (Self::Uninit(_), Self::Static(_)) => *end,
            (Self::Static(_), Self::Uninit(_)) => *end,
            (Self::Static(bgn), Self::Static(end)) => match (bgn,end) {
                (StaticState::High, StaticState::High) => Self::Static(StaticState::High),
                (StaticState::High, StaticState::Low) => Self::Edge(EdgeState::Fall(None)),
                (StaticState::Low, StaticState::High) => Self::Edge(EdgeState::Rise(None)),
                (StaticState::Low, StaticState::Low) => Self::Static(StaticState::Low),
            },
        }
    }
}

/// LogicVector
#[derive(Default)]
#[derive(Debug, Clone)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Ord,PartialOrd)]
pub struct LogicVector {
    value: Vec<LogicState>,
}

impl DerefMut for LogicVector {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
impl Deref for LogicVector {
    type Target = Vec<LogicState>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<Vec<LogicState>> for LogicVector {
    fn from(value: Vec<LogicState>) -> Self {
        Self { value }
    }
}

impl Into<Vec<LogicState>> for LogicVector {
    fn into(self) -> Vec<LogicState> {
        self.value
    }
}

// impl LogicVector {
//     #[inline]
//     pub fn new(value: Vec<LogicState>) -> Self{
//         Self { value }
//     }
// }

impl std::fmt::Display for LogicVector {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().fold(
            Ok(()),
            |result, state| {
                match state.get_change_pattern() {
                    Some(c) => result.and_then(|_| write!(f, "{}{}", state,c)),
                    None => result.and_then(|_| write!(f, "{}", state)),
                }
            }
        )
    }
}

impl LogicLike for LogicVector {
    #[inline]
    fn inverse(&self)->Self{
        self.iter().map(|v_state|
            v_state.inverse()
        ).collect::<Vec<LogicState>>()
        .into()
    }
    #[inline]
    fn variant_eq(&self, other: &Self) -> bool {
        if self.len()!=other.len(){
            return false;
        }
        for (idx, a) in self.iter().enumerate(){
            if !a.variant_eq(&other[idx]){
                return false;
            }
        }
        return true;
    }
}

/// LogicOperator1
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum LogicOperator1 {
    /// invert previous expression & invert following expression
    #[strum(serialize = "'", serialize = "!")]
    Not,
    // /// signal tied to logic 1
    // #[strum(serialize = "1")]
    // Logic1,
    // /// signal tied to logic 0
    // #[strum(serialize = "0")]
    // Logic0,
}

impl LogicOperator1 {
    /// compute one logic state with logic operation, e.g. 
    /// 
    /// `Not` `High` = `Low`
    ///
    /// `Logic1` `Any` = `High`
    #[inline]
    pub fn compute(&self, a: &LogicState)->LogicState{
        match self {
            LogicOperator1::Not => a.inverse(),
            // LogicOperator1::Logic1 => LogicState::Static(StaticState::High),
            // LogicOperator1::Logic0 => LogicState::Static(StaticState::Low),
        }
    }
    /// compute_table
    #[inline]
    pub fn compute_table(&self,  a: &LogicTable)->LogicTable{
        LogicTable::new(
            &a.self_node,
            a.table.iter()
                          .map(
                            |(k_vec,v_state)|
                                (k_vec.clone(),self.compute(v_state))
                            )
                          .collect(),
            a.port_idx.clone(),
                        )
    }
}

/// LogicOperator2
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =132.42
/// &end
/// =133.11
/// ">Reference</a>
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum LogicOperator2 {
    /// FIXME: only sapce `" "` between two expression means `AND`
    #[strum(serialize = "*",serialize = " ",serialize = "&")]
    And,
    /// Or
    #[strum(serialize = "+",serialize = "|")]
    Or,
    /// Xor
    #[strum(serialize = "^")]
    Xor,
}

impl LogicOperator2 {
    /// compute two logic state with logic operation
    /// 
    /// e.g. `High` `or` `Low` = `High`
    pub fn compute(&self,
        a: &LogicState,
        b: &LogicState,
    ) -> LogicState{
        let compute_edge_logic = || -> LogicState {
            let bgn_state = self.compute(&a.get_bgn(), &b.get_bgn());
            let end_state = self.compute(&a.get_end(), &b.get_end());
            let a_pattern = a.get_change_pattern();
            let b_pattern = b.get_change_pattern();
            LogicState::combine_bgn_end(&bgn_state, &end_state)
                        .set_change_pattern(&ChangePattern::combine(&a_pattern, &b_pattern))
        };
        let combine_illegal = || -> LogicState {
            let a_illegal = a.get_illegal_type();
            let b_illegal = b.get_illegal_type();
            LogicState::Uninit(UninitState::Unknown(IllegalType::combine(&a_illegal, &b_illegal)))
        };
        match (self,a,b) {
            (_, _, LogicState::Edge(_)) => compute_edge_logic(),
            (_, LogicState::Edge(_), _) => compute_edge_logic(),
            (_, LogicState::Uninit(_a), LogicState::Uninit(_b)) => combine_illegal(),
            (LogicOperator2::And, LogicState::Uninit(_a), LogicState::Static(_b)) => match (_a,_b) {
                (UninitState::Unknown(_), StaticState::High) => *a,
                (UninitState::Unknown(_), StaticState::Low) => LogicState::Static(StaticState::Low),
                (UninitState::HighImpedance, StaticState::High) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
                (UninitState::HighImpedance, StaticState::Low) => LogicState::Static(StaticState::Low),
            }
            (LogicOperator2::And, LogicState::Static(_a), LogicState::Uninit(_b)) => match (_a,_b) {
                (StaticState::High, UninitState::Unknown(_)) => *b,
                (StaticState::High, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
                (StaticState::Low, UninitState::Unknown(_)) => LogicState::Static(StaticState::Low),
                (StaticState::Low, UninitState::HighImpedance) => LogicState::Static(StaticState::Low),
            },
            (LogicOperator2::And, LogicState::Static(_a), LogicState::Static(_b)) => match (_a,_b) {
                (StaticState::High, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::High, StaticState::Low) => LogicState::Static(StaticState::Low),
                (StaticState::Low, StaticState::High) => LogicState::Static(StaticState::Low),
                (StaticState::Low, StaticState::Low) => LogicState::Static(StaticState::Low),
            },
            (LogicOperator2::Or, LogicState::Uninit(_a), LogicState::Static(_b)) => match (_a,_b) {
                (UninitState::Unknown(_), StaticState::High) => LogicState::Static(StaticState::High),
                (UninitState::Unknown(_), StaticState::Low) => *a,
                (UninitState::HighImpedance, StaticState::High) => LogicState::Static(StaticState::High),
                (UninitState::HighImpedance, StaticState::Low) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
            },
            (LogicOperator2::Or, LogicState::Static(_a), LogicState::Uninit(_b)) => match (_a,_b) {
                (StaticState::High, UninitState::Unknown(_)) => LogicState::Static(StaticState::High),
                (StaticState::High, UninitState::HighImpedance) => LogicState::Static(StaticState::High),
                (StaticState::Low, UninitState::Unknown(_)) => *b,
                (StaticState::Low, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
            },
            (LogicOperator2::Or, LogicState::Static(_a), LogicState::Static(_b)) => match (_a,_b) {
                (StaticState::High, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::High, StaticState::Low) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::Low) => LogicState::Static(StaticState::Low),
            },
            (LogicOperator2::Xor, LogicState::Uninit(_a), LogicState::Static(_b)) => match (_a,_b) {
                (UninitState::Unknown(_), StaticState::High) => *a,
                (UninitState::Unknown(_), StaticState::Low) => *a,
                (UninitState::HighImpedance, StaticState::High) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
                (UninitState::HighImpedance, StaticState::Low) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
            },
            (LogicOperator2::Xor, LogicState::Static(_a), LogicState::Uninit(_b)) => match (_a,_b) {
                (StaticState::High, UninitState::Unknown(_)) => *b,
                (StaticState::High, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
                (StaticState::Low, UninitState::Unknown(_)) => *b,
                (StaticState::Low, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(Some(IllegalType::HighImpedanceInput))),
            },
            (LogicOperator2::Xor, LogicState::Static(_a), LogicState::Static(_b)) => match (_a,_b) {
                (StaticState::High, StaticState::High) => LogicState::Static(StaticState::Low),
                (StaticState::High, StaticState::Low) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::Low) => LogicState::Static(StaticState::Low),
            },
        }
    }
    /// compute_table
    pub fn compute_table(&self,
        a: &LogicTable,
        b: &LogicTable,
    ) -> LogicTable {
        use itertools::iproduct;
        let mut combine = a.clone();
        let vec_a_len = a.port_idx.len();
        let vec_combine_to_a = |vec_combine: &LogicVector|->LogicVector{
            vec_combine[..vec_a_len].to_vec().into()
        };
        let idx_vec_combine_to_b: Vec<usize> = b.port_idx.iter().map(
            |port_b|
            match combine.port_idx.iter().position(|v| v==port_b) {
                Some(idx_combine) => { 
                    // mapping
                    idx_combine },
                None => {
                    // change table
                    combine.port_idx.push(port_b.clone());
                    combine.table = iproduct!(LogicState::iter(), combine.table.iter()).map(
                        |(state,(vec,_))|
                        ({
                            let mut new_key = vec.clone();
                            new_key.push(state);
                            new_key
                        }
                        ,LogicState::default())
                    ).collect();
                    // mapping
                    combine.port_idx.len()-1
                },
            }
        ).collect();
        let vec_combine_to_b = |vec_combine: &LogicVector|->LogicVector{
                idx_vec_combine_to_b.iter().map(|&idx_combine|
                    vec_combine[idx_combine]
                ).collect::<Vec<LogicState>>()
                .into()
        };
        LogicTable::new(
            &format!("{}{}{}",a.self_node,self,b.self_node),
            combine.table.iter().map(
                |(vec_in,_)|
                    (
                        vec_in.clone(), 
                        self.compute(
                            &a.table[&vec_combine_to_a(vec_in)], 
                            &b.table[&vec_combine_to_b(vec_in)],
                        )
                    )
                ).collect::<HashMap<LogicVector, LogicState>>(), 
            combine.port_idx,
        )
    }
}

/// LogicTable
#[derive(Clone,Debug)]
pub struct LogicTable{
    /// self_node
    pub self_node: String,
    /// table
    pub table: HashMap<LogicVector, LogicState>,
    /// port_idx
    pub port_idx: Vec<Port>,
}

impl PartialEq for LogicTable {
    #[inline]
    fn eq(&self, other: &Self) -> bool{
        if self.port_idx.len() != other.port_idx.len(){
            return false;
        }
        let mut other_mapping_self = vec![];
        for port in other.port_idx.iter(){
            match self.port_idx.iter().position(|v| v==port){
                Some(self_idx) => other_mapping_self.push(self_idx),
                None => return false,
            }
        }
        for (other_vec,other_state) in other.table.iter(){
            let self_vec:LogicVector = other_mapping_self.iter().map(
                |self_idx|
                other_vec[*self_idx]
            ).collect::<Vec<LogicState>>().into();
            match self.table.get(&self_vec){
                Some(self_state) => if !self_state.variant_eq(other_state){
                    return false;
                },
                None => return false,
            }
        }
        return true;
    }
}

impl Hash for LogicTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let sorted_table = self.sort();
        _ = sorted_table.table.iter().map(|xy|xy.hash(state));
        sorted_table.port_idx.hash(state);
    }
}

impl LogicTable {
    /// new `LogicTable`
    #[inline]
    pub fn new(
        self_node: &str,
        table: HashMap<LogicVector, LogicState>,
        port_idx: Vec<Port>,
    ) -> Self{
        Self {
            self_node: self_node.to_string(),
            table,
            port_idx,
        }
    }
    /// sort
    pub fn sort(&self)->Self{
        let idx_map = util::misc::argsort(&self.port_idx);
        Self::new(
            &self.self_node, 
            self.table.iter().map(
                |(vec,s)|
                // self.port_idx[idx_map[idx]]
                (vec.iter().enumerate().map(|(idx,_)|
                vec[idx_map[idx]].clone()
            ).collect::<Vec<LogicState>>().into(),
                    s.clone())
            ).collect(),
            self.port_idx.iter().enumerate().map(
                |(idx,_)|
                self.port_idx[idx_map[idx]].clone()
            ).collect(),
        )
    }
    /// TODO: simplify 
    pub fn simplify(&self)->Self{
        todo!()
    }
    /// TODO: to_expression
    pub fn to_expression(&self) -> BooleanExpression {
        let table = self.simplify();
        let _ = table;
        todo!()
    }
}

impl std::fmt::Display for LogicTable {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable::{Table, Row};
        let mut table = Table::new();
        table.set_format(*util::format::FORMAT_NO_BORDER_BOX_CHARS);
        table.set_titles(Row::from({
                            let mut v = self.port_idx.clone();
                            v.push(Port::new(&self.self_node));
                            v
                        }));
        for (vec_in,state_out) in self.table.iter().sorted(){
            let _ = table.add_row(Row::from({
                let mut v:Vec<LogicState> = vec_in.to_vec();
                v.push(state_out.clone());
                v
            }));
        }
        table.fmt(f)
    }
}

impl LogicLike for LogicTable {
    #[inline]
    fn inverse(&self)->Self{
        Self::new(
            &self.self_node,
            self.table.iter()
            .map(
              |(k_vec,v_state)|
                  (k_vec.clone(),v_state.inverse())
              )
            .collect(),
            self.port_idx.clone(),
        )
    }
    #[inline]
    fn variant_eq(&self, other: &Self) -> bool {
        todo!()
    }
}

/// Logic Searcher
#[derive(Debug,Clone)]
pub struct Searcher{
    include_port_state: HashMap<Port,HashSet<LogicState>>,
    include_out_state: Option<HashSet<LogicState>>,
    exclude_port_state: HashMap<Port,HashSet<LogicState>>,
    exclude_out_state: Option<HashSet<LogicState>>,
}

impl std::fmt::Display for Searcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print_hash_set = |name: &str,set: &HashSet<LogicState>| -> String{
            let s = set.iter().fold(
                "".to_string(),
                |result, pair| {
                    format!("{result}/{pair}")
                }
            );
            if s!=""{
                format!("{name}=({})",s.chars().skip(1).collect::<String>())
            }else{
                s
            }
        };
        write!(f,"searcher-[include:{}]-[exclude:{}]",
                        self.include_port_state.iter().fold(
                            {match &self.include_out_state {
                                Some(s) => print_hash_set("Out",s),
                                None => format!("Out=All"),
                            }} ,
                            |result, pair| {
                                format!("{},{}",result,print_hash_set(&pair.0.to_string(),&pair.1))
                            }
                        ),
                        self.exclude_port_state.iter().fold(
                            {match &self.exclude_out_state {
                                Some(s) => print_hash_set("Out",s),
                                None => format!("Out=None"),
                            }},
                            |result, pair| {
                                format!("{},{}",result,print_hash_set(&pair.0.to_string(),&pair.1))
                            }
                        )
                    )
    }
}

impl Searcher {
    /// New Searcher
    pub fn new(
        include_port_state: Vec<(Port,Vec<LogicState>)>,
        include_out_state: Option<Vec<LogicState>>,
        exclude_port_state: Vec<(Port,Vec<LogicState>)>,
        exclude_out_state: Option<Vec<LogicState>>,
    ) -> Self{
        let port_state2map = |port_state: Vec<(Port, Vec<LogicState>)>| -> HashMap<Port, HashSet<LogicState>>{
            let mut map: HashMap<Port, HashSet<LogicState>> = HashMap::new();
            for (p,v) in port_state.iter(){
                match map.get(p){
                    Some(set) => {
                        let mut _set = set.clone();
                        _set.extend(v.iter());
                        let _ = map.insert(
                            p.clone(),
                            _set,
                        );
                    },
                    None => {
                        let _ = map.insert(
                            p.clone(), 
                            v.iter().copied().collect(),
                        );
                    },
                }
            }
            map
        };
        Self { 
            include_port_state: port_state2map(include_port_state),
            include_out_state: match include_out_state{
                Some(v) => Some(v.iter().copied().collect()),
                None => None,
                },
            exclude_port_state: port_state2map(exclude_port_state),
            exclude_out_state: match exclude_out_state{
                    Some(v) => Some(v.iter().copied().collect()),
                    None => None,
                }, 
        }
    }
    /// search `LogicTable` by port-state-pair
    pub fn search(
        &self,
        table: &LogicTable, 
    ) -> LogicTable {
        
        let get_port_idx = |port: &Port|->Option<usize>{table.port_idx.iter().position(|v| v==port)};
        let include_state_idx = self.include_port_state
                                                        .iter()
                                                        .filter(
                                                            |(port,_)|
                                                            match get_port_idx(port){
                                                                Some(_) => true,
                                                                None => {error!("Can Not Find {}, auto skip it.",port);false},
                                                            }
                                                        )
                                                        .map(
                                                            |(port,state_want)|
                                                            (get_port_idx(port).unwrap(),state_want)
                                                        ).collect::<Vec<(usize, &HashSet<LogicState>)>>();
        let exclude_state_idx = self.exclude_port_state
                                                        .iter()
                                                        .filter(
                                                            |(port,_)|
                                                            match get_port_idx(port){
                                                                Some(_) => true,
                                                                None => {error!("Can Not Find {}, auto skip it.",port);false},
                                                            }
                                                        )
                                                        .map(
                                                            |(port,state_want)|
                                                            (get_port_idx(port).unwrap(),state_want)
                                                        ).collect::<Vec<(usize, &HashSet<LogicState>)>>();          
        LogicTable::new(
            &format!("[{}]-[{self}]",table.self_node),
            table.table.iter()
                    .filter(|(k_vec,v_state)|
                        {
                            match &self.include_out_state {
                                Some(_include_out_state) => 
                                if !_include_out_state.contains(v_state) {
                                    return false;
                                },
                                _ => (),
                            }
                            
                            match &self.exclude_out_state {
                                Some(_exclude_out_state) => 
                                if _exclude_out_state.contains(v_state) {
                                    return false;
                                },
                                _ => (),
                            }
                            for (port_idx,state) in include_state_idx.iter(){
                                if !state.contains(&k_vec[*port_idx]) {
                                    return false;
                                }
                            }
                            for (port_idx,state) in exclude_state_idx.iter(){
                                if state.contains(&k_vec[*port_idx]) {
                                    return false;
                                }
                            }
                            return true
                        }
                    )
                    .map(
                        |(k_vec,v_state)|
                        (k_vec.clone(), v_state.clone())
                    )
                    .collect::<HashMap<LogicVector, LogicState>>(),
            table.port_idx.clone(),
        )
    }
}