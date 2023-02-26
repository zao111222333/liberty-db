use crate::{expression, pin::PinId, timing::TimingType};


struct TableId{
    vec: expression::LogicVector,
    pin: PinId,
    related_pin: PinId,
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