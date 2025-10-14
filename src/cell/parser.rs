#![expect(clippy::arithmetic_side_effects)]
use crate::ast::parser::{comment_space_newline, space, space_newline};
use core::{
  iter::zip,
  sync::atomic::{AtomicUsize, Ordering},
};
use nom::{
  IResult, Parser as _,
  branch::alt,
  bytes::complete::{tag, take, take_while},
  character::complete::char,
  combinator::{map, map_res, opt},
  multi::{many1, separated_list1},
  sequence::{preceded, terminated},
};

impl super::InputNodeValue {
  #[inline]
  fn parse(i: &str) -> IResult<&str, Self> {
    alt((
      map(tag("-"), |_| Self::DontCare),
      map(tag("L/H"), |_| Self::LH),
      map(tag("H/L"), |_| Self::HL),
      map(tag("L"), |_| Self::L),
      map(tag("H"), |_| Self::H),
      map(tag("R"), |_| Self::R),
      map(tag("F"), |_| Self::F),
      map(tag("~R"), |_| Self::NotR),
      map(tag("~F"), |_| Self::NotF),
    ))
    .parse_complete(i)
  }
}
impl super::CurrentInternalNodeValue {
  #[inline]
  fn parse(i: &str) -> IResult<&str, Self> {
    alt((
      map(tag("-"), |_| Self::DontCare),
      map(tag("L/H"), |_| Self::LH),
      map(tag("H/L"), |_| Self::HL),
      map(tag("L"), |_| Self::L),
      map(tag("H"), |_| Self::H),
    ))
    .parse_complete(i)
  }
}
impl super::NextInternalNodeValue {
  #[inline]
  fn parse(i: &str) -> IResult<&str, Self> {
    alt((
      map(tag("-"), |_| Self::NotSpecified),
      map(tag("L/H"), |_| Self::LH),
      map(tag("H/L"), |_| Self::HL),
      map(tag("L"), |_| Self::L),
      map(tag("H"), |_| Self::H),
      map(tag("X"), |_| Self::X),
      map(tag("N"), |_| Self::N),
    ))
    .parse_complete(i)
  }
}
static TABLE_LINE_NUM: AtomicUsize = AtomicUsize::new(0);
impl super::TableNodeValues {
  #[inline]
  fn parse(i: &str) -> IResult<&str, Self> {
    fn sep(i: &str) -> IResult<&str, Option<()>> {
      preceded(
        space,
        opt(map((char('\\'), space_newline), |(_, n)| {
          _ = TABLE_LINE_NUM.fetch_add(n, Ordering::SeqCst);
        })),
      )
      .parse_complete(i)
    }
    map_res(
      (
        sep,
        many1(terminated(super::InputNodeValue::parse, sep)),
        char(':'),
        sep,
        many1(terminated(super::CurrentInternalNodeValue::parse, sep)),
        char(':'),
        sep,
        many1(terminated(super::NextInternalNodeValue::parse, sep)),
      ),
      |(
        _,
        input_node_values,
        _,
        _,
        current_internal_node_values,
        _,
        _,
        next_internal_node_values,
      )| {
        if current_internal_node_values.len() == next_internal_node_values.len() {
          Ok(Self {
            input_node_values,
            current_next_internal_node_values: zip(
              current_internal_node_values,
              next_internal_node_values,
            )
            .collect(),
          })
        } else {
          Err("current_internal_node_values & next_internal_node_values number mismatch")
        }
      },
    )
    .parse_complete(i)
  }
}

impl super::Table {
  #[inline]
  pub(super) fn parse(i: &str) -> IResult<&str, (usize, Self)> {
    TABLE_LINE_NUM.store(0, Ordering::SeqCst);
    map(
      (
        space,
        char(':'),
        space,
        char('"'),
        separated_list1(char(','), super::TableNodeValues::parse),
        space,
        char('"'),
        space,
        char(';'),
        comment_space_newline,
      ),
      |(_, _, _, _, inner, _, _, _, _, n)| {
        (TABLE_LINE_NUM.load(Ordering::SeqCst) + n, Self { inner })
      },
    )
    .parse_complete(i)
  }
}

#[inline]
pub(super) fn simple_multi<'a>(
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
  .parse_complete(i)
}
