#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint64,
};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::*;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::*;
use crate::state::{Config, CONFIG, TOTAL_STAKED};

const CONTRACT_NAME: &str = "crates.io:cw-yield-farming";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        admin: _info.sender.clone(),
        staking_token: _deps.api.addr_validate(&_msg.staking_token)?,
        reward_token: _deps.api.addr_validate(&_msg.reward_token)?,
        reward_amount: _msg.reward_amount,
        reward_duration: _msg.reward_duration,
        staking_start_time: env.block.time.seconds(),
    };

    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(_deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("sender", _info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // use ExecuteMsg::*;
    match _msg {
        ExecuteMsg::Receive(receive_msg) => Ok(Response::new()),
        ExecuteMsg::Stake { amount } => execute_stake(_deps, _env, _info, amount),
        ExecuteMsg::Withdraw { amount } => execute_withdraw(_deps, _env, _info, amount),
        ExecuteMsg::ClaimReward {} => execute_claim_reward(_deps, _env, _info),
        ExecuteMsg::SetRewardAmount { amount } => {
            execute_set_reward_amount(_deps, _env, _info, amount)
        }
        ExecuteMsg::SetRewardDuration { duration } => {
            execute_set_reward_duration(_deps, _env, _info, _duration)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::StakerInfo { staker } => {
            query_staker_info(_deps, _deps.api.addr_validate(&staker)?)
        }
        QueryMsg::StakingToken {} => query_staking_token(_deps),
        QueryMsg::RewardToken {} => query_reward_token(_deps),
        QueryMsg::TotalStaked {} => query_total_staked(_deps),
        QueryMsg::RewardRate {} => query_reward_rate(_deps),
    }
}

#[cfg(test)]
mod tests {}
