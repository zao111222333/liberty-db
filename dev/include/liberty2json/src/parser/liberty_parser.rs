use super::{
  attribute_parser::*,
  base::{lib_comment, tstring, ws},
  cell_parser::*,
  group_parser::*,
};

use crate::{LibRes, Liberty, LibertyJson};

use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::map,
  error::context,
  multi::{many0, many1},
  sequence::{delimited, preceded, tuple},
};

enum LibertyAttribue<'a> {
  SingleAttr((&'a str, LibertyJson)),
  GroupedAttr((&'a str, LibertyJson)),
}

use std::collections::HashMap;
pub fn liberty_parser(input: &str) -> LibRes<&str, Liberty> {
  context(
    "Liberty Parser",
    tuple((
      many0(lib_comment),
      preceded(ws(tag("library")), delimited(ws(tag("(")), tstring, ws(tag(")")))),
      delimited(
        ws(tag("{")),
        tuple((
          many1(alt((
            map(header_attribute_parser, |x| LibertyAttribue::SingleAttr(x)),
            map(header_group_parser, |x| LibertyAttribue::GroupedAttr(x)),
          ))),
          many1(cell_parser),
        )),
        ws(tag("}")),
      ),
    )),
  )(input)
  .map(|(res, data)| {
    let mut single_attribute_map = HashMap::new();
    let mut group_attribute_map = HashMap::new();
    let mut filler_cell_map = HashMap::new();
    let mut ff_cell_map = HashMap::new();
    let mut latch_cell_map = HashMap::new();
    let mut icg_cell_map = HashMap::new();
    let mut logic_cell_map = HashMap::new();
    let mut testff_cell_map = HashMap::new();
    for d in (data.2).0 {
      match d {
        LibertyAttribue::SingleAttr((name, v)) => {
          single_attribute_map.insert(name.into(), v);
        }
        LibertyAttribue::GroupedAttr((name, v)) => {
          group_attribute_map.insert(name.into(), v);
        }
      }
    }
    for d in (data.2).1 {
      match d.1 {
        CellEnum::Filler(v) => filler_cell_map.insert(d.0.into(), v),
        CellEnum::FF(v) => ff_cell_map.insert(d.0.into(), v),
        CellEnum::Latch(v) => latch_cell_map.insert(d.0.into(), v),
        CellEnum::ICG(v) => icg_cell_map.insert(d.0.into(), v),
        CellEnum::Logic(v) => logic_cell_map.insert(d.0.into(), v),
        CellEnum::TestFF(v) => testff_cell_map.insert(d.0.into(), v),
      };
    }

    (
      res,
      Liberty {
        name: data.1.to_string(),
        single_attribute: single_attribute_map,
        group_attribute: group_attribute_map,
        ffs: ff_cell_map,
        latchs: latch_cell_map,
        fillers: filler_cell_map,
        icgs: icg_cell_map,
        logics: logic_cell_map,
        testffs: testff_cell_map,
      },
    )
  })
}
