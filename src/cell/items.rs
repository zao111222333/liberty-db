



/// Contains a table consisting of a single string.
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.4&end=141.5
/// ">Reference</a>
/// <script>
/// IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
/// </script>
#[derive(Default,Debug)]
#[derive(liberty_macros::GroupHashed)]
pub struct Statetable{
  _idx: Box<<Self as crate::ast::HashedGroup>::Idx>,
  _undefined: crate::ast::UndefinedAttributes,
  #[arrti_type(simple)]
  table: Table,
}


#[derive(Debug,Default,Clone,Hash,Eq,PartialEq)]
pub struct StatetableIdx{
    pub input_npde: Vec<String>,
    pub internal_node: Vec<String>,
}

impl crate::ast::HashedGroup for Statetable {
  type Idx=StatetableIdx;

  fn title(&self) -> Vec<String> {
    let idx = self.idx_clone();
    vec![idx.input_npde.join(" "),idx.internal_node.join(" ")]
  }

  fn idx(&self) -> &Self::Idx {
    &self._idx
  }

  fn idx_clone(&self) -> Self::Idx {
    (*self._idx).clone()
  }

  fn gen_idx(&self, mut title: Vec<String>) -> Result<Self::Idx,crate::ast::IdxError> {
    let l=title.len();
    if l!=2{
      return Err(crate::ast::IdxError::LengthDismatch(2,l,title));
    }
    let internal_node = if let Some(s) = title.pop(){
      s.split_ascii_whitespace()
       .map(ToString::to_string)
       .collect::<Vec<String>>()
    }else{
      return Err(crate::ast::IdxError::Other("Unkown pop error".into()));
    };
    let input_npde = if let Some(s) = title.pop(){
      s.split_ascii_whitespace()
       .map(ToString::to_string)
       .collect::<Vec<String>>()
    }else{
      return Err(crate::ast::IdxError::Other("Unkown pop error".into()));
    };
    Ok(Self::Idx{input_npde,internal_node})
  }
}

#[derive(Default,Debug)]
pub struct Table{
  pub v: Vec<String>,
}

impl std::fmt::Display for Table {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.v, f)
  }
}

impl crate::ast::SimpleAttri for Table {
  type Error=std::fmt::Error;
  /// To prevent syntax errors, the line continuation character 
  /// must be followed immediately by the next line character.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/user_guide.html?field=null&bgn=141.18&end=141.21
  /// ">Reference</a>
  fn parse(s: &str)->Result<Self, Self::Error> {
    Ok(
      Self { 
        v: s.split("\\\n").filter_map(|line| {
          let _l = line.trim_start().trim_end_matches(|c:char| c == ',' || c.is_ascii_whitespace());
          if _l==""{None}else{Some(_l.to_owned())}
        }).collect()
      } 
    )
  }
  fn nom_parse<'a>(
      i: &'a str, line_num: &mut usize
  ) -> nom::IResult<&'a str, Result<Self,(Self::Error,crate::ast::AttriValue)>, nom::error::Error<&'a str>> {
    let (input,simple_multi) = crate::ast::parser::simple_multi(i,line_num)?;
    match Self::parse(simple_multi){
    Ok(s) => Ok((input,Ok(s))),
    Err(e) => Ok((
      input,
      Err((e,crate::ast::AttriValue::Simple(simple_multi.to_string())))
    )),
    }
  }
}

#[test]
fn statetable_test(){
  let _ = crate::ast::test_parse_group::<Statetable>(r#"(" CLK EN SE",ENL) {
        table : "	H   L  L : - : L ,\
        H   L  H : - : H ,\
        H   H  L : - : H ,\
        H   H  H : - : H ,\
        L   -  - : - : N ";
    }
  "#);
}