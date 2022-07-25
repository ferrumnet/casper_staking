#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec;
use alloc::{boxed::Box, collections::BTreeSet, format, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    runtime_args, CLType, CLTyped, CLValue, ContractPackageHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, Parameter, RuntimeArgs, URef, U256,
};
use cep47::cep47::CEP20STK;
use contract_utils::{ContractContext, OnChainContractStorage};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl CEP20STK<OnChainContractStorage> for Token {}
impl Token {
    fn constructor(
        &mut self,
        name: String,
        address: String,
        staking_starts: u64,
        staking_ends: u64,
        withdraw_starts: u64,
        withdraw_ends: u64,
        staking_total: U256,
    ) {
        CEP20STK::init(
            self,
            name,
            address,
            staking_starts,
            staking_ends,
            withdraw_starts,
            withdraw_ends,
            staking_total,
        );
    }
}


#[no_mangle]
fn approve() {
}

#[no_mangle]
fn constructor() {
    let name = runtime::get_named_arg::<String>("name");
    let address = runtime::get_named_arg::<String>("address");
    let staking_starts: u64 = runtime::get_named_arg::<u64>("staking_starts");
    let staking_ends: u64 = runtime::get_named_arg::<u64>("staking_ends");
    let withdraw_starts: u64 = runtime::get_named_arg::<u64>("withdraw_starts");
    let withdraw_ends: u64 = runtime::get_named_arg::<u64>("withdraw_ends");
    let staking_total: U256 = runtime::get_named_arg::<U256>("staking_total");

    Token::default().constructor(
        name,
        address,
        staking_starts,
        staking_ends,
        withdraw_starts,
        withdraw_ends,
        staking_total,
    );
}

#[no_mangle]
fn name() {
    let ret = Token::default().name();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn address() {
    let ret = Token::default().address();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn staking_starts() {
    let ret = Token::default().staking_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn staking_ends() {
    let ret = Token::default().staking_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn withdraw_starts() {
    let ret = Token::default().withdraw_starts();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn withdraw_ends() {
    let ret = Token::default().withdraw_ends();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn staking_total() {
    let ret = Token::default().staking_total();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn amount_staked() {
    let staker = runtime::get_named_arg::<Key>("staker");
    let ret = Token::default().amount_staked(staker);
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn stake() {
    let amount = runtime::get_named_arg::<U256>("amount");
    let ret = Token::default().stake(amount).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn withdraw() {
    let amount = runtime::get_named_arg::<U256>("amount");
    let ret = Token::default().withdraw(amount).unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn add_reward() {
    let reward_amount = runtime::get_named_arg::<U256>("reward_amount");
    let withdrawable_amount = runtime::get_named_arg::<U256>("withdrawable_amount");
    let ret = Token::default()
        .add_reward(reward_amount, withdrawable_amount)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(ret).unwrap_or_revert());
}

#[no_mangle]
fn call() {
    // Read arguments for the constructor call.
    let name: String = runtime::get_named_arg("name");
    let address = runtime::get_named_arg::<String>("address");
    let staking_starts: u64 = runtime::get_named_arg::<u64>("staking_starts");
    let staking_ends: u64 = runtime::get_named_arg::<u64>("staking_ends");
    let withdraw_starts: u64 = runtime::get_named_arg::<u64>("withdraw_starts");
    let withdraw_ends: u64 = runtime::get_named_arg::<u64>("withdraw_ends");
    let staking_total: U256 = runtime::get_named_arg::<U256>("staking_total");
    // let contract_name: String = runtime::get_named_arg("contract_name");

    /*

    casper-client put-deploy \
      --chain-name casper-test \
      --node-address http://159.65.118.250:7777 \
      --secret-key ./cep47/keys/secret_key.pem \
      --session-path ./cep47/target/wasm32-unknown-unknown/release/cep47.wasm \
      --payment-amount 200000000000 \
      --session-arg "name:string='FerrumX'" \
      --session-arg "address:string='9e7283533626d0c7d43fa9ca745af20d8dac7fc3bfe03cdfe50d523a2a0f498d'" \
      --session-arg "staking_starts:u64='0'" \
      --session-arg "staking_ends:u64='1755994649'" \
      --session-arg "withdraw_starts:u64='0'" \
      --session-arg "withdraw_ends:u64='1755994649'" \
      --session-arg "staking_total:U256='500000'"

        */
/*
        casper-client put-deploy \
        --chain-name casper-test \
        --node-address http://159.65.118.250:7777 \
        --secret-key cep47/keys/secret_key.pem \
        --payment-amount 4000000000 \
        --session-hash 28fdc514d76a3f39bac9cf6a84ff81b586a06374a40d90838d9f44a9f4e5a179 \
        --session-entry-point "stake" \
        --session-arg "amount:u256='20'"
  */


/*
        casper-client put-deploy \
        --chain-name casper-test \
        --node-address http://159.65.118.250:7777 \
        --secret-key cep47/keys/secret_key.pem \
        --payment-amount 4000000000 \
        --session-hash 28fdc514d76a3f39bac9cf6a84ff81b586a06374a40d90838d9f44a9f4e5a179 \
        --session-entry-point "withdraw" \
        --session-arg "amount:u256='10'"
  */


  /*
        casper-client put-deploy \
        --chain-name casper-test \
        --node-address http://159.65.118.250:7777 \
        --secret-key cep47/keys/secret_key.pem \
        --payment-amount 4000000000 \
        --session-hash 3b67aef4633a84feaa6a3e4a6090ae76cc304c948fc92de09356ba114474c3de \
        --session-entry-point "add_reward" \
        --session-arg "reward_amount:u256='20'" \
        --session-arg "withdrawable_amount:u256='19'"
  */

  /*
  casper-client get-deploy \
      --id 2 \
      --node-address http://159.65.118.250:7777 \
      ee07324ad466aad373e94f787b3dbf1ba1ff00175e97a0bce002bb45737ad5e6
  
  */

    // Prepare constructor args
    let constructor_args = runtime_args! {
        "name" => name,
        "address" => address,
        "staking_starts" => staking_starts,
        "staking_ends" => staking_ends,
        "withdraw_starts" => withdraw_starts,
        "withdraw_ends" => withdraw_ends,
        "staking_total" => staking_total
    };

    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        None,
        Some(String::from("contract_package_hash")),
        None,
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key("contract_package_hash")
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );

    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();

    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key(
        &format!("{}_contract_hash", contract_hash),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{}_contract_hash_wrapped", contract_hash),
        storage::new_uref(contract_hash).into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("name", String::cl_type()),
            Parameter::new("address", String::cl_type()),
            Parameter::new("staking_starts", u64::cl_type()),
            Parameter::new("staking_ends", u64::cl_type()),
            Parameter::new("withdraw_starts", u64::cl_type()),
            Parameter::new("withdraw_ends", u64::cl_type()),
            Parameter::new("staking_total", U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "name",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "address",
        vec![],
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "staking_starts",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw_starts",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw_ends",
        vec![],
        u64::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "staking_total",
        vec![],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "withdraw",
        vec![
            Parameter::new("amount", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "stake",
        vec![
            Parameter::new("amount", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points.add_entry_point(EntryPoint::new(
        "add_reward",
        vec![
            Parameter::new("reward_amount", Key::cl_type()),
            Parameter::new("withdrawable_amount", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    // entry_points.add_entry_point(EntryPoint::new(
    //     "transfer",
    //     vec![
    //         Parameter::new("recipient", Key::cl_type()),
    //     ],
    //     <()>::cl_type(),
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract,
    // ));
    // entry_points.add_entry_point(EntryPoint::new(
    //     "transfer_from",
    //     vec![
    //         Parameter::new("sender", Key::cl_type()),
    //         Parameter::new("recipient", Key::cl_type()),
    //     ],
    //     <()>::cl_type(),
    //     EntryPointAccess::Public,
    //     EntryPointType::Contract,
    // ));
    entry_points.add_entry_point(EntryPoint::new(
        "approve",
        vec![
            Parameter::new("spender", Key::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    entry_points
}
