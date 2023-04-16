use std::fmt;
use crate::types::*;
use super::{
    BooleanExpressionLike,
    LogicState,LogicVector,
    LogicTable, BooleanExpression, StaticState,
};

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
#[derive(Ord)]
#[derive(PartialOrd)]
pub struct Port{
    name:  String,
}
impl Port {
    #[inline]
    pub fn new(name: &str) -> Self{
        Self { name: name.to_string() }
    }
}

// impl Into<BooleanExpression> for Port{
//     #[inline]
//     fn into(self) -> BooleanExpression {
//         BooleanExpression{
//             value: Box::new(self)
//         }
//     }
// }

impl fmt::Display for Port{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

lazy_static! {
    static ref BASIC_MAP: HashMap<LogicVector, LogicState> = LogicState::iter().map(|state| 
                                                                    (vec![state].into(),state)
                                                                ).collect();
    static ref ONE_MAP: HashMap<LogicVector, LogicState> = LogicState::iter().map(|state| 
                                                                    (vec![state].into(),LogicState::Static(StaticState::High))
                                                                ).collect();
    static ref ZERO_MAP: HashMap<LogicVector, LogicState> = LogicState::iter().map(|state| 
                                                                    (vec![state].into(),LogicState::Static(StaticState::Low))
                                                                ).collect();
}

impl BooleanExpressionLike for Port{
    #[inline]
    fn table(&self) -> LogicTable {
        LogicTable::new( 
            &self.name,
            BASIC_MAP.clone(),
            vec![self.clone()]
        )
    }
}
