use crate::types::*;
use super::Port;
use strum_macros::Display;
use std::ops::{Deref,DerefMut};
pub trait LogicLike: std::fmt::Display + std::fmt::Debug{
    fn inverse(&self) -> Self;
    fn variant_eq(&self, other: &Self) -> bool;
}

/// ``` text
/// High:          _______
///               /|
///              / |
///             /  |
/// Low: ______/   |
///     |<-  ->|<->|
///      settle transition
/// ```
#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChangePattern{
    pub settle_down_time: f64,
    pub transition_time: f64,
}
impl std::fmt::Display for ChangePattern {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://doc.rust-lang.org/stable/std/fmt/trait.UpperExp.html
        write!(f, "({:.10E}|{:.10E})", 
                self.settle_down_time, 
                self.transition_time)
    }
}
impl ChangePattern {
    #[inline]
    /// new ChangePattern
    pub fn new(
        settle_down_time: f64,
        transition_time: f64,
    )->Option<Self>{
        Some(Self{
            settle_down_time,
            transition_time,
        })
    }
    #[inline]
    pub fn combine(a: &Option<Self>, b: &Option<Self>) -> Option<Self>{
        match (a,b) {
            (None, None) => None,
            (None, Some(b)) => Some(*b),
            (Some(a), None) => Some(*a),
            // TODO: 
            (Some(a), Some(b)) => Some(*a),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(
    strum_macros::Display, 
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
pub enum DynamicState {
    /// Fall
    #[strum(serialize = "f", serialize = "F")]
    Fall(Option<ChangePattern>),
    /// Rise
    #[strum(serialize = "r", serialize = "R")]
    Rise(Option<ChangePattern>),
}
impl LogicLike for DynamicState {
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
            (DynamicState::Fall(_), DynamicState::Fall(_)) => true,
            (DynamicState::Rise(_), DynamicState::Rise(_)) => true,
            _ => false,
        }
    }
}
/// LogicState
#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Display)]
pub enum IllegalType {
    #[default]
    None,
    HighImpedanceInput,
    NoIdea,
    RiseFallAtStatic,
}
impl IllegalType {
    #[inline]
    pub fn combine(a: &Option<Self>, b: &Option<Self>)->Self{
        match (a,b) {
            (None, None) => Self::None,
            (None, Some(b_t)) => *b_t,
            (Some(a_t), None) => *a_t,
            (Some(a_t), Some(b_t)) => {
                match (a_t,b_t) {
                    (Self::None, Self::None) => Self::None,
                    (Self::None, b_vaild) => *b_vaild,
                    (a_vaild, Self::None) => *a_vaild,
                    // TODO:
                    (a_vaild, b_vaild) => *a_vaild,
                }
            },
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(
    strum_macros::Display, 
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
pub enum UninitState {
    /// Unknown
    #[strum(serialize = "x", serialize = "X")]
    Unknown(IllegalType),
    /// HighImpedance
    #[strum(serialize = "z", serialize = "Z")]
    HighImpedance,
}
impl Default for UninitState {
    fn default() -> Self {
        Self::Unknown(IllegalType::default())
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommonState {
    Dynamic(DynamicState),
    Static(StaticState),
}
impl CommonState {
    pub fn to_logice_state(&self) -> LogicState{
        match self {
            CommonState::Dynamic(s) => LogicState::Dynamic(*s),
            CommonState::Static(s) => LogicState::Static(*s),
        }
    }
}
/// LogicState
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogicState {
    Uninit(UninitState),
    Dynamic(DynamicState),
    Static(StaticState),
}
impl std::fmt::Display for LogicState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicState::Uninit(s) => s.fmt(f),
            LogicState::Dynamic(s) => s.fmt(f),
            LogicState::Static(s) => s.fmt(f),
        }
    }
}

impl Default for LogicState {
    fn default() -> Self {
        return Self::Uninit(UninitState::default());
    }
}

impl LogicLike for LogicState {
    #[inline]
    fn inverse(&self) -> Self{
        match self{
            Self::Uninit(s) => Self::Uninit(s.inverse()),
            Self::Dynamic(s) => Self::Dynamic(s.inverse()),
            Self::Static(s) => Self::Static(s.inverse()),
        }
    }
    #[inline]
    fn variant_eq(&self, other: &Self) -> bool{
        match (self,other) {
            (Self::Uninit(a), Self::Uninit(b)) => a.variant_eq(b),
            (Self::Dynamic(a), Self::Dynamic(b)) => a.variant_eq(b),
            (Self::Static(a), Self::Static(b)) => a.variant_eq(b),
            _ => false,
        }
    }
}
impl LogicState {
    const LIST: [Self;6] = [
        Self::Uninit(UninitState::Unknown(IllegalType::None)),
        Self::Uninit(UninitState::HighImpedance),
        Self::Dynamic(DynamicState::Fall(None)),
        Self::Dynamic(DynamicState::Rise(None)),
        Self::Static(StaticState::Low),
        Self::Static(StaticState::High),
    ];
    // pub fn iter() -> std::slice::Iter<'_, LogicState>{
    //     Self::LIST.iter()
    // }
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::LIST.iter().copied()
    }
    #[inline]
    pub fn get_change_pattern(&self) -> Option<ChangePattern>{
        match self {
            LogicState::Dynamic(s) => match s {
                DynamicState::Fall(c) => *c,
                DynamicState::Rise(c) => *c,
            },
            _ => None,
        }
    }
    pub fn set_change_pattern(&self,c: &Option<ChangePattern>) -> Self{
        match self {
            LogicState::Dynamic(s) => match s {
                DynamicState::Fall(_) => Self::Dynamic(DynamicState::Fall(*c)),
                DynamicState::Rise(_) => Self::Dynamic(DynamicState::Rise(*c)),
            },
            _ => *self,
        }
    }
    pub fn get_illegal_type(&self) -> Option<IllegalType>{
        match self {
            Self::Uninit(uninit) => match uninit {
                UninitState::Unknown(t) => Some(*t),
                _ => None,
            },
            _ => None,
        }
    }
    pub fn set_illegal_type(&self,t: &Option<IllegalType>) -> Self{
        match (self,t) {
            (Self::Uninit(uninit), Some(t)) => match uninit {
                UninitState::Unknown(_) => Self::Uninit(UninitState::Unknown(*t)),
                _ => *self,
            },
            (Self::Uninit(uninit), None) => match uninit {
                UninitState::Unknown(_) => Self::Uninit(UninitState::Unknown(IllegalType::default())),
                _ => *self,
            },
            _ => *self,
        }
    }
    /// get_bgn state
    /// 
    /// R -> 0, F -> 1, otherwise not change
    pub fn get_bgn(&self) -> Self{
        match self{
            LogicState::Dynamic(s) => match s {
                DynamicState::Fall(_) => Self::Static(StaticState::High),
                DynamicState::Rise(_) => Self::Static(StaticState::Low),
            },
            _ => *self,
        }
    }
    /// get_end state
    /// 
    /// R -> 1, F -> 1, otherwise not change
    pub fn get_end(&self) -> Self{
        match self{
            LogicState::Dynamic(s) => match s {
                DynamicState::Fall(_) => Self::Static(StaticState::Low),
                DynamicState::Rise(_) => Self::Static(StaticState::High),
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
    pub fn combine_bgn_end(bgn: &Self, end: &Self) -> Self{
        match (bgn,end) {
            (_, Self::Dynamic(_)) => Self::Uninit(UninitState::Unknown(IllegalType::RiseFallAtStatic)),
            (Self::Dynamic(_), _) => Self::Uninit(UninitState::Unknown(IllegalType::RiseFallAtStatic)),
            (Self::Uninit(_), Self::Uninit(_)) => *end,
            (Self::Uninit(_), Self::Static(_)) => *end,
            (Self::Static(_), Self::Uninit(_)) => *end,
            (Self::Static(bgn), Self::Static(end)) => match (bgn,end) {
                (StaticState::High, StaticState::High) => Self::Static(StaticState::High),
                (StaticState::High, StaticState::Low) => Self::Dynamic(DynamicState::Fall(None)),
                (StaticState::Low, StaticState::High) => Self::Dynamic(DynamicState::Rise(None)),
                (StaticState::Low, StaticState::Low) => Self::Static(StaticState::Low),
            },
        }
    }
}

/// LogicVector
#[derive(Default)]
#[derive(Debug, Clone)]
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
    fn deref(&self) -> &Vec<LogicState> {
        &self.value
    }
}

impl LogicVector {
    #[inline]
    pub fn new(value: Vec<LogicState>) -> Self{
        Self { value }
    }
}
impl PartialEq for LogicVector {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl std::cmp::Eq for LogicVector {
}
impl std::hash::Hash for LogicVector {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.to_string().hash(hasher);
    }
}

impl std::fmt::Display for LogicVector {
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
        let mut inversed = Self::new(Vec::with_capacity(self.len()));
        for (idx,v_state) in self.iter().enumerate() {

            inversed[idx]=v_state.inverse();
        }
        inversed
    }

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

/// LogicOperation
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(strum_macros::Display, strum_macros::EnumString)]
pub enum LogicOperation {
    /// And
    #[strum(serialize = "*",serialize = "&")]
    And,
    /// Or
    #[strum(serialize = "+",serialize = "|")]
    Or,
    /// Xor
    #[strum(serialize = "^")]
    Xor,
}

impl LogicOperation {
    /// compute two logic state with logic operation
    /// 
    /// e.g. `High` `or` `Low` = `High`
    pub fn compute(&self,
        a: &LogicState,
        b: &LogicState,
    ) -> LogicState{
        let compute_dynamic_logic = || -> LogicState {
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
            (_, _, LogicState::Dynamic(_)) => compute_dynamic_logic(),
            (_, LogicState::Dynamic(_), _) => compute_dynamic_logic(),
            (_, LogicState::Uninit(_a), LogicState::Uninit(_b)) => combine_illegal(),
            (LogicOperation::And, LogicState::Uninit(_a), LogicState::Static(_b)) => match (_a,_b) {
                (UninitState::Unknown(_), StaticState::High) => *a,
                (UninitState::Unknown(_), StaticState::Low) => LogicState::Static(StaticState::Low),
                (UninitState::HighImpedance, StaticState::High) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
                (UninitState::HighImpedance, StaticState::Low) => LogicState::Static(StaticState::Low),
            }
            (LogicOperation::And, LogicState::Static(_a), LogicState::Uninit(_b)) => match (_a,_b) {
                (StaticState::High, UninitState::Unknown(_)) => *b,
                (StaticState::High, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
                (StaticState::Low, UninitState::Unknown(_)) => LogicState::Static(StaticState::Low),
                (StaticState::Low, UninitState::HighImpedance) => LogicState::Static(StaticState::Low),
            },
            (LogicOperation::And, LogicState::Static(_a), LogicState::Static(_b)) => match (_a,_b) {
                (StaticState::High, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::High, StaticState::Low) => LogicState::Static(StaticState::Low),
                (StaticState::Low, StaticState::High) => LogicState::Static(StaticState::Low),
                (StaticState::Low, StaticState::Low) => LogicState::Static(StaticState::Low),
            },
            (LogicOperation::Or, LogicState::Uninit(_a), LogicState::Static(_b)) => match (_a,_b) {
                (UninitState::Unknown(_), StaticState::High) => LogicState::Static(StaticState::High),
                (UninitState::Unknown(_), StaticState::Low) => *a,
                (UninitState::HighImpedance, StaticState::High) => LogicState::Static(StaticState::High),
                (UninitState::HighImpedance, StaticState::Low) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
            },
            (LogicOperation::Or, LogicState::Static(_a), LogicState::Uninit(_b)) => match (_a,_b) {
                (StaticState::High, UninitState::Unknown(_)) => LogicState::Static(StaticState::High),
                (StaticState::High, UninitState::HighImpedance) => LogicState::Static(StaticState::High),
                (StaticState::Low, UninitState::Unknown(_)) => *b,
                (StaticState::Low, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
            },
            (LogicOperation::Or, LogicState::Static(_a), LogicState::Static(_b)) => match (_a,_b) {
                (StaticState::High, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::High, StaticState::Low) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::Low) => LogicState::Static(StaticState::Low),
            },
            (LogicOperation::Xor, LogicState::Uninit(_a), LogicState::Static(_b)) => match (_a,_b) {
                (UninitState::Unknown(_), StaticState::High) => *a,
                (UninitState::Unknown(_), StaticState::Low) => *a,
                (UninitState::HighImpedance, StaticState::High) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
                (UninitState::HighImpedance, StaticState::Low) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
            },
            (LogicOperation::Xor, LogicState::Static(_a), LogicState::Uninit(_b)) => match (_a,_b) {
                (StaticState::High, UninitState::Unknown(_)) => *b,
                (StaticState::High, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
                (StaticState::Low, UninitState::Unknown(_)) => *b,
                (StaticState::Low, UninitState::HighImpedance) => LogicState::Uninit(UninitState::Unknown(IllegalType::HighImpedanceInput)),
            },
            (LogicOperation::Xor, LogicState::Static(_a), LogicState::Static(_b)) => match (_a,_b) {
                (StaticState::High, StaticState::High) => LogicState::Static(StaticState::Low),
                (StaticState::High, StaticState::Low) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::High) => LogicState::Static(StaticState::High),
                (StaticState::Low, StaticState::Low) => LogicState::Static(StaticState::Low),
            },
        }
    }

    pub fn compute_table(&self,
        a:  &LogicStateTable,
        b: &LogicStateTable,
    ) -> LogicStateTable {
        let mut combine = b.clone();
        let mut vec_b_len: usize = 0;
        for (vec_b,_) in b.table.iter(){
            vec_b_len=vec_b.len();
            break;
        }
        let mut vec_combine_len = vec_b_len;
        let vec_combine_to_b = |vec_combine: &LogicVector|->LogicVector{
            LogicVector::new(vec_combine[..vec_b_len].to_vec())
        };
        let mut idx_map_combine_to_a: HashMap<usize,usize> = HashMap::new();
        for (portid_a,idx_a) in a.portid_idx_map.iter(){
            match combine.portid_idx_map.get(portid_a) {
                Some(idx_combine) => {
                    let _=idx_map_combine_to_a.insert(*idx_combine, *idx_a);
                },
                None => {
                    let _ = combine.portid_idx_map.insert(portid_a.clone(), vec_combine_len);
                    vec_combine_len += 1;
                    let _=idx_map_combine_to_a.insert(vec_combine_len-1, *idx_a);
                    let mut new_table:HashMap<LogicVector,LogicState> = HashMap::default();
                    for state in LogicState::iter(){
                        for (vec,_) in combine.table.iter(){
                            let mut new_key = vec.clone();
                            new_key.push(state);
                            let _ = new_table.insert(new_key, LogicState::default());
                        }
                    }
                    combine.table=new_table;
                },
            }
        }
        let mut count_vec: Vec<_> = idx_map_combine_to_a
                                            .iter()
                                            .collect();
        count_vec.sort_by(|a, b| a.1.cmp(&b.1));
        let vec_combine_to_a = |vec_combine: &LogicVector|->LogicVector{
            let mut vec = LogicVector::new(
                vec![LogicState::default();count_vec.len()]);
            for (&idx_combine,&idx_a)  in count_vec.iter() {
                vec[idx_a] = vec_combine[idx_combine];
            }
            vec
        };
        let mut new_combine = LogicStateTable{ 
            table: HashMap::default(), 
            portid_idx_map: combine.portid_idx_map,
        };
        for (vec_in,_) in combine.table.iter() {
            let state_a  = a.table.get(&vec_combine_to_a(vec_in));
            let state_b = b.table.get(&vec_combine_to_b(vec_in));
            match (state_a,state_b) {
                (Some(a), Some(b)) => {
                    let _ = new_combine.table.insert(vec_in.clone(), self.compute(a, b));
                },
                _ => {
                    error!("Can Not Find Here");
                    panic!();
                    let _ = new_combine.table.insert(vec_in.clone(), LogicState::default());
                },
            }
        }
        new_combine
    }
}


#[derive(Clone,Debug)]
#[derive(PartialEq)]
pub struct LogicStateTable{
    pub table: HashMap<LogicVector, LogicState>,
    pub portid_idx_map: HashMap<Port, usize>,
}

impl LogicStateTable {
    #[inline]
    pub fn new(
        table: HashMap<LogicVector, LogicState>,
        portid_idx_map: HashMap<Port, usize>,
    ) -> Self{
        Self {
            table,
            portid_idx_map,
        }
    }
    pub fn search(
        &self, 
        want_port_state_pair: Vec<(Port,LogicState)>, 
        want_out_state_if_not_none: Option<LogicState>,
    ) -> Self{
        let mut sub = Self{
            table:HashMap::new(),
            portid_idx_map:self.portid_idx_map.clone(),
        };
        let mut idx_state_pair = Vec::new();
        for (port_idx,state_want) in want_port_state_pair.iter(){
            match self.portid_idx_map.get(port_idx) {
                Some(idx) => idx_state_pair.push((*idx, state_want)),
                None => {
                    error!("Can Not Find {}, auto skip it.",port_idx);
                },
            }
        }
        'outer: for (k_vec,v_state) in self.table.iter() {
            match want_out_state_if_not_none {
                Some(want_out_state) => if !want_out_state.variant_eq(v_state){
                    continue 'outer;
                },
                _ => (),
            }
            for (port_idx,state_want) in idx_state_pair.iter(){
                let state_got = k_vec[*port_idx];
                if !state_want.variant_eq(&state_got) {
                    continue 'outer;
                }
            }
            let _=sub.table.insert(k_vec.clone(), *v_state);
        }
        sub
    }
}
impl std::fmt::Display for LogicStateTable {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{self:?}")
    }
}
impl LogicLike for LogicStateTable {
    #[inline]
    fn inverse(&self)->Self{
        let mut inversed = Self{
            table:HashMap::new(),
            portid_idx_map:self.portid_idx_map.clone(),
        };
        for (k_vec,v_state) in self.table.iter() {
            let _=inversed.table.insert(k_vec.clone(), v_state.inverse());
        }
        inversed
    }

    fn variant_eq(&self, other: &Self) -> bool {
        todo!()
    }
}