use nom::{
  branch::alt,
  bytes::complete::{escaped, tag, take_while, is_not},
  character::complete::{alphanumeric1 as alphanumeric, char, one_of, none_of},
  combinator::{cut, map, opt, value},
  error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError, VerboseErrorKind},
  multi::{separated_list0, separated_list1, many0},
  number::complete::double,
  sequence::{delimited, preceded, separated_pair, terminated},
  Err, IResult,
};
use std::str;

use crate::types::HashMap;

#[derive(Debug)]
pub struct Liberty{
  attribute: String,
  value: LibertyValue,
}

#[derive(Debug, PartialEq)]
pub enum LibertyValue_ {
  Simple(String),
  Complex(Vec<String>),
  Group(Vec<(String, HashMap<String, LibertyValue>)>),
}

#[derive(Debug, PartialEq)]
pub enum LibertyValue {
  Simple(String),
  Complex(Vec<String>),
  Group(String, Vec<(String, LibertyValue)>),
}

// #[derive(Debug, PartialEq)]
// pub enum LibertyValueWrapprer {
//   Simple(String),
//   Complex(Vec<String>),
//   Group(Vec<(String,HashMap<String, LibertyValue>)>),
// }

/// parser combinators are constructed from the bottom up:
/// first we write parsers for the smallest elements (here a space character),
/// then we'll combine them in larger parsers
fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  let chars = " \t\r\n";

  // nom combinators like `take_while` return a function. That function is the
  // parser,to which we can pass the input
  take_while(move |c| chars.contains(c))(i)
}

/// A nom parser has the following signature:
/// `Input -> IResult<Input, Output, Error>`, with `IResult` defined as:
/// `type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), Err<E>>;`
///
/// most of the times you can ignore the error type and use the default (but this
/// examples shows custom error types later on!)
///
/// Here we use `&str` as input type, but nom parsers can be generic over
/// the input type, and work directly with `&[u8]` or any other type that
/// implements the required traits.
///
/// Finally, we can see here that the input and output type are both `&str`
/// with the same lifetime tag. This means that the produced value is a subslice
/// of the input data. and there is no allocation needed. This is the main idea
/// behind nom's performance.
// #[inline]
// fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
//   // println!("{}", 111);
//   // println!("{}", i);
  
// }
// #[test]
// fn x1() {
//   println!("{:?}", parse_str::<(&str, ErrorKind)>("wwww,ww"));
//   println!("{:?}", parse_str::<(&str, ErrorKind)>("ww\nww,ww"));
//   println!("{:?}", parse_str::<(&str, ErrorKind)>("\"A&B\";wwww"));
//   println!("{:?}", parse_str::<(&str, ErrorKind)>("A&B;wwww"));
//   println!("{:?}", parse_str::<(&str, ErrorKind)>("\"wwww\"w,y"));
// }

/// this parser combines the previous `parse_str` parser, that recognizes the
/// interior of a string, with a parse to recognize the double quote character,
/// before the string (using `preceded`) and after the string (using `terminated`).
///
/// `context` and `cut` are related to error management:
/// - `cut` transforms an `Err::Error(e)` in `Err::Failure(e)`, signaling to
/// combinators like  `alt` that they should not try other parsers. We were in the
/// right branch (since we found the `"` character) but encountered an error when
/// parsing the string
/// - `context` lets you add a static string to provide more information in the
/// error chain (to indicate which parser had an error)
fn simple<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> {
  alt((
    delimited(
      char('\"'),
      simple,
      char('\"'),
    ),
    delimited(
      sp,
      escaped(
        none_of("\",;\n "),
        '\\', 
        one_of("\"n\\")),
        sp,
    ),
  ))(i)
}
#[test]
fn x2() {
  println!("{:?}", simple::<(&str, ErrorKind)>("\"www\"11111"));
  println!("{:?}", simple::<(&str, ErrorKind)>("\"www\" 11111"));
  println!("{:?}", simple::<(&str, ErrorKind)>("(1,2)"));
}

// fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
//   i: &'a str,
// ) -> IResult<&'a str, &'a str, E> {
//   context(
//     "string",
//     preceded(char('\"'), 
//     cut(terminated(parse_str, char('\"')))),
//   )(i)
// }

/// some combinators, like `separated_list0` or `many0`, will call a parser repeatedly,
/// accumulating results in a `Vec`, until it encounters an error.
/// If you want more control on the parser application, check out the `iterator`
/// combinator (cf `examples/iterator.rs`)
/// 

fn complex_sub<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<String>, E> {
  separated_list0(
    delimited(
      sp,
      char(','),
      sp,
    ),
    map(
      delimited(
        sp,
        escaped(
          none_of("(){},;\t\r\n \""),
          '\\', 
          one_of("\"n\\")),
        sp,
      ), 
      |s|String::from(s),
    ),
  )(i)
}
fn complex<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, Vec<String>, E> {
  delimited(
    char('('),
    alt((
    delimited(
      char('\"'),
      complex_sub,
      char('\"'),
    ),
    complex_sub,
    )),
    char(')'),
  )(i)
}
#[test]
fn x3() {
  assert_eq!(Ok(("",      vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(wwww,bww)"));
  assert_eq!(Ok(("yyyy",  vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(wwww , bww)yyyy"));
  assert_eq!(Ok(("kkkkk", vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(\"wwww,bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(\"wwww, bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(\"wwww ,bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(\"wwww , bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(\" wwww ,bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", vec![String::from("wwww"), String::from("bww")])), complex::<(&str, ErrorKind)>("(\"wwww ,bww \")kkkkk"));
}

fn key_simple<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, (&'a str, &'a str), E> {
  separated_pair(
    preceded(
      sp, 
      escaped(
        none_of("(){},;\t\r\n :"),
        '\\', 
        one_of("\"n\\")),
    ),
    alt((
      tag(":"),
      preceded(
        sp, 
        tag(":"),
      ),  
    )),
    delimited(
      sp,
      simple,
      // map(simple, |s| LibertyValue::Simple(String::from(s))),
      preceded(
        sp, 
        char(';'),
      ),
    ),
  )(i)
}

#[test]
fn x4() {
  let key = "timing_type";
  let value_ok = vec![
    "setup_falling",
    "\"setup_falling\""
  ];
  let value_err = vec![
    "setup_falling",
    "\"setup_falling\""
  ];
  let value1 = "setup_falling";
  let value2 = "\"setup_falling\"";
  let value3 = "\"setup_falling";
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : setup_falling;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : \"setup_falling\";"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : \"setup_falling\" ;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : setup_falling ;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type: setup_falling;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type :setup_falling ; wwww"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type:setup_falling;wwww"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : setup_falling\nwwww"));
  // println!("{:?}", key_simple_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\");"));
  // println!("{:?}", key_simple_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  // println!("{:?}", key_simple_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
}

fn key_complex<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  i: &'a str,
) -> IResult<&'a str, (&'a str, Vec<String>), E> {
  separated_pair(
    preceded(
      sp, 
      escaped(
        none_of("(){},;\t\r\n "),
        '\\', 
        one_of("\"n\\")),
    ),
    sp,
    terminated(
        complex,
        // map(complex, |v | LibertyValue::Complex(v.iter().map(|s|String::from(*s)).collect())),
      preceded(
        sp, 
        char(';'),
      ),
    ),
  )(i)
}

#[test]
fn x5() {
  println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\");"));
  println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\") ; www"));
  println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\"); www"));
  println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);\nwww"));
  println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);www"));
}

fn group<'a>(
  i: &'a str,
) -> IResult<&'a str, Vec<(String, LibertyValue)>, VerboseError<&'a str>> {
  map(
    many0(
      // sp,
      // char(';'),
      // delimited(sp, char(';'),sp), 
      // key_value,
      delimited(sp, key_value,sp),
      ),
    |tuple_vec| {
      tuple_vec
        .into_iter()
        .map(|(k, v)| 
          (String::from(k), v)
        )
        .collect()
    },
  )(i)
}

#[test]
fn x55() {
  //   let shader = r#"
  //   lu_table_template(recovery_template_6x6) {
  //     variable_1 : related_pin_transition;
  //     variable_2 : constrained_pin_transition;
  //     index_1 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0, 1005.0");
  //     index_2 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0, 1005.0");
  //   }
  // "#;
  let shader = r#"
  index_1 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0, 1005.0");
      variable_1 : related_pin_transition;
      variable_2 : related_pin_transition;"#;
    println!("{:?}", group(shader));
    // println!("{:?}", key_value("variable_1 : related_pin_transition;\n      variable_2 : related_pin_transition;"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\") ; www"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\"); www"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);\nwww"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);www"));
  }
fn key_group<'a>(
  i: &'a str,
  // (String, HashMap<String, LibertyValue>)
) -> IResult<&'a str, (&'a str, (&'a str, Vec<(String, LibertyValue)>)), VerboseError<&'a str>> {
  separated_pair(
    delimited(
      sp, 
      escaped(
        none_of("(){},;\t\r\n "),
        '\\', 
        one_of("\"n\\")),
      sp, 
    ),
    char('('),
    separated_pair(
      delimited(
        sp, 
        escaped(
          none_of("(){},;\t\r\n "),
          '\\', 
          one_of("\"n\\")),
        sp, 
      ),
      delimited(
        char(')'),
        sp, 
        char('{'),
      ),
      terminated(
        group,
        preceded(
          sp, 
          char('}'),
        ),
      ),
    ),
  )(i)
}

#[test]
fn x6() {
//   let shader = r#"
//   lu_table_template(recovery_template_6x6) {
//     variable_1 : related_pin_transition;
//     variable_2 : constrained_pin_transition;
//     index_1 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0, 1005.0");
//     index_2 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0, 1005.0");
//   }
// "#;
let shader = r#"
  lu_table_template(recovery_template_6x6) { 
    variable_1 : related_pin_transition;
    variable_2 : related_pin_transition;
    lu_table_template(recovery_template_6x6) { 
      variable_1 : related_pin_transition;
      variable_2 : related_pin_transition;
    }
  }
  "#;
  println!("{:?}", key_group(shader));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\") ; www"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\"); www"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);\nwww"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);www"));
}

fn key_value<'a>(
  i: &'a str,
) -> IResult<&'a str, (&'a str, LibertyValue), VerboseError<&'a str>> {
  alt((
    map(key_simple, |(k,v)| (
      k,
      LibertyValue::Simple(String::from(v)))),
    map(key_complex, |(k,v)| (
      k,
      LibertyValue::Complex(v))),
    map(key_group, |(k,(group_name,pair_list))| (
      k,
      // Group(String, Vec<(String, LibertyValue)>),
      LibertyValue::Group(
        String::from(group_name),
        pair_list,
      )),
    ),
  ))(i)
}

/// the root element of a JSON parser is either an object or an array
fn root<'a>(
  i: &'a str,
) -> IResult<&'a str, Liberty, VerboseError<&'a str>> {
  map(
    delimited(
      sp,
      key_value,
      opt(sp),
    ),
    |(k,v)|(
      Liberty { attribute: String::from(k), value: v }
    ),
  )(i)
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
}
  "#;
  #[test]
  fn x6() {
    println!("{:?}", root(TEMPLATE));
  }
}