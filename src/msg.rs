use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal};
use cw20::Cw20ReceiveMsg;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub staking_token: Addr,
    pub reward_token: Addr,
    pub reward_rate: Decimal,
    pub reward_duration: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Unstake { amount: u128 },
    ClaimRewards,
    UpdateRewardRate { new_reward_rate: Decimal },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(StakerInfoResponse)]
    StakerInfo { staker: Addr },
    #[returns(StakingTokenResponse)]
    StakingToken,
    #[returns(RewardTokenResponse)]
    RewardToken,
    #[returns(TotalStakedResponse)]
    TotalStaked,
    #[returns(RewardRateResponse)]
    RewardRate,
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

#[cw_serde]
pub struct RewardTokenResponse {
    pub reward_token: Addr,
}

#[cw_serde]
pub struct TotalStakedResponse {
    pub total_staked: u128,
}

#[cw_serde]
pub struct RewardRateResponse {
    reward_rate: Decimal,
}
