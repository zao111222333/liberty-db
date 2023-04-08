use nom::{
  branch::alt,
  bytes::streaming::{escaped, tag, take_while, take_until, take_while1},
  character::{streaming::{char, one_of, none_of}, is_alphanumeric},
  combinator::{map, opt, map_res},
  error::{context, ContextError, ErrorKind, ParseError, VerboseError, FromExternalError},
  multi::{separated_list0, many0, many1},
  sequence::{delimited, preceded, terminated, tuple, pair},
  IResult,
};
use std::str::{self, FromStr};

#[derive(Debug)]
pub struct Library<'a>{
  attribute: &'a str,
  content: GroupWrapper<'a>,
  comment: Vec<&'a str>,
}

// #[derive(Debug, PartialEq)]
// pub enum LibertyValue_ {
//   Simple(String),
//   Complex(Vec<String>),
//   Group(Vec<(String, HashMap<String, LibertyValue>)>),
// }

#[derive(Debug)]
pub enum Attribute<'a> {
  ValuePair(&'a str, LibertyValue<'a>),
  Comment(Vec<&'a str>),
}
  #[derive(Debug)]
pub enum LibertyValue<'a> {
  Simple(SimpleWrapper<'a>),
  Complex(ComplexWrapper<'a>),
  Group(GroupWrapper<'a>),
}

#[derive(Debug,PartialEq)]
pub enum SimpleWrapper<'a>{
  Single(&'a str),
  Multi(Vec<&'a str>),
}

#[derive(Debug,PartialEq)]
pub struct ComplexWrapper<'a>{
  list_lines: Vec<Vec<&'a str>>,
}

#[derive(Debug)]
pub struct GroupWrapper<'a>{
  name: ComplexWrapper<'a>, 
  attr_list: Vec<Attribute<'a>>,
}

/// parser combinators are constructed from the bottom up:
/// first we write parsers for the smallest elements (here a space character),
/// then we'll combine them in larger parsers
fn space<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  take_while(move |c| " \t\r".contains(c))(i)
}

fn space_newline<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  take_while(move |c| " \t\r\n".contains(c))(i)
}

fn space_newline_complex<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  terminated(
    space,
    opt(
      tuple((
        char('\\'),
        opt(comment), 
        char('\n'),
        space,
      )),
    )
  )(i)
}



fn unquote_multi<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<&'a str>, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
preceded(
  char('\"'),
  map(
    pair(
      many0(
        delimited(
          space,
          // FIXME: need to optimze
          take_while(move |c:char| 
            !"\"\\\n\r".contains(c)
          ),
          delimited(
            char('\\'),
            space,
            char('\n'),
          ),
        ),
      ),
      delimited(
        space, 
        // FIXME: need to optimze
        take_while(move |c:char| 
          !"\"\\\n\r".contains(c)
        ),
        preceded(
          space, 
          char('\"')),
      ),
    ),
    |(list,last)| {
      let mut v = list.clone();
      v.push(last);
      v
    },
  ),
)(i)
}
#[test]
fn test_unquote_multi() {
  println!("{:?}", unquote_multi::<VerboseError<&str>>(r#""L L L : - : L ,\
              L L H : - : H ,\
              L H L : - : H ,\
              L H H : - : H ,\
              H - - : - : N " ;
  "#));
}


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
fn unquote<'a, E>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "unquote",
    delimited(
      char('"'),
      escaped(opt(none_of(r#"\""#)), '\\', one_of(r#"\"rnt"#)),
      char('"'),
    )
  )(i)
}
#[test]
fn test_unquote() {
  println!("{:?}", unquote::<VerboseError<&str>>(r#""L L L : - : L ,\
              L L H : - : H ,\
              L H L : - : H ,\
              L H H : - : H ,\
              H - - : - : N " ;
  "#));
  println!("{:?}", unquote::<VerboseError<&str>>("\"\"11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\" \"11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\" www\"11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\"www\" 11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\"www 11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("(1,2)"));
}

fn key<'a, E>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "key",
    take_while1(move |c:char| 
      "/_.+-".contains(c)||is_alphanumeric(c as u8)
    ),
    )(i)
}

#[test]
fn test_key() {
  println!("{:?}", key::<VerboseError<&str>>(" 11111"));
  println!("{:?}", key::<VerboseError<&str>>("11._ 111"));
  println!("{:?}", key::<VerboseError<&str>>("11111 "));
  println!("{:?}", key::<VerboseError<&str>>("1_11 11 "));
  println!("{:?}", key::<VerboseError<&str>>("1,11 11 "));
  println!("{:?}", key::<VerboseError<&str>>("1 1111"));
  println!("{:?}", key::<VerboseError<&str>>("\"www\""));
  println!("{:?}", key::<VerboseError<&str>>("\"www 11111"));
  println!("{:?}", key::<VerboseError<&str>>("(1,2)"));
}

fn simple<'a, E>(
  i: &'a str,
) -> IResult<&'a str, SimpleWrapper<'a>, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "simple",
    preceded(
      space, 
      alt((
        map(unquote, SimpleWrapper::Single), 
        map(key, SimpleWrapper::Single), 
        map(unquote_multi, SimpleWrapper::Multi),
      )), 
      // space,
    ),
  )(i)
}
#[test]
fn x2() {
  println!("{:?}", simple::<VerboseError<&str>>("\"\"11111"));
  println!("{:?}", simple::<VerboseError<&str>>(" 11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\" \"11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\" www\"11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\"www\" 11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\"www 11111"));
  println!("{:?}", simple::<VerboseError<&str>>("(1,2)"));
}
fn complex_single_sub<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<&'a str>, E> 
where
  E: ParseError<&'a str>
   + ContextError<&'a str>
   + FromExternalError<&'a str, E>
{
  // println!("{:?}",i);
separated_list0(
  delimited(
    space,
    char(','),
    space_newline_complex,
  ),
  preceded(
    space_newline_complex,
    alt((
      // normal case
      key,
      // [] case
      // e.g. [point_time2, point_current2, bc_id3, ...]
      // map(
      delimited(
        char('['),
        escaped(
          none_of("]\n"),
          '\\', 
          one_of(r#"\"rnt"#),
        ),
        char(']'),
      ),
      //   |(s1,s2,s3)|s2,
      // )
    )),
  ), 
)(i)
}

/// some combinators, like `separated_list0` or `many0`, will call a parser repeatedly,
/// accumulating results in a `Vec`, until it encounters an error.
/// If you want more control on the parser application, check out the `iterator`
/// combinator (cf `examples/iterator.rs`)
fn complex_single<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<&'a str>, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  alt((
    map(
      tuple((
        char('\"'),
        space,
        complex_single_sub,
        opt(char(',')), 
        space,
        char('\"'),
      )),
      |(_,_,list,_,_,_)| list,
    ),
    map(
      unquote,
      |s| vec![s],
    ),
    complex_single_sub,
  ))(i)
}

fn complex<'a, E>(
  i: &'a str,
) -> IResult<&'a str, ComplexWrapper<'a>, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "complex",
    map(
      tuple((
        char('('),
        space_newline_complex,
        separated_list0(
          delimited(
            space,
            char(','),
            space_newline_complex,
          ),
          preceded(
            space,
            complex_single,
          ),
        ),
        space_newline_complex,
        char(')'),
        )
      ),
    |(_,_,list_lines,_,_)| ComplexWrapper { list_lines },
    )
  )(i)
}
#[test]
fn x3() {
  println!("{:?}", space_newline::<VerboseError<&str>>("\n  \n  "));
  println!("{:?}", complex::<VerboseError<&str>>("(wwww,bww)"));
  println!("{:?}", complex::<VerboseError<&str>>("(wwww , bww)yyyy"));
  println!("{:?}", complex::<VerboseError<&str>>("(\"wwww,bww\")kkkkk"));
  println!("{:?}", complex::<VerboseError<&str>>("(\"wwww, bww\")kkkkk"));
  println!("{:?}", complex::<VerboseError<&str>>("(\"wwww ,bww\")kkkkk"));
  println!("{:?}", complex::<VerboseError<&str>>("(\"wwww , bww\")kkkkk"));
  println!("{:?}", complex::<VerboseError<&str>>("(\" wwww ,bww\")kkkkk"));
  println!("{:?}", complex::<VerboseError<&str>>("(\"wwww ,bww \")kkkkk"));
  println!("{:?}", complex::<VerboseError<&str>>(r#"("CK E SE","IQ")"#));
  let values1 = r#"("0.0365012,1", 2, \
   0.0370929);"#;
  println!("{:?}", complex::<VerboseError<&str>>(values1));
  let values = r#"(\
    "0.0365012, 0.0370929, 0.0380363, 0.0394155, 0.0426448, 0.0501891, 0.0664809", \
 					"0.0369991, 0.0373486, 0.0379160, 0.0395053, 0.0428655, 0.0505114, 0.0666039", \
 					"0.0385942, 0.0390795, 0.0397256, 0.0411817, 0.0443358, 0.0517300, 0.0675179", \
 					"0.0476609, 0.0480588, 0.0484215, 0.0496803, 0.0522788, 0.0585718, 0.0728644", \
 					"0.0647267, 0.0650487, 0.0654727, 0.0664164, 0.0689581, 0.0744502, 0.0867139", \
 					"0.0942649, 0.0945947, 0.0949704, 0.0959713, 0.0983067, 0.1038792, 0.1155378", \
 					"0.1471017, 0.1476063, 0.1482057, 0.1494996, 0.1522433, 0.1586312, 0.1706887");
  "#;
  println!("{:?}", complex::<VerboseError<&str>>(values));
  let value3 = r#"(\
    init_time, [point_time2, point_current2, bc_id3, ...], end_time, end_current);
  "#;
  println!("{:?}", complex::<VerboseError<&str>>(value3));

}

fn group<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<Attribute<'a>>, E> 
where 
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "group",
    delimited(
      preceded(space_newline, char('{')),
      many0(
        preceded(space_newline, key_value),
      ),
      preceded(space_newline, char('}')),
    ),
  )(i)
}

#[test]
fn x55() {
  let shader = r#"
  {
    index_1(\
      "1000.0, 1001.0, 1002.0,\
      1003.0, 1004.0, 1005.0");
      comment : "";
      date : "[2012 JAN 31]";
      simulator : "HSPICE -- C-2009.03-SP1 32-BIT (May 25 2009)";
      variable_1 : related_pin_transition;
      index_2("init_time, init_current, bc_id1, point_time1, point_current1, bc_id2, [point_time2, point_current2, bc_id3, ...], end_time, end_current");
      variable_2 : related_pin_transition;
    }
    "#;
    assert!(group::<VerboseError<&str>>(shader).is_ok());
    assert!(key_value::<VerboseError<&str>>("variable_1 : related_pin_transition;\n      variable_2 : related_pin_transition;").is_ok());
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\") ; www"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\"); www"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);\nwww"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);www"));
  }

#[test]
fn x6() {
let shader = r#"lu_table_template ( "recovery_template_6x6") { 
    variable_1 : related_pin_transition;
    variable_2 : related_pin_transition;
    lu_table_template(recovery_template_6x6) { 
      variable_1 : related_pin_transition;
      variable_2 : related_pin_transition;
    }
    lu_table_template() {
      variable_1 : related_pin_transition;
      variable_2 : related_pin_transition;
    }
  }
  "#;
  assert!(key_value::<(&str, ErrorKind)>(shader).is_ok());
}

fn key_value<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Attribute<'a>, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "key_value",
    alt((
      map(
        comment,
        Attribute::Comment,
      ),
      map(
        pair(
          terminated(
            key,
            space_newline_complex,
          ),
          alt((
            map(
              tuple((
                char(':'),
                space_newline_complex,
                simple,
                space,
                char(';'),
              )),
              |(_,_,wrapper,_,_)| LibertyValue::Simple(wrapper),
            ),
            map(
              pair(
                terminated(
                  complex, space_newline_complex,
                ),
                alt((
                  map(
                    char(';'),
                    |_| None,
                  ),
                  map(
                    group,
                    |group| Some(group),
                  ),
                )),
              ),
              |(complex,group)|
              match group {
                Some(group) => LibertyValue::Group(
                  GroupWrapper { name: complex, attr_list: group }),
                None => LibertyValue::Complex(complex),
              }
            ),
          )),
        ),
        |(k,v)| Attribute::ValuePair(k, v)
      )
    ))
  )(i)
}

fn comment<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<&'a str>, E> 
where 
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
    many1(
      // map(
        preceded(
          space_newline, 
          alt((
            comment_single,
            comment_multi,
          )),
        ),
        // String::from,
      // )
  )(i)
}

fn comment_single<'a, E>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> 
where 
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  preceded(
    alt((
      tag("*"),
      tag("//"),
    )),
    terminated(
      take_until("\n"), 
      space_newline,
    ),
  )(i)
}

#[test]
fn test_comment(){
  println!("{:?}",comment_single::<VerboseError<&str>>("*\n"));
  println!("{:?}",comment_single::<VerboseError<&str>>("* www"));
  println!("{:?}",comment_single::<VerboseError<&str>>("* www\nbb"));
  println!("{:?}",comment_single::<VerboseError<&str>>("*\nbb"));
  println!("{:?}",comment_multi::<VerboseError<&str>>(r#"/**/  "#));
  println!("{:?}",comment_multi::<VerboseError<&str>>(r#"/*
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
  */"#));
  println!("{:?}",comment_multi::<VerboseError<&str>>(r#"/* www
  wwaawd
  
  */  "#));
}

fn comment_multi<'a, E>(
  i: &'a str,
) -> IResult<&'a str, &'a str, E> 
where 
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  preceded(
    tag("/*"),
    terminated(
      take_until("*/"),
      tag("*/"),
    ),
  )(i)
}



/// the root element of a JSON parser is either an object or an array
pub fn library_wrapper<'a, E>(  i: &'a str,
) -> IResult<&'a str, Library<'a>, VerboseError<&str>> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "library_wrapper",
    map_res(
      tuple((
        opt(comment),
        space_newline,
        key_value,
      )),
      |(_comment,_,attri)|
        match attri {
        Attribute::ValuePair(k, v) => match v {
          LibertyValue::Simple(_) => Err(VerboseError::from_error_kind("No group in root", ErrorKind::Verify)),
          LibertyValue::Complex(_) => Err(VerboseError::from_error_kind("No group in root", ErrorKind::Verify)),
          LibertyValue::Group(content) => 
          {
            Ok(Library { 
              attribute: k, 
              content,
              comment: match _comment{
                  Some(c) => c,
                  None => vec![],
              },
            })
          }
        },
        Attribute::Comment(_) => Err(VerboseError::from_error_kind("No group in root", ErrorKind::Verify)),
      }
    )
  )(i)
}

// impl<'a> FromStr for Library<'a> {
//     type Err=std::fmt::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//       match library_wrapper::<(&str,ErrorKind)>(s) {
//         Ok((_,l)) => Ok(l),
//         Err(_) => Err(std::fmt::Error),
//     }
//     }
// }

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
    println!("{:?}", library_wrapper::<(&str,ErrorKind)>(TEMPLATE));
    println!("{:?}", library_wrapper::<(&str,ErrorKind)>(TEMPLATE_ERR1));
    println!("{:?}", library_wrapper::<(&str,ErrorKind)>(TEMPLATE_ERR2));
  }
}