use std::fmt;
use strum::IntoEnumIterator;
use crate::types::*;
use super::{
    BooleanExpressionLike,
    LogicState,LogicVector,
    LogicStateTable, BooleanExpression,
};

#[derive(Clone,Debug)]
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
impl PartialEq for Port {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl std::cmp::Eq for Port {}
impl std::hash::Hash for Port {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.name.hash(hasher);
    }
}

lazy_static! {
    static ref BASIC_MAP: HashMap<LogicVector, LogicState> = {
        let mut m = HashMap::new();
        for state in LogicState::iter(){
            let _ = m.insert(
                LogicVector::new(vec![state]),
                state,
            );
        }
        m
    };
}

impl BooleanExpressionLike for Port{
    #[inline]
    fn get_state_stable(&self) -> LogicStateTable {
        LogicStateTable::new( 
            BASIC_MAP.clone(), 
            [(self.clone(), 0)]
                                .iter()
                                .cloned()
                                .collect(),
        )
    }
}



