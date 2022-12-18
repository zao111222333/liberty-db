use compact_str::CompactString;
use crate::common::Group;

/// You define the mode attribute within a timing group. A mode attribute pertains to an individual timing arc. The timing arc is active when mode is instantiated with a name and a value. You can specify multiple instances of the mode attribute, but only one instance for each timing arc.
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2c4c.ff1.fs2.fc2.sc0.ls0.ws0
/// +t.m0.x9.h7.y2c58.ff1.fs2.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h8.y2c56.ff7.fs2.fc2.sc0.ls0.ws0
/// +t.m0.xb.h8.y2c8e.ff7.fs2.fc2.sc0.ls0.ws0
/// ">Reference</a>
#[derive(Debug, Clone, Copy, Default)]
pub struct Mode{

}
/// Defines cell delay lookup tables (independently of transition delay) in CMOS nonlinear timing models.
/// **Note: **
/// 
/// The same k-factors that scale the cell_fall and cell_rise values also scale the 
/// retaining_fall and retaining_rise values. There are no separate k-factors for 
/// the retaining_fall and retaining_rise values.
/// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
/// <a name ="reference_link" href="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x9.h4.y2d8f.ff1.fs2.fc2.sc0.ls0.ws0
/// &end
/// =t.m0.x39.h4.y2daa.ff1.fs2.fc2.sc0.ls0.ws0
/// ">Reference</a>
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub struct CallFall{
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dac.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dac.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_1: Vec<f64>,
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dad.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dad.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_2: Vec<f64>,
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2dae.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2dae.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub index_3: Vec<f64>,
    /// <script src="https://zao111222333.github.io/liberty-rs/iframe.js"></script>
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x39.h8.y2daf.ff7.fs2.fc2.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.y2daf.ff7.fs2.fc2.sc0.ls0.ws0
    /// ">Reference</a>
    pub values: Vec<Vec<Vec<f64>>>,
}


impl Group for CallFall  {
    fn name(&self) -> &CompactString {
        todo!()
    }
}