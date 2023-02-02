mod logic;
pub use logic::{
    ChangePattern,
    LogicState, 
    LogicVector,
    LogicOperation
};


use std::fmt;



pub trait BooleanExpressionLike: fmt::Display + fmt::Debug{
    fn get_type(&self)-> ExpressionType;
    fn set_state(&mut self, state: LogicState);
    fn get_state(&self)-> LogicState;

}

#[derive(Copy,Clone,Debug)]
#[derive(PartialEq)]
pub enum ExpressionType {
    FF,
    Latch,
    Port,
    BooleanExpression,
}



pub struct BooleanExpression{
    logic_state: LogicState,
    sub_expression_vec:  Vec<Box<dyn BooleanExpressionLike>>,
    not_invert_vec: Vec<bool>,
    operation_vec: Vec<LogicOperation>,
}

impl BooleanExpression {
    pub fn new(
        sub_expression_vec: Vec<Box<dyn BooleanExpressionLike>>,
        not_invert_vec: Vec<bool>,
        operation_vec: Vec<LogicOperation>,
    )->Self{
        Self { 
            logic_state: LogicState::default(),
            sub_expression_vec, 
            not_invert_vec, 
            operation_vec,
        }
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
            .field("sub_expression_vec", &sub_expr_debug_str)
            .field("not_invert_vec", &format!("{:?}\n",self.not_invert_vec))
            .field("operation_vec", &format!("{:?}\n",self.operation_vec))
            .finish()
    }
}

impl fmt::Display for BooleanExpression{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sub_expression_vec.len() != self.operation_vec.len()+1 
        || self.sub_expression_vec.len() != self.not_invert_vec.len() {
            return Err(fmt::Error)
        }
        self.sub_expression_vec.iter().enumerate().fold(
            Ok(()),
            |result, (idx,sub_exp)| {
                let sub_exp_str;
                // sub_exp_str = format!("({})",sub_exp);
                match (self.not_invert_vec[idx], sub_exp.get_type()==ExpressionType::BooleanExpression) {
                    (false,false) => sub_exp_str = format!("!{}",sub_exp),
                    (false,true ) => sub_exp_str = format!("!({})",sub_exp),
                    (true, false) => sub_exp_str = format!("{}",sub_exp),
                    (true, true ) => sub_exp_str = format!("({})",sub_exp),
                }
                if idx==0{
                    result.and_then(|_| write!(f, "{}", sub_exp_str))
                }else{
                    result.and_then(|_| write!(f, "{}{}", self.operation_vec[idx-1], sub_exp_str))
                }
            }
        )
    }
}

impl BooleanExpressionLike for  BooleanExpression{
    fn get_type(&self)-> ExpressionType {
        ExpressionType::BooleanExpression
    }
    fn set_state(&mut self, state: LogicState) {
        self.logic_state = state;
    }
    fn get_state(&self)-> LogicState {
        self.logic_state
    }
}

pub struct Port{
    logic_state: LogicState,
    name:  String,
}

impl Port {
    pub fn new(name: &str) -> Self{
        Self { 
            name: name.to_string(),
            logic_state: LogicState::default(),

        }
    }
}

impl fmt::Debug for Port{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Port")
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for Port{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl BooleanExpressionLike for Port{
    fn get_type(&self)-> ExpressionType{
        ExpressionType::Port
    }
    fn set_state(&mut self, state: LogicState) {
        self.logic_state = state;
    }
    fn get_state(&self)-> LogicState {
        self.logic_state
    }
}

pub struct FF {
    logic_state: LogicState,
    name_pair: [String;2],
    is_inverse: bool,
    clock_on: Box<dyn BooleanExpressionLike>,
    next_state: Box<dyn BooleanExpressionLike>,
}

impl fmt::Debug  for FF{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FF")
            .field("name_pair", &format!("{:?}\n",self.name_pair))
            .field("is_inverse", &format!("{:?}\n",self.is_inverse))
            .field("clock_on", &format!("{:?}\n",self.clock_on))
            .field("next_state", &format!("{:?}\n",self.next_state))
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
    fn set_state(&mut self, state: LogicState) {
        self.logic_state = state;
    }
    fn get_state(&self)-> LogicState {
        self.logic_state
    }
}

pub struct Latch{
    logic_state: LogicState,
    name_pair: [String;2],
    is_inverse: bool,
    clock_on: Box<dyn BooleanExpressionLike>,
    next_state: Box<dyn BooleanExpressionLike>,
}

impl fmt::Debug for Latch{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Latch")
            .field("name_pair", &format!("{:?}\n",self.name_pair))
            .field("is_inverse", &format!("{:?}\n",self.is_inverse))
            .field("clock_on", &format!("{:?}\n",self.clock_on))
            .field("next_state", &format!("{:?}\n",self.next_state))
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
    fn set_state(&mut self, state: LogicState) {
        self.logic_state = state;
    }
    fn get_state(&self)-> LogicState {
        self.logic_state
    }
}

