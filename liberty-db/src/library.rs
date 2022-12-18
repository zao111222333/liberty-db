use hashbrown::HashMap;
use compact_str::CompactString;
use crate::units::Units;
use crate::cell::Cell;
#[derive(Debug, Clone)]
pub struct Library{
    pub units: Units,
    pub cell_map: HashMap<CompactString, Cell>,
    pub voltage_map: HashMap<CompactString, f64>,
}