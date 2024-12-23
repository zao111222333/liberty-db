use liberty_db::{library::Library, Group};

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
    pin (D) {}
    pin (CK) {}
    pin (SE) {}
    pin (SI) {}
    pin (SN) {}
    pin (RN) {}
  }
}"#;
fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();
  match Library::parse_lib(TEMPLATE) {
    Ok(ref mut library) => {
      library.comments_this_entry().or_insert("line1\nline2".into());
      library.comments_this_entry().and_modify(|s| s.push_str("line3"));
      library.comments_time_unit_entry().or_insert("line4\nline5".into());
      library.comments_time_unit_entry().and_modify(|s| s.push_str("line6"));
      println!("{library}");
      println!("\niteration cell");
      for cell in &library.cell {
        println!("{}", cell.display());
      }
      println!("\nindex cell");
      if let Some(sdffrs_x2) = library.cell.get("SDFFRS_X2") {
        println!("{}", sdffrs_x2.display());
      }
      println!("borrow index cell");
      if let Some(sdffrs_x2) = library.cell.get("SDFFRS_X2") {
        println!("{}", sdffrs_x2.display());
      }
    }
    Err(e) => panic!("{e:#?}"),
  }
}
