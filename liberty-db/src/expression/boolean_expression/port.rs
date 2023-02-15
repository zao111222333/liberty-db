use std::fmt;
use strum::IntoEnumIterator;
use crate::types::*;
use super::{
    BooleanExpressionLike,
    LogicState,LogicVector,
    LogicTable, BooleanExpression,
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
    static ref BASIC_MAP: HashMap<LogicVector, LogicState> = LogicState::iter().map(|state| 
                                                                    (vec![state].into(),state)
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