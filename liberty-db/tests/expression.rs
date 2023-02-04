mod for_boolean_expression{
    use liberty_db::expression::*;
    use test_log::test;
    use log::info;
    #[test]
    fn logic_operation() {
        use std::str::FromStr;
        assert_eq!(LogicOperation::from_str("&"), Ok(LogicOperation::And));
        assert_eq!(LogicOperation::from_str("*"), Ok(LogicOperation::And));
        assert_eq!(LogicOperation::from_str("|"), Ok(LogicOperation::Or));
        assert_eq!(LogicOperation::from_str("+"), Ok(LogicOperation::Or));
        assert_eq!(LogicOperation::from_str("^"), Ok(LogicOperation::Xor));
        let and = LogicOperation::And;
        let or = LogicOperation::Or;
        let xor = LogicOperation::Xor;
        assert_eq!(format!("{}",and), "&");
        assert_eq!(format!("{}",or),  "|");
        assert_eq!(format!("{}",xor), "^");
    }
    // TODO: 
    #[test]
    fn it_works() {
        info!("Checking whether it still works...");
        info!("Looks good!");
    }

    #[test]
    fn port_as_key() {
        let port_a1 = Port::new("A");
        let port_a2 = Port::new("A");
        println!("{:?}",port_a1.get_state_stable());
        let mut map = hashbrown::HashMap::new();
        let v = 1;
        let _ = map.insert(port_a1, v);
        assert_eq!(
            Some(&v),
            map.get(&port_a2)
        );
    }
    #[test]
    fn port_compute() {
        let port_a1 = Port::new("A");
        let port_a2 = Port::new("A");
        let and = LogicOperation::And;
        for (vec_in,state_out) in and.compute_table(
            &port_a1.get_state_stable(), 
            &port_a2.get_state_stable(),
        ).table.iter(){
            println!("{:?} {:?}", vec_in,state_out);
        }
    }
    #[test_log::test]
    fn expression_nand_table() {
        env_logger::init() ;
        let port_a = Port::new("A").to_box();
        let port_b = Port::new("B").to_box();
        let exp_not_a_and_b = NotExpression::new(
            BooleanExpression::new (
                vec![port_a,port_b],
                vec![LogicOperation::And],
            ).to_box()
        );
        assert_eq!(format!("{}",exp_not_a_and_b), "!(A&B)");
        println!("**** Origin");
        let table = exp_not_a_and_b.get_state_stable();
        for (vec_in,state_out) in table.table.iter(){
            println!("{:?} {:?}", vec_in,state_out);
        }
        println!("**** Search1: A=High, Output=Fall");
        let table1 = table.search(
            vec![(Port::new("A"),LogicState::High)], Some(LogicState::Fall(None)));
        for (vec_in,state_out) in table1.table.iter(){
            println!("{:?} {:?}", vec_in,state_out);
        }
        println!("**** After-Search2: C=High(), Output=Any");
        let table2 = table.search(
            vec![(Port::new("C"),LogicState::High)], None);
        for (vec_in,state_out) in table2.table.iter(){
            println!("{:?} {:?}", vec_in,state_out);
        }
    }
    #[test]
    fn logic_vecter_as_key() {
        use hashbrown::HashMap;
        let mut pin_map= HashMap::new();
        let v = 12345.000;
        {
            let mut v_k1 = LogicVector::new(vec![]);
            v_k1.push(LogicState::High);
            v_k1.push(LogicState::High);
            v_k1.push(LogicState::Rise(ChangePattern::new(345670.7734567893456789456787777771,0.1)));
            v_k1.push(LogicState::Fall(None));
            assert_eq!(format!("{}",v_k1), "11R(3.4567077346E5|1.0000000000E-1)F");
            let _ = pin_map.insert(v_k1, v);
        }
        {
            let mut v_k2 = LogicVector::new(vec![]);
            v_k2.push(LogicState::High);
            v_k2.push(LogicState::High);
            v_k2.push(LogicState::Rise(ChangePattern::new(345670.7734567893456789456787777771,0.1)));
            v_k2.push(LogicState::Fall(None));
            assert_eq!(format!("{}",v_k2), "11R(3.4567077346E5|1.0000000000E-1)F");
            assert_eq!(Some(&v),pin_map.get(&v_k2));
        }
    }
    #[test]
    fn expression_length_check() {
        use std::fmt::{self, write};
        let right = BooleanExpression::new (
            vec![Port::new("A").to_box(),Port::new("B").to_box()],
            vec![LogicOperation::And],
        );
        assert_eq!(format!("{}",right), "(A&B)");
        let mut output = String::new();
        {
            let wrong_on_sub_expression_vec = BooleanExpression::new (
                vec![], 
                vec![LogicOperation::And],
            );
            if let Err(fmt::Error) = write(&mut output, 
                    format_args!("{}", wrong_on_sub_expression_vec)) {
            }else{
                panic!("\nIt should be error!\n {:?}\n", wrong_on_sub_expression_vec);
            }
        }
        {
            let wrong_on_operation_vec = BooleanExpression::new (
                vec![Port::new("A").to_box(),Port::new("B").to_box()], 
                vec![],
            );
            if let Err(fmt::Error) = write(&mut output, 
                    format_args!("{}", wrong_on_operation_vec)) {
            }else{
                panic!("\nIt should be error!\n {:?}\n", wrong_on_operation_vec);
            }
        }
    }
}