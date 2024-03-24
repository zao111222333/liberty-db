use liberty_db::{ast::GroupAttri, common::items::TableLookUp, library::Library};

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
      library.comment_mut().push("line1\nline2".to_owned());
      library.comment_mut().push("line3".to_owned());
      println!("{:#?}", library);
      let mut output = String::new();
      if let Err(e) = library.fmt(&mut output) {
        panic!("{e}");
      }
      println!("{}", output);
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
      let mut output = String::new();
      if let Err(e) = library.fmt(&mut output) {
        panic!("{e}");
      }
      println!("{}", output);
    }
    Err(e) => panic!("[ERROR] {}", e),
  }
}

#[test]
fn case_1() {
  let s = r#"
  library(some){    
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
      let mut output = String::new();
      if let Err(e) = library.fmt(&mut output) {
        panic!("{e}");
      }
      println!("{}", output);
    }
    Err(e) => panic!("[ERROR] {}", e),
  }
}

#[test]
fn case_2() {
  let s = r#"
  library(some){
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
  }  "#;
  match Library::parse(s) {
    Ok(library) => {
      let mut output = String::new();
      if let Err(e) = library.fmt(&mut output) {
        panic!("{e}");
      }
      println!("{}", output);
    }
    Err(e) => panic!("[ERROR] {:?}", e),
  }
}

mod lvf {
  use std::fs::File;
  use std::fs::OpenOptions;
  use std::io::BufWriter;
  use std::path::Path;

  use liberty_db::common::items::TableLookUp;

  #[test]
  fn parse_file() -> anyhow::Result<()> {
    let file_dir = "/OPT/tech/tsmc/22nm/tcbn22ullbwp30p140_110b/AN61001_20201222/TSMCHOME/digital/Front_End/LVF/CCS/tcbn22ullbwp30p140_110b";
    let out_dir = "tcbn22ullbwp30p140_hm_lvf_p_ccs";

    let infos = [
      (["ffg", "0.88", "0", "p"], "tcbn22ullbwp30p140ffg0p88v0c_hm_lvf_p_ccs.lib"),
      (["ffg", "0.88", "0", "raw_p"], "tcbn22ullbwp30p140ffg0p88v0c_hm_lvf_raw_p_ccs.lib"),
      (["ffg", "0.88", "125", "p"], "tcbn22ullbwp30p140ffg0p88v125c_hm_lvf_p_ccs.lib"),
      (["ffg", "0.88", "125", "raw_p"], "tcbn22ullbwp30p140ffg0p88v125c_hm_lvf_raw_p_ccs.lib"),
      (["ffg", "0.88", "-40", "p"], "tcbn22ullbwp30p140ffg0p88vm40c_hm_lvf_p_ccs.lib"),
      (["ffg", "0.88", "-40", "raw_p"], "tcbn22ullbwp30p140ffg0p88vm40c_hm_lvf_raw_p_ccs.lib"),
      (["ffg", "0.99", "0", "p"], "tcbn22ullbwp30p140ffg0p99v0c_hm_lvf_p_ccs.lib"),
      (["ffg", "0.99", "0", "raw_p"], "tcbn22ullbwp30p140ffg0p99v0c_hm_lvf_raw_p_ccs.lib"),
      (["ffg", "0.99", "125", "p"], "tcbn22ullbwp30p140ffg0p99v125c_hm_lvf_p_ccs.lib"),
      (["ffg", "0.99", "125", "raw_p"], "tcbn22ullbwp30p140ffg0p99v125c_hm_lvf_raw_p_ccs.lib"),
      (["ffg", "0.99", "-40", "p"], "tcbn22ullbwp30p140ffg0p99vm40c_hm_lvf_p_ccs.lib"),
      (["ffg", "0.99", "-40", "raw_p"], "tcbn22ullbwp30p140ffg0p99vm40c_hm_lvf_raw_p_ccs.lib"),
      (["ssg", "0.72", "0", "p"], "tcbn22ullbwp30p140ssg0p72v0c_hm_lvf_p_ccs.lib"),
      (["ssg", "0.72", "0", "raw_p"], "tcbn22ullbwp30p140ssg0p72v0c_hm_lvf_raw_p_ccs.lib"),
      (["ssg", "0.72", "125", "p"], "tcbn22ullbwp30p140ssg0p72v125c_hm_lvf_p_ccs.lib"),
      (["ssg", "0.72", "125", "raw_p"], "tcbn22ullbwp30p140ssg0p72v125c_hm_lvf_raw_p_ccs.lib"),
      (["ssg", "0.72", "-40", "p"], "tcbn22ullbwp30p140ssg0p72vm40c_hm_lvf_p_ccs.lib"),
      (["ssg", "0.72", "-40", "raw_p"], "tcbn22ullbwp30p140ssg0p72vm40c_hm_lvf_raw_p_ccs.lib"),
      (["ssg", "0.81", "0", "p"], "tcbn22ullbwp30p140ssg0p81v0c_hm_lvf_p_ccs.lib"),
      (["ssg", "0.81", "0", "raw_p"], "tcbn22ullbwp30p140ssg0p81v0c_hm_lvf_raw_p_ccs.lib"),
      (["ssg", "0.81", "125", "p"], "tcbn22ullbwp30p140ssg0p81v125c_hm_lvf_p_ccs.lib"),
      (["ssg", "0.81", "125", "raw_p"], "tcbn22ullbwp30p140ssg0p81v125c_hm_lvf_raw_p_ccs.lib"),
      (["ssg", "0.81", "-40", "p"], "tcbn22ullbwp30p140ssg0p81vm40c_hm_lvf_p_ccs.lib"),
      (["ssg", "0.81", "-40", "raw_p"], "tcbn22ullbwp30p140ssg0p81vm40c_hm_lvf_raw_p_ccs.lib"),
      (["tt", "0.8", "25", "p"], "tcbn22ullbwp30p140tt0p8v25c_hm_lvf_p_ccs.lib"),
      (["tt", "0.8", "25", "raw_p"], "tcbn22ullbwp30p140tt0p8v25c_hm_lvf_raw_p_ccs.lib"),
      (["tt", "0.8", "85", "p"], "tcbn22ullbwp30p140tt0p8v85c_hm_lvf_p_ccs.lib"),
      (["tt", "0.8", "85", "raw_p"], "tcbn22ullbwp30p140tt0p8v85c_hm_lvf_raw_p_ccs.lib"),
      (["tt", "0.9", "25", "p"], "tcbn22ullbwp30p140tt0p9v25c_hm_lvf_p_ccs.lib"),
      (["tt", "0.9", "25", "raw_p"], "tcbn22ullbwp30p140tt0p9v25c_hm_lvf_raw_p_ccs.lib"),
      (["tt", "0.9", "85", "p"], "tcbn22ullbwp30p140tt0p9v85c_hm_lvf_p_ccs.lib"),
      (["tt", "0.9", "85", "raw_p"], "tcbn22ullbwp30p140tt0p9v85c_hm_lvf_raw_p_ccs.lib"),
    ];
    std::fs::create_dir_all(Path::new(out_dir).join("delay"))?;
    std::fs::create_dir_all(Path::new(out_dir).join("transition"))?;

    let csv_writer = |s: &str| -> anyhow::Result<[csv::Writer<BufWriter<File>>; 2]> {
      let w_delay = csv::Writer::from_writer(BufWriter::new(
        OpenOptions::new()
          .create(true)
          .append(true)
          .open(Path::new(out_dir).join("delay").join(format!("{s}.csv")))?,
      ));
      let w_transition = csv::Writer::from_writer(BufWriter::new(
        OpenOptions::new()
          .create(true)
          .append(true)
          .open(Path::new(out_dir).join("transition").join(format!("{s}.csv")))?,
      ));
      Ok([w_delay, w_transition])
    };
    let [mut w_info_delay, mut w_info_transition] = csv_writer("info")?;
    let [mut w_slew_delay, mut w_slew_transition] = csv_writer("slew")?;
    let [mut w_load_delay, mut w_load_transition] = csv_writer("load")?;
    let [mut w_nominal_delay, mut w_nominal_transition] = csv_writer("nominal")?;
    let [mut w_mean_delay, mut w_mean_transition] = csv_writer("mean")?;
    let [mut w_std_dev_delay, mut w_std_dev_transition] = csv_writer("std_dev")?;
    let [mut w_skewness_delay, mut w_skewness_transition] = csv_writer("skewness")?;
    for (info, file_name) in infos {
      println!("start process {}", file_name);
      let data: String = std::fs::read_to_string(Path::new(file_dir).join(file_name))?;
      process_one_file(
        info,
        data,
        [
          [&mut w_info_delay, &mut w_info_transition],
          [&mut w_slew_delay, &mut w_slew_transition],
          [&mut w_load_delay, &mut w_load_transition],
          [&mut w_nominal_delay, &mut w_nominal_transition],
          [&mut w_mean_delay, &mut w_mean_transition],
          [&mut w_std_dev_delay, &mut w_std_dev_transition],
          [&mut w_skewness_delay, &mut w_skewness_transition],
        ],
      )?;
    }
    Ok(())
  }

  fn process_one_file(
    info: [&str; 4],
    data: String,
    writers: [[&mut csv::Writer<BufWriter<File>>; 2]; 7],
  ) -> anyhow::Result<()> {
    let [[w_info_delay, w_info_transition], [w_slew_delay, w_slew_transition], [w_load_delay, w_load_transition], [w_nominal_delay, w_nominal_transition], [w_mean_delay, w_mean_transition], [w_std_dev_delay, w_std_dev_transition], [w_skewness_delay, w_skewness_transition]] =
      writers;
    match liberty_db::library::Library::parse(&data) {
      Ok(library) => {
        // write!(&mut writer, "{:#?}", library)?;
        for (cell_name, cell) in library.cell.iter() {
          for (pin_name, pin) in cell.pin.iter() {
            for (timing_id, timing) in pin.timing.iter() {
              let get_info = |arc: &str| {
                let mut v: Vec<String> = info.iter().map(|&s| s.to_string()).collect();
                v.append(&mut vec![
                  format!("{}", cell_name),
                  format!("{}", pin_name),
                  format!("{}", timing.related_pin),
                  arc.to_string(),
                  match &timing.when {
                    Some(b) => format!("{}", b),
                    None => "None".to_owned(),
                  },
                ]);
                v
              };
              {
                let info = get_info("rise");
                if let Some([slew, load, nominal, mean, std_dev, skewness]) = get_lvf(
                  &timing.cell_rise,
                  &timing.ocv_mean_shift_cell_rise,
                  &timing.ocv_std_dev_cell_rise,
                  &timing.ocv_skewness_cell_rise,
                ) {
                  w_info_delay.write_record(info)?;
                  w_slew_delay.write_record(slew)?;
                  w_load_delay.write_record(load)?;
                  w_nominal_delay.write_record(nominal)?;
                  w_mean_delay.write_record(mean)?;
                  w_std_dev_delay.write_record(std_dev)?;
                  w_skewness_delay.write_record(skewness)?;
                } else {
                  println!("Missing delay of {}", info.join("-"));
                }
              }
              {
                let info = get_info("fall");
                if let Some([slew, load, nominal, mean, std_dev, skewness]) = get_lvf(
                  &timing.cell_fall,
                  &timing.ocv_mean_shift_cell_fall,
                  &timing.ocv_std_dev_cell_fall,
                  &timing.ocv_skewness_cell_fall,
                ) {
                  w_info_delay.write_record(info)?;
                  w_slew_delay.write_record(slew)?;
                  w_load_delay.write_record(load)?;
                  w_nominal_delay.write_record(nominal)?;
                  w_mean_delay.write_record(mean)?;
                  w_std_dev_delay.write_record(std_dev)?;
                  w_skewness_delay.write_record(skewness)?;
                } else {
                  println!("Missing delay of {}", info.join("-"));
                }
              }
              {
                let info = get_info("rise");
                if let Some([slew, load, nominal, mean, std_dev, skewness]) = get_lvf(
                  &timing.rise_transition,
                  &timing.ocv_mean_shift_rise_transition,
                  &timing.ocv_std_dev_rise_transition,
                  &timing.ocv_skewness_rise_transition,
                ) {
                  w_info_transition.write_record(info)?;
                  w_slew_transition.write_record(slew)?;
                  w_load_transition.write_record(load)?;
                  w_nominal_transition.write_record(nominal)?;
                  w_mean_transition.write_record(mean)?;
                  w_std_dev_transition.write_record(std_dev)?;
                  w_skewness_transition.write_record(skewness)?;
                } else {
                  println!("Missing transition of {}", info.join("-"));
                }
              }
              {
                let info = get_info("fall");
                if let Some([slew, load, nominal, mean, std_dev, skewness]) = get_lvf(
                  &timing.fall_transition,
                  &timing.ocv_mean_shift_fall_transition,
                  &timing.ocv_std_dev_fall_transition,
                  &timing.ocv_skewness_fall_transition,
                ) {
                  w_info_transition.write_record(info)?;
                  w_slew_transition.write_record(slew)?;
                  w_load_transition.write_record(load)?;
                  w_nominal_transition.write_record(nominal)?;
                  w_mean_transition.write_record(mean)?;
                  w_std_dev_transition.write_record(std_dev)?;
                  w_skewness_transition.write_record(skewness)?;
                } else {
                  println!("Missing transition of {}", info.join("-"));
                }
              }
            }
          }
        }
      }
      Err(e) => panic!("[ERROR] {:?}", e),
    }
    Ok(())
  }

  /// mean_shift=mean-nominal
  /// mean=mean_shift+nominal
  fn get_lvf(
    nominal: &Option<TableLookUp>,
    mean_shift: &Option<TableLookUp>,
    std_dev: &Option<TableLookUp>,
    skewness: &Option<TableLookUp>,
  ) -> Option<[Vec<String>; 6]> {
    let mut slew = None;
    let mut load = None;
    let mut _nominal = None;
    let mut _mean_shift = None;
    let mut _std_dev = None;
    let mut _skewness = None;
    if let Some(table) = nominal {
      slew = Some(&table.index_1);
      load = Some(&table.index_2);
      _nominal = Some(&table.values);
    }
    if let Some(table) = mean_shift {
      _mean_shift = Some(&table.values);
    }
    if let Some(table) = std_dev {
      _std_dev = Some(&table.values);
    }
    if let Some(table) = skewness {
      _skewness = Some(&table.values);
    }
    match (slew, load, _nominal, _mean_shift, _std_dev, _skewness) {
      (Some(slew), Some(load), Some(nominal), Some(mean_shift), Some(std_dev), Some(skewness)) => {
        match (
          slew.len(),
          load.len(),
          nominal.len(),
          mean_shift.len(),
          std_dev.len(),
          skewness.len(),
        ) {
          (8, 8, 64, 64, 64, 64) => {
            let mean = mean_shift
              .iter()
              .enumerate()
              .map(|(idx, mean_shift_v)| format!("{}", mean_shift_v + nominal[idx]))
              .collect();
            Some([
              slew.iter().map(ToString::to_string).collect(),
              load.iter().map(ToString::to_string).collect(),
              nominal.iter().map(ToString::to_string).collect(),
              mean,
              std_dev.iter().map(ToString::to_string).collect(),
              skewness.iter().map(ToString::to_string).collect(),
            ])
          }
          _ => None,
        }
      }
      _ => None,
    }
  }
}
