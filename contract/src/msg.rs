use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecuteMsg {
    Increment {},
    Decrement {},
    AddAdmin { admin: String },
}

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QueryMsg {
    GetCount {},
}

//#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CountResponse {
    pub count: i64,
    pub owner: Addr,
    pub admins: Vec<Addr>,
}
