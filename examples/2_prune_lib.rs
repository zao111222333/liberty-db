use liberty_db::{Library, NotNan};
use std::{
  env,
  fs::{read_to_string, File},
  io::{BufWriter, Write},
  path::Path,
};

// cargo run /path/to/xxx.lib
// Use the first arg as input file name,
// parse it, do some process, and then write it into example1_xxx.lib
fn main() {
  simple_logger::SimpleLogger::new().init().unwrap();
  let args: Vec<String> = env::args().collect();

  let input_lib = Path::new(&args[1]);
  log::info!("Parsing [file] {} ...", input_lib.display());
  let mut library =
    Library::parse_lib(read_to_string(input_lib).unwrap().as_str()).unwrap();
  library.technology = "cmos".into();
  for operating_condition in library.operating_conditions.iter_mut() {
    operating_condition.voltage = unsafe { NotNan::<f64>::new_unchecked(0.8) };
  }
  log::warn!("Attributes Items: {:?}", library.attributes);
  for cell in library.cell.iter_mut() {
    log::info!("Loop to [cell] {}", cell.name);
    for pin in cell.pin.iter_mut() {
      log::info!("Loop to [pin] {}", pin.name);
      for timing in pin.timing.iter_mut() {
        log::info!(
          "Loop to [timing] related_pin={} {} {} {}",
          timing.related_pin,
          if let Some(timing_sense) = timing.timing_sense {
            format!("timing_sense={timing_sense}")
          } else {
            String::new()
          },
          if let Some(when) = &timing.when {
            format!("when={when}")
          } else {
            String::new()
          },
          if let Some(timing_type) = &timing.timing_type {
            format!("timing_type={timing_type}")
          } else {
            String::new()
          },
        );
        // Add `sdf_cond` from `when`
        if let Some(when) = &timing.when {
          timing.sdf_cond = Some(when.sdf());
        }
        // remove LVF's LUT
        timing.ocv_mean_shift_cell_fall = None;
        timing.ocv_mean_shift_cell_rise = None;
        timing.ocv_mean_shift_fall_transition = None;
        timing.ocv_mean_shift_rise_transition = None;
        timing.ocv_mean_shift_fall_constraint = None;
        timing.ocv_mean_shift_rise_constraint = None;
        timing.ocv_std_dev_cell_fall = None;
        timing.ocv_std_dev_cell_rise = None;
        timing.ocv_std_dev_fall_transition = None;
        timing.ocv_std_dev_rise_transition = None;
        timing.ocv_std_dev_fall_constraint = None;
        timing.ocv_std_dev_rise_constraint = None;
        timing.ocv_skewness_cell_fall = None;
        timing.ocv_skewness_cell_rise = None;
        timing.ocv_skewness_fall_transition = None;
        timing.ocv_skewness_rise_transition = None;
        timing.ocv_skewness_fall_constraint = None;
        timing.ocv_skewness_rise_constraint = None;
      }
    }
  }
  let out_file_name =
    format!("example2_{}", input_lib.file_name().unwrap().to_str().unwrap());
  log::info!("Output to [file] {} ...", out_file_name);
  let out_file = File::create(out_file_name).unwrap();
  let mut writer = BufWriter::new(out_file);
  write!(&mut writer, "{}", library).unwrap();
}
