use crate::{expression, pin::PinIdx, timing::TimingType};


struct TableId{
    vec: expression::LogicVector,
    pin: PinIdx,
    related_pin: PinIdx,
    info: TableInfo,
}

enum TableInfo {
    Power(PowerInfo),
    Timing(TimingInfo),
}

struct TimingInfo{
    timing_type: TimingType,
}
struct PowerInfo{

}