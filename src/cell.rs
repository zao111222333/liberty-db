use crate::pin::Pin;
use crate::ast::{UndefinedAttributes, GroupAttri, SimpleAttri};
/// cell
#[derive(Debug,Default)]
#[derive(liberty_macros::GroupHashed)]
#[derive(liberty_macros::NameIdx)]
pub struct Cell {
    _undefined: UndefinedAttributes,
    #[arrti_type(simple)]
    pub area: Option<f64>,
    #[arrti_type(group_hashed)]
    pub pin: <Pin as GroupAttri>::Set,
}