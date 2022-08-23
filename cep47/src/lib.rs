#![no_std]
extern crate alloc;

pub mod address;
pub mod cep47;
pub mod data;
pub mod detail;
pub mod event;
pub mod modifiers;

pub use cep47::{Error, CEP20STK};
pub use contract_utils;

use alloc::{collections::BTreeMap, string::String};
use casper_types::U256;
pub type TokenId = U256;
pub type Meta = BTreeMap<String, String>;
