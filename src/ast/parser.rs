#![allow(clippy::arithmetic_side_effects)]
//!
//! All parser utilis.
//!
use crate::{ast::GroupWrapper, ArcStr, NotNan};
use nom::{
  branch::alt,
  bytes::complete::{escaped, is_not, tag, take, take_until, take_while},
  character::complete::{char, digit1, one_of},
  combinator::{map, map_opt, opt},
  error::{ContextError, Error, ErrorKind, FromExternalError, ParseError},
  multi::{many0, separated_list0},
  sequence::{delimited, pair, preceded, terminated, tuple},
  IResult, InputTakeAtPosition,
};
use std::collections::HashMap;

use super::{ComplexParseError, ComplexWrapper};

#[inline]
fn comment_single(i: &str) -> IResult<&str, usize, Error<&str>> {
  map(
    tuple((
      alt((tag("*"), tag("//"))),
      take_while(move |c: char| c != '\n'),
      take(1_usize),
      space,
    )),
    |_| 1,
  )(i)
}

#[inline]
fn comment_multi(i: &str) -> IResult<&str, usize, Error<&str>> {
  map(tuple((tag("/*"), take_until("*/"), take(2_usize), space)), |(_, s, _, _)| {
    s.chars().filter(|&x| x == '\n').count()
  })(i)
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
fn space(i: &str) -> IResult<&str, (), Error<&str>> {
  map(take_while(move |c: char| matches!(c, '\t' | '\r' | ' ')), |_| ())(i)
}
#[inline]
fn space_newline(i: &str) -> IResult<&str, usize, Error<&str>> {
  map(take_while(move |c: char| matches!(c, '\t' | '\n' | '\r' | ' ')), |s: &str| {
    s.chars().filter(|&x| x == '\n').count()
  })(i)
}

/// must have new line!
#[inline]
pub(crate) fn comment_space_newline_many1(i: &str) -> IResult<&str, usize, Error<&str>> {
  match map(
    pair(many0(pair(space_newline, alt((comment_single, comment_multi)))), space_newline),
    |(v, n3)| v.iter().map(|(n1, n2)| n1 + n2).sum::<usize>() + n3,
  )(i)
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
pub(crate) fn comment_space_newline(i: &str) -> IResult<&str, usize, Error<&str>> {
  map(
    pair(many0(pair(space_newline, alt((comment_single, comment_multi)))), space_newline),
    |(v, n3)| v.iter().map(|(n1, n2)| n1 + n2).sum::<usize>() + n3,
  )(i)
}

#[inline]
fn comment_space_newline_slash(i: &str) -> IResult<&str, usize, Error<&str>> {
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
  )(i)
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
) -> IResult<&'a str, super::UndefinedAttriValue, Error<&'a str>> {
  let line_num_back: usize = scope.line_num;
  if let Ok((input, res)) = simple(i, &mut scope.line_num) {
    return Ok((input, super::UndefinedAttriValue::Simple(ArcStr::from(res))));
  }
  scope.line_num = line_num_back;
  match complex1(i, &mut scope.line_num, complex_words) {
    Ok((input, Ok(vec))) => {
      return Ok((
        input,
        super::UndefinedAttriValue::Complex(ComplexWrapper::collect(vec, scope)),
      ))
    }
    Ok((_, Err(_))) => unreachable!(),
    _ => (),
  }
  scope.line_num = line_num_back;
  match title(i, &mut scope.line_num) {
    Ok((mut input, title)) => {
      let mut res = GroupWrapper {
        title: title.into_iter().map(ArcStr::from).collect(),
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
pub(crate) fn unquote<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  delimited(
    char('"'),
    escaped(opt(alt((tag(r#"\""#), is_not(r#"\""#)))), '\\', one_of(r#"\"rnt"#)),
    char('"'),
  )(i)
}

// #[inline]
// pub(crate) fn word_list<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
// where
//   E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
// {
//   escaped(opt(alt((tag(r#"\""#), is_not(r#"\""#)))), '\\', one_of(r#"\"rnt"#))(i)
// }

#[cfg(test)]
mod test_unquote {
  use super::*;
  #[test]
  fn test1() {
    use nom::error::VerboseError;
    println!("{:?}", unquote::<VerboseError<&str>>("\"iwww\" "));
    println!("{:?}", key::<VerboseError<&str>>("iw_ww "));
    println!("{:?}", key::<VerboseError<&str>>("iw_w2w "));
    println!("{:?}", key::<VerboseError<&str>>("iw_w2w';"));
    println!("{:?}", formula::<VerboseError<&str>>("0.3 * VDD ;"));
  }
}

#[inline]
pub(crate) fn key<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  i.split_at_position1(|item| !(item.is_alphanumeric() || item == '_'), ErrorKind::Alpha)
}
#[inline]
pub(super) fn char_in_word(c: char) -> bool {
  c.is_alphanumeric() || "/_.+-:".contains(c)
}

#[inline]
pub(crate) fn word<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  i.split_at_position1(|item| !char_in_word(item), ErrorKind::Alpha)
}

#[inline]
pub(crate) fn parse_arcstr<'a, E>(i: &'a str) -> IResult<&'a str, ArcStr, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  map(word, ArcStr::from)(i)
}

#[inline]
pub(super) fn char_in_formula(c: char) -> bool {
  c.is_ascii_alphanumeric() || " /_.+-*^:".contains(c)
}

#[inline]
pub(crate) fn formula<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  i.split_at_position1(|item| !char_in_formula(item), ErrorKind::Alpha)
}

#[inline]
pub(crate) fn simple_multi<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, &'a str, Error<&'a str>> {
  map(
    tuple((
      space,
      char(':'),
      space,
      char('"'),
      take_while(move |c| c != '"'),
      take(1_usize),
      space,
      char(';'),
      comment_space_newline,
    )),
    |(_, _, _, _, s, _, _, _, n)| {
      *line_num += n + s.chars().filter(|&x| x == '\n').count();
      s
    },
  )(i)
}

#[inline]
pub(crate) fn simple_custom<'a, T>(
  i: &'a str,
  line_num: &mut usize,
  func: fn(&'a str) -> IResult<&'a str, T, Error<&'a str>>,
) -> super::SimpleParseRes<'a, T> {
  map(
    tuple((
      space,
      char(':'),
      space,
      alt((
        map(alt((func, delimited(char('"'), func, char('"')))), Ok),
        map(alt((word, unquote)), |s| Err(ArcStr::from(s))),
      )),
      alt((
        preceded(terminated(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    )),
    |(_, _, _, s, n)| {
      *line_num += n;
      s
    },
  )(i)
}

#[inline]
pub(crate) fn simple_basic<'a, T>(
  i: &'a str,
  line_num: &mut usize,
  f: fn(&'a str) -> IResult<&'a str, T, Error<&'a str>>,
) -> super::SimpleParseRes<'a, T> {
  map(
    tuple((
      space,
      char(':'),
      space,
      alt((
        map(alt((f, delimited(char('\"'), f, char('\"')))), |t| Ok(t)),
        map(alt((word, unquote)), |s| Err(ArcStr::from(s))),
      )),
      alt((
        preceded(terminated(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    )),
    |(_, _, _, s, n)| {
      *line_num += n;
      s
    },
  )(i)
}

#[inline]
pub(crate) fn simple<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, &'a str, Error<&'a str>> {
  map(
    tuple((
      space,
      char(':'),
      space,
      alt((word, unquote)),
      alt((
        preceded(terminated(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    )),
    |(_, _, _, s, n)| {
      *line_num += n;
      s
    },
  )(i)
}
#[inline]
pub(crate) fn parse_float(i: &str) -> IResult<&str, NotNan<f64>, Error<&str>> {
  #[expect(clippy::string_slice, clippy::undocumented_unsafe_blocks)]
  match fast_float2::parse_partial(i) {
    Ok((f, pos)) => Ok((&i[pos..], unsafe { NotNan::new_unchecked(f) })),
    Err(_) => Err(nom::Err::Error(Error::new(i, ErrorKind::Float))),
  }
}
#[inline]
pub(crate) fn float_vec(i: &str) -> IResult<&str, Vec<NotNan<f64>>, Error<&str>> {
  delimited(
    pair(char('"'), space),
    terminated(
      separated_list0(delimited(space, char(','), space), parse_float),
      pair(opt(char(',')), space),
    ),
    char('"'),
  )(i)
}

#[inline]
pub(crate) fn parse_usize(i: &str) -> IResult<&str, usize, Error<&str>> {
  #[expect(clippy::unwrap_used)]
  map(digit1, |s: &str| s.parse().unwrap())(i)
}

#[inline]
pub(crate) fn parse_bool(i: &str) -> IResult<&str, bool, Error<&str>> {
  alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(i)
}

#[inline]
pub(crate) fn complex_multi_line<'a, T, F: nom::Parser<&'a str, T, Error<&'a str>>>(
  i: &'a str,
  line_num: &mut usize,
  f: F,
) -> super::ComplexParseRes<'a, Vec<(usize, T)>> {
  complex1(
    i,
    line_num,
    separated_list0(char(','), pair(comment_space_newline_slash, terminated(f, space))),
  )
}

#[inline]
pub(crate) fn complex1<'a, T, F: nom::Parser<&'a str, T, Error<&'a str>>>(
  i: &'a str,
  line_num: &mut usize,
  f: F,
) -> super::ComplexParseRes<'a, T> {
  let res = map(
    tuple((
      space,
      char('('),
      comment_space_newline_slash,
      alt((
        map(f, |res| Ok(res)),
        map(complex_words, |words: Vec<(usize, &str)>| Err(words)),
      )),
      opt(char(',')),
      comment_space_newline_slash,
      char(')'),
      alt((
        preceded(pair(space, char(';')), comment_space_newline),
        comment_space_newline_many1,
      )),
    )),
    |(_, _, n0, t, _, n1, _, n2)| {
      *line_num += n0 + n1 + n2;
      t
    },
  )(i);
  match res {
    Ok((i, Ok(t))) => Ok((i, Ok(t))),
    Ok((i, Err(words))) => Ok((
      i,
      Err((
        ComplexParseError::Other,
        ComplexWrapper(
          words
            .into_iter()
            .map(|(n, s)| {
              *line_num += n;
              ArcStr::from(s)
            })
            .collect(),
        ),
      )),
    )),
    Err(e) => Err(e),
  }
}

#[inline]
pub(crate) fn complex2<
  'a,
  T1,
  F1: nom::Parser<&'a str, T1, Error<&'a str>>,
  T2,
  F2: nom::Parser<&'a str, T2, Error<&'a str>>,
  T,
  F: FnMut(T1, T2) -> T,
>(
  i: &'a str,
  line_num: &mut usize,
  f1: F1,
  f2: F2,
  mut f: F,
) -> super::ComplexParseRes<'a, T> {
  match complex1(
    i,
    line_num,
    tuple((f1, space, char(','), comment_space_newline_slash, f2)),
  ) {
    Ok((_i, Ok((t1, _, _, n, t2)))) => {
      *line_num += n;
      Ok((_i, Ok(f(t1, t2))))
    }
    Ok((_i, Err(e))) => Ok((_i, Err(e))),
    Err(e) => Err(e),
  }
}
#[inline]
pub(crate) fn complex2_opt<
  'a,
  T1,
  F1: nom::Parser<&'a str, T1, Error<&'a str>>,
  T2,
  F2: nom::Parser<&'a str, T2, Error<&'a str>>,
  T,
  F: Fn(T1, Option<T2>) -> T,
>(
  i: &'a str,
  line_num: &mut usize,
  f1: F1,
  f2: F2,
  f: F,
) -> super::ComplexParseRes<'a, T> {
  match complex1(
    i,
    line_num,
    tuple((f1, opt(tuple((space, char(','), comment_space_newline_slash, f2))))),
  ) {
    Ok((_i, Ok((t1, None)))) => Ok((_i, Ok(f(t1, None)))),
    Ok((_i, Ok((t1, Some((_, _, n, t2)))))) => {
      *line_num += n;
      Ok((_i, Ok(f(t1, Some(t2)))))
    }
    Ok((_i, Err(e))) => Ok((_i, Err(e))),
    Err(e) => Err(e),
  }
}

#[inline]
pub(crate) fn complex5_opt<
  'a,
  T1,
  F1: nom::Parser<&'a str, T1, Error<&'a str>>,
  T2,
  F2: nom::Parser<&'a str, T2, Error<&'a str>>,
  T3,
  F3: nom::Parser<&'a str, T3, Error<&'a str>>,
  T4,
  F4: nom::Parser<&'a str, T4, Error<&'a str>>,
  T5,
  F5: nom::Parser<&'a str, T5, Error<&'a str>>,
  T,
  F: Fn(T1, T2, Option<(T3, T4, T5)>) -> T,
>(
  i: &'a str,
  line_num: &mut usize,
  f1: F1,
  f2: F2,
  f3: F3,
  f4: F4,
  f5: F5,
  f: F,
) -> super::ComplexParseRes<'a, T> {
  match complex1(
    i,
    line_num,
    tuple((
      f1,
      space,
      char(','),
      comment_space_newline_slash,
      f2,
      opt(tuple((
        space,
        char(','),
        comment_space_newline_slash,
        f3,
        space,
        char(','),
        comment_space_newline_slash,
        f4,
        space,
        char(','),
        comment_space_newline_slash,
        f5,
      ))),
    )),
  ) {
    Ok((_i, Ok((t1, _, _, n1, t2, None)))) => {
      *line_num += n1;
      Ok((_i, Ok(f(t1, t2, None))))
    }
    Ok((
      _i,
      Ok((t1, _, _, n1, t2, Some((_, _, n2, t3, _, _, n3, t4, _, _, n4, t5)))),
    )) => {
      *line_num += n1 + n2 + n3 + n4;
      Ok((_i, Ok(f(t1, t2, Some((t3, t4, t5))))))
    }
    Ok((_i, Err(e))) => Ok((_i, Err(e))),
    Err(e) => Err(e),
  }
}

#[inline]
pub(crate) fn complex3<
  'a,
  T1,
  F1: nom::Parser<&'a str, T1, Error<&'a str>>,
  T2,
  F2: nom::Parser<&'a str, T2, Error<&'a str>>,
  T3,
  F3: nom::Parser<&'a str, T3, Error<&'a str>>,
  T,
  F: FnMut(T1, T2, T3) -> T,
>(
  i: &'a str,
  line_num: &mut usize,
  f1: F1,
  f2: F2,
  f3: F3,
  mut f: F,
) -> super::ComplexParseRes<'a, T> {
  match complex1(
    i,
    line_num,
    tuple((
      f1,
      space,
      char(','),
      comment_space_newline_slash,
      f2,
      space,
      char(','),
      comment_space_newline_slash,
      f3,
    )),
  ) {
    Ok((_i, Ok((t1, _, _, n1, t2, _, _, n2, t3)))) => {
      *line_num += n1 + n2;
      Ok((_i, Ok(f(t1, t2, t3))))
    }
    Ok((_i, Err(e))) => Ok((_i, Err(e))),
    Err(e) => Err(e),
  }
}

#[inline]
pub(crate) fn complex_ccs_power_value(
  i: &str,
) -> IResult<&str, crate::common::table::CcsPowerValue, Error<&str>> {
  delimited(
    pair(char('"'), space),
    map(
      tuple((
        terminated(parse_float, delimited(space, char(','), space)),
        terminated(parse_float, delimited(space, char(','), space)),
        separated_list0(
          delimited(space, char(','), space),
          map(
            tuple((
              terminated(parse_usize, delimited(space, char(','), space)),
              terminated(parse_float, delimited(space, char(','), space)),
              parse_float,
            )),
            |(bc_id, point_time, point_current)| crate::common::table::CcsPowerPoint {
              bc_id,
              point_time,
              point_current,
            },
          ),
        ),
        pair(opt(char(',')), space),
      )),
      |(init_time, init_current, points, _)| crate::common::table::CcsPowerValue {
        init_time,
        init_current,
        points,
      },
    ),
    char('"'),
  )(i)
}

#[expect(clippy::type_complexity)]
#[inline]
pub(crate) fn complex_words<'a>(
  i: &'a str,
) -> IResult<&'a str, Vec<(usize, &'a str)>, Error<&'a str>> {
  separated_list0(
    char(','),
    pair(comment_space_newline_slash, terminated(alt((word, unquote)), space)),
  )(i)
}

// #[expect(clippy::type_complexity)]
// #[inline]
// pub(crate) fn complex<'a>(
//   i: &'a str,
//   line_num: &mut usize,
// ) -> IResult<&'a str, Vec<(usize, &'a str)>, Error<&'a str>> {
//   complex1(i, line_num, complex_words)
// }

#[cfg(test)]
mod test_key {
  use super::*;
  // #[test]
  // fn test_complex1() {
  //   assert_eq!(
  //     Ok(("}", vec![(0, "3"), (0, "4"), (0, "5")])),
  //     complex(r#" (3, 4, 5,); }"#, &mut 1)
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![(0, "3"), (0, "4"), (0, "5")])),
  //     complex(r#" (3, 4, 5); }"#, &mut 1)
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![(0, "3"), (0, "4"), (1, "5")])),
  //     complex(
  //       r#" (3, 4, \
  //       5); }"#,
  //       &mut 1
  //     )
  //   );
  //   assert_eq!(
  //     Ok((
  //       "}",
  //       vec![
  //         unsafe { NotNan::new_unchecked(3.0) },
  //         unsafe { NotNan::new_unchecked(4.0) },
  //         unsafe { NotNan::new_unchecked(5.0) },
  //       ]
  //     )),
  //     complex_float_vec(r#" ("3, 4, 5"); }"#, &mut 1)
  //   );
  //   assert_eq!(
  //     Ok((
  //       "}",
  //       vec![
  //         unsafe { NotNan::new_unchecked(3.0) },
  //         unsafe { NotNan::new_unchecked(4.0) },
  //         unsafe { NotNan::new_unchecked(5.0) },
  //       ]
  //     )),
  //     complex_float_vec(r#" ("3, 4, 5,"); }"#, &mut 1)
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![(0, "Q1 Q2 Q3 "), (0, " QB1 QB2")])),
  //     complex(
  //       r#" ("Q1 Q2 Q3 ", " QB1 QB2") ;
  //       }"#,
  //       &mut 1
  //     )
  //   );
  // }
  // #[test]
  // fn test_complex() {
  //   assert_eq!(
  //     Ok(("}", vec![("3", 0), ("4", 0), ("5", 0)])),
  //     complex(r#" (3, 4, 5); }"#, &mut 1)
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![("1", 0), ("2", 0), ("3", 0)])),
  //     complex(r#" (1,2,3); }"#, &mut 1)
  //   );
  //   assert_eq!(Ok(("}", vec![("1", 0)])), complex(r#" (1); }"#, &mut 1));
  //   assert_eq!(
  //     Ok(("}", vec![("1", 0), ("2", 0), ("3", 0)])),
  //     complex(
  //       r#" (1,2,3)
  //    }"#,
  //       &mut 1
  //     )
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![("1", 0), ("2", 0), ("3", 0)])),
  //     complex(
  //       r#" ("1,2,", 3 );
  //         }"#,
  //       &mut 1
  //     )
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![("1", 0), ("2", 0), ("3", 0)])),
  //     complex(
  //       r#" ( \
  //           1,2,3 \
  //         )
  //    }"#,
  //       &mut 1
  //     )
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![("1", 0), ("2", 1), ("3", 0), ("4", 0)])),
  //     complex(
  //       r#" (1,2,\
  //             3,4);
  //       }"#,
  //       &mut 1
  //     )
  //   );
  //   assert_eq!(
  //     Ok(("}", vec![("1", 0), ("2", 1), ("3", 0)])),
  //     complex(
  //       r#" (1,2,\
  //             3,);
  //       }"#,
  //       &mut 1
  //     )
  //   );

  //   // assert_eq!(
  //   //   Ok(("}", vec![(vec!["Q1 Q2 Q3", "QB1 QB2"], 0)])),
  //   //   complex(
  //   //     r#" (" Q1 Q2 Q3 ", "QB1 QB2") ;
  //   //     }"#,
  //   //     &mut 1
  //   //   )
  //   // );
  //   assert_eq!(
  //     Ok((
  //       "}",
  //       vec![
  //         ("init_time", 0),
  //         ("init_current", 0),
  //         ("bc_id1", 0),
  //         ("point_time1", 0),
  //         ("point_current1", 0),
  //         ("bc_id2", 0),
  //         ("[point_time2", 0),
  //         ("point_current2", 0),
  //         ("bc_id3", 0),
  //         ("...]", 0),
  //         ("end_time", 0),
  //         ("end_current", 0),
  //       ],
  //     )),
  //     complex(
  //       r#" (init_time, init_current, bc_id1, point_time1, point_current1, bc_id2, [point_time2, point_current2, bc_id3, ...], end_time, end_current);
  //       }"#,
  //       &mut 1
  //     )
  //   );
  // }
  #[test]
  fn test1() {
    use nom::error::VerboseError;
    println!("{:?}", comment_space_newline("\n\r\t\n : b ; "));
    println!("{:?}", simple(" : b; }", &mut 1));
    println!("{:?}", simple(" : iwww ; ", &mut 1));
    println!("{:?}", simple(" : 0.3 * VDD ;", &mut 1));
    println!("{:?}", key::<VerboseError<&str>>("iwww "));
    println!("{:?}", key::<VerboseError<&str>>("iw_ww "));
    println!("{:?}", key::<VerboseError<&str>>("iw_w2w "));
    println!("{:?}", key::<VerboseError<&str>>("iw_w2w';"));
  }
}

#[inline]
pub(crate) fn title<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<&'a str>, Error<&'a str>> {
  map(
    tuple((
      space,
      char('('),
      separated_list0(char(','), delimited(space, alt((word, unquote)), space)),
      char(')'),
      space,
      char('{'),
      comment_space_newline,
    )),
    |(_, _, v, _, _, _, n)| {
      *line_num += n;
      v
    },
  )(i)
}

#[inline]
pub(crate) fn end_group(i: &str) -> IResult<&str, (), Error<&str>> {
  map(char('}'), |_| ())(i)
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
