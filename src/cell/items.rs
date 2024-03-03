use std::{hash::Hash, str::FromStr, sync::Arc};

use crate::{
  ast::{GroupComments, GroupId},
  common::items::WordSet,
  expression::{BooleanExpression, ConditionExpression},
  timing::items::Mode,
};

/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=199.5&end=199.6
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
pub struct LeakagePower {
  #[liberty(id)]
  _id: GroupId<Self>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
  // TODO:
  #[liberty(simple(type=Option))]
  power_level: Option<String>,
  // TODO:
  #[liberty(simple)]
  related_pg_pin: WordSet,
  // TODO:
  #[liberty(simple(type=Option))]
  when: Option<ConditionExpression>,
  #[liberty(simple)]
  value: f64,
  #[liberty(complex(type=Option))]
  mode: Option<Mode>,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct LeakagePowerId {
  power_level: Option<String>,
  related_pg_pin: WordSet,
  when: Option<ConditionExpression>,
}

impl crate::ast::HashedGroup for LeakagePower {
  type Id = LeakagePowerId;

  fn title(&self) -> Vec<String> {
    vec![]
  }

  fn gen_id(&self, _: Vec<String>) -> Result<Self::Id, crate::ast::IdError> {
    Ok(Self::Id {
      power_level: self.power_level.clone(),
      related_pg_pin: self.related_pg_pin.clone(),
      when: self.when.clone(),
    })
  }

  fn id(&self) -> GroupId<Self> {
    self._id.clone()
  }
}

/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.4&end=141.5
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default, Debug)]
#[derive(liberty_macros::Group)]
pub struct Statetable {
  #[liberty(id)]
  _id: GroupId<Self>,
  #[liberty(comments)]
  _comments: GroupComments<Self>,
  #[liberty(undefined)]
  _undefined: crate::ast::AttributeList,
  #[liberty(simple)]
  table: Table,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct StatetableId {
  pub input_npde: Vec<String>,
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
pub struct Table {
  pub v: Vec<String>,
}

impl std::fmt::Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.v, f)
  }
}

impl FromStr for Table {
  type Err = std::fmt::Error;
  /// To prevent syntax errors, the line continuation character
  /// must be followed immediately by the next line character.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.18&end=141.21
  /// ">Reference</a>
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      v: s
        .split("\\\n")
        .filter_map(|line| {
          let _l = line
            .trim_start()
            .trim_end_matches(|c: char| c == ',' || c.is_ascii_whitespace());
          if _l == "" {
            None
          } else {
            Some(_l.to_owned())
          }
        })
        .collect(),
    })
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
