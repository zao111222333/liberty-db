//! All trait `impl` for
//! `Timing`.<script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>

mod check{
    //! impl [Check](crate::common::Check) for
    //! `Timing`.<script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    
    use crate::{common::Check, timing::Timing};

    // TODO:
    /// Timing arcs with a timing type of clear or preset require a timing_sense attribute. 
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x9.h7.y2b31.ff1.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x9.h7.y2b31.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    fn need_timing_sense_when_timing_type_is_clear_or_preset(timing: &Timing){
        if timing.timing_sense==None{
            
        }
    }
    
    // TODO:
    /// If related_pin is an output pin, you must define a timing_sense attribute for that pin.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x9.h7.y2b32.ff1.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x9.h7.y2b32.ff1.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    fn need_timing_sense_when_related_pin_is_output(){}
    
    impl Check for Timing {
        fn check(&self) {
            need_timing_sense_when_timing_type_is_clear_or_preset(&self);
            need_timing_sense_when_related_pin_is_output();
        }
    }
}

