mod logic;
pub use logic::{
    ChangePattern,
    LogicLike,
    LogicState, 
    LogicVector,
    LogicStateTable,
    LogicOperation,
};

mod port;
pub use port::{PortId, Port,};

mod latch_ff;
pub use latch_ff::{Latch, FF};

mod boolean_expression;
pub use boolean_expression::BooleanExpression;

/// BooleanExpressionLike
pub trait BooleanExpressionLike: std::fmt::Display + std::fmt::Debug{
    fn get_type(&self)-> ExpressionType;
    fn get_state_stable(&self) -> LogicStateTable;
}

#[derive(Copy,Clone,Debug)]
#[derive(PartialEq)]
pub enum ExpressionType {
    FF,
    Latch,
    Port,
    BooleanExpression,
}

type HashMap<K, V> = hashbrown::HashMap<K, V>;