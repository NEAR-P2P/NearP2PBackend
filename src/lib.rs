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
use near_sdk::{json_types::U128, env, near_bindgen, AccountId, Balance};
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
    mediator: bool,
    admin: bool,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentMethodsOfferObject {
    id: i128,
    payment_method: String,
}


/*
User OfferObject: Struct for offer that will be listed.
This object contains, order_id, owner_id, asset, exchange_rate, email, country
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferObject {
    offer_id: i128,
    owner_id: String,
    asset: String, // NEAR, USD
    exchange_rate: String,
    amount: u128,
    remaining_amount: u128,
    min_limit: u128,
    max_limit: u128,
    payment_method: PaymentMethodsOfferObject, // Info concerning to payment asociated to payment contract
    fiat_method: i128,
    merchant_ad: bool,
    status: i8,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct OrderObject {
    offer_id: i128,
    order_id: i128,
    owner_id: String,
    signer_id: String,
    exchange_rate: String,
    operation_amount: u128,
    payment_method: i128, // Info concerning to payment asociated to payment contract
    fiat_method: i128,
    confirmation_owner_id: i8,
    confirmation_signer_id: i8,
    confirmation_current: i8,
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
    total_orders: i64,
    orders_completed: i64,
    percentaje_completion: f64, // pioasjoidjasoi
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


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentMethodUserObject {
    user_id: String,
    payment_method_id: i128,
    payment_method: String,
    desc1: String,
    input1: String,
    desc2: String,
    input2: String,
    desc3: String,
    input3: String,
    desc4: String,
    input4: String,
    desc5: String,
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
    ///Offer object
    pub offers_sell: Vec<OfferObject>,
    ///Offer Sell Id
    pub offer_sell_id: i128,
    ///Offer object
    pub offers_buy: Vec<OfferObject>,
    ///Order Buy Id
    pub offer_buy_id: i128,
    //Order object
    pub orders_sell: Vec<OrderObject>,
    //Order object
    pub order_sell_id: i128,
    //Order object
    pub orders_buy: Vec<OrderObject>,
    //Order object
    pub order_buy_id: i128,
    ///Merchant object
    pub merchant: Vec<MerchantObject>,
    ///Payment Method object
    pub payment_method: Vec<PaymentMethodsObject>,
    ///Payment Method object
    pub payment_method_user: Vec<PaymentMethodUserObject>,
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
            users: vec![UserObject {
                user_id: "hrpalencia.testnet".to_string(),
                name: "Hector".to_string(),
                last_name: "Palencia".to_string(),
                phone: "0413-4158733".to_string(),
                email: "hpalenciatestnet@gmail.com".to_string(),
                country: "Venezuela".to_string(),
                mediator: false,
                admin: true,
            }],
            offers_sell: Vec::new(),
            offer_sell_id: 0,
            offers_buy: Vec::new(),
            offer_buy_id: 0,
            orders_sell: Vec::new(),
            order_sell_id: 0,
            orders_buy: Vec::new(),
            order_buy_id: 0,
            merchant: vec![MerchantObject {
                user_id: "hrpalencia.testnet".to_string(),
                total_orders: 0,
                orders_completed: 0,
                percentaje_completion: 0.0,
                badge: "".to_string(),
            }],
            payment_method: Vec::new(),
            payment_method_user: Vec::new(),
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
                        mediator: self.users[i].mediator,
                        admin: self.users[i].admin,
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
            mediator: false,
            admin: false,
        };
        self.users.push(data);
        let data2 = MerchantObject {
            user_id: user_id.to_string(),
            total_orders: 0,
            orders_completed: 0,
            percentaje_completion: 0.0,
            badge: "".to_string()
        };
        self.merchant.push(data2);
       // set_merchant(user_id: user_id.to_string(), total_orders: 0, orders_completed: 0 , badge: "".to_string());
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
        , country: String
        , mediator: bool
        , admin: bool) {
        let mut administrator: bool = false;
        let mut user: String = "".to_string();
        for i in 0..self.users.len() {    
            if self.users[i].user_id == env::signer_account_id().to_string() {
                administrator = self.users[i].admin;
            }
        }
        if administrator {
            user = user_id;
        } else {
            user = env::signer_account_id().to_string()
        }
        for i in 0..self.users.len() {
            if self.users[i].user_id == user {
                self.users[i].name = name.to_string();
                self.users[i].last_name = last_name.to_string();
                self.users[i].phone = phone.to_string();
                self.users[i].email = email.to_string();
                self.users[i].country = country.to_string();
                if admin {
                    self.users[i].mediator = mediator;
                    self.users[i].admin = admin;
                } else {
                    self.users[i].mediator = self.users[i].mediator;
                    self.users[i].admin = self.users[i].admin;
                }
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
        , amount: U128
        , min_limit: U128
        , max_limit: U128
        , payment_method: PaymentMethodsOfferObject
        , fiat_method: i128) -> i128{
        self.offer_sell_id += 1;
        let data = OfferObject {
            offer_id: self.offer_sell_id,
            owner_id: String::from(owner_id),
            asset: String::from(asset),
            exchange_rate: String::from(exchange_rate),
            amount: amount.0,
            remaining_amount: amount.0,
            min_limit: min_limit.0,
            max_limit: max_limit.0,
            payment_method: payment_method,
            fiat_method: fiat_method,
            merchant_ad: false,
            status: 1,
        };
        self.offers_sell.push(data);
        env::log(b"Order Created");
        self.offer_sell_id
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
        , amount: U128
        , min_limit: U128
        , max_limit: U128
        , payment_method: PaymentMethodsOfferObject
        , fiat_method: i128) -> i128{
        self.offer_buy_id += 1;
        let data = OfferObject {
            offer_id: self.offer_buy_id,
            owner_id: String::from(owner_id),
            asset: String::from(asset),
            exchange_rate: String::from(exchange_rate),
            amount: amount.0,
            remaining_amount: amount.0,
            min_limit: min_limit.0,
            max_limit: max_limit.0,
            payment_method: payment_method,
            fiat_method: fiat_method,
            merchant_ad: false,
            status: 1,
        };
        self.offers_buy.push(data);
        env::log(b"Order Created");
        self.offer_buy_id
    }


    /// Returns the merchant object loaded in contract
    pub fn get_merchant(self, user_id: String) -> Vec<MerchantObject> {
        if user_id == "%" {
            self.merchant  // Return all merchants   
        } else {
            let mut result: Vec<MerchantObject> = Vec::new();
            for i in 0..self.merchant.len() {
                if self.merchant[i].user_id == user_id.to_string() {
                    result.push(MerchantObject {
                        user_id: self.merchant[i].user_id.to_string(),
                        total_orders: self.merchant[i].total_orders,
                        orders_completed: self.merchant[i].orders_completed,
                        percentaje_completion: self.merchant[i].percentaje_completion,
                        badge: self.merchant[i].badge.to_string(),
                    });
                }
            }
            result
        }
    }


    /// Set the merchant object into the contract
    /// Params: user_id: String, total_orders: i128, orders_completed: i128
    /// badge: String
    pub fn put_merchant(&mut self, user_id: String
        , total_orders: i64
        , orders_completed: i64 
        , badge: String) {
        for i in 0..self.merchant.len() {
            if self.merchant[i].user_id == user_id {
                self.merchant[i].total_orders = total_orders;
                self.merchant[i].orders_completed = orders_completed;
                self.merchant[i].percentaje_completion = (orders_completed as f64 / total_orders as f64) * 100.0;
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
        //self.payment_method[0].payment_method = String::from("Transferencia Bancaria 2");
        for i in 0..self.payment_method.len() {
            if self.payment_method.get(i).unwrap().id == id {
                self.payment_method[i].payment_method = payment_method.to_string();
                self.payment_method[i].input1 = input1.to_string();
                self.payment_method[i].input2 = input2.to_string();
                self.payment_method[i].input3 = input3.to_string();
                self.payment_method[i].input4 = input4.to_string();
                self.payment_method[i].input5 = input5.to_string();
                break;
            }
        }
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == id {
                self.payment_method_user[i].payment_method = payment_method.to_string();
                self.payment_method_user[i].desc1 = input1.to_string();
                self.payment_method_user[i].desc2 = input2.to_string();
                self.payment_method_user[i].desc3 = input3.to_string();
                self.payment_method_user[i].desc4 = input4.to_string();
                self.payment_method_user[i].desc5 = input5.to_string();
            }
        }
        env::log(b"Payment Method Update");
        //self.merchant.get(0).unwrap().user_id.clone()
        //self.payment_method
    }

    /// delete the Payment Method object into the contract
    /// Params: id: i128
    pub fn delete_payment_method(&mut self, id: i128) {
        for i in 0..self.payment_method.len() {
            if self.payment_method.get(i).unwrap().id == id {
                self.payment_method.remove(i);
            }
        }
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == id {
                self.payment_method_user.remove(i);
            }
        }
        env::log(b"Payment Method Delete");
        //self.merchant.get(0).unwrap().user_id.clone()
        //self.payment_method
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


     /// Returns the users object loaded in contract
     pub fn get_payment_method_user(self, user_id: String) -> Vec<PaymentMethodUserObject> {
        let mut result: Vec<PaymentMethodUserObject> = Vec::new();
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user[i].user_id == user_id.to_string() {
                result.push(PaymentMethodUserObject {
                    user_id: self.payment_method_user[i].user_id.to_string(),
                    payment_method_id: self.payment_method_user[i].payment_method_id,
                    payment_method: self.payment_method_user[i].payment_method.to_string(),
                    desc1: self.payment_method_user[i].desc1.to_string(),
                    input1: self.payment_method_user[i].input1.to_string(),
                    desc2: self.payment_method_user[i].desc2.to_string(),
                    input2: self.payment_method_user[i].input2.to_string(),
                    desc3: self.payment_method_user[i].desc3.to_string(),
                    input3: self.payment_method_user[i].input3.to_string(),
                    desc4: self.payment_method_user[i].desc4.to_string(),
                    input4: self.payment_method_user[i].input4.to_string(),
                    desc5: self.payment_method_user[i].desc5.to_string(),
                    input5: self.payment_method_user[i].input5.to_string(),
                });
            }
        }
        result
    }

    //Set the Payment Method User object into the contract
    pub fn set_payment_method_user(&mut self, user_id: String
        , payment_method_id: i128
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String) -> String{
        let mut duplicate: bool = false;
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == payment_method_id && self.payment_method_user.get(i).unwrap().user_id == user_id.to_string() {
                duplicate = true;
            }
        }
        if duplicate == false {
            for i in 0..self.payment_method.len() {
                if self.payment_method[i].id == payment_method_id {
                    let data = PaymentMethodUserObject {
                        user_id: user_id,
                        payment_method_id: payment_method_id,
                        payment_method: self.payment_method[i].payment_method.to_string(),
                        desc1: self.payment_method[i].input1.to_string(),
                        input1: input1,
                        desc2: self.payment_method[i].input2.to_string(),
                        input2: input2,
                        desc3: self.payment_method[i].input3.to_string(),
                        input3: input3,
                        desc4: self.payment_method[i].input4.to_string(),
                        input4: input4,
                        desc5: self.payment_method[i].input5.to_string(),
                        input5: input5,
                    };
                    self.payment_method_user.push(data);
                    env::log(b"Payment Method User Created");
                    break;
                }
            }
            String::from("Payment Method User Created")
        } else {
            String::from("Repeated payment methods are not allowed")
        }
    }

    /// put the Payment Method object into the contract
    pub fn put_payment_method_user(&mut self, user_id: String
        , payment_method_id: i128
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String) {
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == payment_method_id && self.payment_method_user.get(i).unwrap().user_id == user_id.to_string() {
                self.payment_method_user[i].input1 = input1.to_string();
                self.payment_method_user[i].input2 = input2.to_string();
                self.payment_method_user[i].input3 = input3.to_string();
                self.payment_method_user[i].input4 = input4.to_string();
                self.payment_method_user[i].input5 = input5.to_string();
                break;
            }
        }
        env::log(b"Payment Method User Update");
    }

    /// delete the Payment Method user object into the contract
    pub fn delete_payment_method_user(&mut self, user_id: String
        , payment_method_id: i128) {
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == payment_method_id && self.payment_method_user.get(i).unwrap().user_id == user_id.to_string() {
                self.payment_method_user.remove(i);
            }
        }
        env::log(b"Payment Method User Delete");
    }


    /// accept offer into the contract
    pub fn accept_offer(&mut self, offer_type: i8
        , offer_id: i128
        , amount: U128
        , payment_method: i128) -> String {
        if offer_type == 1 {
            for i in 0..self.offers_sell.len() {
                if self.offers_sell.get(i).unwrap().offer_id == offer_id {
                    if self.offers_sell[i].remaining_amount >= amount.0 {
                        if self.offers_sell[i].min_limit >= amount.0 && self.offers_sell[i].max_limit <= amount.0 {  
                            let remaining: u128 = self.offers_sell[i].remaining_amount - amount.0;
                            if remaining == 0 {
                                self.offers_sell[i].status = 2;
                            }
                            self.offers_sell[i].remaining_amount = remaining;
                            self.order_sell_id += 1;
                            let data = OrderObject {
                                offer_id: offer_id,
                                order_id: self.order_sell_id,
                                owner_id: self.offers_sell[i].owner_id.to_string(),
                                signer_id: env::signer_account_id(),
                                exchange_rate: self.offers_sell[i].exchange_rate.to_string(),
                                operation_amount: amount.0,
                                payment_method: payment_method,
                                fiat_method: self.offers_sell[i].fiat_method,
                                confirmation_owner_id: 0,
                                confirmation_signer_id: 0,
                                confirmation_current: 0,
                                status: 1,
                            };
                            self.orders_sell.push(data);
                            //actualizar total ordenes owner_id
                            for j in 0..self.merchant.len() {
                                if self.merchant.get(j).unwrap().user_id == self.offers_sell[i].owner_id {
                                    self.merchant[j].total_orders = self.merchant[j].total_orders + 1;
                                    self.merchant[j].percentaje_completion = (self.merchant[j].orders_completed as f64 / self.merchant[j].total_orders as f64) * 100.0;
                                }
                            }
                            return String::from("Offer accepted");
                        } else {
                            return String::from("amount of change out of range");
                        }
                    } else {
                        return String::from("the quantity is greater than the offer amount");
                    }
                }
            }
            return String::from("Offer not found");
        } else if offer_type == 2 {
            for i in 0..self.offers_buy.len() {
                if self.offers_buy.get(i).unwrap().offer_id == offer_id {
                    if self.offers_buy[i].remaining_amount >= amount.0 {
                        if self.offers_buy[i].min_limit >= amount.0 && self.offers_buy[i].max_limit <= amount.0 {  
                            let remaining: u128 = self.offers_buy[i].remaining_amount - amount.0;
                            if remaining == 0 {
                                self.offers_buy[i].status = 2;
                            }
                            self.offers_buy[i].remaining_amount = remaining;
                            self.order_buy_id += 1;
                            let data = OrderObject {
                                offer_id: offer_id,
                                order_id: self.order_buy_id,
                                owner_id: self.offers_buy[i].owner_id.to_string(),
                                signer_id: env::signer_account_id(),
                                exchange_rate: self.offers_buy[i].exchange_rate.to_string(),
                                operation_amount: amount.0,
                                payment_method: payment_method,
                                fiat_method: self.offers_buy[i].fiat_method,
                                confirmation_owner_id: 0,
                                confirmation_signer_id: 0,
                                confirmation_current: 0,
                                status: 1,
                            };
                            self.orders_buy.push(data);
                            //actualizar total ordenes owner_id
                            for j in 0..self.merchant.len() {
                                if self.merchant.get(j).unwrap().user_id == self.offers_buy[i].owner_id {
                                    self.merchant[j].total_orders = self.merchant[j].total_orders + 1;
                                    self.merchant[j].percentaje_completion = (self.merchant[j].orders_completed as f64 / self.merchant[j].total_orders as f64) * 100.0;
                                }
                            }
                            return String::from("Offer accepted");
                        } else {
                            return String::from("amount of change out of range");
                        }
                    } else {
                        return String::from("the quantity is greater than the offer amount");
                    }
                }
            }
            return String::from("Offer not found");
        }   else {
            return String::from("Invalid offer type");
        }
    }
    
    /// accept offer into the contract
    pub fn offer_confirmation(&mut self, offer_type: i8, order_id: i128) -> String {
        if offer_type == 1 {
            for i in 0..self.orders_sell.len() {
                if self.orders_sell.get(i).unwrap().order_id == order_id {
                    let mut tranfer: bool = false;
                    if self.orders_sell[i].owner_id == env::signer_account_id().to_string() {
                        self.orders_sell[i].confirmation_owner_id = 1;  
                        if self.orders_sell[i].confirmation_signer_id == 1 {
                            tranfer = true;
                        }
                    } else if self.orders_sell[i].signer_id == env::signer_account_id().to_string() {
                        self.orders_sell[i].confirmation_signer_id = 1;
                        if self.orders_sell[i].confirmation_owner_id == 1{
                            tranfer = true;
                        }
                    } else {
                        return String::from("Server internar error, signer not found or order id not found");    
                    }
                    if tranfer == true {
                        // here transfer function
                        self.orders_sell[i].status = 2;
                        //actualizar transacciones culminadas owner_id
                        for j in 0..self.merchant.len() {
                            if self.merchant.get(j).unwrap().user_id == self.offers_sell[i].owner_id {
                                self.merchant[j].orders_completed = self.merchant[j].orders_completed + 1;
                                self.merchant[j].percentaje_completion = (self.merchant[j].orders_completed as f64 / self.merchant[j].total_orders as f64) * 100.0;
                            }
                        }
                        return String::from("Offer Completed");
                    } else {
                        return String::from("Offer Confirmation");
                    }
                }
            }
            return String::from("Offer not found");
    
        }   else {
            return String::from("Invalid offer type");
        }
    }
}


fn search_offer(data: Vec<OfferObject>, campo: String, valor: String) -> Vec<OfferObject> {
    fn object_offer(index: usize, data: &Vec<OfferObject>) -> OfferObject {
        return OfferObject {
            offer_id: data[index].offer_id,
            owner_id: data[index].owner_id.to_string(),
            asset: data[index].asset.to_string(),
            exchange_rate: data[index].exchange_rate.to_string(),
            amount: data[index].amount,
            remaining_amount: data[index].remaining_amount,
            min_limit: data[index].min_limit,
            max_limit: data[index].max_limit,
            payment_method: PaymentMethodsOfferObject {
                id: data[index].payment_method.id,
                payment_method: data[index].payment_method.payment_method.to_string(),
            },
            fiat_method: data[index].fiat_method,
            merchant_ad: data[index].merchant_ad,
            status: data[index].status,
        }
    }
    if campo == "%" && valor == "%" {
        return data;
    } else {
        let mut vector: Vec<OfferObject> = Vec::new();
        for i in 0..data.len() {
            if campo == "offer_id" && data[i].offer_id == valor.parse::<i128>().unwrap().into() {
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
            if campo == "amount" && data[i].amount == valor.parse::<u128>().unwrap().into() {
                vector.push(object_offer(i, &data));
            }
            if campo == "remaining_amount" && data[i].remaining_amount == valor.parse::<u128>().unwrap().into() {
                vector.push(object_offer(i, &data));
            }
            if campo == "min_limit" && data[i].min_limit == valor.parse::<u128>().unwrap().into() {
                vector.push(object_offer(i, &data));
            }
            if campo == "max_limit" && data[i].max_limit == valor.parse::<u128>().unwrap().into() {
                vector.push(object_offer(i, &data));
            }
            if campo == "status" && data[i].status == valor.parse::<i8>().unwrap() {
                vector.push(object_offer(i, &data));
            }
        }
        return vector
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

