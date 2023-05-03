use crate::pin::Pin;

#[derive(Debug,Default)]
#[derive(liberty_macros::GroupHashed)]
#[derive(liberty_macros::NameIdx)]
pub struct Cell{
    _undefined: crate::ast::UndefinedAttributes,
    #[arrti_type(group_hashed)]
    pub pin: <Pin as crate::ast::GroupAttri>::Set,
    pub pin1: <Pin as crate::ast::GroupAttri>::Set,
}
