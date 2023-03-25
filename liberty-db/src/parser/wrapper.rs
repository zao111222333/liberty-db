use nom::{
  branch::alt,
  bytes::complete::{escaped, tag, take_while, is_not, take_until},
  character::{complete::{alphanumeric1 as alphanumeric, char, one_of, none_of, crlf, anychar}, streaming::alphanumeric1},
  combinator::{cut, map, opt, value, map_res, all_consuming, recognize, verify},
  error::{context, convert_error, ContextError, ErrorKind, ParseError, VerboseError, VerboseErrorKind, FromExternalError},
  multi::{separated_list0, separated_list1, many0, many0_count, many1},
  number::complete::double,
  sequence::{delimited, preceded, separated_pair, terminated, tuple, pair},
  IResult,
};
use std::str::{self, FromStr};

use crate::types::HashMap;

#[derive(Debug)]
pub struct Library{
  attribute: String,
  comment: Vec<String>,
  content: GroupWrapper,
}

// #[derive(Debug, PartialEq)]
// pub enum LibertyValue_ {
//   Simple(String),
//   Complex(Vec<String>),
//   Group(Vec<(String, HashMap<String, LibertyValue>)>),
// }

#[derive(Debug)]
pub enum Attribute {
  ValuePair(String, LibertyValue),
  Comment(Vec<String>),
}
  #[derive(Debug)]
pub enum LibertyValue {
  Simple(SimpleWrapper),
  Complex(ComplexWrapper),
  Group(GroupWrapper),
}

#[derive(Debug,PartialEq)]
pub struct SimpleWrapper{
  value: String,
}

#[derive(Debug,PartialEq)]
pub struct ComplexWrapper{
  list: Vec<String>,
}

#[derive(Debug)]
pub struct GroupWrapper{
  name: ComplexWrapper, 
  attr_list: Vec<Attribute>,
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
  alt((
    delimited(
      space,
      tag("\\"),
      preceded(
        opt(comment), 
        space_newline,
      ),
    ),
    space,
  ))(i)
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
) -> IResult<&'a str, String, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "unquote",
    map(
      delimited(
        char('\"'),
        alt((
          escaped(
            none_of("\"\n\r"),
            '\\', 
            one_of(r#"\"rnt"#),
          ),
          space,
        )),
        char('\"'),
      ),  
      String::from,
    )
  )(i)
}
#[test]
fn test_unquote() {
  println!("{:?}", unquote::<VerboseError<&str>>("\"\"11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\" \"11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\" www\"11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\"www\" 11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("\"www 11111"));
  println!("{:?}", unquote::<VerboseError<&str>>("(1,2)"));
}

fn key<'a, E>(
  i: &'a str,
) -> IResult<&'a str, String, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "key",
    alt((
      map(
        many1(
          alt((
            alphanumeric1,
            tag("_"),
            tag("."),
            tag("/"),
            tag("-"),
            tag("+"),
          )),
        ),
        |list| list.concat(),
      ),
    )),
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
) -> IResult<&'a str, SimpleWrapper, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "simple",
    map(
      delimited(
        space, 
        alt((unquote, key)), 
        space,
      ),
      |s| SimpleWrapper { value: s }
    ),
  )(i)
}
#[test]
fn x2() {
  println!("{:?}", simple::<VerboseError<&str>>("\"\"11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\" \"11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\" www\"11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\"www\" 11111"));
  println!("{:?}", simple::<VerboseError<&str>>("\"www 11111"));
  println!("{:?}", simple::<VerboseError<&str>>("(1,2)"));
}

/// some combinators, like `separated_list0` or `many0`, will call a parser repeatedly,
/// accumulating results in a `Vec`, until it encounters an error.
/// If you want more control on the parser application, check out the `iterator`
/// combinator (cf `examples/iterator.rs`)
fn complex_single<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<String>, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  alt((
    delimited(
      char('\"'),
      delimited(
        space,
        complex_single,
        preceded(
          opt(char(',')), 
          space),
      ),
      char('\"'),
    ),
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
          map(
            tuple((
              char('['),
              escaped(
                none_of("]\n"),
                '\\', 
                one_of(r#"\"rnt"#),
              ),
              char(']'),
            )),
            |(s1,s2,s3)|format!("{s1}{s2}{s3}"),
          )
        )),
      ), 
    ),
  ))(i)
}

fn complex<'a, E>(
  i: &'a str,
) -> IResult<&'a str, ComplexWrapper, E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "complex",
    delimited(
      delimited(
        space,
        char('('),
        space_newline_complex,
      ),
      map(
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
        |list| {
          ComplexWrapper { list:list.into_iter().flatten().collect() }
        }
      ),
      preceded(
        space_newline_complex,
        char(')'),
      ),
    )
  )(i)
}
#[test]
fn x3() {
  assert_eq!(Ok(("",      ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(wwww,bww)"));
  assert_eq!(Ok(("yyyy",  ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(wwww , bww)yyyy"));
  assert_eq!(Ok(("kkkkk", ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(\"wwww,bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(\"wwww, bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(\"wwww ,bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(\"wwww , bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(\" wwww ,bww\")kkkkk"));
  assert_eq!(Ok(("kkkkk", ComplexWrapper{list:vec![String::from("wwww"), String::from("bww")]})), complex::<VerboseError<&str>>("(\"wwww ,bww \")kkkkk"));
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
}

fn key_simple<'a,E>(
  i: &'a str,
) -> IResult<&'a str, (String, SimpleWrapper), E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "key_simple",
    separated_pair(
      preceded(
        space_newline,
        key,
      ),
      preceded(
        space_newline_complex,
        tag(":"),
      ),  
      delimited(
        space_newline_complex,
        simple,
        preceded(
          space,
          char(';'),
        ),
      ),
    )
  )(i)
}

#[test]
fn x4() {
  println!("{:?}", key_simple::<(&str, ErrorKind)>("comment : \"\";"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("date : \"[2012 JAN 31]\";"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : setup_falling;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : \\ \n \"setup_falling\";"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : \"setup_falling\" ;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : setup_falling ;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type: setup_falling;"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type :setup_falling ; wwww"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type :setup_falling wwww"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type:setup_falling;wwww"));
  println!("{:?}", key_simple::<(&str, ErrorKind)>("timing_type : setup_falling\nwwww"));
}

fn key_complex<'a, E>(
  i: &'a str,
) -> IResult<&'a str, (String, ComplexWrapper), E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "key_complex",
    separated_pair(
      preceded(
        space_newline_complex,
        key,
      ),
      space_newline_complex,
      terminated(
          complex,
        preceded(
          space,
          char(';'),
        ),
      ),
    )
  )(i)
}

#[test]
fn x5() {
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2 \n (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\");"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2 \\\n (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\");"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2 \\ \n (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\");"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\") ; www"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\"); www"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);\nwww"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);www"));
  println!("{:?}", key_complex::<VerboseError<&str>>("index_2(\"init_time, init_current, bc_id1, point_time1, point_current1, bc_id2, [point_time2, point_current2, bc_id3, ...], end_time, end_current\");"));
}

fn group<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<Attribute>, E> 
where 
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "group",
    // map(
      many0(
        preceded(space_newline, key_value),
        ),
      // |tuple_vec| {
      //   tuple_vec
      //     .into_iter()
      //     .map(|(k, v)|
      //       (String::from(k), v)
      //     )
      //     .collect()
      // },
    // )
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
  let shader = r#"index_1(\
    "1000.0, 1001.0, 1002.0,\
    1003.0, 1004.0, 1005.0");
    comment : "";
    date : "[2012 JAN 31]";
    simulator : "HSPICE -- C-2009.03-SP1 32-BIT (May 25 2009)";
    variable_1 : related_pin_transition;
    index_2("init_time, init_current, bc_id1, point_time1, point_current1, bc_id2, [point_time2, point_current2, bc_id3, ...], end_time, end_current");
    variable_2 : related_pin_transition;"#;
    println!("{:?}", group::<VerboseError<&str>>(shader));
    // println!("{:?}", key_value("variable_1 : related_pin_transition;\n      variable_2 : related_pin_transition;"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\") ; www"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\"); www"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);\nwww"));
    // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);www"));
  }
fn key_group<'a, E>(
  i: &'a str,
) -> IResult<&'a str, (String, GroupWrapper), E> 
where
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
  context(
    "key_group",
    pair(
      delimited(
        space_newline,
        key,
        space,
      ),
      map(
        pair(
          complex,
          delimited(
            preceded(space_newline, char('{')),
            group,
            preceded(space_newline, char('}')),
          ),
        ),
        |(name,attr_list)|(
          GroupWrapper{
            name,
            attr_list,
          }
        ),
      ),
    )
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
  println!("{:?}", key_group::<(&str, ErrorKind)>(shader));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\") ; www"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (\"0.06, 0.24, 0.48, 0.9, 1.2, 1.8\"); www"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2 (0.06, 0.24, 0.48, 0.9, 1.2, 1.8);\nwww"));
  // println!("{:?}", key_complex::<(&str, ErrorKind)>("index_2(0.06, 0.24, 0.48, 0.9, 1.2, 1.8);www"));
}

fn key_value<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Attribute, E> 
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
        key_simple, 
        |(k,v)| Attribute::ValuePair(k, LibertyValue::Simple(v)),
      ),
      map(
        key_complex, 
        |(k,list)| Attribute::ValuePair(k, LibertyValue::Complex(list)),
      ),
      map(
        key_group, 
        |(k,group_wrapper)| Attribute::ValuePair(k, LibertyValue::Group(group_wrapper)),
      ),
    )),
  )(i)
}

fn comment<'a, E>(
  i: &'a str,
) -> IResult<&'a str, Vec<String>, E> 
where 
  E: ParseError<&'a str> 
   + ContextError<&'a str> 
   + FromExternalError<&'a str, E>
{
    many1(
      map(
        preceded(
          space_newline, 
          alt((
            comment_single,
            comment_multi,
          )),
        ),
        String::from,
      )
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
    char('*'),
    alt((
      terminated(
        take_until("\n"), 
        space_newline),
      terminated(
        space, 
        space_newline,
      ),
    )),
  )(i)
}

#[test]
fn test_comment(){
  println!("{:?}",comment_single::<VerboseError<&str>>("* www"));
  println!("{:?}",comment_single::<VerboseError<&str>>("* www\nbb"));
  println!("{:?}",comment_single::<VerboseError<&str>>("*\nbb"));
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
    alt((
      terminated(
        take_until("*/"),
        tag("*/"),),
      terminated(
        space, 
        tag("*/"),
      ),
    )),
  )(i)
}


/// the root element of a JSON parser is either an object or an array
pub fn library_wrapper<'a>(
  i: &'a str,
) -> IResult<&'a str, Library, VerboseError<&str>> 
{
  context(
    "library_wrapper",
    map_res(
      pair(
        terminated(opt(comment), space_newline),
        terminated(key_value, space_newline),
      ),
      |(comment,attri)|(
        match attri {
            Attribute::ValuePair(k, v) => match v {
              LibertyValue::Simple(_) => Err(VerboseError::from_error_kind("No group in root", ErrorKind::Verify)),
              LibertyValue::Complex(_) => Err(VerboseError::from_error_kind("No group in root", ErrorKind::Verify)),
              LibertyValue::Group(content) => {
                match comment{
                    Some(comment) => Ok(Library { 
                      attribute: String::from(k), 
                      content,
                      comment,
                    }),
                    None => Ok(Library { 
                      attribute: String::from(k), 
                      content,
                      comment:vec![],
                    }),
                }
              },
            },
            Attribute::Comment(_) => todo!(),
        }
      ),
    )
  )(i)
}

impl FromStr for Library {
    type Err=std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      match library_wrapper(s) {
        Ok((_,l)) => Ok(l),
        Err(_) => Err(std::fmt::Error),
    }
    }
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
  static  TEMPLATE_ERR1: &str = r#"
  library (/home/jzzhou/project/liberate_demo/LIBRARY/debug_liberate_INV0_hspice) {
    comment : "www";
  }"#;
  static  TEMPLATE_ERR2: &str = r#"
  delay_model : table_lookup;
  in_place_swap_mode : match_footprint;
  time_unit : "1ns";
  voltage_unit : "1V";
  current_unit : "1uA";
  pulling_resistance_unit : "1kohm";
  leakage_power_unit : "1nW";
  capacitive_load_unit (1,pf);
  slew_upper_threshold_pct_rise : 80;
  "#;
  #[test]
  fn x6() {
    println!("{:?}", library_wrapper(TEMPLATE_ERR1));
    // match library_wrapper(TEMPLATE) {
    //     Ok((_,l)) => println!("{:?}",l),
    //     Result::Err(_) => todo!(),
    // }
    // println!("{:?}", library_wrapper(TEMPLATE_ERR1));
  }
}