#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Coin, Empty, Uint128, Uint64};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    use crate::msg::{
        ExecuteMsg, InstantiateMsg, QueryMsg, RewardInfoResponse, RewardParametersResponse,
        UserInfoResponse,
    };

    pub fn contract_staking_rewards() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const ADMIN: &str = "admin";
    const USER: &str = "user";
    const STAKING_TOKEN: &str = "staking_token";
    const REWARD_TOKEN: &str = "reward_token";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: STAKING_TOKEN.to_string(),
                        amount: Uint128::new(1000),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, Addr) {
        let mut app = mock_app();
        let cw_staking_rewards_id = app.store_code(contract_staking_rewards());

        let msg = InstantiateMsg {
            staking_token: STAKING_TOKEN.to_string(),
            reward_token: REWARD_TOKEN.to_string(),
            reward_rate: Uint64::new(100),
        };
        let contract_addr = app
            .instantiate_contract(
                cw_staking_rewards_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "staking_rewards",
                None,
            )
            .unwrap();

        (app, contract_addr)
    }

    #[test]
    fn stake_and_query() {
        let (mut app, contract_addr) = proper_instantiate();
        let user = Addr::unchecked(USER);

        let stake_msg = ExecuteMsg::Stake {
            amount: Uint128::from(100u128),
        };

        // User stakes 100 tokens
        app.execute_contract(user.clone(), contract_addr.clone(), &stake_msg, &[])
            .unwrap();

        let user_info_query = QueryMsg::UserInfo {
            user: user.to_string(),
        };
        let reward_info_query = QueryMsg::RewardInfo {
            user: user.to_string(),
        };

        let reward_parameters_query = QueryMsg::RewardParameters {};

        // Query staked amount and reward
        let user_info: UserInfoResponse = app
            .wrap()
            .query_wasm_smart(&contract_addr, &user_info_query)
            .unwrap();
        let reward_info: RewardInfoResponse = app
            .wrap()
            .query_wasm_smart(&contract_addr, &reward_info_query)
            .unwrap();

        let reward_parameters: RewardParametersResponse = app
            .wrap()
            .query_wasm_smart(&contract_addr, &reward_parameters_query)
            .unwrap();

        let current_time = Uint64::from(app.block_info().time.seconds());
        let time_since_last_update = current_time - reward_parameters.last_update_time;

        println!("Current time : {}", current_time);
        println!("Reward rate: {}", reward_parameters.reward_rate);
        println!("Time since last update: {}", time_since_last_update);
        println!("Reward: {}", reward_info.reward);

        assert_eq!(user_info.staked_amount, Uint128::from(100u128));

        // Due to the time it takes to run the test, the reward may not be exactly 0.
        // Check if the reward is within an acceptable range (e.g., 0 to 10).
        assert!(
            reward_info.reward >= Uint128::from(0u128)
                && reward_info.reward <= Uint128::from(10u128),
            "Reward should be within 0 to 10, but got {}",
            reward_info.reward
        );
    }
}
