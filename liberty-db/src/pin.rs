use compact_str::CompactString;

use crate::timing::Timing;

/// You can define a pin group within a cell, test_cell, scaled_cell, model, or bus group.
/// 
/// Reference:
/// <iframe 
/// src="
/// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
/// ?field=test
/// &bgn
/// =145.43
/// &end
/// =146.8
/// " 
/// style="width: 90%; height: 600px;"></iframe>
#[derive(Debug, Default)]
pub struct Pin<'a>{
    pub group_name: CompactString,
    /// A timing group is defined within a pin group.
    /// 
    /// Reference:
    /// <iframe 
    /// src="
    /// https://zao111222333.github.io/liberty-rs/2007.03/_user_guide.html
    /// ?field=test
    /// &bgn
    /// =67.26
    /// &end
    /// =67.43
    /// " 
    /// style="width: 90%; height: 600px;"></iframe>
    pub timing_list: Vec<Timing<'a>>,
}