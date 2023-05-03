//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
mod test;
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
// use strum_macros::Display;
pub use tri_state::TriState;
use std::{fmt::{Display,Debug}, hash::Hash, collections::HashMap};
/// BooleanExpressionLike
#[enum_dispatch::enum_dispatch(BooleanExpression)]
pub trait BooleanExpressionLike: Display + Debug + Clone{
    /// get table with function
    fn table(&self) -> LogicTable;
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
/// ?field=test
/// &bgn
/// =132.36
/// &end
/// =132.38
/// ">Reference</a>
#[enum_dispatch::enum_dispatch]
#[derive(Debug,Clone)]
pub enum BooleanExpression {
    Port(Port),
    FF(FfExpression),
    Latch(LatchExpression),
    Function(FunctionExpression),
    TriState(TriState),
}

impl Display for BooleanExpression {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanExpression::Port(exp) => std::fmt::Display::fmt(&exp, f),
            BooleanExpression::FF(exp) => std::fmt::Display::fmt(&exp, f),
            BooleanExpression::Latch(exp) => std::fmt::Display::fmt(&exp, f),
            BooleanExpression::Function(exp) => std::fmt::Display::fmt(&exp, f),
            BooleanExpression::TriState(exp) => std::fmt::Display::fmt(&exp, f),
        }
    }
}

impl PartialEq for BooleanExpression {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.table() == other.table()
    }
}


impl Hash for BooleanExpression {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.table().hash(state);
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