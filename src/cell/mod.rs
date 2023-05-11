//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>


use std::collections::HashMap;

use crate::{pin::Pin, ast::{HashedGroup, AttributeList}};
mod items;
pub use items::*;

/// cell
#[derive(Debug,Default)]
#[derive(liberty_macros::Group)]
pub struct Cell {
    #[idx_len(1)]
    _idx: Box<<Self as HashedGroup>::Idx>,
    _undefined: AttributeList,
    
    #[arrti_type(simple)]
    pub area: Option<f64>,
    #[arrti_type(group)]
    pub pin: HashMap<<Pin as HashedGroup>::Idx, Pin>,
    #[arrti_type(group)]
    pub statetable: Option<Statetable>,
}