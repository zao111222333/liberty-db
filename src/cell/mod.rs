//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

use std::collections::HashSet;

use crate::{
  ast::{AttributeList, HashedGroup},
  pin::Pin,
};
mod items;
pub use items::*;

/// cell
#[derive(Debug, Default)]
// #[derive(liberty_macros::Group)]
pub struct Cell {
  // #[id_len(1)]
  _id: <Self as HashedGroup>::Id,
  _undefined: AttributeList,

  // #[arrti_type(simple)]
  pub area: Option<f64>,
  // #[arrti_type(group)]
  pub pin: HashSet<Pin>,
  // #[arrti_type(group)]
  pub statetable: Option<Statetable>,
}

impl Eq for Cell {}
impl PartialEq for Cell {
  fn eq(&self, other: &Self) -> bool {
    <Cell as crate::ast::HashedGroup>::id(&self)
      == <Cell as crate::ast::HashedGroup>::id(other)
  }
}
impl std::hash::Hash for Cell {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    <Cell as crate::ast::HashedGroup>::id(&self).hash(state);
  }
}
impl std::borrow::Borrow<<Cell as crate::ast::HashedGroup>::Id> for Cell {
  fn borrow(&self) -> &<Self as crate::ast::HashedGroup>::Id {
    <Cell as crate::ast::HashedGroup>::id(&self)
  }
}
impl crate::ast::HashedGroup for Cell {
  type Id = String;
  #[inline]
  fn title(&self) -> Vec<String> {
    vec![self._id.clone()]
  }
  #[inline]
  fn id(&self) -> &Self::Id {
    &self._id
  }
  #[inline]
  fn gen_id(&self, mut title: Vec<String>) -> Result<Self::Id, crate::ast::IdError> {
    let l = title.len();
    if l != 1 {
      return Err(crate::ast::IdError::LengthDismatch(1, l, title));
    }
    if let Some(name) = title.pop() {
      Ok(name)
    } else {
      return Err(crate::ast::IdError::Other("Unkown pop error".into()));
    }
  }
}
impl crate::ast::GroupAttri for Cell {
  #[inline]
  fn undefined_list(&mut self) -> &mut crate::ast::AttributeList {
    &mut self._undefined
  }
  fn fmt_liberty<T: std::fmt::Write>(
    &self,
    key: &str,
    f: &mut crate::ast::CodeFormatter<'_, T>,
  ) -> std::fmt::Result {
    use itertools::Itertools;
    use std::fmt::Write;
    f.write_fmt(format_args!(
      "\n{0} ({1}) {{",
      key,
      crate::ast::HashedGroup::title(self)
        .iter()
        .map(|s| if crate::ast::is_word(s) {
          s.clone()
        } else {
          "\"".to_owned() + s + "\""
        })
        .join(",")
    ))?;
    f.indent(1);
    if let Some(simple) = &self.area {
      crate::ast::SimpleAttri::fmt_liberty(simple, "area", f)?;
    }
    for group in self.pin.iter() {
      crate::ast::GroupAttri::fmt_liberty(group, "pin", f)?;
    }
    if let Some(group) = &self.statetable {
      crate::ast::GroupAttri::fmt_liberty(group, "statetable", f)?;
    }
    if !self._undefined.is_empty() {
      f.write_fmt(format_args!("\n/* Undefined attributes from here */"))?;
      crate::ast::liberty_attr_list(&self._undefined, f)?;
    }
    f.dedent(1);
    f.write_fmt(format_args!("\n}}"))
  }
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> nom::IResult<&'a str, Result<Self, crate::ast::IdError>, nom::error::Error<&'a str>>
  {
    let (mut input, title) = crate::ast::parser::title(i, line_num)?;
    let mut res = Self::default();
    loop {
      match crate::ast::parser::key(input) {
        Err(nom::Err::Error(_)) => {
          (input, _) = crate::ast::parser::end_group(input)?;
          match crate::ast::HashedGroup::gen_id(&res, title) {
            Ok(id) => {
              res._id = id;
              return Ok((input, Ok(res)));
            }
            Err(e) => {
              return Ok((input, Err(e)));
            }
          }
        }
        Err(e) => return Err(e),
        Ok((_input, key)) => {
          input = _input;
          match key {
            "area" => {
              let simple_res: _;
              (input, simple_res) =
                <_ as crate::ast::SimpleAttri>::nom_parse(input, line_num)?;
              match simple_res {
                Ok(simple) => {
                  res.area = Some(simple);
                }
                Err((e, undefined)) => {
                  {
                    // ::std::io::_print(
                    //     format_args!(
                    //         "Line={0}; Key={1}; Value={2:?}; Err={3}\n", line_num, key,
                    //         undefined, e
                    //     ),
                    // );
                  };
                  res.undefined_list().push((key.to_owned(), undefined));
                }
              }
            }
            "pin" => {
              let group_res: _;
              (input, group_res) =
                <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
              match group_res {
                Ok(group) => {
                  if !res.pin.insert(group) {
                    let e = crate::ast::IdError::RepeatIdx;
                    {
                      // ::std::io::_print(
                      //     format_args!("Line={0}, error={1}\n", line_num, e),
                      // );
                    };
                  }
                }
                Err(e) => {
                  {
                    // ::std::io::_print(
                    //     format_args!("Line={0}, error={1}\n", line_num, e),
                    // );
                  };
                }
              }
              let n: usize;
              (input, n) = crate::ast::parser::comment_space_newline(input)?;
              *line_num += n;
            }
            "statetable" => {
              let group_res: _;
              (input, group_res) =
                <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
              match group_res {
                Ok(group) => {
                  if let Some(old) = res.statetable {
                    let e = crate::ast::IdError::RepeatIdx;
                    {
                      // ::std::io::_print(
                      //     format_args!("Line={0}, error={1}\n", line_num, e),
                      // );
                    };
                  }
                  res.statetable = Some(group);
                }
                Err(e) => {
                  {
                    // ::std::io::_print(
                    //     format_args!("Line={0}, error={1}\n", line_num, e),
                    // );
                  };
                }
              }
              let n: usize;
              (input, n) = crate::ast::parser::comment_space_newline(input)?;
              *line_num += n;
            }
            _ => {
              let undefined: crate::ast::AttriValue;
              (input, undefined) = crate::ast::parser::undefine(input, line_num)?;
              res.undefined_list().push((key.to_owned(), undefined));
              let n: usize;
              (input, n) = crate::ast::parser::comment_space_newline(input)?;
              *line_num += n;
            }
          }
        }
      }
    }
  }
}
