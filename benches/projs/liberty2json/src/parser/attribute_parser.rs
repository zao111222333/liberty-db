use super::base::{
  complex_attribue_value, simple_attribute_value, tstring, very_complex_attribute_value,
  ws,
};
use crate::{LibRes, LibertyJson};
use nom::{
  branch::alt,
  bytes::complete::tag,
  error::context,
  sequence::{separated_pair, terminated, tuple},
};

pub fn simple_attribute(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
  terminated(separated_pair(tstring, tag(":"), simple_attribute_value), ws(tag(";")))(
    input,
  )
}

pub fn complex_attribute(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
  terminated(tuple((tstring, complex_attribue_value)), ws(tag(";")))(input)
}

fn very_complex_attribute(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
  terminated(tuple((tstring, very_complex_attribute_value)), ws(tag(";")))(input)
}

pub fn group_attribute_parser(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
  context("Group Attribute Parser", alt((complex_attribute, simple_attribute)))(input)
}

pub fn header_attribute_parser(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
  context("Header Attribute Parser", alt((very_complex_attribute, simple_attribute)))(
    input,
  )
}
