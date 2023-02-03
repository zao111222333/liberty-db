use std::process::id;

use log::{warn, error};
use strum::IntoEnumIterator;

use super::{HashMap,PortId};

pub trait LogicLike: std::fmt::Display + std::fmt::Debug{
    fn inverse(&self) -> Self;
    fn inverse_if_need(&self, need: bool) -> Self;
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // https://doc.rust-lang.org/stable/std/fmt/trait.UpperExp.html
        write!(f, "({:.10E}|{:.10E})", 
                self.settle_down_time, 
                self.transition_time)
    }
}
impl ChangePattern {
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
            LogicState::Rise(_)  => LogicState::Fall(None),
            LogicState::Fall(_)  => LogicState::Rise(None),
            _ => *self,
        }
    }
    #[inline]
    fn inverse_if_need(&self, need: bool) -> Self{
        if need {
            self.inverse()
        }else{
            *self
        }
    }
}
impl LogicState {
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
    pub fn get_change_pattern(&self) -> &Option<ChangePattern>{
        match self {
            LogicState::Rise(c) => c,
            LogicState::Fall(c) => c,
            _ => &None,
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
    /// | Any       | F/R  | X(warn) |
    /// | F/R       | Any  | X(warn) |
    pub fn combine_end(&self, end: &Self) -> &Self{
        let unknown_and_warn = || -> &Self {
            let warn_description = "Should Not Happen Here!";
            warn!("[warn] {}!", warn_description);
            &LogicState::Unknown
        };
        match (self,end) {
            (LogicState::Unknown,       LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicState::Unknown,       LogicState::HighImpedance) => &LogicState::HighImpedance,
            (LogicState::Unknown,       LogicState::High         ) => &LogicState::High,
            (LogicState::Unknown,       LogicState::Low          ) => &LogicState::Low,
            (LogicState::HighImpedance, LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicState::HighImpedance, LogicState::HighImpedance) => &LogicState::HighImpedance,
            (LogicState::HighImpedance, LogicState::High         ) => &LogicState::High,
            (LogicState::HighImpedance, LogicState::Low          ) => &LogicState::Low,
            (LogicState::High,          LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicState::High,          LogicState::HighImpedance) => &LogicState::HighImpedance,
            (LogicState::High,          LogicState::High         ) => &LogicState::High,
            (LogicState::High,          LogicState::Low          ) => &LogicState::Fall(None),
            (LogicState::Low,           LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicState::Low,           LogicState::HighImpedance) => &LogicState::HighImpedance,
            (LogicState::Low,           LogicState::High         ) => &LogicState::Rise(None),
            (LogicState::Low,           LogicState::Low          ) => &LogicState::Low,
            (LogicState::Rise(_)|LogicState::Fall(_),_) => unknown_and_warn(),
            (_,LogicState::Rise(_)|LogicState::Fall(_)) => unknown_and_warn(),
        }
    }
}

/// LogicVector
#[derive(Default)]
#[derive(Debug, Clone)]
pub struct LogicVector {
    pub vec: Vec<LogicState>,
}
impl LogicVector {
    pub fn new() -> Self{
        Self { vec: vec![] }
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
        self.vec.iter().fold(
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
    fn inverse(&self)->Self{
        let mut inversed = Self{
            vec:Vec::with_capacity(self.vec.len()),
        };
        for (idx,v_state) in self.vec.iter().enumerate() {

            inversed.vec[idx]=v_state.inverse();
        }
        inversed
    }
    fn inverse_if_need(&self, need: bool)->Self{
        if need {
            self.inverse()
        }else{
            self.clone()
        }
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
    ) -> &LogicState{
        let compute_dynamic_logic = || -> &LogicState {
            let bgn_state = self.compute_static(left.get_bgn(), right.get_bgn());
            let end_state = self.compute_static(left.get_end(), right.get_end());
            bgn_state.combine_end(end_state)
        };
        match (self,left,right) {
            (_, _, LogicState::Rise(_)|LogicState::Fall(_)) => compute_dynamic_logic(),
            (_, LogicState::Rise(_)|LogicState::Fall(_), _) => compute_dynamic_logic(),
            _ => self.compute_static(left,right),
        }
    }
    pub fn compute_table(&self,
        left:  &LogicStateTable,
        right: &LogicStateTable,
    ) -> LogicStateTable {
        let mut combine = right.clone();
        let mut vec_right_len: usize = 0;
        for (vec_right,_) in right.table.iter(){
            vec_right_len=vec_right.vec.len();
            break;
        }
        let mut vec_combine_len = vec_right_len;
        let vec_combine_to_right = |vec_combine: &LogicVector|->LogicVector{
            LogicVector{
                vec: vec_combine.vec[..vec_right_len].to_vec(),
            }
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
                            let mut new_key = LogicVector { vec: vec.vec.clone()};
                            new_key.vec.push(state);
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
            let mut vec = LogicVector{
                vec:vec![LogicState::Unknown;count_vec.len()]
            };
            for (&idx_combine,&idx_left)  in count_vec.iter() {
                vec.vec[idx_left] = vec_combine.vec[idx_combine];
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
                    let _ = new_combine.table.insert(vec_in.clone(), *self.compute(left, right));
                },
                _ => {
                    panic!();
                    error!("Can Not Find Here");
                    let _ = new_combine.table.insert(vec_in.clone(), LogicState::Unknown);
                },
            }
        }
        new_combine
    }
    fn compute_static(&self,
            left:  &LogicState,
            right: &LogicState,
        ) -> &LogicState{
        match (self,left,right) {
            (LogicOperation::And, LogicState::Unknown,       LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::Unknown,       LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::Unknown,       LogicState::High         ) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::Unknown,       LogicState::Low          ) => &LogicState::Low,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::High         ) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::HighImpedance, LogicState::Low          ) => &LogicState::Low,
            (LogicOperation::And, LogicState::High,          LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::High,          LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::And, LogicState::High,          LogicState::High         ) => &LogicState::High,
            (LogicOperation::And, LogicState::High,          LogicState::Low          ) => &LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::Unknown      ) => &LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::HighImpedance) => &LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::High         ) => &LogicState::Low,
            (LogicOperation::And, LogicState::Low,           LogicState::Low          ) => &LogicState::Low,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::High         ) => &LogicState::High,
            (LogicOperation::Or,  LogicState::Unknown,       LogicState::Low          ) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::High         ) => &LogicState::High,
            (LogicOperation::Or,  LogicState::HighImpedance, LogicState::Low          ) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::High,          LogicState::Unknown      ) => &LogicState::High,
            (LogicOperation::Or,  LogicState::High,          LogicState::HighImpedance) => &LogicState::High,
            (LogicOperation::Or,  LogicState::High,          LogicState::High         ) => &LogicState::High,
            (LogicOperation::Or,  LogicState::High,          LogicState::Low          ) => &LogicState::High,
            (LogicOperation::Or,  LogicState::Low,           LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Low,           LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::Or,  LogicState::Low,           LogicState::High         ) => &LogicState::High,
            (LogicOperation::Or,  LogicState::Low,           LogicState::Low          ) => &LogicState::Low,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::High         ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Unknown,       LogicState::Low          ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::High         ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::HighImpedance, LogicState::Low          ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::High,          LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::High,          LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::High,          LogicState::High         ) => &LogicState::Low,
            (LogicOperation::Xor, LogicState::High,          LogicState::Low          ) => &LogicState::High,
            (LogicOperation::Xor, LogicState::Low,           LogicState::Unknown      ) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Low,           LogicState::HighImpedance) => &LogicState::Unknown,
            (LogicOperation::Xor, LogicState::Low,           LogicState::High         ) => &LogicState::High,
            (LogicOperation::Xor, LogicState::Low,           LogicState::Low          ) => &LogicState::Low,
            _ => {
                let warn_description = "Should Not Happen Here!";
                warn!("[warn] {}!", warn_description);
                self.compute(left,right)
            },
        }
    }
}


#[derive(Clone,Debug)]
#[derive(PartialEq)]
pub struct LogicStateTable{
    pub table: HashMap<LogicVector, LogicState>,
    pub portid_idx_map: HashMap<PortId, usize>,
}

impl LogicStateTable {
    pub fn new() -> Self{
        Self { 
            table: HashMap::default(),
            portid_idx_map: HashMap::default(),
        }
    }
    pub fn search(
        &self, 
        want_port_state_pair: Vec<(PortId,LogicState)>, 
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
                    warn!("Can not find");
                    panic!()
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
                let state_got = k_vec.vec[*port_idx];
                if !state_want.variant_eq(&state_got) {
                    continue 'outer;
                }
            }
            let _=sub.table.insert(k_vec.clone(), v_state.clone());
        }
        sub
    }
}
impl std::fmt::Display for LogicStateTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{self:?}")
    }
}
impl LogicLike for LogicStateTable {
    fn inverse(&self)->Self{
        let mut inversed = Self{
            table:HashMap::new(),
            portid_idx_map:self.portid_idx_map.clone(),
        };
        for (k_vec,v_state) in self.table.iter() {
            // v_state = v_state.inverse();
            let _=inversed.table.insert(k_vec.clone(), v_state.inverse());
        }
        inversed
    }
    fn inverse_if_need(&self, need: bool)->Self{
        if need {
            self.inverse()
        }else{
            self.clone()
        }
    }
}