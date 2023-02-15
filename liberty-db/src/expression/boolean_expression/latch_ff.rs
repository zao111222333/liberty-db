use std::fmt;
use super::{
    BooleanExpressionLike,
    LogicTable,
    BooleanExpression,
};

#[derive(Debug)]
pub struct Ff {
    name_pair: [String;2],
    clock_on: BooleanExpression,
    next_state: BooleanExpression,
}

#[derive(Debug)]
pub struct FfExpression {
    pub ff: Box<Ff>,
    pub is_inverse: bool,
}

impl FfExpression {
    #[inline]
    pub fn new() -> Self{
        todo!()
    }
    #[inline]
    pub fn to_box(self) -> Box<Self>{
        Box::new(self)
    }
}

impl fmt::Display for FfExpression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_inverse {
            return write!(f, "{}", self.ff.name_pair[1]);
        } else {
            return write!(f, "{}", self.ff.name_pair[0]);
        }
    }
}

impl BooleanExpressionLike for FfExpression{
    fn to_table(&self) -> LogicTable {
        todo!()
    }
}

#[derive(Debug)]
pub struct Latch{
    name_pair: [String;2],
    clock_on: BooleanExpression,
    next_state: BooleanExpression,
}

#[derive(Debug)]
pub struct LatchExpression {
    pub latch: Box<Latch>,
    pub is_inverse: bool,
}

impl fmt::Display for LatchExpression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_inverse {
            return write!(f, "{}", self.latch.name_pair[1]);
        } else {
            return write!(f, "{}", self.latch.name_pair[0]);
        }
        
    }
}

impl BooleanExpressionLike for LatchExpression{
    fn to_table(&self) -> LogicTable {
        todo!()
    }
}

