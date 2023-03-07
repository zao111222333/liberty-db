use std::fmt;
use super::{
    BooleanExpressionLike,
    LogicTable,
    BooleanExpression, function::{SYMBOL_LEFT, SYMBOL_RIGHT},
};

#[derive(Debug,Clone)]
pub struct TriState {
    enable: BooleanExpression,
    logic: BooleanExpression,
}

impl TriState {
    #[inline]
    pub fn new() -> Self{
        todo!()
    }
    #[inline]
    pub fn to_box(self) -> Box<Self>{
        Box::new(self)
    }
}

impl fmt::Display for TriState{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{SYMBOL_LEFT}{}{SYMBOL_RIGHT}@Z{SYMBOL_LEFT}{}{SYMBOL_RIGHT}",self.logic,self.enable)
    }
}

impl BooleanExpressionLike for TriState{
    #[inline]
    fn to_table(&self) -> LogicTable {
        todo!()
    }
}