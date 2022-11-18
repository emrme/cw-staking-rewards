#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::*;
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
    // unimplemented!()
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // unimplemented!()
    use QueryMsg::*;
    match _msg {
        StakerInfo { staker } => query_staker_info(_deps, staker),
        StakingToken {} => query_staking_token(_deps),
        RewardToken {} => query_reward_token(_deps),
        TotalStaked {} => query_total_staked(_deps),
        RewardRate {} => query_reward_rate(_deps),
    }
}

#[cfg(test)]
mod tests {}
