use std::collections::BTreeMap;

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, CLTyped, Key, RuntimeArgs, U256,
};
use test_env::{TestContract, TestEnv};

pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;

pub struct CEP47Instance(TestContract);

impl CEP47Instance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        name: String,
        address: String,
        staking_starts: u64,
        staking_ends: u64,
        withdraw_starts: u64,
        withdraw_ends: u64,
        staking_total: U256,
    ) -> CEP47Instance {
        CEP47Instance(TestContract::new(
            env,
            "/home/ubuntu/forks/casper_staking/cep47/target/wasm32-unknown-unknown/release/cep47.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "address" => address,
                "staking_starts" => staking_starts,
                "staking_ends" => staking_ends,
                "withdraw_starts" => withdraw_starts,
                "withdraw_ends" => withdraw_ends,
                "staking_total" => staking_total,
            },
        ))
    }

    pub fn constructor(
        &self,
        sender: AccountHash,
        name: String,
        address: String,
        staking_starts: u64,
        staking_ends: u64,
        withdraw_starts: u64,
        withdraw_ends: u64,
        staking_total: U256,
    ) {
        self.0.call_contract(
            sender,
            "constructor",
            runtime_args! {
                "name" => name,
                "address" => address,
                "staking_starts" => staking_starts,
                "staking_ends" => staking_ends,
                "withdraw_starts" => withdraw_starts,
                "withdraw_ends" => withdraw_ends,
                "staking_total" => staking_total,
            },
        );
    }

    pub fn withdraw(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "withdraw",
            runtime_args! {
                "amount" => amount,
            },
        )
    }

    pub fn stake(&self, sender: AccountHash, amount: U256) {
        self.0.call_contract(
            sender,
            "stake",
            runtime_args! {
                "amount" => amount,
            },
        )
    }

    pub fn add_reward(&self, sender: AccountHash, reward_amount: U256, withdrawable_amount: U256) {
        self.0.call_contract(
            sender,
            "add_reward",
            runtime_args! {
                "reward_amount" => reward_amount,
                "withdrawable_amount" => withdrawable_amount,
            },
        )
    }

    pub fn total_reward<T: Into<Key>>(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "total_reward", runtime_args! {})
    }

    pub fn reward_balance<T: Into<Key>>(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "reward_balance", runtime_args! {})
    }

    pub fn early_withdraw_reward<T: Into<Key>>(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "early_withdraw_reward", runtime_args! {})
    }

    pub fn staking_total<T: Into<Key>>(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "staking_total", runtime_args! {})
    }

    pub fn staked_balance<T: Into<Key>>(&self, sender: AccountHash) {
        self.0
            .call_contract(sender, "staked_balance", runtime_args! {})
    }
}
