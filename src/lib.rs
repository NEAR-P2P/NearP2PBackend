//! This contract implements all the necesary operations to work with p2p transactions.
//!
//!
//! [lock]: struct.lock.html#method.lock
//! [unlock]: struct.unlock.html#method.unlock
//! [get_locked_balance]: struct.get_locked_balance.html#method.get_locked_balance
//! [set_offers]: struct.get_orders.html#method.get_orders
//! [get_offers]: struct.set_order.html#method.set_order


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::Serialize;
use serde::Deserialize;
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance};
use std::collections::HashMap;
near_sdk::setup_alloc!();

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Account {
    /// Current unlocked balance.
    pub balance: Balance,
    /// Allowed account to the allowance amount.
    pub allowances: HashMap<AccountId, Balance>,
    /// Allowed account to locked balance.
    pub locked_balances: HashMap<AccountId, Balance>,
}

impl Account {

    pub fn set_locked_balance(&mut self, escrow_account_id: &AccountId, locked_balance: Balance) {
        if locked_balance > 0 {
            self.locked_balances.insert(escrow_account_id.clone(), locked_balance);
        } else {
            self.locked_balances.remove(escrow_account_id);
        }
    }

    pub fn get_locked_balance(&self, escrow_account_id: &AccountId) -> Balance {
        *self.locked_balances.get(escrow_account_id).unwrap_or(&0)
    }
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OFFERObject {
    order_id: i8,
    owner_id: String,
    asset: String, // NEAR, USD
    price: String,
    amount: String,
    min_limit: String,
    max_limit: String,
    order_type: i8, // 1 = sell, 2 = buy
    payment_method: i8, // Info concerning to payment asociated to payment contract
    orders_completed: i8,
    percentaje_completion: i8,
    badge: String, //Badge that represent a merchant verified
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearP2P {
    /// AccountID -> Account details.
    pub accounts: UnorderedMap<AccountId, Account>,
    ///Orders object
    pub offers: Vec<OFFERObject>,
    ///Order Id
    pub order_id: i8,
}

///Initializing deafult impl
impl Default for NearP2P {
    fn default() -> Self {
        Self {
            accounts: UnorderedMap::new(b"a"),
            offers: Vec::new(),
            order_id: 0,
        }
    }
}

#[near_bindgen]
impl NearP2P {

    /// Locks an amount from
    /// the `owner_id`.
    /// Requirements:
    /// * The owner should have enough unlocked balance.
    #[payable]
    pub fn lock(&mut self, owner_id: AccountId) {
        let lock_amount = env::attached_deposit();
        if lock_amount == 0 {
            env::panic("Can't lock 0 tokens".as_bytes());
        }
        let escrow_account_id = env::signer_account_id();
        let mut account = self.get_account(&owner_id);
        account.balance = env::account_balance();
        // Checking and updating unlocked balance
        if account.balance < lock_amount {
            env::panic("Not enough unlocked balance".as_bytes());
        }
        account.balance -= lock_amount;

        // Updating total lock balance
        let locked_balance = account.get_locked_balance(&escrow_account_id);
        account.set_locked_balance(&escrow_account_id, locked_balance + lock_amount);

        self.accounts.insert(&owner_id, &account);
        env::log(b"Amount locked, waiting for transaction");
    }

    /// Returns current locked balance for the `owner_id` locked by `escrow_account_id`.
    pub fn get_locked_balance(&self, owner_id: AccountId, escrow_account_id: AccountId) -> String {
        self.get_account(&owner_id).get_locked_balance(&escrow_account_id).to_string()
    }

    /// Returns the order object loaded in contract
    pub fn get_offers(self) -> Vec<OFFERObject> {
        self.offers
    }

    //Set the offer object into the contract
    pub fn set_offers(&mut self, owner_id: String
        , asset: String
        , price: String
        , amount: String
        , min_limit: String
        , max_limit: String
        , order_type: i8
        , payment_method: i8
        , orders_completed: i8 
        , percentaje_completion: i8
        , badge: String) -> i8{
        self.order_id += 1;
        let data = OFFERObject {
            order_id: self.order_id,
            owner_id: String::from(owner_id),
            asset: String::from(asset),
            price: String::from(price),
            amount: String::from(amount),
            min_limit: String::from(min_limit),
            max_limit: String::from(max_limit),
            order_type: order_type,
            payment_method: payment_method,
            orders_completed: orders_completed,
            percentaje_completion: percentaje_completion,
            badge: badge,
        };
        self.offers.push(data);
        env::log(b"Order Created");
        self.order_id
    }

}

impl NearP2P {
    /// Helper method to get the account details for `owner_id`.
    fn get_account(&self, owner_id: &AccountId) -> Account {
        self.accounts.get(owner_id).unwrap_or_default()
    }
}



// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_order() {
        let context = get_context(vec![], false);
        testing_env!(context);
        //let attached_deposit = env::attached_deposit();
        let mut contract = NearP2P::default();
        let account_id = "info.testnet".to_string();
        let asset = "NEAR".to_string();
        let price = "14.5".to_string();
        let amount = "100".to_string();
        let min_limit = "10".to_string();
        let max_limit = "100".to_string();
        let order_type = 1;
        let payment_method = 1;
        let orders_completed = 0;
        let percentaje_completion = 0;
        let badge = "super star".to_string();
        contract.set_offers(account_id, asset, price, amount, min_limit, max_limit, order_type, payment_method, orders_completed, percentaje_completion, badge);
        assert_eq!(contract.get_offers().len(), 1);
    }

    #[test]
    fn test_lock() {
        let context = get_context(vec![], false);
        testing_env!(context);
        //let attached_deposit = env::attached_deposit();
        let mut contract = NearP2P::default();
        let escrow_account_id = env::predecessor_account_id();
        let account_id = "info.testnet".to_string();
        contract.lock(account_id.to_string());
        print!("Locked balance: {}", contract.get_locked_balance(account_id.to_string(), escrow_account_id));
    }
    
}