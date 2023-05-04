//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
// use crate::types::*;
use crate::units;
use crate::cell::Cell;
use std::collections::HashMap;
#[derive(Debug,Default)]
#[derive(liberty_macros::GroupHashed)]
#[derive(liberty_macros::NameIdx)]
pub struct Library{
    _undefined: crate::ast::UndefinedAttributes,
    #[arrti_type(complex)]
    pub time_unit: units::Time,
    #[arrti_type(complex)]
    pub capacitance_unit: units::Capacitance,
    pub voltage_unit: units::ElectricPotential,
    pub resistance_unit: units::ElectricalResistance,
    pub pulling_resistance_unit: units::ElectricalResistance,
    pub current_unit: units::ElectricCurrent,
    pub power_unit: units::Power,
    pub distance_unit: units::Length,
    pub scalar_unit: units::Ratio,
    #[arrti_type(group_hashed)]
    pub cell: <Cell as crate::ast::GroupAttri>::Set,
    pub voltage_map: HashMap<String, f64>,
    pub sensitization_map: HashMap<String, Sensitization>,
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2007.03/_user_guide.html
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