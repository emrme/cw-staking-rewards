use std::ops::Add;

use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response, StdError, Uint128, Uint64};
use cw20::Cw20ReceiveMsg;

use crate::{
    state::{StakerInfo, CONFIG, STAKERS, TOTAL_STAKED},
    ContractError,
};

// pub fn handle_receive(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     receive_msg: Cw20ReceiveMsg,
// ) -> Result<Response, ContractError> {
//     let config = CONFIG.load(deps.storage)?;

//     let sender_addr = info.sender.clone();

//     let staking_token_addr = deps.api.addr_validate(&receive_msg.sender)?;

//     // Check if the received token is the staking token
//     if staking_token_addr != config.staking_token {
//         return Err(ContractError::InvalidStakingToken);
//     }

//     // let mut staker_info = STAKERS
//     //     .load(deps.storage, &sender_addr)
//     //     .unwrap_or_else(|_| StakerInfo {
//     //         staked_amount: Uint128::new(0),
//     //         last_claim: Uint64::new(env.block.time.seconds()),
//     //     });
//     // staker_info.staked_amount += receive_msg.amount;
//     // STAKERS.save(deps.storage, &sender_addr, &staker_info);

//     STAKERS.update(deps.storage, &sender_addr, |staker_info| {
//         let mut staker_info = staker_info.get_or_insert_with(|| StakerInfo {
//             staked_amount: Uint128::zero(),
//             last_claim: Uint64::new(env.block.time.seconds()),
//         });

//         staker_info.staked_amount += receive_msg.amount;
//         // staker_info.last_claim = env.block.time.seconds();
//         Ok(*staker_info)
//     })?;

//     STAKERS.update(deps.storage, &sender_addr, |staker_info| {
//         let mut staker_info = staker_info.unwrap_or_else(|| StakerInfo {
//             staked_amount: Uint128::zero(),
//             last_claim: env.block.time.seconds(),
//             pending_rewards: Uint128::zero(),
//         });

//         staker_info.staked_amount += receive_msg.amount;
//         Ok(staker_info)
//     })?;

//     TOTAL_STAKED.update(
//         deps.storage,
//         |total: Uint128| Ok(total + receive_msg.amount),
//     )?;

//     Ok(Response::default())
// }

fn execute_stake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Decimal,
) -> StdResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;
    let staking_token = &config.staking_token;

    let send_msg = Cw20ExecuteMsg::Send {
        contract: info.sender.to_string(),
        amount,
        msg: to_binary(&ExecuteMsg::Receive(Cw20ReceiveMsg {
            sender: info.sender.to_string(),
            amount,
        }))?,
    };

    let cosmos_msg = WasmMsg::Execute {
        contract_addr: staking_token.to_string(),
        msg: to_binary(&send_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(cosmos_msg)
        .add_attribute("action", "stake"))
}

fn execute_set_reward_amount(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Decimal,
) -> StdResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    config.reward_amount = amount;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "set_reward_amount"))
}

fn execute_set_reward_duration(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    duration: u64,
) -> StdResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    config.reward_duration = duration;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "set_reward_duration"))
}
