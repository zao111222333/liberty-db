#![cfg(test)]
use crate::{
  ast::{AttriValues, SimpleDefined},
  util::text_diff,
  NotNan,
};

use super::*;

fn parse_cmp(text: &str, want: &str) -> Library {
  match Library::parse_lib(text) {
    Ok(mut library) => {
      library.comments.this.push("test".into());
      let s = library.to_string();
      println!("{s}");
      text_diff(&s, want);
      library
    }
    Err(e) => panic!("{e:#?}"),
  }
}

#[test]
fn define() {
  let library = parse_cmp(
    r#"
  library(define) {
      define(my_define_float, library, float);
      define(my_define_bool, library, boolean);
      define(my_define_str, library, string);
      define(my_define_integer, cell, integer);
      my_define_float: 1;
      my_define_float: 2;
      my_define_float: 3;
      my_define_float: 4;
      my_define_float: abc;
      my_define_bool: true;
      my_define_bool: abc;
      my_define_bool: false;
      my_define_str: "abc";
      cell (test1) {
          my_define_integer: 1;
          my_define_integer: 2;
          my_define_integer: 3;
          my_define_integer: 4;
      }
  }"#,
    r#"/* test */
library (define) {
  technology (cmos);
  delay_model : table_lookup;
  define (my_define_bool, library, boolean);
  define (my_define_float, library, float);
  define (my_define_integer, cell, integer);
  define (my_define_str, library, string);
  time_unit : 1ns;
  voltage_unit : 1V;
  slew_upper_threshold_pct_rise : 80.0;
  slew_lower_threshold_pct_rise : 20.0;
  slew_derate_from_library : 1.0;
  slew_lower_threshold_pct_fall : 20.0;
  slew_upper_threshold_pct_fall : 80.0;
  input_threshold_pct_fall : 50.0;
  input_threshold_pct_rise : 50.0;
  output_threshold_pct_rise : 50.0;
  output_threshold_pct_fall : 50.0;
  my_define_bool : true; /* user defined attribute */
  my_define_bool : abc; /* user defined attribute */
  my_define_bool : false; /* user defined attribute */
  my_define_float : 1.0; /* user defined attribute */
  my_define_float : 2.0; /* user defined attribute */
  my_define_float : 3.0; /* user defined attribute */
  my_define_float : 4.0; /* user defined attribute */
  my_define_float : abc; /* user defined attribute */
  my_define_str : abc; /* user defined attribute */
  cell (test1) {
    my_define_integer : 1; /* user defined attribute */
    my_define_integer : 2; /* user defined attribute */
    my_define_integer : 3; /* user defined attribute */
    my_define_integer : 4; /* user defined attribute */
  }
}
"#,
  );
  let cell_test1 = library.cell.get(&ArcStr::from("test1")).unwrap();
  println!("{:?}", library.attributes);
  println!("{:?}", cell_test1.attributes);
  assert_eq!(
    &library.attributes["my_define_float"],
    &AttriValues::Simple(SimpleDefined::Float(vec![
      Ok(NotNan::new(1.0_f64).unwrap()),
      Ok(NotNan::new(2.0_f64).unwrap()),
      Ok(NotNan::new(3.0_f64).unwrap()),
      Ok(NotNan::new(4.0_f64).unwrap()),
      Err("abc".into()),
    ]))
  );
  assert_eq!(
    &library.attributes["my_define_bool"],
    &AttriValues::Simple(SimpleDefined::Boolean(vec![
      Ok(true),
      Err("abc".into()),
      Ok(false),
    ]))
  );
  assert_eq!(
    &library.attributes["my_define_str"],
    &AttriValues::Simple(SimpleDefined::String(vec!["abc".into()]))
  );
  assert_eq!(
    &cell_test1.attributes["my_define_integer"],
    &AttriValues::Simple(SimpleDefined::Integer(vec![Ok(1), Ok(2), Ok(3), Ok(4),]))
  );
}