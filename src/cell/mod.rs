//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>


use crate::pin::Pin;
mod items;
pub use items::*;

/// cell
#[derive(Debug,Default)]
#[derive(liberty_macros::NameIdx)]
#[derive(liberty_macros::GroupHashed)]
pub struct Cell {
    #[idx_len(1)]
    _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
    _undefined: crate::ast::UndefinedAttributes,
    
    #[arrti_type(simple)]
    pub area: Option<f64>,
    #[arrti_type(group_hashed)]
    pub pin: <Pin as crate::ast::GroupAttri>::Set,
    #[arrti_type(group_hashed)]
    pub statetable: <Statetable as crate::ast::GroupAttri>::Set,
}
