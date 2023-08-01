//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

mod items;

use crate::ast::{AttributeList, GroupId, GroupMap, HashedGroup};
use crate::cell::Cell;
use crate::pin::Pin;
use crate::units;
use std::collections::HashMap;
#[derive(Debug, derivative::Derivative)]
#[derivative(Default)]
#[derive(liberty_macros::Group)]
pub struct Library {
  #[liberty(id(auto_impl_len = 1))]
  _id: GroupId<Self>,
  #[liberty(undefined)]
  _undefined: AttributeList,
  /// Valid values are 1ps, 10ps, 100ps, and 1ns. The default is 1ns.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=42.25&end=42.30
  /// ">Reference</a>
  #[liberty(simple)]
  pub time_unit: units::TimeUnit,
  /// This attribute specifies the unit for all capacitance
  /// values within the logic library, including
  /// default capacitances, max_fanout capacitances,
  /// pin capacitances, and wire capacitances.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.7&end=44.19
  /// ">Reference</a>
  #[liberty(complex(type=Option))]
  pub capacitive_load_unit: Option<units::CapacitiveLoadUnit>,
  /// Valid values are 1mV, 10mV, 100mV, and 1V. The default is 1V.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.2&end=43.9
  /// ">Reference</a>
  #[liberty(simple)]
  pub voltage_unit: units::VoltageUnit,
  /// The valid values are 1uA, 10uA, 100uA, 1mA, 10mA, 100mA, and 1A.
  /// **No default exists for the `current_unit` attribute if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.12&end=43.24
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub current_unit: Option<units::CurrentUnit>,
  /// Valid unit values are 1ohm, 10ohm, 100ohm, and 1kohm.
  /// **No default exists for `pulling_resistance_unit` if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.25&end=44.4
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub pulling_resistance_unit: Option<units::PullingResistanceUnit>,
  /// This attribute indicates the units of the power values
  /// in the library. If this attribute is missing, the
  /// leakage-power values are expressed without units.
  /// Valid values are 1W, 100mW, 10mW, 1mW, 100nW, 10nW, 1nW, 100pW, 10pW, and 1pW.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.22&end=44.31
  /// ">Reference</a>
  #[liberty(simple(type=Option))]
  pub leakage_power_unit: Option<units::LeakagePowerUnit>,
  #[liberty(simple)]
  #[derivative(Default(value = "80.0"))]
  pub slew_upper_threshold_pct_rise: f64,
  #[liberty(group(type=Map))]
  pub cell: GroupMap<Cell>,
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
pub struct Sensitization {
  pub group_name: String,
  pub pin_names: Vec<<Pin as HashedGroup>::Id>,
  pub vector: Vector,
}

#[derive(Debug, Clone)]
pub struct Vector {
  pub id: usize,
  pub string: String,
}

use crate::ast::parser;
use crate::ast::{GroupAttri, ParserError};
impl Library {
  /// Parse `.lib` file as a [Library] struct.
  pub fn parse<'a>(i: &'a str) -> Result<Self, ParserError<'a>> {
    let mut line_num = 1;
    let input = match parser::comment_space_newline(i) {
      Ok((input, n)) => {
        line_num += n;
        input
      }
      Err(e) => return Err(ParserError::NomError(line_num, e)),
    };
    let (input, key) = match parser::key::<nom::error::Error<&str>>(input) {
      Ok(res) => res,
      Err(e) => return Err(ParserError::NomError(line_num, e)),
    };
    if key == "library" {
      match <Self as GroupAttri>::nom_parse(input, &mut line_num) {
        Err(e) => return Err(ParserError::NomError(line_num, e)),
        Ok((_, Err(e))) => return Err(ParserError::IdError(line_num, e)),
        Ok((_, Ok(l))) => return Ok(l),
      }
    } else {
      Err(ParserError::Other(line_num, format!("Need key=library, find={}", key)))
    }
  }
}
