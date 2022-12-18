use hashbrown::HashMap;
use compact_str::CompactString;
use crate::pin::Pin;
#[derive(Debug, Clone)]
pub struct Cell{
    pub pin_map: HashMap<CompactString,Pin>
}