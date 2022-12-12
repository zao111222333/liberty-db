/// Error for Unit
#[derive(Debug, Copy, Clone)]
pub enum Error{
    /// Prefix Convert Error
    PrefixConvert
}

/// ## Metric Prefix
/// 
/// The BIPM specifies twenty-four prefixes for the International System of Units (SI).
/// 
/// NOT implement `Deca (10e1)`, due to the symbol of it is `"da"`, can not use char to express.
/// 
/// [reference (wiki)]
/// 
/// [reference (wiki)]: https://en.wikipedia.org/wiki/International_System_of_Units#Prefixes
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Prefix {
    /// 10e30
    Quetta = b'Q',
    /// 10e27
    Ronna = b'R', 
    /// 10e24
    Yotta = b'Y', 
    /// 10e21
    Zetta = b'Z',
    /// 10e18
    Exa = b'E',
    /// 10e15
    Peta = b'P',
    /// 10e12
    Tera = b'T',
    /// 10e9
    Giga = b'G',
    /// 10e6
    Mega = b'M',
    /// 10e3
    Kilo = b'k',
    /// 10e2
    Hecto = b'h',
    // Deca 10e1 da,
    /// 10e0
    _Unit = b'\0',
    /// 10e-1
    Deci = b'd',
    /// 10e-2
    Centi = b'c',
    /// 10e-3
    Milli = b'm',
    /// 10e-6
    Micro = b'u',
    /// 10e-9
    Nano = b'n',
    /// 10e-12
    Pico = b'p',
    /// 10e-15
    Femto = b'f',
    /// 10e-18
    Atto = b'a',
    /// 10e-21
    Zepto = b'z',
    /// 10e-24
    Yocto = b'y',
    /// 10e-27
    Ronto = b'r',
    /// 10e-30
    Quecto = b'q',
}
/// e.g. Prefix::Kilo.into() == 'k'
impl Into<char> for Prefix {
    #[inline]
    fn into(self) -> char {
        self as u8 as char
    }
}
impl Into<f64> for Prefix {
    #[inline]
    fn into(self) -> f64 {
        match self {
            Prefix::Quetta=> 1e30,
            Prefix::Ronna=> 1e27,
            Prefix::Yotta=> 1e24,
            Prefix::Zetta=> 1e21,
            Prefix::Exa=> 1e18,
            Prefix::Peta=> 1e15,
            Prefix::Tera=> 1e12,
            Prefix::Giga=> 1e9,
            Prefix::Mega=> 1e6,
            Prefix::Kilo=> 1e3,
            Prefix::Hecto=> 1e2,
            Prefix::_Unit=> 1e0,
            Prefix::Deci=> 1e-1,
            Prefix::Centi=> 1e-2,
            Prefix::Milli=> 1e-3,
            Prefix::Micro=> 1e-6,
            Prefix::Nano=> 1e-9,
            Prefix::Pico=> 1e-12,
            Prefix::Femto=> 1e-15,
            Prefix::Atto=> 1e-18,
            Prefix::Zepto=> 1e-21,
            Prefix::Yocto=> 1e-24,
            Prefix::Ronto=> 1e-27,
            Prefix::Quecto=> 1e-30,
        }
    }
}

impl Prefix {
    /// e.g. Prefix::from('k') == Ok(Prefix::Kilo)
    #[inline]
    pub fn from(c: char) -> Result<Prefix, Error> {
        match c {
            'Q' => Ok(Prefix::Quetta),
            'R' => Ok(Prefix::Ronna),
            'Y' => Ok(Prefix::Yotta),
            'Z' => Ok(Prefix::Zetta),
            'E' => Ok(Prefix::Exa),
            'P' => Ok(Prefix::Peta),
            'T' => Ok(Prefix::Tera),
            'G' => Ok(Prefix::Giga),
            'M' => Ok(Prefix::Mega),
            'k' => Ok(Prefix::Kilo),
            'h' => Ok(Prefix::Hecto),
            '\0' => Ok(Prefix::_Unit),
            'd' => Ok(Prefix::Deci),
            'c' => Ok(Prefix::Centi),
            'm' => Ok(Prefix::Milli),
            'u' => Ok(Prefix::Micro),
            'n' => Ok(Prefix::Nano),
            'p' => Ok(Prefix::Pico),
            'f' => Ok(Prefix::Femto),
            'a' => Ok(Prefix::Atto),
            'z' => Ok(Prefix::Zepto),
            'y' => Ok(Prefix::Yocto),
            'r' => Ok(Prefix::Ronto),
            'q' => Ok(Prefix::Quecto),
            _ => Err(Error::PrefixConvert),
        }
    }
}

impl Default for Prefix {
    #[inline]
    fn default() -> Self { Prefix::_Unit}
}

/// 
#[derive(Debug,Copy,Clone)]
#[allow(non_camel_case_types)]
pub enum Suffix {
    // time  - second
    // s,
    // R,
    // US,
    // NS,
    // PS,
    // FS,
}


/// # Unit structre for liberty
/// 
/// Reference:
///  <iframe
///   src="reference/liberty07_03.pdf#page=4"
///   frameBorder="0"
///   scrolling="auto"
///   height="600px"
///   width="100%"
/// ></iframe>
/// 
/// ## Example:
/// 
/// ```rust
/// use liberty_db;
/// let result = liberty_db::add(2, 2);
/// ``` 
#[derive(Debug,Copy,Clone)]
pub struct Unit {
    /// Scale of that unit, eg. Km.scale = 1000 
    pub scale: f64,
    /// 
    pub prefix: Prefix,
    /// 
    pub suffix: Suffix,
}

#[cfg(test)]
mod test {
    mod prefix {
        use std::slice::Iter;
        use crate::unit::Prefix;
        impl Prefix {
            fn iter() -> Iter<'static, Prefix> {
                static PREFIXS: [Prefix; 24] = 
                [
                    Prefix::Quetta,
                    Prefix::Ronna,
                    Prefix::Yotta,
                    Prefix::Zetta,
                    Prefix::Exa,
                    Prefix::Peta,
                    Prefix::Tera,
                    Prefix::Giga,
                    Prefix::Mega,
                    Prefix::Kilo,
                    Prefix::Hecto,
                    Prefix::_Unit,
                    Prefix::Deci,
                    Prefix::Centi,
                    Prefix::Milli,
                    Prefix::Micro,
                    Prefix::Nano,
                    Prefix::Pico,
                    Prefix::Femto,
                    Prefix::Atto,
                    Prefix::Zepto,
                    Prefix::Yocto,
                    Prefix::Ronto,
                    Prefix::Quecto,
                ];
                PREFIXS.iter()
            }
        }
        #[test]
        fn convert() {
            for prefix in Prefix::iter(){
                let b: char = prefix.clone().into();
                match Prefix::from(b) {
                    Ok(new_prefix) => { assert_eq!(new_prefix,prefix.clone()); }
                    Err(e) => { panic!("{:?}",e) }
                }
            }

        }
    }
}