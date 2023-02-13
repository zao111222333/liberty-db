mod logic;
pub use logic::{
    CommonState,
    StaticState,
    DynamicState,
    UninitState,
    LogicState,
    ChangePattern,
    LogicLike,
    LogicVector,
    LogicStateTable,
    LogicOperation,
};

mod port;
pub use port::Port;

mod latch_ff;
pub use latch_ff::{
    Ff,FfExpression,
    Latch,LatchExpression, 
};

mod boolean;
pub use boolean::FunctionExpression;

mod not;
pub use not::NotExpression;

/// BooleanExpressionLike
pub trait BooleanExpressionLike: std::fmt::Display + std::fmt::Debug{
    fn get_state_stable(&self) -> LogicStateTable;
}

#[derive(Debug)]
pub struct BooleanExpression{
    value: Box<dyn BooleanExpressionLike>,
}

impl PartialEq for BooleanExpression {
    fn eq(&self, other: &Self) -> bool {
        self.get_state_stable() == other.get_state_stable()
    }
}

use std::{ops::{Deref,DerefMut}, fmt::Display};
impl DerefMut for BooleanExpression {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
impl Deref for BooleanExpression {
    type Target = Box<dyn BooleanExpressionLike>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Display for BooleanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.value)
    }
}