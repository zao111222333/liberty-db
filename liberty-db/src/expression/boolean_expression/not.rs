use std::fmt;
use super::{
    BooleanExpressionLike,
    LogicStateTable,
    LogicLike,
    BooleanExpression,
};

static SYMBOL_NOT: &str = "!";
pub struct NotExpression{
    sub_expression:  BooleanExpression,
}
impl NotExpression {
    /// new BooleanExpression
    #[inline]
    pub fn new(sub_expression: BooleanExpression)->Self{
        Self { sub_expression, }
    }
}

impl Into<BooleanExpression> for NotExpression{
    #[inline]
    fn into(self) -> BooleanExpression {
        BooleanExpression{
            value: Box::new(self)
        }
    }
}

impl BooleanExpressionLike for NotExpression{
    #[inline]
    fn get_state_stable(&self) -> LogicStateTable {
        self.sub_expression.get_state_stable().inverse()
    }
}

impl fmt::Debug for NotExpression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotExpression")
            .field(
                "sub_expression", 
                &self.sub_expression)
            .finish()
    }
}

impl fmt::Display for NotExpression{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{SYMBOL_NOT}{}",self.sub_expression)
    }
}