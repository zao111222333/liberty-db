#![allow(clippy::arithmetic_side_effects)]
//!
//! All parser utilis.
//!
use crate::ast::GroupWrapper;
use nom::{
  IResult, Input as _, Parser as _,
  branch::alt,
  bytes::{
    complete::{tag, take, take_until, take_while},
    take_till,
  },
  character::complete::char,
  combinator::{map, map_opt, opt},
  error::{Error, ErrorKind},
  multi::{many0, separated_list0},
  sequence::{delimited, pair, preceded, terminated},
};
use std::collections::HashMap;

use super::ComplexWrapper;

#[inline]
fn comment_single(i: &str) -> IResult<&str, usize> {
  map(
    (
      alt((tag("*"), tag("//"))),
      take_while(move |c: char| c != '\n'),
      take(1_usize),
      space,
    ),
    |_| 1,
  )
  .parse(i)
}

#[inline]
fn comment_multi(i: &str) -> IResult<&str, usize> {
  map((tag("/*"), take_until("*/"), take(2_usize), space), |(_, s, _, _)| {
    s.chars().filter(|&x| x == '\n').count()
  })
  .parse(i)
}

#[cfg(test)]
mod test_comment {
  use super::*;
  #[test]
  fn test1() {
    println!("{:?}", comment_single("iwww\" \n \nw"));
    println!("{:?}", comment_single("*iwww\" \n \nw"));
    println!("{:?}", comment_single("//iwww\" \n \nw"));
    println!("{:?}", comment_multi("//iwww\" \n \nw"));
    println!(
      "{:?}",
      comment_multi(
        r#"/*iwww\

  */
  w"#
      )
    );
    println!(
      "{:?}",
      comment_multi(
        r#"/*iwww\ */
  w"#
      )
    );
  }
}

#[inline]
fn space(i: &str) -> IResult<&str, ()> {
  map(take_while(|c| matches!(c, '\t' | '\r' | ' ')), |_| ()).parse(i)
}
#[inline]
fn space_newline(i: &str) -> IResult<&str, usize> {
  map(take_while(|c| matches!(c, '\t' | '\n' | '\r' | ' ')), |s: &str| {
    s.chars().filter(|&x| x == '\n').count()
  })
  .parse(i)
}

/// must have new line!
#[inline]
pub(crate) fn comment_space_newline_many1(i: &str) -> IResult<&str, usize> {
  match map(
    pair(many0(pair(space_newline, alt((comment_single, comment_multi)))), space_newline),
    |(v, n3)| v.iter().map(|(n1, n2)| n1 + n2).sum::<usize>() + n3,
  )
  .parse(i)
  {
    Ok((s, n)) => {
      if n == 0 {
        Err(nom::Err::Error(Error::new(i, ErrorKind::Many1)))
      } else {
        Ok((s, n))
      }
    }
    Err(e) => Err(e),
  }
}

#[inline]
pub(crate) fn comment_space_newline(i: &str) -> IResult<&str, usize> {
  map(
    pair(many0(pair(space_newline, alt((comment_single, comment_multi)))), space_newline),
    |(v, n3)| v.iter().map(|(n1, n2)| n1 + n2).sum::<usize>() + n3,
  )
  .parse(i)
}

#[inline]
fn comment_space_newline_slash(i: &str) -> IResult<&str, usize> {
  map(
    pair(
      space,
      opt(preceded(
        preceded(char('\\'), space),
        alt((
          map_opt(
            pair(opt(comment_multi), space_newline),
            |(n_comment, n_newline)| match (n_comment, n_newline) {
              (_, 0) => None,
              (None | Some(0), _) => Some(n_newline),
              (_, _) => None,
            },
          ),
          comment_single,
        )),
      )),
    ),
    |(_, n)| n.unwrap_or(0),
  )
  .parse(i)
}

#[cfg(test)]
mod test_space {
  use super::*;
  #[test]
  fn space_test() {
    println!("{:?}", comment_space_newline_slash(r#" w"#));
    println!("{:?}", comment_space_newline_slash(r#" );"#));
    println!(
      "{:?}",
      comment_space_newline_slash(
        r#"/*iwww\
  
  */
  w"#
      )
    );
    println!(
      "{:?}",
      comment_space_newline_slash(
        r#"\ /*iwww\*/
  w"#
      )
    );
    println!(
      "{:?}",
      comment_space_newline_slash(
        r#"\
  w"#
      )
    );
    println!(
      "{:?}",
      comment_space_newline_slash(
        r#"\ //www
  w"#
      )
    );
  }
}
#[inline]
pub(crate) fn undefine<'a>(
  i: &'a str,
  group_name: &str,
  scope: &mut super::ParseScope,
) -> IResult<&'a str, super::UndefinedAttriValue> {
  let line_num_back: usize = scope.line_num;
  if let Ok((input, res)) = simple(i, &mut scope.line_num) {
    return Ok((input, super::UndefinedAttriValue::Simple(String::from(res))));
  }
  scope.line_num = line_num_back;
  if let Ok((input, vec)) = complex(i, &mut scope.line_num) {
    return Ok((
      input,
      super::UndefinedAttriValue::Complex(ComplexWrapper::collect(vec, scope)),
    ));
  }
  scope.line_num = line_num_back;
  match title(i, &mut scope.line_num) {
    Ok((mut input, title)) => {
      let mut res = GroupWrapper {
        title: title.into_iter().map(String::from).collect(),
        attri_map: HashMap::with_hasher(foldhash::fast::FixedState::default()),
      };
      loop {
        match key(input) {
          Err(nom::Err::Error(_)) => {
            (input, _) = end_group(input)?;
            let (new_input, n) = comment_space_newline(input)?;
            input = new_input;
            scope.line_num += n;
            return Ok((input, super::UndefinedAttriValue::Group(res)));
          }
          Err(e) => return Err(e),
          Ok((input1, key)) => {
            let (new_input, undefined) = undefine(input1, group_name, scope)?;
            input = new_input;
            super::attributs_set_undefined_attri(
              &mut res.attri_map,
              key,
              group_name,
              scope,
              undefined,
            );
          }
        }
      }
    }
    Err(e) => Err(e),
  }
}
#[inline]
pub(crate) fn unquote(i: &str) -> IResult<&str, &str> {
  delimited(char('"'), take_till(|c| c == '"'), take(1_usize)).parse(i)
}

#[cfg(test)]
mod test_unquote {
  use super::*;
  #[test]
  fn test1() {
    println!("{:?}", unquote("\"iwww\" "));
    println!("{:?}", key("iw_ww "));
    println!("{:?}", key("iw_w2w "));
    println!("{:?}", key("iw_w2w';"));
    println!("{:?}", formula("0.3 * VDD ;"));
  }
}

#[inline]
pub(crate) fn key(i: &str) -> IResult<&str, &str> {
  i.split_at_position1(|item| !(item.is_alphanumeric() || item == '_'), ErrorKind::Alpha)
}
#[inline]
pub(super) fn char_in_word(c: char) -> bool {
  c.is_alphanumeric() || "/_.+-:".contains(c)
}

#[inline]
pub(crate) fn word(i: &str) -> IResult<&str, &str> {
  i.split_at_position1(|item| !char_in_word(item), ErrorKind::Alpha)
}

#[inline]
pub(super) fn char_in_formula(c: char) -> bool {
  c.is_ascii_alphanumeric() || " /_.+-*^:".contains(c)
}

#[inline]
pub(crate) fn formula(i: &str) -> IResult<&str, &str> {
  i.split_at_position1(|item| !char_in_formula(item), ErrorKind::Alpha)
}

#[inline]
pub(crate) fn simple_multi<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, &'a str> {
  map(
    (
      space,
      char(':'),
      space,
      char('"'),
      take_while(move |c| c != '"'),
      take(1_usize),
      space,
      char(';'),
      comment_space_newline,
    ),
    |(_, _, _, _, s, _, _, _, n)| {
      *line_num += n + s.chars().filter(|&x| x == '\n').count();
      s
    },
  )
  .parse(i)
}
#[inline]
pub(crate) fn simple_custom<'a, T>(
  i: &'a str,
  line_num: &mut usize,
  func: fn(&'a str) -> IResult<&'a str, T>,
  unquote: fn(&str) -> IResult<&str, &str>,
) -> super::SimpleParseRes<'a, T> {
  map(
    (
      space,
      char(':'),
      space,
      alt((
        map(alt((func, delimited(char('"'), func, char('"')))), Ok),
        map(alt((word, unquote)), |s| Err(String::from(s))),
      )),
      alt((
        preceded(terminated(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    ),
    |(_, _, _, s, n)| {
      *line_num += n;
      s
    },
  )
  .parse(i)
}

#[inline]
pub(crate) fn simple<'a>(i: &'a str, line_num: &mut usize) -> IResult<&'a str, &'a str> {
  map(
    (
      space,
      char(':'),
      space,
      alt((unquote, word)),
      alt((
        preceded(terminated(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    ),
    |(_, _, _, s, n)| {
      *line_num += n;
      s
    },
  )
  .parse(i)
}
#[inline]
pub(crate) fn float_one(i: &str) -> IResult<&str, f64> {
  #[expect(clippy::string_slice)]
  match fast_float2::parse_partial(i) {
    Ok((f, pos)) => Ok((&i[pos..], f)),
    Err(_) => Err(nom::Err::Error(Error::new(i, ErrorKind::Float))),
  }
}
#[inline]
fn float_vec(i: &str) -> IResult<&str, Vec<f64>> {
  delimited(
    pair(char('"'), space),
    terminated(
      separated_list0(delimited(space, char(','), space), float_one),
      pair(opt(char(',')), space),
    ),
    char('"'),
  )
  .parse(i)
}

#[inline]
pub(crate) fn int_isize(i: &str) -> IResult<&str, isize> {
  #[expect(clippy::string_slice)]
  match lexical_core::parse_partial(i.as_bytes()) {
    Ok((n, pos)) => Ok((&i[pos..], n)),
    Err(_) => Err(nom::Err::Error(Error::new(i, ErrorKind::Digit))),
  }
}

#[inline]
pub(crate) fn int_usize(i: &str) -> IResult<&str, usize> {
  #[expect(clippy::string_slice)]
  match lexical_core::parse_partial(i.as_bytes()) {
    Ok((n, pos)) => Ok((&i[pos..], n)),
    Err(_) => Err(nom::Err::Error(Error::new(i, ErrorKind::Digit))),
  }
}

#[inline]
pub(crate) fn complex_id_vector<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, (usize, Vec<f64>)> {
  map(
    (
      space,
      char('('),
      comment_space_newline_slash,
      int_usize,
      space,
      char(','),
      comment_space_newline_slash,
      float_vec,
      comment_space_newline_slash,
      char(')'),
      alt((
        preceded(pair(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    ),
    |(_, _, n0, id, _, _, n1, vec, n2, _, n3)| {
      *line_num += n0 + n1 + n2 + n3;
      (id, vec)
    },
  )
  .parse(i)
}

#[inline]
pub(crate) fn complex_multi_line<'a, T>(
  i: &'a str,
  line_num: &mut usize,
  f: fn(i: &str) -> IResult<&str, T>,
) -> IResult<&'a str, Vec<(usize, T)>> {
  map(
    (
      space,
      char('('),
      comment_space_newline_slash,
      separated_list0(char(','), pair(comment_space_newline_slash, terminated(f, space))),
      opt(char(',')),
      comment_space_newline_slash,
      char(')'),
      alt((
        preceded(pair(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    ),
    |(_, _, n0, vec, _, n1, _, n2)| {
      *line_num += n0 + n1 + n2;
      vec
    },
  )
  .parse(i)
}

#[inline]
pub(crate) fn complex_float_vec<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<f64>> {
  map(
    (
      space,
      char('('),
      comment_space_newline_slash,
      float_vec,
      comment_space_newline_slash,
      char(')'),
      alt((
        preceded(pair(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    ),
    |(_, _, n0, vec, n1, _, n2)| {
      *line_num += n0 + n1 + n2;
      vec
    },
  )
  .parse(i)
}

#[inline]
pub(crate) fn complex_values<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<(usize, Vec<f64>)>> {
  complex_multi_line(i, line_num, float_vec)
}

#[inline]
pub(crate) fn complex_ccs_power_values<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<(usize, crate::table::CcsPowerValue)>> {
  #[inline]
  fn complex_ccs_power_value(
    i: &str,
  ) -> IResult<&str, crate::table::CcsPowerValue> {
    delimited(
      pair(char('"'), space),
      map(
        (
          terminated(float_one, delimited(space, char(','), space)),
          terminated(float_one, delimited(space, char(','), space)),
          separated_list0(
            delimited(space, char(','), space),
            map(
              (
                terminated(int_usize, delimited(space, char(','), space)),
                terminated(float_one, delimited(space, char(','), space)),
                float_one,
              ),
              |(bc_id, point_time, point_current)| crate::table::CcsPowerPoint {
                bc_id,
                point_time,
                point_current,
              },
            ),
          ),
          pair(opt(char(',')), space),
        ),
        |(init_time, init_current, points, _)| crate::table::CcsPowerValue {
          init_time,
          init_current,
          points,
        },
      ),
      char('"'),
    )
    .parse(i)
  }

  complex_multi_line(i, line_num, complex_ccs_power_value)
}

#[inline]
pub(crate) fn complex<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<(usize, &'a str)>> {
  map(
    (
      space,
      char('('),
      comment_space_newline_slash,
      separated_list0(
        char(','),
        pair(comment_space_newline_slash, terminated(alt((word, unquote)), space)),
      ),
      opt(char(',')),
      comment_space_newline_slash,
      char(')'),
      alt((
        preceded(pair(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    ),
    |(_, _, n0, vec, _, n1, _, n2)| {
      *line_num += n0 + n1 + n2;
      vec
    },
  )
  .parse(i)
}

#[cfg(test)]
mod test_key {
  use super::*;
  #[test]
  fn test_complex1() {
    assert_eq!(
      Ok(("}", vec![(0, "3"), (0, "4"), (0, "5")])),
      complex(r#" (3, 4, 5,); }"#, &mut 1)
    );
    assert_eq!(
      Ok(("}", vec![(0, "3"), (0, "4"), (0, "5")])),
      complex(r#" (3, 4, 5); }"#, &mut 1)
    );
    assert_eq!(
      Ok(("}", vec![(0, "3"), (0, "4"), (1, "5")])),
      complex(
        r#" (3, 4, \
        5); }"#,
        &mut 1
      )
    );
    assert_eq!(
      Ok(("}", vec![3.0, 4.0, 5.0,])),
      complex_float_vec(r#" ("3, 4, 5"); }"#, &mut 1)
    );
    assert_eq!(
      Ok(("}", vec![3.0, 4.0, 5.0,])),
      complex_float_vec(r#" ("3, 4, 5,"); }"#, &mut 1)
    );
    assert_eq!(
      Ok(("}", vec![(0, "Q1 Q2 Q3 "), (0, " QB1 QB2")])),
      complex(
        r#" ("Q1 Q2 Q3 ", " QB1 QB2") ;
        }"#,
        &mut 1
      )
    );
    assert_eq!(
      Ok((
        "}",
        vec![(
          0,
          "init_time, init_current, bc_id1, point_time1, point_current1, bc_id2, [point_time2, point_current2, bc_id3, ...], end_time, end_current"
        )]
      )),
      complex(
        r#" ("init_time, init_current, bc_id1, point_time1, point_current1, bc_id2, [point_time2, point_current2, bc_id3, ...], end_time, end_current") ;
        }"#,
        &mut 1
      )
    );
  }

  #[test]
  fn test1() {
    println!("{:?}", comment_space_newline("\n\r\t\n : b ; "));
    println!("{:?}", simple(" : b; }", &mut 1));
    println!("{:?}", simple(" : iwww ; ", &mut 1));
    println!("{:?}", simple(" : 0.3 * VDD ;", &mut 1));
    println!("{:?}", key("iwww "));
    println!("{:?}", key("iw_ww "));
    println!("{:?}", key("iw_w2w "));
    println!("{:?}", key("iw_w2w';"));
  }
}

#[inline]
pub(crate) fn title<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<&'a str>> {
  map(
    (
      space,
      char('('),
      separated_list0(char(','), delimited(space, alt((unquote, word)), space)),
      char(')'),
      space,
      char('{'),
      comment_space_newline,
    ),
    |(_, _, v, _, _, _, n)| {
      *line_num += n;
      v
    },
  )
  .parse(i)
}

#[inline]
pub(crate) fn end_group(i: &str) -> IResult<&str, ()> {
  map(char('}'), |_| ()).parse(i)
}

#[cfg(test)]
mod test_end_group {
  use super::*;
  #[test]
  fn test1() {
    println!("{:?}", end_group("}"));
    println!("{:?}", end_group("}\n"));
  }
}
