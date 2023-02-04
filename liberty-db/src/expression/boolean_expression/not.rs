use std::fmt;
use super::{
    BooleanExpressionLike,
    LogicStateTable,
    LogicLike,
};

static SYMBOL_NOT: &str = "!";
pub struct NotExpression{
    sub_expression:  Box<dyn BooleanExpressionLike>,
}
impl NotExpression {
    /// new BooleanExpression
    #[inline]
    pub fn new(sub_expression: Box<dyn BooleanExpressionLike>)->Self{
        Self { sub_expression, }
    }
    #[inline]
    pub fn to_box(self) -> Box<Self>{
        Box::new(self)
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