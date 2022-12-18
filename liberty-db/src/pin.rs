use compact_str::CompactString;
use hashbrown::HashMap;

use crate::timing::Timing;

/// You can define a pin group within a cell, test_cell, scaled_cell, model, or bus group.
/// 
/// Reference:
/// <iframe 
/// src="
/// https://zao111222333.github.io/liberty-rs/liberty07_03.html
/// ?field=test
/// &bgn
/// =t.m0.x6.h1.y1e34.ff1.fs0.fc1.sc0.ls0.ws0
/// &end
/// =t.m0.xb.h4.y1e48.ff1.fs2.fc2.sc0.ls0.ws0
/// " 
/// style="width: 90%; height: 600px;"></iframe>
#[derive(Debug, Clone)]
pub struct Pin{
    /// A timing group is defined within a pin group.
    /// 
    /// Reference:
    /// <iframe 
    /// src="
    /// https://zao111222333.github.io/liberty-rs/liberty07_03.html
    /// ?field=test
    /// &bgn
    /// =t.m0.x9.hc.ye79.ffc.fs6.fc1.sc0.ls0.ws0
    /// &end
    /// =t.m0.x39.h8.ye8a.ff7.fs2.fc2.sc0.ls0.ws0
    /// " 
    /// style="width: 90%; height: 600px;"></iframe>
    pub timing_map: HashMap<CompactString,Timing>,
}