use hashbrown::HashMap;
use compact_str::CompactString;
use crate::pin::Pin;
use crate::units::Units;
use crate::cell::Cell;
#[derive(Debug, Clone)]
pub struct Library<'a>{
    pub units: Units,
    pub cell_map: HashMap<CompactString, Cell<'a>>,
    pub voltage_map: HashMap<CompactString, f64>,
    pub sensitization_map: HashMap<CompactString, Sensitization>,

}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =66.4
/// &end
/// =66.21
/// ">Reference-Definition</a>
#[derive(Debug, Clone)]
pub struct Sensitization{
    pub group_name: CompactString,
    pub pin_names: PinNames,
    pub vector: Vector,
}

#[derive(Debug, Clone, Copy)]
pub struct PinNames{

}

#[derive(Debug, Clone)]
pub struct Vector{
    pub id: usize,
    pub string: CompactString,
}