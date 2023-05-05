//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
use nom::error::ErrorKind;

use crate::ast::{ComplexAttri, HashedGroup, IdxError};
// use crate::types::*;
use crate::units;
use crate::cell::Cell;
use std::collections::HashMap;
#[derive(Debug,Default)]
#[derive(liberty_macros::NameIdx)]
#[derive(liberty_macros::GroupHashed)]
pub struct Library{
    #[idx_len(any)]
    _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
    _undefined: crate::ast::UndefinedAttributes,
    // #[arrti_type(simple)]
    pub time_unit: units::Time,
    // #[arrti_type(complex)]
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
    pub pin_names: Vec<crate::pin::PinIdx>,
    pub vector: Vector,
}

impl ComplexAttri for Vec<crate::pin::PinIdx> {
    type Error=std::fmt::Error;

    fn parse(v: &Vec<Vec<&str>>)->Result<Self,Self::Error> {
        todo!()
    }

    fn to_wrapper(&self) -> crate::ast::ComplexWrapper {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}

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
        Ok((_,Err(e))) => return Err(ParserError::IdxError(line_num, e)),
        Ok((_,Ok(l))) => return Ok(l),
    }
    }else{
      Err(ParserError::Other(line_num, format!("Need key=library, find={}",key)))
    }
  }
}

mod test{
    use super::*;
    // TODO: 
    // 1. support comment
    // 2. support multi-line `\`
    static  TEMPLATE: &str = r#"
  library(gscl45nm) {
  
    delay_model : table_lookup;
    in_place_swap_mode : match_footprint;
  
    time_unit : "1ns";
    voltage_unit : "1V";
    current_unit : "1uA";
    pulling_resistance_unit : "1kohm";
    leakage_power_unit : "1nW";
    capacitive_load_unit (1,pf);
  
    slew_upper_threshold_pct_rise : 80;
    slew_lower_threshold_pct_rise : 20;
    slew_upper_threshold_pct_fall : 80;
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
      println!("{:?}", Library::parse(TEMPLATE));
      // println!("{:?}", Library::parse(TEMPLATE_ERR1));
      // println!("{:?}", Library::parse(TEMPLATE_ERR2));
    }
  }