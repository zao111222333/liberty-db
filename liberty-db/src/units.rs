use strum_macros::{Display, EnumString};

/// Error for Unit
#[derive(Debug, Copy, Clone)]
pub enum Error{
    /// Prefix Convert Error
    PrefixConvert(strum::ParseError)
}

/// ## Metric Prefix
/// The BIPM specifies twenty-four prefixes for the International System of Units (SI).
/// [reference (wiki)]
/// [reference (wiki)]: https://en.wikipedia.org/wiki/International_System_of_Units#Prefixes
/// https://docs.rs/strum_macros/latest/strum_macros/derive.EnumString.html
#[derive(Debug, Clone, PartialEq)]
#[derive(Default, Display, EnumString)]
pub enum Prefix {
    /// Q - 10e30
    #[strum(serialize = "Q")]
    Quetta = 30,
    /// R - 10e27
    #[strum(serialize = "R")]
    Ronna = 27,
    /// Y - 10e24
    #[strum(serialize = "Y")]
    Yotta = 24, 
    /// Z - 10e21
    #[strum(serialize = "Z")]
    Zetta = 21,
    /// E - 10e18
    #[strum(serialize = "E")]
    Exa = 18,
    /// P - 10e15
    #[strum(serialize = "P")]
    Peta = 15,
    /// T - 10e12
    #[strum(serialize = "T")]
    Tera = 12,
    /// G - 10e9
    #[strum(serialize = "G")]
    Giga = 9,
    /// M - 10e6
    #[strum(serialize = "M")]
    Mega = 6,
    /// k - 10e3
    #[strum(serialize = "k")]
    Kilo = 3,
    /// h - 10e2
    #[strum(serialize = "h")]
    Hecto = 2,
    /// da - 10e1
    #[strum(serialize = "da")]
    Deca = 1,
    /// "\0" - 10e0
    #[default]
    #[strum(serialize = "\0")]
    _Unit = 0,
    /// d - 10e-1
    #[strum(serialize = "d")]
    Deci = -1,
    /// c - 10e-2
    #[strum(serialize = "c")]
    Centi = -2,
    /// m - 10e-3
    #[strum(serialize = "m")]
    Milli = -3,
    /// u - 10e-6
    #[strum(serialize = "u")]
    Micro = -6,
    /// n - 10e-9
    #[strum(serialize = "n")]
    Nano = -9,
    /// p - 10e-12
    #[strum(serialize = "p")]
    Pico = -12,
    /// f - 10e-15
    #[strum(serialize = "f")]
    Femto = -15,
    /// a - 10e-18
    #[strum(serialize = "a")]
    Atto = -18,
    /// z - 10e-21
    #[strum(serialize = "z")]
    Zepto = -21,
    /// y - 10e-24
    #[strum(serialize = "y")]
    Yocto = -24,
    /// r - 10e-27
    #[strum(serialize = "r")]
    Ronto = -27,
    /// q - 10e-30
    #[strum(serialize = "q")]
    Quecto = -30,
}
impl Prefix {
    pub(crate) fn from_symbol(s: &str) -> Result<Prefix, Error> {
        use std::str::FromStr;
        match Self::from_str(s) {
            Ok(prefix) => { Ok(prefix) }
            Err(e) => { Err(Error::PrefixConvert(e)) }
        }
    }
    
}

#[derive(Debug, Copy, Clone)]
#[derive(Default, Display, EnumString)]
pub enum Suffix {
    /// Scalar
    #[default]
    #[strum(serialize = "")]
    Scalar,
    /// Current 
    #[strum(serialize = "A")]
    Ampere,
    /// Voltage
    #[strum(serialize = "V")]
    Volt,
    /// Power
    #[strum(serialize = "W")]
    Watt,
    /// Length
    #[strum(serialize = "m",serialize = "M")]
    Metre,
    /// Time
    Second,
    /// Temperature
    DegreeCelsius,
    /// Capacity
    Farad,
    /// Engery
    Joule,
    /// Resistence
    Ohm,
}


/// # Unit structre for liberty
#[derive(Debug, Clone)]
pub struct Unit {
    /// Scale of that unit, eg. Km.scale = 1000 
    pub scale: f64,
    pub prefix: Prefix,
    pub suffix: Suffix,
}

impl Unit {

}

/// All units for liberty
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct Units{
    pub time_unit: Unit,
    pub capacitance_unit: Unit,
    pub voltage_unit: Unit,
    pub resistance_unit: Unit,
    pub pulling_resistance_unit: Unit,
    pub current_unit: Unit,
    pub power_unit: Unit,
    pub distance_unit: Unit,
    pub scalar_unit: Unit,
}
impl Units {
}

#[cfg(test)]
mod test {
    mod prefix {
        use crate::units::Prefix;
        static SYMBOLS_PREFIXS: [(&str,Prefix); 25] = 
        [
            ("Q",  Prefix::Quetta ),
            ("R",  Prefix::Ronna  ),
            ("Y",  Prefix::Yotta  ),
            ("Z",  Prefix::Zetta  ),
            ("E",  Prefix::Exa    ),
            ("P",  Prefix::Peta   ),
            ("T",  Prefix::Tera   ),
            ("G",  Prefix::Giga   ),
            ("M",  Prefix::Mega   ),
            ("k",  Prefix::Kilo   ),
            ("h",  Prefix::Hecto  ),
            ("da", Prefix::Deca   ),
            ("\0", Prefix::_Unit  ),
            ("d",  Prefix::Deci   ),
            ("c",  Prefix::Centi  ),
            ("m",  Prefix::Milli  ),
            ("u",  Prefix::Micro  ),
            ("n",  Prefix::Nano   ),
            ("p",  Prefix::Pico   ),
            ("f",  Prefix::Femto  ),
            ("a",  Prefix::Atto   ),
            ("z",  Prefix::Zepto  ),
            ("y",  Prefix::Yocto  ),
            ("r",  Prefix::Ronto  ),
            ("q",  Prefix::Quecto ),
        ];
        #[test]
        fn convert() {
            for (symbol,prefix) in SYMBOLS_PREFIXS.iter(){
                println!("{:?} <-> {:?}",symbol, prefix);
                match Prefix::from_symbol(symbol) {
                    Ok(new_prefix) => { assert_eq!(new_prefix,prefix.clone()); }
                    Err(e) => { panic!("{:?}",e) }
                }
            }
        }
    }
}

