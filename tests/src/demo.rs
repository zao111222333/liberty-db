use core::fmt::Display as _;

use liberty_db::{ast::GroupAttri, library::Library};

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
#[test]
fn parse_str() {
  match Library::parse(TEMPLATE) {
    Ok(ref mut library) => {
      library.comments.this.push("line1\nline2".into());
      library.comments.this.push("line3".into());
      println!("{:#?}", library);
      println!("{}", library);
    }
    Err(e) => panic!("{:#?}", e),
  }
}

#[test]
fn parse_error() {
  let s = r#"
  library(some){    
    type :  ;  
  }
  "#;
  match Library::parse(s) {
    Ok(library) => {
      panic!("{}", library);
    }
    Err(e) => println!("{:?}", e),
  }
}

#[test]
fn case_1() {
  let s = r#"library(some){    
    input_voltage(cmos_schmitt) {
         vil : 0.3 * VDD ;
         vih : 0.7 * VDD ;
         vimin : -0.5 ;
         vimax : VDD + 0.5 ;
    }
  }
  "#;
  match Library::parse(s) {
    Ok(library) => {
      println!("{}", library);
    }
    Err(e) => panic!("[ERROR] {}", e),
  }
}

#[test]
fn case_2() {
  let s = r#"library(some){
    resistance : 0.00001 ;
    capacitance : 1 ;
    area : 0
    slope : 0 ;
    fanout_length(1,0.0000)
    fanout_length(2,0.0000)
    fanout_length(3,0.0000)
    fanout_length(4,0.0000)
    fanout_length(5,0.0000)
    fanout_length(6,0.0000)
  }"#;
  match Library::parse(s) {
    Ok(library) => {
      println!("{}", library);
    }
    Err(e) => panic!("[ERROR] {:?}", e),
  }
}

#[test]
fn parse_file() -> anyhow::Result<()> {
  use std::fs::File;
  use std::io::{BufWriter, Write};
  let filepath = "tech/cases/ocv.lib";
  let data = std::fs::read_to_string(filepath).expect("Failed to open file.");
  match liberty_db::library::Library::parse(&data) {
    Ok(library) => {
      let file = File::create("output.lib")?;
      let mut writer = BufWriter::new(file);
      write!(&mut writer, "{}", library)?;
    }
    Err(e) => panic!("[ERROR] {:?}", e),
  }
  Ok(())
}
