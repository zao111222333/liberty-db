//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-rs/2020.09/reference_manual.html');
//! </script>
use crate::types::*;
use crate::units;
use crate::cell::Cell;
#[derive(Debug)]
pub struct Library<'a>{
    pub time_unit: units::Time,
    pub capacitance_unit: units::Capacitance,
    pub voltage_unit: units::ElectricPotential,
    pub resistance_unit: units::ElectricalResistance,
    pub pulling_resistance_unit: units::ElectricalResistance,
    pub current_unit: units::ElectricCurrent,
    pub power_unit: units::Power,
    pub distance_unit: units::Length,
    pub scalar_unit: units::Ratio,
    pub cell_map: HashMap<String, Cell<'a>>,
    pub voltage_map: HashMap<String, f64>,
    pub sensitization_map: HashMap<String, Sensitization>,
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
    pub group_name: String,
    pub pin_names: PinNames,
    pub vector: Vector,
}

#[derive(Debug, Clone, Copy)]
pub struct PinNames{

}

#[derive(Debug, Clone)]
pub struct Vector{
    pub id: usize,
    pub string: String,
}