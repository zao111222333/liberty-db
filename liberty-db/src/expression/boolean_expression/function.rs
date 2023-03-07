//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
//! </script>
//! 
use std::fmt;
use super::{
    BooleanExpressionLike, BooleanExpression,
    LogicTable, 
    LogicOperator2, LogicOperator1,
};
pub(super) static SYMBOL_LEFT: &str = "(";
pub(super) static SYMBOL_RIGHT: &str = ")";

/// FunctionExpression is the basic expression
#[derive(Debug,Clone)]
pub struct FunctionExpression{
    sub_expression_vec:  Vec<BooleanExpression>,
    op1_vec: Vec<Option<LogicOperator1>>,
    op2_vec: Vec<LogicOperator2>,
}

impl FunctionExpression {
    /// new FunctionExpression
    #[inline]
    pub fn new(
        sub_expression_vec: Vec<BooleanExpression>,
        op1_vec: Vec<Option<LogicOperator1>>,
        op2_vec: Vec<LogicOperator2>,
    )->Self{
        Self { 
            sub_expression_vec, 
            op1_vec,
            op2_vec,
        }
    }
    #[inline]
    fn len_not_match(&self) -> bool {
        self.sub_expression_vec.len() != self.op2_vec.len()+1 || self.sub_expression_vec.len() != self.op1_vec.len()
    }
}

impl Into<BooleanExpression> for FunctionExpression{
    #[inline]
    fn into(self) -> BooleanExpression {
        BooleanExpression{
            value: Box::new(self)
        }
    }
}
impl BooleanExpressionLike for FunctionExpression{
    #[inline]
    /// TODO: Add test case for: `XOR` > `AND` > `OR`.
    /// 
    /// The order of precedence of the operators is left to right, with inversion performed first, then XOR, then AND, then OR.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html
    /// ?field=test
    /// &bgn
    /// =133.12
    /// &end
    /// =133.13
    /// ">Reference</a>
    fn to_table(&self) -> LogicTable {
        #[inline]
        fn compute_op2(
            target_op2: &LogicOperator2, 
            op2_vec: &Vec<LogicOperator2>, 
            state_table_vec: &Vec<LogicTable>
        )->(Vec<LogicOperator2>,Vec<LogicTable>){
            let mut _op2_vec: Vec<LogicOperator2> = vec![];
            let mut _state_table_vec: Vec<LogicTable> = vec![state_table_vec[0].clone()];
            for (op2_idx,op2) in op2_vec.iter().enumerate(){
                if op2 == target_op2 {
                    match _state_table_vec.pop(){
                        Some(state_table) => {       
                            _state_table_vec.push(op2.compute_table(&state_table, &state_table_vec[op2_idx+1]))
                        },
                        None => panic!(),
                    }
                }else{
                    _op2_vec.push(*op2);
                    _state_table_vec.push(state_table_vec[op2_idx+1].clone());
                }
            }
            (_op2_vec,_state_table_vec)
        }
        if self.len_not_match() {
            panic!();
        }
        let op_vec = &self.op2_vec;
        let state_table_vec: &Vec<LogicTable> = &self.sub_expression_vec.iter().enumerate()
                                                                          .map(
                                                                                |(idx,x)|
                                                                                    match self.op1_vec[idx] {
                                                                                        Some(op1) => op1.compute_table(&x.to_table()),
                                                                                        None => x.to_table(),
                                                                                    }
                                                                            )
                                                                          .collect();
        let (_op_xor,_state_xor) = compute_op2(&LogicOperator2::Xor,op_vec,state_table_vec);
        let (_op_and,_state_and) = compute_op2(&LogicOperator2::And,&_op_xor,&_state_xor);
        let (_op_or,_state_or) = compute_op2(&LogicOperator2::Or,&_op_and,&_state_and);
        let mut out = _state_or[0].clone();
        out.self_node = self.to_string();
        out
    }
}

impl fmt::Display for FunctionExpression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let append_sub = |sub_idx: usize, f: &mut fmt::Formatter<'_>| -> fmt::Result {
            match self.op1_vec[sub_idx] {
                Some(op1) => write!(f, "{op1}{}",self.sub_expression_vec[sub_idx]),
                None => write!(f, "{}",self.sub_expression_vec[sub_idx]),
            }
        };
        if self.len_not_match() {
            return Err(fmt::Error)
        }
        if self.op2_vec.len()==0{
            append_sub(0,f)
        }else{
            self.sub_expression_vec[1..].iter().enumerate().fold(
                Ok(()).and_then(|_| write!(f, "{SYMBOL_LEFT}{}",self.sub_expression_vec[0])),
                |result, (idx,_)| {
                    result.and_then(|_| write!(f, "{}", 
                        self.op2_vec[idx])).and_then(|_|append_sub(idx+1,f) )
                }
            ).and_then(|_| write!(f, "{SYMBOL_RIGHT}"))
        }
    }
}