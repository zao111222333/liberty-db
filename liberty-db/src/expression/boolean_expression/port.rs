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

impl Into<BooleanExpression> for Port{
    #[inline]
    fn into(self) -> BooleanExpression {
        BooleanExpression{
            value: Box::new(self)
        }
    }
}

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
    fn to_table(&self) -> LogicTable {
        LogicTable::new( 
            &self.name,
            BASIC_MAP.clone(),
            vec![self.clone()]
        )
    }
}

#[derive(Clone,Debug,Hash,PartialEq,Eq)]
pub struct StaticExpression{
    state:  StaticState,
}
impl StaticExpression {
    #[inline]
    pub fn new(state:  StaticState,) -> Self{
        Self { state }
    }
}

impl Into<BooleanExpression> for StaticExpression{
    #[inline]
    fn into(self) -> BooleanExpression {
        BooleanExpression{
            value: Box::new(self)
        }
    }
}

impl fmt::Display for StaticExpression{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.state.fmt(f)
    }
}

impl std::ops::Deref for StaticExpression {
    type Target=StaticState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Into<StaticState> for StaticExpression {
    fn into(self) -> StaticState {
        self.state
    }
}

impl BooleanExpressionLike for StaticExpression{
    #[inline]
    fn to_table(&self) -> LogicTable {
        LogicTable::new(
            format!("{}",self.state).as_str(),
            {match self.state {
                StaticState::High => ONE_MAP.clone(),
                StaticState::Low => ZERO_MAP.clone(),
            }},
            vec![Port::new(format!("{}",self.state).as_str())],
        )
    }
}