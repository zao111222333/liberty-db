//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
//! </script>
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
    LogicTable,
    LogicOperator1,
    LogicOperator2,
};

mod port;
pub use port::Port;

mod latch_ff;
pub use latch_ff::{
    Ff,FfExpression,
    Latch,LatchExpression, 
};

mod function;
pub use function::FunctionExpression;

/// BooleanExpressionLike
pub trait BooleanExpressionLike: std::fmt::Display + std::fmt::Debug{
    fn to_table(&self) -> LogicTable;
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =132.36
/// &end
/// =132.38
/// ">Reference</a>
#[derive(Debug)]
pub struct BooleanExpression{
    value: Box<dyn BooleanExpressionLike>,
}

impl PartialEq for BooleanExpression {
    fn eq(&self, other: &Self) -> bool {
        self.to_table() == other.to_table()
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
        self.value.fmt(f)
    }
}