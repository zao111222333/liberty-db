//!
//! All parser utilis.
//!
use nom::{
  branch::alt,
  bytes::streaming::{escaped, is_not, tag, take, take_until, take_while},
  character::streaming::{char, one_of},
  combinator::{map, map_opt, opt},
  error::{ContextError, Error, ErrorKind, FromExternalError, ParseError},
  multi::{many0, separated_list0},
  sequence::{delimited, pair, preceded, terminated, tuple},
  IResult, InputTakeAtPosition,
};

use crate::{ast::GroupWrapper, ArcStr};

fn comment_single<'a>(i: &'a str) -> IResult<&'a str, usize, Error<&'a str>> {
  map(
    tuple((
      alt((tag("*"), tag("//"))),
      take_while(move |c: char| c != '\n'),
      take(1usize),
      space,
    )),
    |_| 1,
  )(i)
}

fn comment_multi<'a>(i: &'a str) -> IResult<&'a str, usize, Error<&'a str>> {
  map(tuple((tag("/*"), take_until("*/"), take(2usize), space)), |(_, s, _, _)| {
    s.chars().filter(|&x| x == '\n').count()
  })(i)
}

#[test]
fn comment_test() {
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

#[inline]
fn space<'a>(i: &'a str) -> IResult<&'a str, (), Error<&'a str>> {
  map(take_while(move |c: char| matches!(c, '\t' | '\r' | ' ')), |_| ())(i)
}
#[inline]
fn space_newline<'a>(i: &'a str) -> IResult<&'a str, usize, Error<&'a str>> {
  map(take_while(move |c: char| matches!(c, '\t' | '\n' | '\r' | ' ')), |s: &str| {
    s.chars().filter(|&x| x == '\n').count()
  })(i)
}

/// must have new line!
#[inline]
pub(in crate) fn comment_space_newline_many1<'a>(
  i: &'a str,
) -> IResult<&'a str, usize, Error<&'a str>> {
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
pub(in crate) fn comment_space_newline<'a>(
  i: &'a str,
) -> IResult<&'a str, usize, Error<&'a str>> {
  map(
    pair(many0(pair(space_newline, alt((comment_single, comment_multi)))), space_newline),
    |(v, n3)| v.iter().map(|(n1, n2)| n1 + n2).sum::<usize>() + n3,
  )(i)
}

#[inline]
fn comment_space_newline_slash<'a>(
  i: &'a str,
) -> IResult<&'a str, usize, Error<&'a str>> {
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
              (Some(0), _) => Some(n_newline),
              (None, _) => Some(n_newline),
              (_, _) => None,
            },
          ),
          comment_single,
        )),
      )),
    ),
    |(_, n)| match n {
      Some(n) => n,
      None => 0,
    },
  )(i)
}

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

#[inline]
pub(in crate) fn undefine<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, super::AttriValue, Error<&'a str>> {
  let line_num_back = line_num.clone();
  if let Ok((input, res)) = simple(i, line_num) {
    return Ok((input, super::AttriValue::Simple(ArcStr::from(res))));
  }
  *line_num = line_num_back;
  if let Ok((input, res)) = complex(i, line_num) {
    return Ok((
      input,
      super::AttriValue::Complex(vec![res.into_iter().map(ArcStr::from).collect()]),
    ));
  }
  *line_num = line_num_back;
  match title(i, line_num) {
    Ok((mut input, title)) => {
      let mut res = GroupWrapper { title, attr_list: vec![] };
      loop {
        match key(input) {
          Err(nom::Err::Error(_)) => {
            (input, _) = end_group(input)?;
            return Ok((input, super::AttriValue::Group(res)));
          }
          Err(e) => return Err(e),
          Ok((_input, _key)) => {
            input = _input;
            if let Ok((_input, attri)) = undefine(_input, line_num) {
              input = _input;
              if let super::AttriValue::Group(_) = attri {
                let n: usize;
                (input, n) = comment_space_newline(input)?;
                *line_num += n;
              }
              res.attr_list.push((ArcStr::from(_key), attri));
            }
          }
        }
      }
    }
    Err(e) => return Err(e),
  }
}

#[inline]
fn unquote<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  delimited(
    char('"'),
    escaped(opt(alt((tag(r#"\""#), is_not(r#"\""#)))), '\\', one_of(r#"\"rnt"#)),
    char('"'),
  )(i)
}

#[test]
fn unquote_test() {
  use nom::error::VerboseError;
  println!("{:?}", unquote::<VerboseError<&str>>("\"iwww\" "));
  println!("{:?}", key::<VerboseError<&str>>("iw_ww "));
  println!("{:?}", key::<VerboseError<&str>>("iw_w2w "));
  println!("{:?}", key::<VerboseError<&str>>("iw_w2w';"));
  println!("{:?}", formula::<VerboseError<&str>>("0.3 * VDD ;"));
}

pub(in crate) fn key<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  i.split_at_position1(|item| !(item.is_alphanumeric() || item == '_'), ErrorKind::Alpha)
}
#[inline]
pub(super) fn char_in_word(c: char) -> bool {
  c.is_alphanumeric() || "/_.+-:".contains(c)
}

pub(in crate) fn word<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  i.split_at_position1(|item| !char_in_word(item), ErrorKind::Alpha)
}

#[inline]
pub(super) fn char_in_formula(c: char) -> bool {
  c.is_ascii_alphanumeric() || " /_.+-*^:".contains(c)
}

pub(in crate) fn formula<'a, E>(i: &'a str) -> IResult<&'a str, &'a str, E>
where
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, E>,
{
  i.split_at_position1(|item| !char_in_formula(item), ErrorKind::Alpha)
}

pub(in crate) fn simple_multi<'a>(
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
      take(1usize),
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

pub(in crate) fn simple<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, &'a str, Error<&'a str>> {
  map(
    tuple((
      space,
      char(':'),
      space,
      alt((
        tuple((unquote, preceded(terminated(space, char(';')), comment_space_newline))),
        tuple((unquote, comment_space_newline_many1)),
        tuple((word, preceded(terminated(space, char(';')), comment_space_newline))),
        tuple((word, comment_space_newline_many1)),
        tuple((formula, preceded(terminated(space, char(';')), comment_space_newline))),
      )),
    )),
    |(_, _, _, (s, n))| {
      *line_num += n;
      s
    },
  )(i)
}

fn complex_complex<'a>(i: &'a str) -> IResult<&'a str, Vec<&'a str>, Error<&'a str>> {
  let (input, words) = unquote(i)?;
  Ok((
    input,
    words
      .split(',')
      .filter_map(|s| {
        let _s = s.trim();
        if _s == "" {
          None
        } else {
          Some(_s)
        }
      })
      .collect(),
  ))
}

pub(in crate) fn complex<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<&'a str>, Error<&'a str>> {
  map(
    tuple((
      space,
      char('('),
      comment_space_newline_slash,
      many0(pair(
        alt((map(word, |s| vec![s]), complex_complex)),
        tuple((space, char(','), comment_space_newline_slash)),
      )),
      opt(pair(
        alt((complex_complex, map(word, |s| vec![s]))),
        comment_space_newline_slash,
      )),
      char(')'),
      space,
      alt((preceded(char(';'), comment_space_newline), comment_space_newline_many1)),
    )),
    |(_, _, n0, res, last, _, _, n1)| {
      *line_num += n0 + n1;
      let mut vec: Vec<&'a str> = res
        .into_iter()
        .map(|(v, (_, _, n))| {
          *line_num += n;
          v
        })
        .flatten()
        .collect();
      if let Some((last_vec, n)) = last {
        *line_num += n;
        vec.extend(last_vec);
      }
      vec
    },
  )(i)
}

#[test]
fn key_test() {
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

pub(in crate) fn title<'a>(
  i: &'a str,
  line_num: &mut usize,
) -> IResult<&'a str, Vec<ArcStr>, Error<&'a str>> {
  map(
    tuple((
      space,
      char('('),
      separated_list0(char(','), delimited(space, alt((unquote, word)), space)),
      char(')'),
      space,
      char('{'),
      comment_space_newline,
    )),
    |(_, _, v, _, _, _, n)| {
      *line_num += n;
      v.into_iter().map(ArcStr::from).collect()
    },
  )(i)
}

pub(in crate) fn end_group<'a>(i: &'a str) -> IResult<&'a str, (), Error<&'a str>> {
  map(char('}'), |_| ())(i)
}

#[test]
fn end_group_test() {
  println!("{:?}", end_group("}"));
  println!("{:?}", end_group("}\n"));
}
