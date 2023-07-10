static TEMPLATE: &str = r#"
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
static TEMPLATE_ERR1: &str = r#"
    library (/home/jzzhou/project/liberate_demo/LIBRARY/debug_liberate_INV0_hspice) {
    comment : "www";
    }
    *"#;
static TEMPLATE_ERR2: &str = r#"/*
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
pub fn demo() {
  use liberty_db::{
    ast::{CodeFormatter, GroupAttri},
    library::Library,
  };

  match Library::parse(TEMPLATE) {
    Ok(library) => {
      println!("{:#?}", library);
      let mut output = String::new();
      let mut f = CodeFormatter::new(&mut output, "| ");
      if let Err(e) = library.fmt_liberty("library", &mut f) {
        panic!("");
      }
      println!("{}", output);
    }
    Err(e) => println!("{:#?}", e),
  }
}
