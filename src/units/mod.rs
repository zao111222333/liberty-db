//! <script>
//! IFRAME('https://en.wikipedia.org/wiki/International_System_of_Units');
//! </script>

// #[cfg(not(feature = "f32"))]
pub use uom::si::f64::{
    Capacitance,
    ElectricalResistance,
    Length,
    ElectricCharge, 
    ElectricCurrent, 
    ElectricPotential, 
    Energy, 
    Power, 
    Ratio, 
    ThermodynamicTemperature, 
    Time,
};


pub use uom::fmt::DisplayStyle;
pub use uom::si::Unit;
pub use uom::si::{
    capacitance,
    electrical_resistance,
    electric_current, 
    electric_potential, 
    ratio, 
    thermodynamic_temperature, 
};
pub mod length;
pub mod electric_charge;
pub mod power;
pub mod energy;
pub mod time;

use crate::types::Float;

/// Create `Length` quantity with `meter` unit
#[inline]
pub fn meter(v: Float) -> Length{
    Length::new::<length::meter>(v)
}

/// Create `Time` quantity with `second` unit
#[inline]
pub fn second(v: Float) -> Time{
    Time::new::<time::second>(v)
}

/// Create `Time` quantity with `microsecond` unit
#[inline]
pub fn microsecond(v: Float) -> Time{
    Time::new::<time::microsecond>(v)
}

/// Create `Time` quantity with `nanosecond` unit
#[inline]
pub fn nanosecond(v: Float) -> Time{
    Time::new::<time::nanosecond>(v)
}