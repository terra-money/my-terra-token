// src/msg.rs
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct InitialBalance {
    pub address: HumanAddr,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub symbol: String,
    pub initial_balances: Vec<InitialBalance>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Transfer {
        recipient: HumanAddr,
        amount: Uint128,
    },
    Burn {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: HumanAddr },
    Config {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub name: String,
    pub symbol: String,
    pub owner: HumanAddr,
}
