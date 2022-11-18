use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub staking_token: Addr,
    pub reward_token: Addr,
    pub reward_rate: Decimal,
    pub reward_duration: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct StakerInfo {
    pub staked_amount: u128,
    pub last_claim: u64,
}

pub const STAKERS: Map<&Addr, StakerInfo> = Map::new("stakers");
pub const TOTAL_STAKED: Item<u128> = Item::new("total_staked");
