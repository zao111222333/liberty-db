//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>
//!
use super::{logic, BooleanExpression, BooleanExpressionLike, BRACKET_L, BRACKET_R};
use core::fmt;

/// FunctionExpression is the basic expression
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FunctionExpression {
  sub_expression_vec: Vec<BooleanExpression>,
  op1_vec: Vec<Option<logic::Operator1>>,
  op2_vec: Vec<logic::Operator2>,
}

impl FunctionExpression {
  /// new FunctionExpression
  #[inline]
  pub fn new(
    sub_expression_vec: Vec<BooleanExpression>,
    op1_vec: Vec<Option<logic::Operator1>>,
    op2_vec: Vec<logic::Operator2>,
  ) -> Self {
    Self { sub_expression_vec, op1_vec, op2_vec }
  }
  #[inline]
  fn len_not_match(&self) -> bool {
    self.sub_expression_vec.len() != self.op2_vec.len() + 1
      || self.sub_expression_vec.len() != self.op1_vec.len()
  }
}

// impl Into<BooleanExpression> for FunctionExpression{
//     fn into(self) -> BooleanExpression {
//         todo!()
//     }
// }
impl BooleanExpressionLike for FunctionExpression {
  /// TODO: Add test case for: `XOR` > `AND` > `OR`.
  ///
  /// The order of precedence of the operators is left to right, with inversion performed first, then XOR, then AND, then OR.
  /// <a name ="reference_link" href="
  /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html
  /// ?field=test
  /// &bgn
  /// =133.12
  /// &end
  /// =133.13
  /// ">Reference</a>
  #[inline]
  fn table(&self) -> logic::Table {
    #[inline]
    fn compute_op2(
      target_op2: &logic::Operator2,
      op2_vec: &Vec<logic::Operator2>,
      state_table_vec: &Vec<logic::Table>,
    ) -> (Vec<logic::Operator2>, Vec<logic::Table>) {
      let mut _op2_vec: Vec<logic::Operator2> = vec![];
      let mut _state_table_vec: Vec<logic::Table> = vec![state_table_vec[0].clone()];
      for (op2_idx, op2) in op2_vec.iter().enumerate() {
        if op2 == target_op2 {
          match _state_table_vec.pop() {
            Some(state_table) => _state_table_vec
              .push(op2.compute_table(&state_table, &state_table_vec[op2_idx + 1])),
            None => panic!(),
          }
        } else {
          _op2_vec.push(*op2);
          _state_table_vec.push(state_table_vec[op2_idx + 1].clone());
        }
      }
      (_op2_vec, _state_table_vec)
    }
    if self.len_not_match() {
      panic!();
    }
    let op_vec = &self.op2_vec;
    let state_table_vec: &Vec<logic::Table> = &self
      .sub_expression_vec
      .iter()
      .enumerate()
      .map(|(idx, x)| match self.op1_vec[idx] {
        Some(op1) => op1.compute_table(&x.table()),
        None => x.table(),
      })
      .collect();
    let (_op_xor, _state_xor) =
      compute_op2(&logic::Operator2::Xor, op_vec, state_table_vec);
    let (_op_and, _state_and) =
      compute_op2(&logic::Operator2::And, &_op_xor, &_state_xor);
    let (_op_or, _state_or) = compute_op2(&logic::Operator2::Or, &_op_and, &_state_and);
    logic::Table {
      self_node: self.to_string(),
      table: _state_or[0].table.clone(),
      port_idx: _state_or[0].port_idx.clone(),
    }
  }
}

impl fmt::Display for FunctionExpression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let append_sub = |sub_idx: usize, f: &mut fmt::Formatter<'_>| -> fmt::Result {
      match self.op1_vec[sub_idx] {
        Some(op1) => write!(f, "{op1}{}", self.sub_expression_vec[sub_idx]),
        None => write!(f, "{}", self.sub_expression_vec[sub_idx]),
      }
    };
    if self.len_not_match() {
      return Err(fmt::Error);
    }
    if self.op2_vec.is_empty() {
      append_sub(0, f)
    } else {
      self.sub_expression_vec[1..]
        .iter()
        .enumerate()
        .fold(
          Ok(()).and_then(|_| write!(f, "{}{}", BRACKET_L, self.sub_expression_vec[0])),
          |result, (idx, _)| {
            result
              .and_then(|_| write!(f, "{}", self.op2_vec[idx]))
              .and_then(|_| append_sub(idx + 1, f))
          },
        )
        .and_then(|_| write!(f, "{}", BRACKET_R))
    }
  }
}
