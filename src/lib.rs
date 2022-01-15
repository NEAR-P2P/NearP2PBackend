/*
NEAR-Dex

It’s all about to create a Peer to Peer (P2P) DAPP to allow change your’s NEAR tokens in a easy, 
secure and fast way.
A Decentralized Wallet that will provide P2P like Binance or Airtm where you can change your 
available Crypto’s in local money, fiat, other cryptos. Using your services online that are available already.
The idea is similiar to our friend @FritzWorm NearWalletDapp 5.

Issue & Solution thinking

At the moment people ask where they can change their NEAR and do not get a way to do it quickly and transparently.
This P2P feature will ease the process where people come into crypto, will help them in daily basic s
ending money/remittances and NEAR protocol will get a lot of transactions and volume helping REF Finance with 
liquidity from investors that will come with the mass adoption.
The process

MIT license
Develop by GlobalDv @2022
*/


use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::Serialize;
use serde::Deserialize;
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance};
use std::collections::HashMap;
near_sdk::setup_alloc!();

/// Create Struct About the account
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Account {
    /// Current unlocked balance.
    pub balance: Balance,
    /// Allowed account to the allowance amount.
    pub allowances: HashMap<AccountId, Balance>,
    /// Allowed account to locked balance.
    pub locked_balances: HashMap<AccountId, Balance>,
}

/// Implementing Struct
impl Account {
    /*
    Set the locked balance for user to start the p2p trade
    When User iniziate the process of buy, sell. Is nedded to set the locked balance for the user.
    */
    //Params:
    //escrow_account: AccountId,
    //locked_balance: Balance,
    pub fn set_locked_balance(&mut self, escrow_account_id: &AccountId, locked_balance: Balance) {
        if locked_balance > 0 {
            self.locked_balances.insert(escrow_account_id.clone(), locked_balance);
        } else {
            self.locked_balances.remove(escrow_account_id);
        }
    }

    //Get the locked balance for user to start the p2p trade
    //Param:
    //escrow_account: AccountId,
    pub fn get_locked_balance(&self, escrow_account_id: &AccountId) -> Balance {
        *self.locked_balances.get(escrow_account_id).unwrap_or(&0)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////

/*
User UserObject: Struct for the user that contains info about the logged user.
This object contains, user_id, name, last_name, phone, email, country
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserObject {
    user_id: String,
    name: String,
    last_name: String,
    phone: String,
    email: String,
    country: String,
}

/*
User OfferObject: Struct for offer that will be listed.
This object contains, order_id, owner_id, asset, exchange_rate, email, country
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferObject {
    order_id: i128,
    owner_id: String,
    asset: String, // NEAR, USD
    exchange_rate: String,
    amount: String,
    remaining_amount: String,
    min_limit: String,
    max_limit: String,
    payment_method: i128,
    confirmation: bool,
    status: i8,
}


/*
User MerchantObject: Struct for Merchants.
This object contains, user_id, total_orders, orders_completed, percentaje_completion, badge
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MerchantObject {
    user_id: String,
    total_orders: i8,
    orders_completed: i8,
    percentaje_completion: i8, // pioasjoidjasoi
    badge: String, //Badge that represent a merchant verified
}

/*
User PaymentMethodsObject: Struct for Payments.
This object contains, id, payment_method, input1, input2, input3, input4, input5
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentMethodsObject {
    id: i128,
    payment_method: String,
    input1: String,
    input2: String,
    input3: String,
    input4: String,
    input5: String,
}

/*
User FiatMethodsObject: Struct for Fiat list.
This object contains, id, fiat_method, flagcdn
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FiatMethodsObject {
    id: i128,
    fiat_method: String,
    flagcdn: String,
}
//////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////
/// 

/*
Near P2P Struct
*/
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearP2P {
    /// AccountID -> Account details.
    pub accounts: UnorderedMap<AccountId, Account>,
    // Users
    pub users: Vec<UserObject>,
    ///Orders object
    pub offers_sell: Vec<OfferObject>,
    ///Order Sell Id
    pub order_sell_id: i128,
    ///Orders object
    pub offers_buy: Vec<OfferObject>,
    ///Order Buy Id
    pub order_buy_id: i128,
    ///Merchant object
    pub merchant: Vec<MerchantObject>,
    ///Payment Method object
    pub payment_method: Vec<PaymentMethodsObject>,
    // Payment Method Id
    pub payment_method_id: i128,
    ///Payment Method object
    pub fiat_method: Vec<FiatMethodsObject>,
    // Payment Method Id
    pub fiat_method_id: i128,
}

/// Initializing deafult impl
/// We are using default inizialization for the structs
impl Default for NearP2P {
    fn default() -> Self {
        Self {
            accounts: UnorderedMap::new(b"a"),
            users: Vec::new(),
            offers_sell: Vec::new(),
            order_sell_id: 0,
            offers_buy: Vec::new(),
            order_buy_id: 0,
            merchant: Vec::new(),
            payment_method: Vec::new(),
            payment_method_id: 0,
            fiat_method: Vec::new(),
            fiat_method_id: 0,
        }
    }
}

/// Implementing Struct
#[near_bindgen]
impl NearP2P {

    /// Locks an amount from
    /// the `owner_id`.
    /// Requirements:
    /// * The owner should have enough unlocked balance.
    /// Params: owner_id: AccountId
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
    /// params: owner_id: AccountId, escrow_account_id: AccountId
    pub fn get_locked_balance(&self, owner_id: AccountId, escrow_account_id: AccountId) -> String {
        self.get_account(&owner_id).get_locked_balance(&escrow_account_id).to_string()
    }

    /// Returns the users object loaded in contract
    /// Params: user_id: AccountId
    pub fn get_user(self, user_id: String) -> Vec<UserObject> {
        if user_id == "%" {
            self.users     
        } else {
            let mut result: Vec<UserObject> = Vec::new();
            for i in 0..self.users.len() {
                if self.users[i].user_id == user_id.to_string() {
                    result.push(UserObject {
                        user_id: self.users[i].user_id.to_string(),
                        name: self.users[i].name.to_string(),
                        last_name: self.users[i].last_name.to_string(),
                        phone: self.users[i].phone.to_string(),
                        email: self.users[i].email.to_string(),
                        country: self.users[i].country.to_string(),
                    });
                }
            }
            result
        }
    }

    /// Set the users object into the contract
    /// Params: user_id: String, name: String
    /// last_name: String, phone: String, email: String, country: String
    pub fn set_user(&mut self, user_id: String,
        name: String,
        last_name: String,
        phone: String,
        email: String,
        country: String) -> String{
        let data = UserObject {
            user_id: user_id.to_string(),
            name: name.to_string(),
            last_name: last_name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            country: country.to_string(),
        };
        self.users.push(data);
        env::log(b"User Created");
        user_id.to_string()
    }
    
    /// Set the users object into the contract
    /// Params: user_id: String, name: String
    /// name: String, last_name: String, phone: String, email: String, country: String
    pub fn put_user(&mut self, user_id: String
        , name: String
        , last_name: String
        , phone: String
        , email: String
        , country: String) {
        for i in 0..self.users.len() {
            if self.users[i].user_id == user_id {
                self.users[i].name = name.to_string();
                self.users[i].last_name = last_name.to_string();
                self.users[i].phone = phone.to_string();
                self.users[i].email = email.to_string();
                self.users[i].country = country.to_string();
            }
        }
        env::log(b"User Updated");
    }


    /// Returns the order object loaded in contract
    /// Params: campo: String, valor: String
    pub fn get_offers_sell(self, campo: String, valor: String) -> Vec<OfferObject> {
        search_offer(self.offers_sell, campo.to_string(), valor.to_string())
    }

    /// Set the offer sell object into the contract
    /// Params: owner_id: String, asset: String, exchange_rate: String, amount: String
    /// min_limit: String, max_limit: String, payment_method_id: String, status: i8
    /// This is a list of offers for sellings operations, will be called by the user
    pub fn set_offers_sell(&mut self, owner_id: String
        , asset: String
        , exchange_rate: String
        , amount: String
        , min_limit: String
        , max_limit: String
        , payment_method: i128
        , status: i8) -> i128{
        self.order_sell_id += 1;
        let data = OfferObject {
            order_id: self.order_sell_id,
            owner_id: String::from(owner_id),
            asset: String::from(asset),
            exchange_rate: String::from(exchange_rate),
            amount: amount.to_string(),
            remaining_amount: amount.to_string(),
            min_limit: String::from(min_limit),
            max_limit: String::from(max_limit),
            payment_method: payment_method,
            confirmation: bool::from(false),
            status: status,
        };
        self.offers_sell.push(data);
        env::log(b"Order Created");
        self.order_sell_id
    }


    /// Returns the order object loaded in contract
    /// Params: campo: String, valor: String
    pub fn get_offers_buy(self, campo: String, valor: String) -> Vec<OfferObject> {
        search_offer(self.offers_buy, campo.to_string(), valor.to_string())
    }


    /// Set the offer buy object into the contract
    /// Params: owner_id: String, asset: String, exchange_rate: String, amount: String
    /// min_limit: String, max_limit: String, payment_method_id: String, status: i8
    /// This is a list of offers for buying operations, will be called by the user
    pub fn set_offers_buy(&mut self, owner_id: String
        , asset: String
        , exchange_rate: String
        , amount: String
        , min_limit: String
        , max_limit: String
        , payment_method: i128
        , status: i8) -> i128{
        self.order_buy_id += 1;
        let data = OfferObject {
            order_id: self.order_buy_id,
            owner_id: String::from(owner_id),
            asset: String::from(asset),
            exchange_rate: String::from(exchange_rate),
            amount: amount.to_string(),
            remaining_amount: amount.to_string(),
            min_limit: String::from(min_limit),
            max_limit: String::from(max_limit),
            payment_method: payment_method,
            confirmation: bool::from(false),
            status: status,
        };
        self.offers_buy.push(data);
        env::log(b"Order Created");
        self.order_buy_id
    }


    /// Returns the merchant object loaded in contract
    pub fn get_merchant(self) -> Vec<MerchantObject> {
        self.merchant
    }

    /// Set the merchant object into the contract
    /// Params: user_id: String, total_orders: i128, orders_completed: i128
    /// badges: String
    /// This is a list of merchants, will be called by the user
    /// The Dapp has a list of merchants authorized to avoid scams
    pub fn set_merchant(&mut self, user_id: String
        , total_orders: i8
        , orders_completed: i8 
        , badge: String) {
        let data = MerchantObject {
            user_id: String::from(user_id),
            total_orders: total_orders,
            orders_completed: orders_completed,
            percentaje_completion: (orders_completed / total_orders) * 100,
            badge: badge,
        };
        self.merchant.push(data);
        env::log(b"Merchant Created");
    }

    /// Set the merchant object into the contract
    /// Params: user_id: String, total_orders: i128, orders_completed: i128
    /// badge: String
    pub fn put_merchant(&mut self, user_id: String
        , total_orders: i8
        , orders_completed: i8 
        , badge: String) {
        for i in 0..self.merchant.len() {
            if self.merchant[i].user_id == user_id {
                self.merchant[i].total_orders = total_orders;
                self.merchant[i].orders_completed = orders_completed;
                self.merchant[i].percentaje_completion = (orders_completed / total_orders) * 100;
                self.merchant[i].badge = badge.to_string();
            }
        }
        env::log(b"Merchant Updated");
    }

    /// Delete the merchant object into the contract
    /// Params: user_id: String
    pub fn delete_merchant(&mut self, user_id: String) {
        for i in 0..self.merchant.len() {
            if self.merchant.get(i).unwrap().user_id == user_id.to_string() {
                self.merchant.remove(i);
            }
        }
        env::log(b"Merchant Delete");
    }


    /// Returns the Payment Method object loaded in contract
    pub fn get_payment_method(self) -> Vec<PaymentMethodsObject> {
        self.payment_method
    }

     /// Set the Payment Method object into the contract
     /// Params: payment_method_id: String, input1: String, input2: String
     /// input3: String, input4: String, input5: String
     /// List of payment methods, will be called by the user
     /// It is used to filter wich oayment method is offering for trading
     pub fn set_payment_method(&mut self, payment_method: String
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String) -> i128 {
        self.payment_method_id += 1;
        let data = PaymentMethodsObject {
            id: self.payment_method_id,
            payment_method: payment_method,
            input1: input1,
            input2: input2,
            input3: input3,
            input4: input4,
            input5: input5,
        };
        self.payment_method.push(data);
        env::log(b"Payment Method Created");
        self.payment_method_id
    }

    /// put the Payment Method object into the contract
    /// Params: id: i128, payment_method: String, input1: String, input2: String
    /// input3: String, input4: String, input5: String
    pub fn put_payment_method(&mut self, id: i128
        , payment_method: String
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String) {
        for i in 0..self.payment_method.len() {
            if self.payment_method.get(i).unwrap().id == id {
                self.payment_method[i].payment_method = payment_method.to_string();
                self.payment_method[i].input1 = input1.to_string();
                self.payment_method[i].input2 = input2.to_string();
                self.payment_method[i].input3 = input3.to_string();
                self.payment_method[i].input4 = input4.to_string();
                self.payment_method[i].input5 = input5.to_string();
            }
        }
        env::log(b"Payment Method Update");
    }

    /// delete the Payment Method object into the contract
    /// Params: id: i128
    pub fn delete_payment_method(&mut self, id: i128) {
        for i in 0..self.payment_method.len() {
            if self.payment_method.get(i).unwrap().id == id {
                self.payment_method.remove(i);
            }
        }
        env::log(b"Payment Method Delete");
    }

    /// Returns the Fiat Method object loaded in contract
    pub fn get_fiat_method(self) -> Vec<FiatMethodsObject> {
        self.fiat_method
    }

    /// Set the Fiat Method object into the contract
    /// Params: fiat_method_id: String, flagcdn: String
    /// List of fiat methods, will be called by the user
    pub fn set_fiat_method(&mut self, fiat_method: String, flagcdn: String) -> i128 {
        self.fiat_method_id += 1;
        let data = FiatMethodsObject {
            id: self.fiat_method_id,
            fiat_method: fiat_method,
            flagcdn: flagcdn,
        };
        self.fiat_method.push(data);
        env::log(b"Fiat Method Created");
        self.fiat_method_id
    }

    /// Put the Fiat Method object into the contract
    /// Params: id: i128, fiat_method: String, flagcdn: String
    pub fn put_fiat_method(&mut self, id: i128
        , fiat_method: String, flagcdn: String) {
        for i in 0..self.fiat_method.len() {
            if self.fiat_method.get(i).unwrap().id == id {
                self.fiat_method[i].fiat_method = fiat_method.to_string();
                self.fiat_method[i].flagcdn = flagcdn.to_string();
            }
        }
        env::log(b"Fiat Method Update");
    }

    /// Delete the Fiat Method object into the contract
    /// Params: id: i128
    pub fn delete_fiat_method(&mut self, id: i128) {
        for i in 0..self.fiat_method.len() {
            if self.fiat_method.get(i).unwrap().id == id {
                self.fiat_method.remove(i);
            }
        }
        env::log(b"Fiat Method Delete");
    }
    

}

/*
This method is created to search into de offer object the values you want to filter
Params: data: OfferObject, campo: String, valor: String
*/
fn search_offer(data: Vec<OfferObject>, campo: String, valor: String) -> Vec<OfferObject> {
    fn object_offer(index: usize, data: &Vec<OfferObject>) -> OfferObject {
        return OfferObject {
            order_id: data[index].order_id,
            owner_id: data[index].owner_id.to_string(),
            asset: data[index].asset.to_string(),
            exchange_rate: data[index].exchange_rate.to_string(),
            amount: data[index].amount.to_string(),
            remaining_amount: data[index].remaining_amount.to_string(),
            min_limit: data[index].min_limit.to_string(),
            max_limit: data[index].max_limit.to_string(),
            payment_method: data[index].payment_method,
            confirmation: bool::from(data[index].confirmation),
            status: data[index].status,
        }
    }
    if campo == "%" && valor == "%" {
        return data;
    } else {
        let mut vector: Vec<OfferObject> = Vec::new();
        for i in 0..data.len() {
            if campo == "order_id" && data[i].order_id == valor.parse::<i128>().unwrap().into() {
                vector.push(object_offer(i, &data));
            }
            if campo == "owner_id" && data[i].owner_id == valor.to_string() {
                vector.push(object_offer(i, &data));
            }
            if campo == "asset" && data[i].asset == valor.to_string() {
                vector.push(object_offer(i, &data));
            }
            if campo == "exchange_rate" && data[i].exchange_rate == valor.to_string() {
                vector.push(object_offer(i, &data));
            }
            if campo == "amount" && data[i].amount == valor.to_string() {
                vector.push(object_offer(i, &data));
            }
            if campo == "remaining_amount" && data[i].remaining_amount == valor.to_string() {
                vector.push(object_offer(i, &data));
            }
            if campo == "min_limit" && data[i].min_limit == valor.to_string() {
                vector.push(object_offer(i, &data));
            }
            if campo == "max_limit" && data[i].max_limit == valor.to_string() {
                vector.push(object_offer(i, &data));
            }
            if campo == "payment_method" && data[i].payment_method == valor.parse::<i128>().unwrap().into() {
                vector.push(object_offer(i, &data));
            }
            if campo == "confirmation" && data[i].confirmation == valor.parse::<bool>().unwrap() {
                vector.push(object_offer(i, &data));
            }
            if campo == "status" && data[i].status == valor.parse::<i8>().unwrap() {
                vector.push(object_offer(i, &data));
            }
        }
        return vector
    }
    
}

/// Other implement of Near, used to get the account ID
impl NearP2P {
    /// Helper method to get the account details for `owner_id`.
    fn get_account(&self, owner_id: &AccountId) -> Account {
        self.accounts.get(owner_id).unwrap_or_default()
    }
}



/// use the attribute below for unit tests
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
        //contract.set_offers(account_id, asset, price, amount, min_limit, max_limit, order_type, payment_method, orders_completed, percentaje_completion, badge);
        //assert_eq!(contract.get_offers().len(), 1);
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