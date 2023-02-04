use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::Display;

// use hashbrown::HashMap;
use super::{
    BooleanExpressionLike,ExpressionType,
    LogicStateTable, HashMap,
    LogicState,LogicVector,LogicOperation,
};

#[derive(Clone,Debug)]
pub struct PortId{
    name:  String,
}
impl PortId {
    #[inline]
    pub fn new(name: &str) -> Self{
        Self { name: name.to_string() }
    }
}
impl fmt::Display for PortId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"PortId(name: {})",self.name)
    }
}
impl PartialEq for PortId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl std::cmp::Eq for PortId {}
impl std::hash::Hash for PortId {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.name.hash(hasher);
    }
}

/// Port
#[derive(Clone)]
pub struct Port{
    id: PortId,
}

impl Port {
    #[inline]
    pub fn new(name: &str) -> Self {
        Self {id: PortId::new(name)}
    }
    #[inline]
    pub fn get_id(&self) -> PortId {
        self.id.clone()
    }
}

lazy_static! {
    static ref BASIC_MAP: HashMap<LogicVector, LogicState> = {
        let mut m = HashMap::new();
        for state in LogicState::iter(){
            let _ = m.insert(
                LogicVector{vec:vec![state]}, 
                state,
            );
        }
        m
    };
}

impl BooleanExpressionLike for Port{
    #[inline]
    fn get_type(&self)-> ExpressionType{
        ExpressionType::Port
    }
    #[inline]
    fn get_state_stable(&self) -> LogicStateTable {
        LogicStateTable::new( 
            BASIC_MAP.clone(), 
            [(self.get_id(), 0)]
                                .iter()
                                .cloned()
                                .collect(),
        )
    }
}

impl fmt::Debug for Port{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Port")
            .field("id", &self.id)
            .finish()
    }
}

impl fmt::Display for Port{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.id.name)
    }
}

