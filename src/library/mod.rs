//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

mod items;
// pub use items::*;

use crate::ast::HashedGroup;
use crate::pin::Pin;
use crate::units;
use crate::cell::Cell;
use std::collections::{HashMap, HashSet};
#[derive(Debug,derivative::Derivative)]
#[derivative(Default)]
#[derive(liberty_macros::Group)]
pub struct Library{
  #[id_len(-1)]
  _id: <Self as crate::ast::HashedGroup>::Id,
  _undefined: crate::ast::AttributeList,
  /// Valid values are 1ps, 10ps, 100ps, and 1ns. The default is 1ns.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=42.25&end=42.30
  /// ">Reference</a>
  #[arrti_type(simple)]
  pub time_unit: units::TimeUnit,
  /// This attribute specifies the unit for all capacitance 
  /// values within the logic library, including 
  /// default capacitances, max_fanout capacitances, 
  /// pin capacitances, and wire capacitances.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.7&end=44.19
  /// ">Reference</a>
  #[arrti_type(complex)]
  pub capacitive_load_unit: units::CapacitiveLoadUnit,
  /// Valid values are 1mV, 10mV, 100mV, and 1V. The default is 1V.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.2&end=43.9
  /// ">Reference</a>
  #[arrti_type(simple)]
  pub voltage_unit: units::VoltageUnit,
  /// The valid values are 1uA, 10uA, 100uA, 1mA, 10mA, 100mA, and 1A. 
  /// **No default exists for the `current_unit` attribute if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.12&end=43.24
  /// ">Reference</a>
  #[arrti_type(simple)]
  pub current_unit: Option<units::CurrentUnit>,
  /// Valid unit values are 1ohm, 10ohm, 100ohm, and 1kohm. 
  /// **No default exists for `pulling_resistance_unit` if the attribute is omitted.**
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=43.25&end=44.4
  /// ">Reference</a>
  #[arrti_type(simple)]
  pub pulling_resistance_unit: Option<units::PullingResistanceUnit>,
  /// This attribute indicates the units of the power values 
  /// in the library. If this attribute is missing, the 
  /// leakage-power values are expressed without units.
  /// Valid values are 1W, 100mW, 10mW, 1mW, 100nW, 10nW, 1nW, 100pW, 10pW, and 1pW.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=44.22&end=44.31
  /// ">Reference</a>
  #[arrti_type(simple)]
  pub leakage_power_unit: Option<units::LeakagePowerUnit>,
  // pub distance_unit: units::Length,
  // pub scalar_unit: units::Ratio,
  #[arrti_type(simple)]
  #[derivative(Default(value = "80.0"))]
  pub slew_upper_threshold_pct_rise: f64,
  #[arrti_type(group)]
  pub cell: HashSet<Cell>,
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
    pub pin_names: Vec<<Pin as HashedGroup>::Id>,
    pub vector: Vector,
}

// impl ComplexAttri for Vec<crate::pin::PinIdx> {
//     type Error=std::fmt::Error;

//     fn parse(v: Vec<&str>)->Result<Self,Self::Error> {
//         todo!()
//     }

//     fn to_wrapper(&self) -> Option<crate::ast::ComplexWrapper> {
//         todo!()
//     }
// }

#[derive(Debug, Clone)]
pub struct Vector{
  pub id: usize,
  pub string: String,
}



use crate::ast::{GroupAttri,ParserError};
use crate::ast::parser;
impl Library {
  pub fn parse<'a>(i: &'a str) -> Result<Self,ParserError<'a>> {
    let mut line_num =1;
    let input = match parser::comment_space_newline(i){
        Ok((input,n)) => {line_num+=n;input},
        Err(e) => return Err(ParserError::NomError(line_num, e)),
    };
    let (input,key) = match parser::key::<nom::error::Error<&str>>(input){
        Ok(res) => res,
        Err(e) => return Err(ParserError::NomError(line_num, e)),
    };
    if key=="library"{
      match <Self as GroupAttri>::nom_parse(input,&mut line_num){
        Err(e) => return Err(ParserError::NomError(line_num, e)),
        Ok((_,Err(e))) => return Err(ParserError::IdError(line_num, e)),
        Ok((_,Ok(l))) => return Ok(l),
    }
    }else{
      Err(ParserError::Other(line_num, format!("Need key=library, find={}",key)))
    }
  }
}

mod test{
    use super::*;
    
    static  TEMPLATE: &str = r#"
  library(gscl45nm) {
  
    delay_model : table_lookup;
    in_place_swap_mode : match_footprint;
  
    time_unit : "1ps";
    voltage_unit : "10mV";
    current_unit : "1uA";
    pulling_resistance_unit : "1kohm";
    leakage_power_unit : "1nW";
    capacitive_load_unit (1,pf);
  
    slew_upper_threshold_pct_rise : 80;
    slew_lower_threshold_pct_rise : 20;
    slew_upper_threshold_pct_fall : 70;
    slew_lower_threshold_pct_fall : 20;
    input_threshold_pct_rise : 50;
    input_threshold_pct_fall : 50;
    output_threshold_pct_rise : 50;
    output_threshold_pct_fall : 50;
    nom_process : 1;
    nom_voltage : 1.1;
    nom_temperature : 27;
    operating_conditions ( typical ) {
        process : 1;
        voltage : 1.1;
        temperature : 27;
    }
    default_operating_conditions : typical;
  
    lu_table_template(delay_template_4x5) {
      variable_1 : total_output_net_capacitance;
      variable_2 : input_net_transition;
      index_1 ("1000.0, 1001.0, 1002.0, 1003.0");
      index_2 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0");
    }

      cell (SDFFRS_X2) {
    
    
      ff ("IQ","IQN") {
        next_state         	: "((SE * SI) + (D * !SE))";
        clocked_on         	: "CK";
        preset             	: "!SN";
        clear              	: "!RN";
        clear_preset_var1  	: L;
        clear_preset_var2  	: L;
      }
      }
  }
    "#;
    static  TEMPLATE_ERR1: &str = r#"
    library (/home/jzzhou/project/liberate_demo/LIBRARY/debug_liberate_INV0_hspice) {
      comment : "www";
    }
    *"#;
    static  TEMPLATE_ERR2: &str = r#"/*
  SPDX-FileCopyrightText: 2022 Thomas Kramer <code@tkramer.ch>
  SPDX-FileCopyrightText: 2011 W. Rhett Davis, and Harun Demircioglu, North Carolina State University
  SPDX-FileCopyrightText: 2008 W. Rhett Davis, Michael Bucher, and Sunil Basavarajaiah, North Carolina State University (ncsu_basekit subtree), James Stine, and Ivan Castellanos, and Oklahoma State University (osu_soc subtree)
  SPDX-FileCopyrightText: 2007 W. Rhett Davis, Paul Franzon, Michael Bucher, and Sunil Basavarajaiah, North Carolina State University
  
  SPDX-License-Identifier: Apache-2.0
  */
  
  /*
   Test liberty file for the liberty parser.
   This is copied from the FreePDK45 but stripped down to save space.
  */
        
  /*
   delay model :       typ
   check model :       typ
   power model :       typ
   capacitance model : typ
   other model :       typ
  */
  library(gscl45nm) {}
    "#;
    #[test]
    fn x6() {
      match Library::parse(TEMPLATE){
        Ok(library) => {
          println!("{:#?}", library);
          let mut output = String::new();
          let mut f = crate::ast::CodeFormatter::new(&mut output , "| ");
          if let Err(e) = library.fmt_liberty("library", &mut f){
              panic!("");
          }
          println!("{}",output);
        }
        Err(e) => println!("{:#?}", e),
      }
    }
  }