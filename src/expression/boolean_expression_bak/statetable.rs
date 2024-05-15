/// search: `and`, `or`, `excluden`
use std::{collections::HashMap, hash::Hash, str::FromStr};

use crate::ast::{GroupComments, GroupId};

use super::logic;

/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.4&end=141.5
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Statetable {
  #[liberty(id)]
  _id: GroupId<Self>,
  /// group comments
  #[liberty(comments)]
  pub comments: GroupComments<Self>,
  /// group undefined attributes
  #[liberty(undefined)]
  pub undefined: crate::ast::AttributeList,
  #[liberty(simple)]
  table: Table,
}

/// ```
/// statetable ("J  K  CN  CD",  "IQ" ){}
/// ```
/// will get:
/// ```
///   input_npde = vec!["J","K","CN","CN"]
///   internal_node = vec!["IQ"]
/// ```
#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StatetableId {
  /// Input node names
  pub input_npde: Vec<String>,
  /// Internal node names
  pub internal_node: Vec<String>,
}

impl crate::ast::HashedGroup for Statetable {
  type Id = StatetableId;

  fn title(&self) -> Vec<String> {
    let id = self.id().clone();
    vec![id.input_npde.join(" "), id.internal_node.join(" ")]
  }

  fn gen_id(&self, mut title: Vec<String>) -> Result<Self::Id, crate::ast::IdError> {
    let l = title.len();
    if l != 2 {
      return Err(crate::ast::IdError::LengthDismatch(2, l, title));
    }
    let internal_node = if let Some(s) = title.pop() {
      s.split_ascii_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
    } else {
      return Err(crate::ast::IdError::Other("Unkown pop error".into()));
    };
    let input_npde = if let Some(s) = title.pop() {
      s.split_ascii_whitespace()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
    } else {
      return Err(crate::ast::IdError::Other("Unkown pop error".into()));
    };
    Ok(Self::Id { input_npde, internal_node })
  }

  fn id(&self) -> GroupId<Self> {
    self._id.clone()
  }
}

#[derive(Default, Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Table {
  pub v: Vec<(InputNodeValue, CurrentInternalNodeValue, NextInternalNodeValue)>,
}

impl std::fmt::Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.v, f)
  }
}

/// Input node values
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=142.4&end=142.5
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum InputNodeValue {
  /// Low
  #[strum(serialize = "L")]
  Low,
  /// High
  #[strum(serialize = "H")]
  High,
  /// Don't Care
  #[strum(serialize = "-")]
  DontCare,
  /// Expands to both L and H
  #[strum(serialize = "L/H")]
  LowHigh,
  /// Expands to both H and L
  #[strum(serialize = "H/L")]
  HighLow,
  /// Rise
  #[strum(serialize = "R")]
  Rise,
  /// Fall
  #[strum(serialize = "F")]
  Fall,
  /// Not Rise
  #[strum(serialize = "~R")]
  NotFall,
  /// Not Fall
  #[strum(serialize = "~F")]
  NotRise,
}

/// Input node values
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=142.6&end=142.7
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CurrentInternalNodeValue {
  /// Low
  #[strum(serialize = "L")]
  Low,
  /// High
  #[strum(serialize = "H")]
  High,
  /// Not specified
  #[strum(serialize = "-")]
  DontCare,
  /// Expands to both L and H
  #[strum(serialize = "L/H")]
  LowHigh,
  /// Expands to both H and L
  #[strum(serialize = "H/L")]
  HighLow,
  /// Unknown
  #[strum(serialize = "X")]
  Unknown,
  /// No event from current value. Hold.
  /// Use only when all asynchronous inputs and clocks are inactive
  /// NOTICE: Hold is not High-Impedance
  #[strum(serialize = "N")]
  Hold,
}

/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=142.8&end=142.8
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Debug, Clone, Copy)]
#[derive(Hash, PartialEq, Eq)]
#[derive(strum_macros::EnumString, strum_macros::EnumIter)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NextInternalNodeValue {
  /// Low
  #[strum(serialize = "L")]
  Low,
  /// High
  #[strum(serialize = "H")]
  High,
  /// Don't Care
  #[strum(serialize = "-")]
  DontCare,
  /// Expands to both L and H
  #[strum(serialize = "L/H")]
  LowHigh,
  /// Expands to both H and L
  #[strum(serialize = "H/L")]
  HighLow,
}

impl FromStr for Table {
  type Err = std::fmt::Error;
  /// To prevent syntax errors, the line continuation character
  /// must be followed immediately by the next line character.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.18&end=141.21
  /// ">Reference</a>
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // Ok(Self {
    //   v: s
    //     .split("\\\n")
    //     .filter_map(|line| {
    //       let _l = line
    //         .trim_start()
    //         .trim_end_matches(|c: char| c == ',' || c.is_ascii_whitespace());
    //       if _l == "" {
    //         None
    //       } else {
    //         Some(_l.to_owned())
    //       }
    //     })
    //     .collect(),
    // })
    todo!()
  }
}

impl crate::ast::SimpleAttri for Table {
  fn nom_parse<'a>(
    i: &'a str,
    line_num: &mut usize,
  ) -> nom::IResult<
    &'a str,
    Result<Self, (Self::Err, crate::ast::AttriValue)>,
    nom::error::Error<&'a str>,
  > {
    let (input, simple_multi) = crate::ast::parser::simple_multi(i, line_num)?;
    match Self::parse(simple_multi) {
      Ok(s) => Ok((input, Ok(s))),
      Err(e) => {
        Ok((input, Err((e, crate::ast::AttriValue::Simple(simple_multi.to_string())))))
      }
    }
  }
}

#[test]
fn statetable_test() {
  let _ = crate::ast::test_parse_group::<Statetable>(
    r#"(" CLK EN SE",ENL) {
        table : "	H   L  L : - : L ,\
        H   L  H : - : H ,\
        H   H  L : - : H ,\
        H   H  H : - : H ,\
        L   -  - : - : N ";
    }
  "#,
  );
}
