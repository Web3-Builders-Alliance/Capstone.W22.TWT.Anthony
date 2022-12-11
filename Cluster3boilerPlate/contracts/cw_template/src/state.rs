use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub counter: i32,
}

pub const STATE: Item<State> = Item::new("state");
