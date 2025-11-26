use liberty_db::{
  DefaultCtx, Library, MutSetExt as _, cell::CellCtx, expression::SdfExpression,
};
use std::{env, path::Path, process::ExitCode};

// cargo run /path/to/xxx.lib
// Use the first arg as input file name,
// parse it, do some process, and then write it into example1_xxx.lib
fn main() -> ExitCode {
  dev_utils::init_logger();
  let args: Vec<String> = env::args().collect();

  let input_lib = Path::new(&args[1]);
  log::info!("Parsing [file] {} ...", input_lib.display());
  let mut library = match Library::<DefaultCtx>::parse_lib_file(input_lib) {
    Ok(l) => l,
    Err(e) => {
      log::error!("{e}");
      return ExitCode::FAILURE;
    }
  };
  library.technology = Some(liberty_db::library::Technology::CMOS);
  for operating_condition in library.operating_conditions.iter_mut() {
    operating_condition.voltage = 0.8;
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
          timing.sdf_cond =
            Some(SdfExpression::new(&when.bdd, cell.extra_ctx.logic_variables()));
        }
        // remove LVF's LUT
        if let Some(table) = timing.cell_rise.as_mut() {
          table.lvf_moments_values.clear()
        };
        if let Some(table) = timing.cell_fall.as_mut() {
          table.lvf_moments_values.clear()
        };
        if let Some(table) = timing.rise_transition.as_mut() {
          table.lvf_moments_values.clear()
        };
        if let Some(table) = timing.fall_transition.as_mut() {
          table.lvf_moments_values.clear()
        };
        if let Some(table) = timing.rise_constraint.as_mut() {
          table.lvf_moments_values.clear()
        };
        if let Some(table) = timing.fall_constraint.as_mut() {
          table.lvf_moments_values.clear()
        };
      }
    }
  }
  let out_file_name =
    format!("example2_{}", input_lib.file_name().unwrap().to_str().unwrap());
  log::info!("Output to [file] {out_file_name} ...");
  library.write_lib_file(&out_file_name).unwrap();
  ExitCode::SUCCESS
}
