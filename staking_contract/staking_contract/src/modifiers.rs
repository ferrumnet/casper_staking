use crate::error::Error;
use casper_contract::contract_api::runtime;
use casper_types::{BlockTime, U256};

pub fn positive(amount: U256) -> Result<(), Error> {
    if amount <= U256::from(0) {
        Err(Error::NegativeAmount)
    } else {
        Ok(())
    }
}

pub fn after(event_time: u64) -> Result<(), Error> {
    if runtime::get_blocktime() < BlockTime::new(event_time) {
        Err(Error::AfterBadTiming)
    } else {
        Ok(())
    }
}

pub fn before(event_time: u64) -> Result<(), Error> {
    if runtime::get_blocktime() >= BlockTime::new(event_time) {
        Err(Error::BeforeBadTiming)
    } else {
        Ok(())
    }
}
