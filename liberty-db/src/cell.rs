use hashbrown::HashMap;
use crate::pin::{Pin, PinId};
#[derive(Debug)]
// #[derive(liberty_macros::Group)]
pub struct Cell{
    pub pin: HashMap<PinId,Pin>
}