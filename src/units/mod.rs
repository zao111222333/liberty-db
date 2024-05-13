//! <script>
//! IFRAME('https://en.wikipedia.org/wiki/International_System_of_Units');
//! </script>

use ordered_float::NotNan;
pub use uom::si::f64::{
  Capacitance, ElectricCharge, ElectricCurrent, ElectricPotential, ElectricalResistance,
  Energy, Length, Power, Ratio, ThermodynamicTemperature, Time,
};

pub use uom::fmt::DisplayStyle;
pub use uom::si::Unit;
pub use uom::si::{
  capacitance, electric_current, electric_potential, electrical_resistance, ratio,
  thermodynamic_temperature,
};
pub mod electric_charge;
pub mod energy;
pub mod length;
pub mod power;
pub mod time;

/// Create `Length` quantity with `meter` unit
#[inline]
pub fn meter(v: f64) -> Length {
  Length::new::<length::meter>(v)
}

/// Create `Time` quantity with `second` unit
#[inline]
pub fn second(v: f64) -> Time {
  Time::new::<time::second>(v)
}

/// Create `Time` quantity with `microsecond` unit
#[inline]
pub fn microsecond(v: f64) -> Time {
  Time::new::<time::microsecond>(v)
}

/// Create `Time` quantity with `nanosecond` unit
#[inline]
pub fn nanosecond(v: f64) -> Time {
  Time::new::<time::nanosecond>(v)
}

use crate::ast::{ComplexAttri, ComplexParseError, SimpleAttri};
use std::ops::Deref;

/// Valid values are 1ps, 10ps, 100ps, and 1ns. The default is 1ns.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=42.25&end=42.30
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Default, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum TimeUnit {
  /// 1ps, 1e-12
  #[strum(serialize = "1ps")]
  _1ps,
  /// 10ps, 1e-11
  #[strum(serialize = "10ps")]
  _10ps,
  /// 100ps, 1e-10
  #[strum(serialize = "100ps")]
  _100ps,
  /// 1ns, 1e-9
  #[default]
  #[strum(serialize = "1ns")]
  _1ns,
}

impl TimeUnit {
  const LUT: [<Self as Deref>::Target; 4] = [
    // 1ps
    Time {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-12,
    },
    // 10ps
    Time {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-11,
    },
    // 100ps
    Time {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-10,
    },
    // 1ns
    Time {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-9,
    },
  ];
}

impl Deref for TimeUnit {
  type Target = Time;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &Self::LUT[*self as usize]
  }
}

impl SimpleAttri for TimeUnit {}

/// Valid values are 1mV, 10mV, 100mV, and 1V. The default is 1V.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.2&end=43.9
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Default, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum VoltageUnit {
  /// 1mV, 1e-3
  #[strum(serialize = "1mV")]
  _1mV,
  /// 10mV, 1e-2
  #[strum(serialize = "10mV")]
  _10mV,
  /// 100mV, 1e-1
  #[strum(serialize = "100mV")]
  _100mV,
  /// 1V, 1e0
  #[default]
  #[strum(serialize = "1V")]
  _1V,
}

impl VoltageUnit {
  const LUT: [<Self as Deref>::Target; 4] = [
    // 1mV
    ElectricPotential {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-3,
    },
    // 10mV
    ElectricPotential {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-2,
    },
    // 100mV
    ElectricPotential {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-1,
    },
    // 1V
    ElectricPotential {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E0,
    },
  ];
}

impl Deref for VoltageUnit {
  type Target = ElectricPotential;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &Self::LUT[*self as usize]
  }
}

impl SimpleAttri for VoltageUnit {}

/// The valid values are 1uA, 10uA, 100uA, 1mA, 10mA, 100mA, and 1A.
/// **No default exists for the current_unit attribute if the attribute is omitted.**
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.12&end=43.24
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum CurrentUnit {
  /// 1uA, 1e-6
  #[strum(serialize = "1uA")]
  _1uA,
  /// 10uA, 1e-5
  #[strum(serialize = "10uA")]
  _10uA,
  /// 100uA, 1e-4
  #[strum(serialize = "100uA")]
  _100uA,
  /// 1mA, 1e-3
  #[strum(serialize = "1mA")]
  _1mA,
  /// 10mA, 1e-2
  #[strum(serialize = "10mA")]
  _10mA,
  /// 100mA, 1e-1
  #[strum(serialize = "100mA")]
  _100mA,
  /// 1A, 1e0
  #[strum(serialize = "1A")]
  _1A,
}

impl CurrentUnit {
  const LUT: [<Self as Deref>::Target; 7] = [
    // 1uA
    ElectricCurrent {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-6,
    },
    // 10uA
    ElectricCurrent {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-5,
    },
    // 100uA
    ElectricCurrent {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-4,
    },
    // 1mA
    ElectricCurrent {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-3,
    },
    // 10mA
    ElectricCurrent {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-2,
    },
    // 100mA
    ElectricCurrent {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-1,
    },
    // 1A
    ElectricCurrent {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E0,
    },
  ];
}

impl Deref for CurrentUnit {
  type Target = ElectricCurrent;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &Self::LUT[*self as usize]
  }
}

impl SimpleAttri for CurrentUnit {}

/// Valid unit values are 1ohm, 10ohm, 100ohm, and 1kohm.
/// **No default exists for `pulling_resistance_unit` if the attribute is omitted.**
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.25&end=44.4
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum PullingResistanceUnit {
  /// 1ohm, 1
  #[strum(serialize = "1ohm")]
  _1ohm,
  /// 10ohm, 10
  #[strum(serialize = "10ohm")]
  _10ohm,
  /// 100ohm, 100
  #[strum(serialize = "100ohm")]
  _100ohm,
  /// 1kohm, 1000
  #[strum(serialize = "1kohm")]
  _1kohm,
}

impl PullingResistanceUnit {
  const LUT: [<Self as Deref>::Target; 4] = [
    // 1ohm
    ElectricalResistance {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E0,
    },
    // 10ohm
    ElectricalResistance {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E1,
    },
    // 100ohm
    ElectricalResistance {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E2,
    },
    // 1kohm
    ElectricalResistance {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E3,
    },
  ];
}

impl Deref for PullingResistanceUnit {
  type Target = ElectricalResistance;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &Self::LUT[*self as usize]
  }
}

impl SimpleAttri for PullingResistanceUnit {}

/// This attribute specifies the unit for all capacitance
/// values within the logic library, including
/// default capacitances, max_fanout capacitances,
/// pin capacitances, and wire capacitances.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.7&end=44.19
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Default, Clone, Copy)]
pub struct CapacitiveLoadUnit {
  ff_pf: bool,
  _v: Capacitance,
}

// impl CapacitiveLoadUnit {
//   const NIL: Capacitance = Capacitance {
//     dimension: std::marker::PhantomData,
//     units: std::marker::PhantomData,
//     value: 0.0,
//   };
// }

impl Deref for CapacitiveLoadUnit {
  type Target = Capacitance;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self._v
  }
}

impl ComplexAttri for CapacitiveLoadUnit {
  fn parse(v: Vec<&str>) -> Result<Self, ComplexParseError> {
    let mut i = v.into_iter();
    let value: NotNan<f64> = match i.next() {
      Some(s) => match s.parse() {
        Ok(f) => f,
        Err(e) => return Err(ComplexParseError::Float(e)),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    let (ff_pf, _v): (bool, Capacitance) = match i.next() {
      Some(s) => match s {
        "ff" => (true, Capacitance::new::<capacitance::femtofarad>(*value)),
        "pf" => (false, Capacitance::new::<capacitance::picofarad>(*value)),
        _ => return Err(ComplexParseError::UnsupportedWord),
      },
      None => return Err(ComplexParseError::LengthDismatch),
    };
    if let Some(_) = i.next() {
      return Err(ComplexParseError::LengthDismatch);
    }
    Ok(Self { ff_pf, _v })
  }

  fn to_wrapper(&self) -> crate::ast::ComplexWrapper {
    // match self._v {
    //   Some(cap) => {
    let mut buffer = ryu::Buffer::new();
    if self.ff_pf {
      vec![vec![
        buffer.format(self._v.get::<capacitance::femtofarad>()).to_owned(),
        "ff".to_owned(),
      ]]
    } else {
      vec![vec![
        buffer.format(self._v.get::<capacitance::picofarad>()).to_owned(),
        "pf".to_owned(),
      ]]
    }
  }
  //   None => None,
  // }
  // }
}

/// This attribute indicates the units of the power values
/// in the library. If this attribute is missing, the
/// leakage-power values are expressed without units.
///
/// Valid values are 1W, 100mW, 10mW, 1mW, 100nW, 10nW, 1nW, 100pW, 10pW, and 1pW.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.22&end=44.31
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/user_guide.html');
/// </script>
#[derive(Debug, Clone, Copy, Default)]
#[derive(strum_macros::EnumString, strum_macros::Display)]
pub enum LeakagePowerUnit {
  /// No units
  #[strum(serialize = "")]
  #[default]
  None,
  /// 1pW, 1e-12
  #[strum(serialize = "1pW")]
  _1pW,
  /// 10pW, 1e-11
  #[strum(serialize = "10pW")]
  _10pW,
  /// 100pW, 1e-10
  #[strum(serialize = "100pW")]
  _100pW,
  /// 1nW, 1e-9
  #[strum(serialize = "1nW")]
  _1nW,
  /// 10nW, 1e-8
  #[strum(serialize = "10nW")]
  _10nW,
  /// 100nW, 1e-7
  #[strum(serialize = "100nW")]
  _100nW,
  /// 1uW, 1e-6
  #[strum(serialize = "1uW")]
  _1uW,
  /// 10uW, 1e-5
  #[strum(serialize = "10uW")]
  _10uW,
  /// 100uW, 1e-4
  #[strum(serialize = "100uW")]
  _100uW,
  /// 1mW, 1e-3
  #[strum(serialize = "1mW")]
  _1mW,
  /// 10mW, 1e-2
  #[strum(serialize = "10mW")]
  _10mW,
  /// 100mW, 1e-1
  #[strum(serialize = "100mW")]
  _100mW,
  /// 1W, 1e0
  #[strum(serialize = "1W")]
  _1W,
}
impl LeakagePowerUnit {
  const LUT: [<Self as Deref>::Target; 14] = [
    // No Unit
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1.0,
    },
    // 1pW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-12,
    },
    // 10pW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-11,
    },
    // 100pW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-10,
    },
    // 1nW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-9,
    },
    // 10nW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-8,
    },
    // 100nW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-7,
    },
    // 1uW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-6,
    },
    // 10uW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-5,
    },
    // 100uW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-4,
    },
    // 1mW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-3,
    },
    // 10mW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-2,
    },
    // 100mW
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E-1,
    },
    // 1W
    Power {
      dimension: std::marker::PhantomData,
      units: std::marker::PhantomData,
      value: 1E0,
    },
  ];
}

impl Deref for LeakagePowerUnit {
  type Target = Power;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &Self::LUT[*self as usize]
  }
}

impl SimpleAttri for LeakagePowerUnit {}
