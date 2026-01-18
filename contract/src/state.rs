use cosmwasm_std::Addr;
use cw_storage_plus::Item;

//[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub owner: Addr,
    pub admins: Vec<Addr>,
    pub count: i64,
}

pub const STATE: Item<State> = Item::new("state");
