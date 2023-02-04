use std::fmt;
use super::{
    BooleanExpressionLike,
    LogicStateTable,
    BooleanExpression,
};

pub struct Ff {
    name_pair: [String;2],
    clock_on: Box<dyn BooleanExpressionLike>,
    next_state: Box<dyn BooleanExpressionLike>,
}

impl fmt::Debug for Ff{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ff")
            .field(
                "name_pair", 
                &format!("{:?}\n",self.name_pair))
            .field(
                "clock_on", 
                &format!("{:?}\n",self.clock_on))
            .field(
                "next_state", 
                &format!("{:?}\n",self.next_state))
            .finish()
    }
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
    fn get_state_stable(&self) -> LogicStateTable {
        todo!()
    }
}

pub struct Latch{
    name_pair: [String;2],
    clock_on: BooleanExpression,
    next_state: BooleanExpression,
}

impl fmt::Debug for Latch{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Latch")
            .field(
                "name_pair", 
                &self.name_pair)
            .field(
                "clock_on", 
                &format!("{:?}\n",self.clock_on))
            .field(
                "next_state", 
                &format!("{:?}\n",self.next_state))
            .finish()
    }
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
    fn get_state_stable(&self) -> LogicStateTable {
        todo!()
    }
}

