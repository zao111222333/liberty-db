use crate::{expression, timing::TimingType, ast::HashedGroup, pin::Pin};


struct TableId{
    vec: expression::LogicVector,
    pin: <Pin as HashedGroup>::Id,
    related_pin: <Pin as HashedGroup>::Id,
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