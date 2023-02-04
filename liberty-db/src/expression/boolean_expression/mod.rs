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
pub use port::Port;

mod latch_ff;
pub use latch_ff::{
    Ff,FfExpression,
    Latch,LatchExpression, 
};

mod boolean;
pub use boolean::BooleanExpression;

mod not;
pub use not::NotExpression;

/// BooleanExpressionLike
pub trait BooleanExpressionLike: std::fmt::Display + std::fmt::Debug{
    fn get_state_stable(&self) -> LogicStateTable;
}