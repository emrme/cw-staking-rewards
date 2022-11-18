#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::*;
use crate::state::{Config, CONFIG, TOTAL_STAKED};
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-yield-farming";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let validated_address = deps.api.addr_validate(&msg.owner)?;

    let config = Config {
        owner: validated_address,
        staking_token: _msg.staking_token,
        reward_token: _msg.reward_token,
        reward_duration: _msg.reward_duration,
        reward_rate: _msg.reward_rate,
    };

    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(_deps.s, &config)?;

    // TOTAL_STAKED.save(_deps.storage, &0u128)?; // TODO Check if its needed?

    Ok(Response::new()
        .add_attribute("method", "instantiate")
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
        ExecuteMsg::Receive(receive_msg) => {}
        ExecuteMsg::Unstake { amount } => {}
        ExecuteMsg::ClaimRewards {} => {}
        ExecuteMsg::UpdateRewardRate { new_reward_rate } => {}
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::StakerInfo { staker } => query_staker_info(_deps, staker),
        QueryMsg::StakingToken {} => query_staking_token(_deps),
        QueryMsg::RewardToken {} => query_reward_token(_deps),
        QueryMsg::TotalStaked {} => query_total_staked(_deps),
        QueryMsg::RewardRate {} => query_reward_rate(_deps),
    }
}

#[cfg(test)]
mod tests {}
