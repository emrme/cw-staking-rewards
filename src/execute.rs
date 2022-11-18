use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdError, Uint128, Uint64};
use cw20::Cw20ReceiveMsg;

use crate::{
    state::{StakerInfo, CONFIG, STAKERS},
    ContractError,
};

pub fn handle_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receive_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.staking_token {
        // Only staking token contract
        return Err(ContractError::Unauthorized {});
    }

    let sender_addr = deps.api.addr_validate(&receive_msg.sender)?;

    let mut staker_info = STAKERS
        .load(deps.storage, &sender_addr)
        .unwrap_or_else(|_| StakerInfo {
            staked_amount: Uint128::new(0),
            last_claim: Uint64::new(env.block.time.seconds()),
        });

    staker_info.staked_amount += receive_msg.amount;

    Ok(Response::new())
}
