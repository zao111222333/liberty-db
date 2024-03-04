// // #[test]
// // fn hash_1() {
// //   let n = |s: &str| Expr::Terminal(s.to_string());
// //   let expr = n("A") | n("B") | n("C") | n("D");
// //   // (A+B)*(C+D)
// //   let expr1 = (n("A") | n("B")) & (n("C") | n("D"));
// //   // B*D+B*C+A*D+A*C
// //   let expr2 = n("B") & n("D") | n("B") & n("C") | n("A") & n("D") | n("A") & n("C");
// //   let mut bdd = BDD::new();
// //   let _ = bdd.from_expr(&expr);
// //   let bdd_func1 = bdd.clone().from_expr(&expr1);
// //   let bdd_func2 = bdd.clone().from_expr(&expr2);
// //   println!("{bdd_func1}");
// //   println!("{bdd_func2}");
// // }

// // #[test]
// // fn hash_2() {
// //   let n = |s: &str| Expr::Terminal(s.to_string());
// //   // (A+B)*(C+D)
// //   let expr1 = (n("A") | n("B")) & (n("C") | n("D"));
// //   // B*D+B*C+A*D+A*C
// //   let expr2 = n("B") & n("D") | n("B") & n("C") | n("A") & n("D") | n("A") & n("C");
// //   let mut bdd = BDD::new();
// //   let bdd_func1 = bdd.from_expr(&expr1);
// //   let bdd_func2 = bdd.clone().from_expr(&expr2);
// //   println!("{bdd_func1}");
// //   println!("{bdd_func2}");
// // }

// // #[test]
// // fn hash_3() {
// //   let mut bdd1 = BDD::new();
// //   let f1_A = bdd1.terminal("A".to_owned());
// //   let f1_B = bdd1.terminal("B".to_owned());
// //   let f1_C = bdd1.terminal("C".to_owned());
// //   let f1_D = bdd1.terminal("D".to_owned());
// //   let f1_A_or_B = bdd1.or(f1_A, f1_B);
// //   let f1_C_or_D = bdd1.or(f1_C, f1_D);
// //   let bdd_func1 = bdd1.and(f1_A_or_B, f1_C_or_D);
// //   let mut bdd2 = BDD::new();
// //   let f2_A = bdd2.terminal("A".to_owned());
// //   let f2_B = bdd2.terminal("B".to_owned());
// //   let f2_C = bdd2.terminal("C".to_owned());
// //   let f2_D = bdd2.terminal("D".to_owned());
// //   let f2_B_and_D = bdd2.and(f2_B, f2_D);
// //   let f2_B_and_C = bdd2.and(f2_B, f2_C);
// //   let f2_A_and_D = bdd2.and(f2_A, f2_D);
// //   let f2_A_and_C = bdd2.and(f2_A, f2_C);
// //   let f2_B_and_D_or_B_and_C = bdd2.or(f2_B_and_D, f2_B_and_C);
// //   let f2_A_and_D_or_A_and_C = bdd2.or(f2_A_and_D, f2_A_and_C);
// //   let bdd_func2 = bdd2.or(f2_B_and_D_or_B_and_C, f2_A_and_D_or_A_and_C);
// //   println!("{bdd_func1}");
// //   println!("{bdd_func2}");
// // }

// // #[test]
// // fn hash_4() {
// //   let mut bdd1 = BDD::new();
// //   let f1_A = bdd1.terminal("A".to_owned());
// //   let f1_B = bdd1.terminal("B".to_owned());
// //   let f1_C = bdd1.terminal("C".to_owned());
// //   let f1_D = bdd1.terminal("D".to_owned());
// //   let f1_A_or_B = bdd1.or(f1_A, f1_B);
// //   let f1_C_or_D = bdd1.or(f1_C, f1_D);
// //   let bdd_func1 = bdd1.and(f1_A_or_B, f1_C_or_D);
// //   let mut bdd2 = BDD::new();
// //   let f2_A = bdd2.terminal("A".to_owned());
// //   let f2_B = bdd2.terminal("B".to_owned());
// //   let f2_C = bdd2.terminal("C".to_owned());
// //   let f2_D = bdd2.terminal("D".to_owned());
// //   let f2_B_and_D = bdd2.and(f2_B, f2_D);
// //   let f2_B_and_C = bdd2.and(f2_B, f2_C);
// //   let f2_A_and_D = bdd2.and(f2_A, f2_D);
// //   let f2_A_and_C = bdd2.and(f2_A, f2_C);
// //   let f2_B_and_D_or_B_and_C = bdd2.or(f2_B_and_D, f2_B_and_C);
// //   let f2_B_and_D_or_B_and_C_or_A_and_D = bdd2.or(f2_B_and_D_or_B_and_C, f2_A_and_D);
// //   let bdd_func2 = bdd2.or(f2_B_and_D_or_B_and_C_or_A_and_D, f2_A_and_C);
// //   println!("{bdd_func1}");
// //   println!("{bdd_func2}");
// // }

// use biodivine_lib_bdd::*;

// #[test]
// fn lid_bdd() {
//   let mut builder = BddVariableSetBuilder::new();
//   let [a, b, c, d] = builder.make(&["A", "B", "C", "D"]);
//   let variables: BddVariableSet = builder.build();

//   // String expressions:
//   //   variables.safe_eval_expression(expression)
//   let x1 = variables.eval_expression_string("(A|B)&(C|D)");
//   let x2 = variables.eval_expression_string("B&D | B&C | A&D | A&C");
//   //   assert!(!x.is_false());
//   //   assert_eq!(6.0, x.cardinality());
//   assert_eq!(x1, x2);
//   println!("{}", x1);
//   println!("{}", x2);

//   for valuation in x1.sat_valuations() {
//     println!("{}", valuation);
//     // assert!(x.eval_in(&valuation));
//   }
// }
