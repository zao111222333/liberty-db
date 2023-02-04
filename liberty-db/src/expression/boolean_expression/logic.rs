use strum::IntoEnumIterator;

use super::Port;
use crate::HashMap;
use std::ops::{Deref,DerefMut};
pub trait LogicLike: std::fmt::Display + std::fmt::Debug{
    fn inverse(&self) -> Self;
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
/// LogicState
#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateIllegalType {
    #[default]
    NoIdea,
    RiseFallAtStatic,
}
impl StateIllegalType {
    #[inline]
    pub fn combine(a: &Option<Self>, b: &Option<Self>)->Self{
        match (a,b) {
            (None, None) => Self::NoIdea,
            (None, Some(b_t)) => *b_t,
            (Some(a_t), None) => *a_t,
            (Some(a_t), Some(b_t)) => {
                match (a_t,b_t) {
                    (Self::NoIdea, Self::NoIdea) => Self::NoIdea,
                    (Self::NoIdea, b_vaild) => *b_vaild,
                    (a_vaild, Self::NoIdea) => *a_vaild,
                    // TODO:
                    (a_vaild, b_vaild) => *a_vaild,
                }
            },
        }
    }
}
/// LogicState
#[derive(Default)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(
    strum_macros::Display, 
    strum_macros::EnumString,
    strum_macros::EnumIter,
)]
pub enum LogicState {
    /// Unknown
    #[default]
    #[strum(serialize = "x", serialize = "X")]
    Unknown,
    /// HighImpedance
    #[strum(serialize = "z", serialize = "Z")]
    HighImpedance,
    /// Illegal
    #[strum(serialize = "?")]
    Illegal(StateIllegalType),
    /// High
    #[strum(serialize = "1")]
    High,
    /// Low
    #[strum(serialize = "0")]
    Low,
    /// Fall
    #[strum(serialize = "f", serialize = "F")]
    Fall(Option<ChangePattern>),
    /// Rise
    #[strum(serialize = "r", serialize = "R")]
    Rise(Option<ChangePattern>),
}

impl LogicLike for LogicState {
    #[inline]
    fn inverse(&self) -> Self{
        match self{
            LogicState::Low      => LogicState::High,
            LogicState::High     => LogicState::Low,
            LogicState::Rise(c)  => LogicState::Fall(*c),
            LogicState::Fall(c)  => LogicState::Rise(*c),
            _ => *self,
        }
    }
}
impl LogicState {
    #[inline]
    pub fn variant_eq(&self, other: &Self) -> bool{
        match (self,other) {
            (LogicState::Unknown, LogicState::Unknown) => true,
            (LogicState::HighImpedance, LogicState::HighImpedance) => true,
            (LogicState::High, LogicState::High) => true,
            (LogicState::Low, LogicState::Low) => true,
            (LogicState::Fall(_), LogicState::Fall(_)) => true,
            (LogicState::Rise(_), LogicState::Rise(_)) => true,
            _ => false,
        }
    }
    pub fn get_change_pattern(&self) -> Option<ChangePattern>{
        match self {
            LogicState::Rise(c) => *c,
            LogicState::Fall(c) => *c,
            _ => None,
        }
    }
    pub fn set_change_pattern(&self,c: &Option<ChangePattern>) -> Self{
        match self {
            LogicState::Fall(_) => LogicState::Fall(*c),
            LogicState::Rise(_) => LogicState::Rise(*c),
            _ => *self,
        }
    }
    pub fn get_illegal_type(&self) -> Option<StateIllegalType>{
        match self {
            LogicState::Illegal(c) => Some(*c),
            _ => None,
        }
    }
    pub fn set_illegal_type(&self,t: &Option<StateIllegalType>) -> Self{
        match (self,t) {
            (LogicState::Illegal(_), Some(t)) => LogicState::Illegal(*t),
            (LogicState::Illegal(_), None) => LogicState::Illegal(StateIllegalType::default()),
            _ => *self,
        }
    }
    /// get_bgn state
    /// 
    /// R -> 0, F -> 1, otherwise not change
    pub fn get_bgn(&self) -> &Self{
        match self{
            LogicState::Fall(_) => &LogicState::High,
            LogicState::Rise(_) => &LogicState::Low,
            _ => self,
        }
    }
    /// get_end state
    /// 
    /// R -> 1, F -> 1, otherwise not change
    pub fn get_end(&self) -> &Self{
        match self{
            LogicState::Fall(_) => &LogicState::Low,
            LogicState::Rise(_) => &LogicState::High,
            _ => self,
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
            (LogicState::Unknown,       LogicState::Unknown      ) => LogicState::Unknown,
            (LogicState::Unknown,       LogicState::HighImpedance) => LogicState::HighImpedance,
            (LogicState::Unknown,       LogicState::High         ) => LogicState::High,
            (LogicState::Unknown,       LogicState::Low          ) => LogicState::Low,
            (LogicState::HighImpedance, LogicState::Unknown      ) => LogicState::Unknown,
            (LogicState::HighImpedance, LogicState::HighImpedance) => LogicState::HighImpedance,
            (LogicState::HighImpedance, LogicState::High         ) => LogicState::High,
            (LogicState::HighImpedance, LogicState::Low          ) => LogicState::Low,
            (LogicState::High,          LogicState::Unknown      ) => LogicState::Unknown,
            (LogicState::High,          LogicState::HighImpedance) => LogicState::HighImpedance,
            (LogicState::High,          LogicState::High         ) => LogicState::High,
            (LogicState::High,          LogicState::Low          ) => LogicState::Fall(None),
            (LogicState::Low,           LogicState::Unknown      ) => LogicState::Unknown,
            (LogicState::Low,           LogicState::HighImpedance) => LogicState::HighImpedance,
            (LogicState::Low,           LogicState::High         ) => LogicState::Rise(None),
            (LogicState::Low,           LogicState::Low          ) => LogicState::Low,
            (LogicState::Rise(_)|LogicState::Fall(_),_) => LogicState::Illegal(StateIllegalType::RiseFallAtStatic),
            (_,LogicState::Rise(_)|LogicState::Fall(_)) => LogicState::Illegal(StateIllegalType::RiseFallAtStatic),
            (LogicState::Illegal(info),_) => LogicState::Illegal(*info),
            (_,LogicState::Illegal(info)) => LogicState::Illegal(*info),
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
        left:  &LogicState,
        right: &LogicState,
    ) -> LogicState{
        let compute_dynamic_logic = || -> LogicState {
            let bgn_state = self.compute(left.get_bgn(), right.get_bgn());
            let end_state = self.compute(left.get_end(), right.get_end());
            let left_pattern = left.get_change_pattern();
            let right_pattern = right.get_change_pattern();
            LogicState::combine_bgn_end(&bgn_state, &end_state)
                        .set_change_pattern(&ChangePattern::combine(&left_pattern, &right_pattern))
        };
        let combine_illegal = || -> LogicState {
            let left_illegal = left.get_illegal_type();
            let right_illegal = right.get_illegal_type();
            LogicState::Illegal(StateIllegalType::combine(&left_illegal, &right_illegal))
        };
        match (self,left,right) {
            (_, _, LogicState::Illegal(_)) => combine_illegal(),
            (_, LogicState::Illegal(_), _) => combine_illegal(),
            (_, _, LogicState::Rise(_)|LogicState::Fall(_)) => compute_dynamic_logic(),
            (_, LogicState::Rise(_)|LogicState::Fall(_), _) => compute_dynamic_logic(),
            (LogicOperation::And, LogicState::Unknown,       LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::And, LogicState::Unknown,       LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::And, LogicState::Unknown,       LogicState::High         ) => LogicState::Unknown,
            (LogicOperation::And, LogicState::Unknown,       LogicState::Low          ) => LogicState::Low,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::High         ) => LogicState::Unknown,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::Low          ) => LogicState::Low,
            (LogicOperation::And, LogicState::High,          LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::And, LogicState::High,          LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::And, LogicState::High,          LogicState::High         ) => LogicState::High,
            (LogicOperation::And, LogicState::High,          LogicState::Low          ) => LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::Unknown      ) => LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::HighImpedance) => LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::High         ) => LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::Low          ) => LogicState::Low,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::High         ) => LogicState::High,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::Low          ) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::High         ) => LogicState::High,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::Low          ) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::High,          LogicState::Unknown      ) => LogicState::High,
            (LogicOperation::Or,  LogicState::High,          LogicState::HighImpedance) => LogicState::High,
            (LogicOperation::Or,  LogicState::High,          LogicState::High         ) => LogicState::High,
            (LogicOperation::Or,  LogicState::High,          LogicState::Low          ) => LogicState::High,
            (LogicOperation::Or,  LogicState::Low,           LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Low,           LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Low,           LogicState::High         ) => LogicState::High,
            (LogicOperation::Or,  LogicState::Low,           LogicState::Low          ) => LogicState::Low,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::High         ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::Low          ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::High         ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::Low          ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::High,          LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::High,          LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::High,          LogicState::High         ) => LogicState::Low,
            (LogicOperation::Xor, LogicState::High,          LogicState::Low          ) => LogicState::High,
            (LogicOperation::Xor, LogicState::Low,           LogicState::Unknown      ) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Low,           LogicState::HighImpedance) => LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Low,           LogicState::High         ) => LogicState::High,
            (LogicOperation::Xor, LogicState::Low,           LogicState::Low          ) => LogicState::Low,
        }
    }

    pub fn compute_table(&self,
        left:  &LogicStateTable,
        right: &LogicStateTable,
    ) -> LogicStateTable {
        let mut combine = right.clone();
        let mut vec_right_len: usize = 0;
        for (vec_right,_) in right.table.iter(){
            vec_right_len=vec_right.len();
            break;
        }
        let mut vec_combine_len = vec_right_len;
        let vec_combine_to_right = |vec_combine: &LogicVector|->LogicVector{
            LogicVector::new(vec_combine[..vec_right_len].to_vec())
        };
        let mut idx_map_combine_to_left: HashMap<usize,usize> = HashMap::new();
        for (portid_left,idx_left) in left.portid_idx_map.iter(){
            match combine.portid_idx_map.get(portid_left) {
                Some(idx_combine) => {
                    let _=idx_map_combine_to_left.insert(*idx_combine, *idx_left);
                },
                None => {
                    let _ = combine.portid_idx_map.insert(portid_left.clone(), vec_combine_len);
                    vec_combine_len += 1;
                    let _=idx_map_combine_to_left.insert(vec_combine_len-1, *idx_left);
                    let mut new_table:HashMap<LogicVector,LogicState> = HashMap::default();
                    for state in LogicState::iter(){
                        for (vec,_) in combine.table.iter(){
                            let mut new_key = vec.clone();
                            new_key.push(state);
                            let _ = new_table.insert(new_key, LogicState::Unknown);
                        }
                    }
                    combine.table=new_table;
                },
            }
        }
        let mut count_vec: Vec<_> = idx_map_combine_to_left
                                            .iter()
                                            .collect();
        count_vec.sort_by(|a, b| a.1.cmp(&b.1));
        let vec_combine_to_left = |vec_combine: &LogicVector|->LogicVector{
            let mut vec = LogicVector::new(
                vec![LogicState::Unknown;count_vec.len()]);
            for (&idx_combine,&idx_left)  in count_vec.iter() {
                vec[idx_left] = vec_combine[idx_combine];
            }
            vec
        };
        let mut new_combine = LogicStateTable{ 
            table: HashMap::default(), 
            portid_idx_map: combine.portid_idx_map,
        };
        for (vec_in,_) in combine.table.iter() {
            let state_left  = left.table.get(&vec_combine_to_left(vec_in));
            let state_right = right.table.get(&vec_combine_to_right(vec_in));
            match (state_left,state_right) {
                (Some(left), Some(right)) => {
                    let _ = new_combine.table.insert(vec_in.clone(), self.compute(left, right));
                },
                _ => {
                    error!("Can Not Find Here");
                    panic!();
                    let _ = new_combine.table.insert(vec_in.clone(), LogicState::Unknown);
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
}