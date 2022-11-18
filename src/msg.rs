use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128, Uint64};
use cw20::Cw20ReceiveMsg;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub staking_token: String,
    pub reward_token: String,
    pub reward_amount: Uint128,
    pub reward_duration: Uint64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Stake { amount: Uint128 },
    Withdraw { amount: Uint128 },
    ClaimReward {},
    SetRewardAmount { amount: Decimal },
    SetRewardDuration { duration: Uint64 },
    Receive(Cw20ReceiveMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StakerInfoResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct StakerInfoResponse {
    pub staker: Addr,
    pub staked_amount: u128,
    pub pending_rewards: u128,
}

#[cw_serde]
pub struct StakingTokenResponse {
    pub staking_token: Addr,
}
