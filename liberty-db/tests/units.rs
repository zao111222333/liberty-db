// #[cfg(feature = "f32")]
// #[cfg("f64")]
// #[cfg(test)]
#[cfg(not(feature = "f64"))]
#[test]
fn uom() {
    use liberty_db::units;
    println!("323232");
    let l1: units::Length = units::meter(1.0);
    let t1 = units::Time::new::<units::time::second>(2.0);
    println!("{}",l1.get::<units::length::kilometer>());
    println!("{}",l1.into_format_args(units::length::kilometer, units::DisplayStyle::Abbreviation));
    println!("{:?}",l1*2.0);
    println!("{:?}",2.0*l1*2.0);
    println!("{:?}",l1+l1);
    // assert_eq!(
    //     units::coulomb!(1.0), 
    //     units::ampere!(0.5)*units::second!(2.0));
    // assert_eq!(
    //     units::ampere!(0.5_f64),
    //     units::ElectricCurrent::new::<units::electric_current::milliampere>(500.0));
    // let ma=units::electric_current::milliampere;
    // let t2 = units::minute!(1.0);
    // let x:units::ElectricCurrent = ma.from_primitive(5.0);
}
#[test]
fn t() {
    fn tt(s: &str) {
        println!("{:?}",s);
    }
    let s: &str = "646464";
    println!("{}",&s);
    tt(s);
    tt(&s);
    // assert_eq!(s,&s)
}
#[cfg(feature = "f64")]
// #[cfg("f64")]
// #[cfg(test)]
#[test]
fn uom() {
    use liberty_db::units;
    println!("646464");
    // let l1: units::Length = units::meter(1.0_f64);
    // let t1: units::Time = units::Time::new::<units::time::second>(2.0);
    // println!("{}",l1.get::<units::length::kilometer>());
    // assert_eq!(
    //     units::coulomb!(1.0), 
    //     units::ampere!(0.5)*units::second!(2.0));
    // assert_eq!(
    //     units::ampere!(0.5_f64),
    //     units::ElectricCurrent::new::<units::electric_current::milliampere>(500.0));
    // let ma=units::electric_current::milliampere;
    // let t2 = units::minute!(1.0);
    // let x:units::ElectricCurrent = ma.from_primitive(5.0);
}