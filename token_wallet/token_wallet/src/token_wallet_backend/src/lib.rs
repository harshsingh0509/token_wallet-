use ic_cdk::storage;
use ic_cdk_macros::{update, query, init, pre_upgrade, post_upgrade};
use candid::{CandidType, Deserialize};
use serde::{Serialize, Deserialize};

#[derive(CandidType, Serialize, Deserialize, Debug, Default)]
pub struct Wallet {
    balance: u64,
}

#[init]
fn init_wallet(initial_balance: u64) {
    let wallet = Wallet { balance: initial_balance };
    storage::stable_save((wallet,)).expect("Failed to initialize wallet.");
}

#[update]
fn deposit(amount: u64) {
    let mut wallet: Wallet = storage::stable_restore().unwrap_or_default();
    wallet.balance += amount;
    storage::stable_save((wallet,)).expect("Failed to save wallet.");
}

#[update]
fn withdraw(amount: u64) -> Result<u64, String> {
    let mut wallet: Wallet = storage::stable_restore().unwrap_or_default();
    if amount > wallet.balance {
        return Err("Insufficient balance.".to_string());
    }
    wallet.balance -= amount;
    storage::stable_save((wallet,)).expect("Failed to save wallet.");
    Ok(wallet.balance)
}

#[query]
fn balance() -> u64 {
    let wallet: Wallet = storage::stable_restore().unwrap_or_default();
    wallet.balance
}

#[pre_upgrade]
fn pre_upgrade() {
    let wallet: Wallet = storage::stable_restore().expect("Failed to retrieve wallet for upgrade.");
    storage::stable_save((wallet,)).expect("Failed to save wallet before upgrade.");
}

#[post_upgrade]
fn post_upgrade() {
    let wallet: Wallet = storage::stable_restore().unwrap_or_default();
    storage::stable_save((wallet,)).expect("Failed to restore wallet after upgrade.");
}
