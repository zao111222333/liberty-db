use std::fmt;
use super::{
    BooleanExpressionLike,ExpressionType,
    PortId,LogicStateTable,
    LogicState,LogicVector,LogicOperation, LogicLike,
};


/// BooleanExpression is the basic expression
pub struct BooleanExpression{
    sub_expression_vec:  Vec<Box<dyn BooleanExpressionLike>>,
    need_inverse_vec: Vec<bool>,
    operation_vec: Vec<LogicOperation>,
}

impl BooleanExpression {
    #[inline]
    /// new BooleanExpression
    pub fn new(
        sub_expression_vec: Vec<Box<dyn BooleanExpressionLike>>,
        need_inverse_vec: Vec<bool>,
        operation_vec: Vec<LogicOperation>,
    )->Self{
        Self { 
            sub_expression_vec, 
            need_inverse_vec, 
            operation_vec,
        }
    }
    #[inline]
    fn len_not_match(&self)->bool{
        self.sub_expression_vec.len() != self.operation_vec.len()+1 
        || self.sub_expression_vec.len() != self.need_inverse_vec.len()
    }
}

impl BooleanExpressionLike for BooleanExpression{
    #[inline]
    fn get_type(&self)-> ExpressionType {
        ExpressionType::BooleanExpression
    }
    fn get_state_stable(&self) -> LogicStateTable {
        if self.len_not_match() {
            panic!();
        }
        let mut logic_state = self.sub_expression_vec[0].get_state_stable();
        logic_state = logic_state.inverse_if_need(self.need_inverse_vec[0]);
        for (idx,op) in self.operation_vec.iter().enumerate(){
            let mut right = self.sub_expression_vec[idx+1].get_state_stable();
            right = right.inverse_if_need(self.need_inverse_vec[idx+1]);
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
                "need_inverse_vec", 
                &format!("{:?}\n",self.need_inverse_vec))
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
        self.sub_expression_vec.iter().enumerate().fold(
            Ok(()),
            |result, (idx,sub_exp)| {
                let sub_exp_str;
                match (
                    self.need_inverse_vec[idx], 
                    sub_exp.get_type()==ExpressionType::BooleanExpression,
                ) {
                    (true, false) => sub_exp_str = format!("!{}",sub_exp),
                    (true, true ) => sub_exp_str = format!("!({})",sub_exp),
                    (false,false) => sub_exp_str = format!("{}",sub_exp),
                    (false,true ) => sub_exp_str = format!("({})",sub_exp),
                }
                if idx==0{
                    result.and_then(|_| write!(f, "{}", 
                        sub_exp_str))
                }else{
                    result.and_then(|_| write!(f, "{}{}", 
                        self.operation_vec[idx-1], 
                        sub_exp_str))
                }
            }
        )
    }
}