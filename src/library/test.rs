#![cfg(test)]
use crate::{
  DefaultCtx,
  ast::{AttriValues, SimpleDefined},
};

use super::*;

fn parse_cmp(text: &str, want: &str) -> Library<DefaultCtx> {
  match Library::parse_lib(text) {
    Ok(mut library) => {
      library.comments_this_entry().or_insert("test".into());
      let s = library.to_string();
      println!("{s}");
      dev_utils::text_diff(&s, want);
      library
    }
    Err(e) => panic!("{e:#?}"),
  }
}

fn fmt_cmp(library: &Library<DefaultCtx>, want: &str) {
  let s = library.to_string();
  println!("{s}");
  dev_utils::text_diff(&s, want);
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
| define (my_define_bool, library, boolean);
| define (my_define_float, library, float);
| define (my_define_integer, cell, integer);
| define (my_define_str, library, string);
| delay_model : table_lookup;
| time_unit : 1ns;
| voltage_unit : 1V;
| slew_upper_threshold_pct_rise : 80.0;
| slew_lower_threshold_pct_rise : 20.0;
| slew_derate_from_library : 1.0;
| slew_lower_threshold_pct_fall : 20.0;
| slew_upper_threshold_pct_fall : 80.0;
| input_threshold_pct_fall : 50.0;
| input_threshold_pct_rise : 50.0;
| output_threshold_pct_rise : 50.0;
| output_threshold_pct_fall : 50.0;
| my_define_bool : true; /* user defined attribute */
| my_define_bool : abc; /* user defined attribute */
| my_define_bool : false; /* user defined attribute */
| my_define_float : 1.0; /* user defined attribute */
| my_define_float : 2.0; /* user defined attribute */
| my_define_float : 3.0; /* user defined attribute */
| my_define_float : 4.0; /* user defined attribute */
| my_define_float : abc; /* user defined attribute */
| my_define_str : abc; /* user defined attribute */
| cell (test1) {
| | my_define_integer : 1; /* user defined attribute */
| | my_define_integer : 2; /* user defined attribute */
| | my_define_integer : 3; /* user defined attribute */
| | my_define_integer : 4; /* user defined attribute */
| }
}
"#,
  );
  let cell_test1 = library.cell.get("test1").unwrap();
  println!("{:?}", library.attributes);
  println!("{:?}", cell_test1.attributes);
  assert_eq!(
    &library.attributes["my_define_float"],
    &AttriValues::Simple(SimpleDefined::Float(vec![
      Ok(1.0),
      Ok(2.0),
      Ok(3.0),
      Ok(4.0),
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

#[test]
fn comment() {
  let mut library = Library::default();
  library.comments_this_entry().or_insert("comment1".into());
  library
    .comments_this_entry()
    .and_modify(|comment| comment.push_str("\ncomment2\ncomment3"));
  library.technology = Some(Technology::Cmos);
  library.comments_technology_entry().or_insert("comment1".into());
  library
    .comments_technology_entry()
    .and_modify(|comment| comment.push_str("\ncomment2\ncomment3"));
  library
    .comments_time_unit_entry()
    .or_insert("one line comment".into());
  let mut cell_test1 = Cell::default();
  cell_test1.name = "test1".into();
  cell_test1.comments_this_entry().or_insert("comment1".into());
  cell_test1
    .comments_this_entry()
    .and_modify(|comment| comment.push_str("\ncomment2\ncomment3"));
  library.cell.insert(cell_test1);
  library.cell = library.cell.into_iter().collect();
  fmt_cmp(
    &library,
    r#"/* comment1
** comment2
** comment3 */
library (undefined) {
| /* comment1
| ** comment2
| ** comment3 */
| technology (cmos);
| delay_model : table_lookup;
| /* one line comment */
| time_unit : 1ns;
| voltage_unit : 1V;
| slew_upper_threshold_pct_rise : 80.0;
| slew_lower_threshold_pct_rise : 20.0;
| slew_derate_from_library : 1.0;
| slew_lower_threshold_pct_fall : 20.0;
| slew_upper_threshold_pct_fall : 80.0;
| input_threshold_pct_fall : 50.0;
| input_threshold_pct_rise : 50.0;
| output_threshold_pct_rise : 50.0;
| output_threshold_pct_fall : 50.0;
| /* comment1
| ** comment2
| ** comment3 */
| cell (test1) {
| }
}
"#,
  );
}

#[test]
fn entry() {
  let mut library = Library::default();
  library.comments_this_entry().or_insert("comment1".into());
  library
    .cell
    .entry("CELL1".into())
    .and_modify(|cell| cell.area = Some(1.0))
    .or_insert_with(|cell| cell.area = Some(0.0));
  library
    .cell
    .entry("CELL2".into())
    .and_modify(|cell| cell.area = Some(1.0))
    .or_insert_with(|cell| cell.area = Some(0.0));
  library
    .cell
    .entry("CELL2".into())
    .and_modify(|cell| cell.area = Some(1.0))
    .or_insert_with(|cell| cell.area = Some(0.0));
  fmt_cmp(
    &library,
    r#"/* comment1 */
library (undefined) {
| delay_model : table_lookup;
| time_unit : 1ns;
| voltage_unit : 1V;
| slew_upper_threshold_pct_rise : 80.0;
| slew_lower_threshold_pct_rise : 20.0;
| slew_derate_from_library : 1.0;
| slew_lower_threshold_pct_fall : 20.0;
| slew_upper_threshold_pct_fall : 80.0;
| input_threshold_pct_fall : 50.0;
| input_threshold_pct_rise : 50.0;
| output_threshold_pct_rise : 50.0;
| output_threshold_pct_fall : 50.0;
| cell (CELL1) {
| | area : 0.0;
| }
| cell (CELL2) {
| | area : 1.0;
| }
}
"#,
  );
}

#[test]
fn serde() {
  let mut library = Library::default();
  library.comments_this_entry().or_insert("comment1".into());
  library
    .cell
    .entry("CELL1".into())
    .and_modify(|cell| cell.area = Some(1.0))
    .or_insert_with(|cell| cell.area = Some(0.0));
  library
    .cell
    .entry("CELL2".into())
    .and_modify(|cell| cell.area = Some(1.0))
    .or_insert_with(|cell| cell.area = Some(0.0));
  library
    .cell
    .entry("CELL2".into())
    .and_modify(|cell| cell.area = Some(1.0))
    .or_insert_with(|cell| cell.area = Some(0.0));
  let want = r#"/* comment1 */
library (undefined) {
| delay_model : table_lookup;
| time_unit : 1ns;
| voltage_unit : 1V;
| slew_upper_threshold_pct_rise : 80.0;
| slew_lower_threshold_pct_rise : 20.0;
| slew_derate_from_library : 1.0;
| slew_lower_threshold_pct_fall : 20.0;
| slew_upper_threshold_pct_fall : 80.0;
| input_threshold_pct_fall : 50.0;
| input_threshold_pct_rise : 50.0;
| output_threshold_pct_rise : 50.0;
| output_threshold_pct_fall : 50.0;
| cell (CELL1) {
| | area : 0.0;
| }
| cell (CELL2) {
| | area : 1.0;
| }
}
"#;
  fmt_cmp(&library, want);
  let serialized = bincode::serialize(&library).unwrap();
  fmt_cmp(&bincode::deserialize(&serialized).unwrap(), want);
}
