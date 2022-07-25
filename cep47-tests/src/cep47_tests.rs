use std::collections::BTreeMap;

use casper_types::{account::AccountHash, Key, U256};
use test_env::TestEnv;

use crate::cep47_instance::{CEP47Instance, Meta, TokenId};

const NAME: &str = "staking";
const ADDRESS: &str = "9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d";

fn deploy() -> (TestEnv, CEP47Instance, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let contract = CEP47Instance::new(
        &env,
        NAME,
        owner,
        NAME.to_string(),
        ADDRESS.to_string(),
        0,
        1000000,
        0,
        1000000,
        U256::from(100000i64),
    );
    (env, contract, owner)
}

#[test]
fn test_stake() {
    let (env, contract, owner) = deploy();
    let user = env.next_user();

    let res = contract.stake(owner, U256::from(1i64));
}

#[test]
fn test_withdraw() {
    let (env, contract, owner) = deploy();
    let user = env.next_user();

    // contract.stake(owner, U256::from(10i64));
    contract.withdraw(owner, U256::from(1i64));
    // assert_eq!(token.total_supply(), U256::from(1));
    // assert_eq!(token.balance_of(Key::Account(user)), U256::from(1));
}

#[test]
fn test_add_reward() {
    let (env, contract, owner) = deploy();
    let user = env.next_user();

    contract.add_reward(owner, U256::from(1i64), U256::from(1i64));
    // assert_eq!(token.total_supply(), U256::from(1));
    // assert_eq!(token.balance_of(Key::Account(user)), U256::from(1));
}
