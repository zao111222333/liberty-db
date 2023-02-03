use log::warn;

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
    settle_down_time: f64,
    transition_time: f64,
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
#[derive(strum_macros::Display, strum_macros::EnumString)]
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

impl LogicState {
    pub fn get_change_pattern(&self) -> &Option<ChangePattern>{
        match self {
            LogicState::Rise(c) => c,
            LogicState::Fall(c) => c,
            _ => &None,
        }
    }
    pub fn get_inverse(&self, need_inverse: bool) -> &Self{
        match (need_inverse,self){
            (false,_) => self,
            (true,LogicState::Low ) => &LogicState::High,
            (true,LogicState::High) => &LogicState::Low,
            (true,LogicState::Rise(_)) => &LogicState::Fall(None),
            (true,LogicState::Fall(_)) => &LogicState::Rise(None),
            _ => self,
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
// #[derive(Eq)]
#[derive(Debug, Clone)]
pub struct LogicVector {
    pub state_vec: Vec<LogicState>,
}
impl LogicVector {
    pub fn new() -> Self{
        Self { state_vec: vec![] }
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
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.to_string().hash(hasher);
    }
}

impl std::fmt::Display for LogicVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.state_vec.iter().fold(
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

