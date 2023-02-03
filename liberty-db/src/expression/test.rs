
mod for_boolean_expression{
    use log::{warn, info};
    use test_log::test;

    use crate::expression::*;

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

    #[test]
    fn logic_state() {
        // use std::str::FromStr;
        let h = LogicState::High;
        match h {
            LogicState::Unknown => (),
            LogicState::High => (),
            LogicState::Low => (),
            LogicState::Fall(_) => (),
            LogicState::Rise(_) => (),
            LogicState::HighImpedance => todo!(),
        }
    }
    #[test_log::test]
    fn it_works() {
        info!("Checking whether it still works...");
        let a = 4;
        let b = 2 + 2== a;
        assert!(b);
        info!("Looks good!");
    }

    #[test]
    fn port_as_key() {
        let port_a1 = Port::new("A");
        let port_a2 = Port::new("A");
        println!("{:?}",port_a1.get_state_stable());
        let mut map = hashbrown::HashMap::new();
        let v = 1;
        let _ = map.insert(port_a1.get_id(), v);
        assert_eq!(
            Some(&v),
            map.get(&port_a2.get_id())
        );
    }
    #[test]
    fn port_compute() {
        let port_a1 = Port::new("A");
        let port_a2 = Port::new("A");
        let and = LogicOperation::And;
        for (vec_in,state_out) in and.compute_table(
            &port_a1.get_state_stable(), 
            &port_a2.get_state_stable()
        ).table.iter(){
            println!("{:?} {:?}", vec_in,state_out);
        }
    }
    #[test]
    fn expression_nand_table() {
        let port_a = Port::new("A");
        let port_b = Port::new("B");
        let exp_a_and_b = BooleanExpression::new (
            vec![Box::new(port_a),Box::new(port_b)], 
            vec![false,false],
            vec![LogicOperation::And],
        );
        let exp_not_a_and_b = BooleanExpression::new (
            vec![Box::new(exp_a_and_b)], 
            vec![true],
            vec![],
        );
        assert_eq!(format!("{}",exp_not_a_and_b), "!(A&B)");
        println!("**** Origin");
        let table = exp_not_a_and_b.get_state_stable();
        for (vec_in,state_out) in table.table.iter(){
            println!("{:?} {:?}", vec_in,state_out);
        }
        println!("**** Search1: A=High, Output=Fall");
        let table1 = table.search(
            vec![(PortId::new("A"),LogicState::High)], Some(LogicState::Fall(None)));
        for (vec_in,state_out) in table1.table.iter(){
            println!("{:?} {:?}", vec_in,state_out);
        }
        println!("**** Search2: A=High, Output=Any");
        let table2 = table.search(
            vec![(PortId::new("A"),LogicState::High)], None);
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
            let mut v_k1 = LogicVector::new();
            v_k1.vec.push(LogicState::High);
            v_k1.vec.push(LogicState::High);
            v_k1.vec.push(LogicState::Rise(ChangePattern::new(345670.7734567893456789456787777771,0.1)));
            v_k1.vec.push(LogicState::Fall(None));
            assert_eq!(format!("{}",v_k1), "11R(3.4567077346E5|1.0000000000E-1)F");
            let _ = pin_map.insert(v_k1, v);
        }
        {
            let mut v_k2 = LogicVector::new();
            v_k2.vec.push(LogicState::High);
            v_k2.vec.push(LogicState::High);
            v_k2.vec.push(LogicState::Rise(ChangePattern::new(345670.7734567893456789456787777771,0.1)));
            v_k2.vec.push(LogicState::Fall(None));
            assert_eq!(format!("{}",v_k2), "11R(3.4567077346E5|1.0000000000E-1)F");
            assert_eq!(Some(&v),pin_map.get(&v_k2));
        }
    }
    #[test]
    fn expression_length_check() {
        use std::fmt::{self, write};
        let right = BooleanExpression::new (
            vec![Box::new(Port::new("A")),Box::new(Port::new("B"))], 
            vec![false,false],
            vec![LogicOperation::And],
        );
        assert_eq!(format!("{}",right), "A&B");

        let mut output = String::new();
        {
            let wrong_on_not_invert_vec = BooleanExpression::new (
                vec![Box::new(Port::new("A")),Box::new(Port::new("B"))], 
                vec![true,true,true],
                vec![LogicOperation::And],
            );
            if let Err(fmt::Error) = write(&mut output, 
                format_args!("{}", wrong_on_not_invert_vec)) {
            }else{
                panic!("\nIt should be error!\n {:?}\n", wrong_on_not_invert_vec);
            }
        }
        {
            let wrong_on_sub_expression_vec = BooleanExpression::new (
                vec![], 
                vec![true,true],
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
                vec![Box::new(Port::new("A")),Box::new(Port::new("B"))], 
                vec![true,true],
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