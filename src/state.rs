use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub staking_token: Addr,
    pub reward_token: Addr,
    pub reward_rate: Decimal,
    pub reward_duration: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
