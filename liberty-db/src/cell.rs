use hashbrown::HashMap;
use crate::pin::Pin;
#[derive(Debug, Clone)]
pub struct Cell<'a>{
    pub pin_map: HashMap<&'a str,Pin<'a>>
}