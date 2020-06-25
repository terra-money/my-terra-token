// src/contract.rs

use cosmwasm_std::{
    generic_err, log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdResult, Storage, Uint128,
};

use crate::msg::{BalanceResponse, ConfigResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{balance_of, balance_set, config, config_read, Config};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    // Initial balances
    for row in msg.initial_balances {
        let address = deps.api.canonical_address(&row.address)?;
        balance_set(&mut deps.storage, &address, &row.amount)?;
    }
    config(&mut deps.storage).save(&Config {
        name: msg.name,
        symbol: msg.symbol,
        owner: env.message.sender,
    })?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Transfer { recipient, amount } => try_transfer(deps, env, &recipient, &amount),
        HandleMsg::Burn { amount } => try_burn(deps, env, &amount),
    }
}

fn try_transfer<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    recipient: &HumanAddr,
    amount: &Uint128,
) -> StdResult<HandleResponse> {
    // canonical address
    let sender_address = &env.message.sender;
    let recipient_address = &deps.api.canonical_address(recipient)?;

    // check that sender's funds covers
    let mut sender_balance = balance_of(&deps.storage, sender_address);
    if sender_balance < *amount {
        return Err(generic_err(format!(
            "Insufficient funds to send: balance={}, required={}",
            sender_balance, amount
        )));
    }
    // update balances
    sender_balance = (sender_balance - *amount)?;
    let mut recipient_balance = balance_of(&deps.storage, recipient_address);
    recipient_balance = recipient_balance + *amount;

    balance_set(&mut deps.storage, sender_address, &sender_balance)?;
    balance_set(&mut deps.storage, recipient_address, &recipient_balance)?;

    // report what happened in the log
    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "send"),
            log("sender", deps.api.human_address(sender_address)?),
            log("recipient", recipient),
            log("amount", amount),
        ],
        data: None,
    };

    Ok(res)
}

fn try_burn<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
) -> StdResult<HandleResponse> {
    // canonical address
    let sender_address = &env.message.sender;

    let mut sender_balance = balance_of(&deps.storage, sender_address);
    if sender_balance < *amount {
        return Err(generic_err(format!(
            "Insufficient funds to burn: balance={}, required={}",
            sender_balance, amount
        )));
    }
    // update balance
    sender_balance = (sender_balance - *amount)?;
    balance_set(&mut deps.storage, sender_address, &sender_balance)?;

    let res = HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "burn"),
            log("sender", deps.api.human_address(sender_address)?),
            log("amount", amount),
        ],
        data: None,
    };

    Ok(res)
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => {
            let address = deps.api.canonical_address(&address)?;
            let balance = balance_of(&deps.storage, &address);
            let out = to_binary(&BalanceResponse { balance })?;
            Ok(out)
        }
        QueryMsg::Config {} => {
            let config = config_read(&deps.storage).load()?;
            let out = to_binary(&ConfigResponse {
                name: config.name,
                symbol: config.symbol,
                owner: deps.api.human_address(&config.owner)?,
            })?;
            Ok(out)
        }
    }
}
