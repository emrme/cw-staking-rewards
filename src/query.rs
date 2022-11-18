use crate::state::{CONFIG, STAKERS, TOTAL_STAKED};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult};

pub fn query_staker_info(deps: Deps, staker: Addr) -> StdResult<Binary> {
    let info = STAKERS.load(deps.storage, &staker)?;
    to_binary(&info)
}

pub fn query_staking_token(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&config.staking_token)
}

pub fn query_reward_token(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&config.reward_token)
}

pub fn query_total_staked(deps: Deps) -> StdResult<Binary> {
    let total_staked = TOTAL_STAKED.load(deps.storage)?;
    to_binary(&total_staked)
}

pub fn query_reward_rate(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&config.reward_rate)
}
