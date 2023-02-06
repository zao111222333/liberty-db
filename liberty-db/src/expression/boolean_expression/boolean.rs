use std::fmt;
use super::{
    BooleanExpressionLike,
    LogicStateTable, LogicOperation, 
};
static SYMBOL_LEFT: &str = "(";
static SYMBOL_RIGHT: &str = ")";

/// BooleanExpression is the basic expression
pub struct BooleanExpression{
    sub_expression_vec:  Vec<Box<dyn BooleanExpressionLike>>,
    operation_vec: Vec<LogicOperation>,
}

impl BooleanExpression {
    /// new BooleanExpression
    #[inline]
    pub fn new(
        sub_expression_vec: Vec<Box<dyn BooleanExpressionLike>>,
        operation_vec: Vec<LogicOperation>,
    )->Self{
        Self { 
            sub_expression_vec, 
            operation_vec,
        }
    }
    #[inline]
    pub fn to_box(self) -> Box<Self>{
        Box::new(self)
    }
    #[inline]
    fn len_not_match(&self)->bool{
        self.sub_expression_vec.len() != self.operation_vec.len()+1 
    }
}

impl BooleanExpressionLike for BooleanExpression{
    #[inline]
    fn get_state_stable(&self) -> LogicStateTable {
        if self.len_not_match() {
            panic!();
        }
        let mut logic_state = self.sub_expression_vec[0].get_state_stable();
        for (idx,op) in self.operation_vec.iter().enumerate(){
            let right = self.sub_expression_vec[idx+1].get_state_stable();
            logic_state = op.compute_table(
                &logic_state,
                &right,
            );
        }
        logic_state
    }
}

impl fmt::Debug for BooleanExpression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sub_expr_debug_str= self.sub_expression_vec.iter().fold(
            "".to_string(),
            |result, sub_exp| {
                format!("{result} {sub_exp:?}")
            }
        );
        f.debug_struct("BooleanExpression")
            .field(
                "sub_expression_vec", 
                &sub_expr_debug_str)
            .field(
                "operation_vec", 
                &format!("{:?}\n",self.operation_vec))
            .finish()
    }
}

impl fmt::Display for BooleanExpression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.len_not_match() {
            return Err(fmt::Error)
        }
        if self.operation_vec.len()==0{
            write!(f, "{}",self.sub_expression_vec[0])
        }else{
            self.sub_expression_vec[1..].iter().enumerate().fold(
                Ok(()).and_then(|_| write!(f, "{SYMBOL_LEFT}{}",self.sub_expression_vec[0])),
                |result, (idx,sub_exp)| {
                    result.and_then(|_| write!(f, "{}{}", 
                        self.operation_vec[idx], 
                        sub_exp))
                }
            ).and_then(|_| write!(f, "{SYMBOL_RIGHT}"))
        }
    }
}