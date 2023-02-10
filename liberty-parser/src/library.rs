use hashbrown::HashMap;

use crate::units::Units;
use crate::cell::Cell;
#[derive(Debug, Clone)]
pub struct Library{
        pub units: Units,
        cell_map: HashMap<String, Cell>,
    voltage_map: HashMap<String, f64>,
}