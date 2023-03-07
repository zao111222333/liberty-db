//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
//! </script>
mod logic;
pub use logic::{
    CommonState,
    StaticState,
    EdgeState,
    UninitState,
    LogicState,
    ChangePattern,
    LogicLike,
    LogicVector,
    LogicTable,
    LogicOperator1,
    LogicOperator2,
    Searcher,
};

mod port;
pub use port::Port;

mod ff;
pub use ff::{
    Ff,FfExpression,
};
mod latch;
pub use latch::{
    Latch,LatchExpression, 
    LatchFfId,
};

mod function;
pub use function::FunctionExpression;

mod tri_state;
pub use tri_state::TriState;

/// BooleanExpressionLike
pub trait BooleanExpressionLike: std::fmt::Display + std::fmt::Debug + BooleanExpressionClone{
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

pub trait BooleanExpressionClone {
    fn clone_box(&self) -> Box<dyn BooleanExpressionLike>;
}

impl<T> BooleanExpressionClone for T
where
    T: 'static + BooleanExpressionLike + Clone,
{
    fn clone_box(&self) -> Box<dyn BooleanExpressionLike> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn BooleanExpressionLike> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for BooleanExpression {
    fn clone(&self) -> Self {
        BooleanExpression { value: self.value.clone_box() }
    }
}

use std::{ops::{Deref,DerefMut}, fmt::Display, hash::Hash};

use crate::types::HashMap;
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

impl Hash for BooleanExpression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.to_table().hash(state);
    }
}
impl BooleanExpression {
    const BRACKET_L: char = '(';
    const BRACKET_R: char = ')';
    // TODO:
    pub fn from_str(
        s: &str, 
        ff_map: &HashMap<LatchFfId,Ff>,
        latch_map: &HashMap<LatchFfId,Latch>,
    ) -> Result<Self, std::fmt::Error> {
        let l_pos_list = s.match_indices(Self::BRACKET_L).map(|(i, _)|i).collect::<Vec<usize>>();
        let r_pos_list = s.match_indices(Self::BRACKET_R).map(|(i, _)|i).collect::<Vec<usize>>();
        // match (s.find(Self::BRACKET_L),s.find(Self::BRACKET_R)){
        //     (None, None) => todo!(),
        //     (None, Some(_)) => Err(std::fmt::Error),
        //     (Some(_), None) => Err(std::fmt::Error),
        //     (Some(idx_l), Some(idx_r)) => todo!(),
        // }
        todo!()
    }
}