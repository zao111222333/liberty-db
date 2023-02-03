use std::fmt;
use super::{
    BooleanExpressionLike,
    ExpressionType,
    LogicStateTable,
    LogicState,LogicVector,LogicOperation,
};

pub struct FF {
    name_pair: [String;2],
    is_inverse: bool,
    clock_on: Box<dyn BooleanExpressionLike>,
    next_state: Box<dyn BooleanExpressionLike>,
}

impl fmt::Debug  for FF{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FF")
            .field(
                "name_pair", 
                &format!("{:?}\n",self.name_pair))
            .field(
                "is_inverse", 
                &format!("{:?}\n",self.is_inverse))
            .field(
                "clock_on", 
                &format!("{:?}\n",self.clock_on))
            .field(
                "next_state", 
                &format!("{:?}\n",self.next_state))
            .finish()
    }
}

impl fmt::Display for FF{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_inverse {
            return write!(f, "{}", self.name_pair[1]);
        } else {
            return write!(f, "{}", self.name_pair[0]);
        }
        
    }
}

impl BooleanExpressionLike for FF{
    fn get_type(&self)-> ExpressionType{
        ExpressionType::FF
    }
    fn get_state_stable(&self) -> LogicStateTable {
        todo!()
    }
}

pub struct Latch{
    name_pair: [String;2],
    is_inverse: bool,
    clock_on: Box<dyn BooleanExpressionLike>,
    next_state: Box<dyn BooleanExpressionLike>,
}

impl fmt::Debug for Latch{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Latch")
            .field(
                "name_pair", 
                &self.name_pair)
            .field(
                "is_inverse", 
                &self.is_inverse)
            .field(
                "clock_on", 
                &format!("{:?}\n",self.clock_on))
            .field(
                "next_state", 
                &format!("{:?}\n",self.next_state))
            .finish()
    }
}

impl fmt::Display for Latch{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_inverse {
            return write!(f, "{}", self.name_pair[1]);
        } else {
            return write!(f, "{}", self.name_pair[0]);
        }
        
    }
}

impl BooleanExpressionLike for Latch{
    fn get_type(&self)-> ExpressionType{
        ExpressionType::Latch
    }
    fn get_state_stable(&self) -> LogicStateTable {
        todo!()
    }
}

