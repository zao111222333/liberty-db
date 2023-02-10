use hashbrown::HashMap;

use crate::pin::Pin;
#[derive(Debug, Clone)]
pub struct Cell{
    pin_map: HashMap<String,Pin>
}