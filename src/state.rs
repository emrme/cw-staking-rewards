use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub staking_token: Addr,
    pub reward_token: Addr,
    pub reward_rate: Decimal,
    pub reward_duration: Uint64,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct StakerInfo {
    pub staked_amount: Uint128,
    pub last_claim: Uint64,
}

pub const STAKERS: Map<&Addr, StakerInfo> = Map::new("stakers");
pub const TOTAL_STAKED: Item<Uint128> = Item::new("total_staked");
