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
use near_sdk::{env, near_bindgen, AccountId, Promise, assert_one_yocto, ext_contract, Gas, promise_result_as_success, require,
                serde_json::json}; // json_types::U128, 
use near_sdk::json_types::U128;
use std::collections::HashMap;
//near_sdk::setup_alloc!();

const KEY_TOKEN: &str = "qbogcyqiqO7Utwqm3VgKhxrmQIc0ROjj";
const FEE_TRANSACTION_NEAR: u128 = 30; // 0.003%

//const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_TRANSFER: Gas = Gas(20_000_000_000_000);
const GAS_FOR_BLOCK: Gas = Gas(30_000_000_000_000);
const GAS_ON_WITHDRAW_NEAR: Gas = Gas(40_000_000_000_000);
const GAS_ON_WITHDRAW_TOKEN_BLOCK: Gas = Gas(60_000_000_000_000);
const GAS_ON_WITHDRAW_TOKEN: Gas = Gas(45_000_000_000_000);
const GAS_ON_CONFIRMATION: Gas = Gas(46_000_000_000_000);
const GAS_ON_ACCEPT_OFFER_SELL: Gas = Gas(3_000_000_000_000);
const BASE_GAS: Gas = Gas(3_000_000_000_000);

//const CONSUMO_STORAGE_NEAR_SUBCONTRACT: u128 = 1412439322253799699999999;
const CONTRACT_USDC: &str = "usdc.fakes.testnet";

//const INITIAL_BALANCE: Balance = 2_50_000_000_000_000_000_000_000; // 1e24yN, 0.25N
//const INITIAL_BALANCE: Balance = 1_080_000_000_000_000_000_000_000; // 1e24yN, 0.25N
const CODE: &[u8] = include_bytes!("./wasm/subcontract_p2_p_v16.wasm");
/////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition///////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////

use crate::external::*;
use crate::internal::*;


mod external;
mod internal;
mod subcontract;
mod sell;
mod buy;
mod offer;
mod process;
mod dispute;
/*
User UserObject: Struct for the user that contains info about the logged user.
This object contains, user_id, name, last_name, phone, email, country
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UserObject {
    user_id: String,
    name: String,
    last_name: String,
    phone: String,
    email: String,
    country: String,
    mediator: bool,
    is_active: bool,
    campo1: String,
    campo2: String,
    campo3: String,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentMethodsOfferObject {
    id: String,
    payment_method: String,
}


/*
User OfferObject: Struct for offer that will be listed.
This object contains, order_id, owner_id, asset, exchange_rate, email, country
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OfferObject {
    offer_id: i128,
    owner_id: AccountId,
    asset: String, // NEAR, USD
    exchange_rate: String,
    amount: u128,
    remaining_amount: u128,
    min_limit: u128,
    max_limit: u128,
    payment_method: Vec<PaymentMethodsOfferObject>, // Info concerning to payment asociated to payment contract
    fiat_method: i128,
    is_merchant: bool,
    time: i64,
    terms_conditions: String,
    status: i8, // 1: active, 2: closed
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct OrderObject {
    offer_id: i128,
    order_id: i128,
    owner_id: AccountId,
    asset: String,
    signer_id: AccountId,
    exchange_rate: String,
    operation_amount: u128,
    amount_delivered: u128,
    fee_deducted: u128,
    payment_method: i128, // Info concerning to payment asociated to payment contract
    fiat_method: i128,
    confirmation_owner_id: i8,
    confirmation_signer_id: i8,
    confirmation_current: i8,
    time: i64,
    datetime: String,
    terms_conditions: String,
    status: i8, // 1 = pending, 2 = completed, 3 = disputed
}

/*
User MerchantObject: Struct for Merchants.
This object contains, user_id, total_orders, orders_completed, percentaje_completion, badge
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MerchantObject {
    user_id: AccountId,
    total_orders: i64,
    orders_completed: i64,
    percentaje_completion: f64, // pioasjoidjasoi
    badge: String, //Badge that represent a merchant verified
    is_merchant: bool,
}


/*
User PaymentMethodsObject: Struct for Payments.
This object contains, id, payment_method, input1, input2, input3, input4, input5
*/
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
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


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentMethodUserObject {
    user_id: AccountId,
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
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FiatMethodsObject {
    id: i128,
    fiat_method: String,
    flagcdn: String,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SearchOfferObject {
    total_index: i128,
    data: Vec<OfferObject>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SearchOrderObject {
    total_index: i128,
    data: Vec<OrderObject>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractList {
    contract: AccountId,
    type_contract: i8,
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////
/// 

/*
Near P2P Struct
*/
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NearP2P {
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
    //Order History sell
    pub order_history_sell: Vec<OrderObject>,
    //Order History buy
    pub order_history_buy: Vec<OrderObject>,
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

    pub vault: AccountId,

    pub administrators: Vec<AccountId>,

    pub contract_list: HashMap<AccountId, ContractList>,

    pub activate_token_list: HashMap<AccountId, Vec<String>>,

    pub disputer: AccountId,
}

/// Initializing deafult impl
/// We are using default inizialization for the structs
impl Default for NearP2P {
    fn default() -> Self {
        Self {
            users: vec![UserObject {
                user_id: "info.testnet".to_string(),
                name: "Andrés".to_string(),
                last_name: "Dominguez".to_string(),
                phone: "0413-4158733".to_string(),
                email: "adominguez@dvconsultores.com".to_string(),
                country: "Venezuela".to_string(),
                mediator: true,
                is_active: true,
                campo1: "".to_string(),
                campo2: "".to_string(),
                campo3: "".to_string(),
            }],
            offers_sell: Vec::new(),
            offer_sell_id: 0,
            offers_buy: Vec::new(),
            offer_buy_id: 0,
            orders_sell: Vec::new(),
            order_sell_id: 0,
            orders_buy: Vec::new(),
            order_buy_id: 0,
            order_history_sell: Vec::new(),
            order_history_buy: Vec::new(),
            merchant: vec![MerchantObject {
                user_id: AccountId::new_unchecked("info.testnet".to_string()),
                total_orders: 1,
                orders_completed: 1,
                percentaje_completion: 0.0,
                badge: "check-circle".to_string(),
                is_merchant: true,
            }],
            payment_method: Vec::new(),
            payment_method_user: Vec::new(),
            payment_method_id: 0,
            fiat_method: Vec::new(),
            fiat_method_id: 0,
            vault: AccountId::new_unchecked("v.nearp2p.testnet".to_string()),
            administrators: vec![
                AccountId::new_unchecked("andromeda2018.testnet".to_string()),
                AccountId::new_unchecked("gperez.testnet".to_string()),
                        ],
            contract_list: HashMap::new(),
            activate_token_list: HashMap::new(),
            disputer: AccountId::new_unchecked("nearp2p.sputnikv2.testnet".to_string()),
        }
    }
}

/// Implementing Struct
#[near_bindgen]
impl NearP2P {
    /*pub fn prueba_balance(&mut self, account_id: String) -> Promise {
        let nft_contract: AccountId = CONTRACT_USDC.parse().unwrap();
        let gas_internal: Gas = Gas(1_000_000_000_000);
        ext_usdc::ft_balance_of(
            account_id,
            nft_contract,
            0,
            BASE_GAS,
        )
        .then(int_sub_contract::on_ft_balance_of(
            env::current_account_id(),
            0,
            gas_internal,
        ))
    }

    #[private]
    pub fn on_ft_balance_of(&mut self) -> String {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("balance is None".as_ref());
        }
        let ret = near_sdk::serde_json::from_slice::<String>(&result.unwrap()).expect("balance is None");
        return ret;
    }*/

    
   
    pub fn set_admin(&mut self, user_id: AccountId) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let valid = self.administrators.iter().find(|&x| x == &user_id);
        if valid.is_some() {
            env::panic_str("the user is already in the list of administrators");
        }
        self.administrators.push(user_id);
    }

    pub fn delete_admin(&mut self, user_id: AccountId) {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let index = self.administrators.iter().position(|x| x == &AccountId::new_unchecked(user_id.to_string())).expect("the user is not in the list of administrators");
        self.administrators.remove(index);
    }

    pub fn update_vault(&mut self, account_id: AccountId) {      
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        self.vault = account_id;
    }

    /// Returns the users object loaded in contract
    /// Params: user_id: AccountId
    pub fn get_user(self, 
        user_id: Option<AccountId>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<UserObject> {
        if self.users.len() > 0 {
            let start_index: u128 = from_index.map(From::from).unwrap_or_default();
            assert!(
                (self.users.len() as u128) > start_index,
                "Out of bounds, please use a smaller from_index."
            );
            let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
            assert_ne!(limit, 0, "Cannot provide limit of 0.");

            if user_id.is_some() {
                let user = user_id.unwrap().clone();
                self.users.iter().filter(|x| x.user_id == user.to_string())
                .skip(start_index as usize)
                .take(limit)
                .map(|x| x.clone()).collect()
            } else {
                self.users.iter()
                .skip(start_index as usize)
                .take(limit)
                .map(|x| x.clone()).collect()
            }
        } else {
            [].to_vec()
        }
    }

    pub fn delete_user_admin(&mut self, user_id: String) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        
        let index = self.users.iter().position(|x| x.user_id == user_id.to_string());

        if index.is_some() {
            self.users.remove(index.unwrap());
        }

        let index2 = self.merchant.iter().position(|x| x.user_id == AccountId::new_unchecked(user_id.clone()));
        
        if index2.is_some() {
            self.merchant.remove(index2.unwrap());
        }
    }

    /// Set the users object into the contract
    /// Params: user_id: String, name: String
    /// last_name: String, phone: String, email: String, country: String
    pub fn set_user(&mut self,
        name: String,
        last_name: String,
        phone: String,
        email: String,
        country: String,
        campo1: String,
        campo2: String,
        campo3: String,
    ) -> String {
        let user = self.users.iter().find(|x| x.user_id == env::signer_account_id().to_string());
        if user.is_some() {
            env::panic_str("profile already exists");
        }
        let data = UserObject {
            user_id: env::signer_account_id().to_string(),
            name: name.to_string(),
            last_name: last_name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            country: country.to_string(),
            mediator: false,
            is_active: true,
            campo1: campo1.to_string(),
            campo2: campo2.to_string(),
            campo3: campo3.to_string(),
        };
        self.users.push(data);
        let data2 = MerchantObject {
            user_id: env::signer_account_id(),
            total_orders: 0,
            orders_completed: 0,
            percentaje_completion: 0.0,
            badge: "".to_string(),
            is_merchant: false,
        };
        self.merchant.push(data2);
       // set_merchant(user_id: user_id.to_string(), total_orders: 0, orders_completed: 0 , badge: "".to_string());
        env::log_str(
            &json!({
                "type": "set_user",
                "params": {
                    "user_id": env::signer_account_id().to_string(),
                    "name": name.to_string(),
                    "last_name": last_name.to_string(),
                    "phone": phone.to_string(),
                    "email": email.to_string(),
                    "country": country.to_string(),
                    "mediator": false,
                    "is_active": true,
                    "badge": "".to_string(),
                    "is_merchant": false,
                    "campo1": campo1.to_string(),
                    "campo2": campo2.to_string(),
                    "campo3": campo3.to_string(),
                }
            }).to_string(),
        );
        env::signer_account_id().to_string()
    }

    pub fn set_user_admin(&mut self,
        user_id: String,
        name: String,
        last_name: String,
        phone: String,
        email: String,
        country: String,
        campo1: String,
        campo2: String,
        campo3: String,
    ) -> String {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let user = self.users.iter().find(|x| x.user_id == user_id.to_string());
        if user.is_some() {
            env::panic_str("profile already exists");
        }
        let data = UserObject {
            user_id: user_id.clone(),
            name: name.to_string(),
            last_name: last_name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            country: country.to_string(),
            mediator: false,
            is_active: true,
            campo1: campo1.to_string(),
            campo2: campo2.to_string(),
            campo3: campo3.to_string(),
        };
        self.users.push(data);
        let data2 = MerchantObject {
            user_id: AccountId::new_unchecked(user_id.clone()),
            total_orders: 0,
            orders_completed: 0,
            percentaje_completion: 0.0,
            badge: "".to_string(),
            is_merchant: false,
        };
        self.merchant.push(data2);
       // set_merchant(user_id: user_id.to_string(), total_orders: 0, orders_completed: 0 , badge: "".to_string());
        env::log_str(
            &json!({
                "type": "set_user_admin",
                "params": {
                    "user_id": user_id.to_string(),
                    "name": name.to_string(),
                    "last_name": last_name.to_string(),
                    "phone": phone.to_string(),
                    "email": email.to_string(),
                    "country": country.to_string(),
                    "mediator": false,
                    "is_active": true,
                    "badge": "".to_string(),
                    "is_merchant": false,
                    "campo1": campo1.to_string(),
                    "campo2": campo2.to_string(),
                    "campo3": campo3.to_string(),
                }
            }).to_string(),
        );
        user_id.to_string()
    }
    
    /// Set the users object into the contract
    /// Params: user_id: String, name: String
    /// name: String, last_name: String, phone: String, email: String, country: String
    pub fn put_user(&mut self, name: String
        , last_name: String
        , phone: String
        , email: String
        , country: String) {
        
        let i = self.users.iter().position(|x| x.user_id == env::signer_account_id().to_string()).expect("user does not exist");
        self.users[i].name = name.to_string();
        self.users[i].last_name = last_name.to_string();
        self.users[i].phone = phone.to_string();
        self.users[i].email = email.to_string();
        self.users[i].country = country.to_string();
        self.users[i].mediator = self.users[i].mediator;
        self.users[i].is_active = self.users[i].is_active;
            
        env::log_str(
            &json!({
                "type": "put_user",
                "params": {
                    "user_id": env::signer_account_id().to_string(),
                    "name": name.to_string(),
                    "last_name": last_name.to_string(),
                    "phone": phone.to_string(),
                    "email": email.to_string(),
                    "country": country.to_string(),
                    "mediator": self.users[i].mediator,
                    "is_active": self.users[i].is_active,
                }
            }).to_string(),
        );
    }

    pub fn put_users(&mut self, user_id: AccountId
        , name: String
        , last_name: String
        , phone: String
        , email: String
        , country: String
        , mediator: bool
        , is_active: bool
    ) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let i = self.users.iter().position(|x| x.user_id == user_id.to_string()).expect("user does not exist");
        self.users[i].name = name.to_string();
        self.users[i].last_name = last_name.to_string();
        self.users[i].phone = phone.to_string();
        self.users[i].email = email.to_string();
        self.users[i].country = country.to_string();
        self.users[i].mediator = mediator;
        self.users[i].is_active = is_active;
                            
        env::log_str(
            &json!({
                "type": "put_users",
                "params": {
                    "user_id": user_id.to_string(),
                    "name": name.to_string(),
                    "last_name": last_name.to_string(),
                    "phone": phone.to_string(),
                    "email": email.to_string(),
                    "country": country.to_string(),
                    "mediator": mediator,
                    "is_active": is_active,
                }
            }).to_string(),
        );
    }

    /// Returns the merchant object loaded in contract
    pub fn get_merchant(self,
        user_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<MerchantObject> {
        if self.merchant.len() > 0 {
            let start_index: u128 = from_index.map(From::from).unwrap_or_default();
            assert!(
                (self.merchant.len() as u128) > start_index,
                "Out of bounds, please use a smaller from_index."
            );
            let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
            assert_ne!(limit, 0, "Cannot provide limit of 0.");

            if user_id.to_string() == "%".to_string() {
                self.merchant.iter()  // Return all merchants
                .skip(start_index as usize)
                .take(limit) 
                .map(|x| x.clone()).collect()
            } else {
                self.merchant.iter().filter(|x| x.user_id == user_id)
                .skip(start_index as usize)
                .take(limit)
                .map(|x| x.clone()).collect()                
            }
        } else {
            [].to_vec()
        }
    }


    /// Set the merchant object into the contract
    /// Params: user_id: String, total_orders: i128, orders_completed: i128
    /// badge: String
    pub fn put_merchant(&mut self, user_id: AccountId
        , total_orders: i64
        , orders_completed: i64 
        , badge: String
        , is_merchant: bool
    ) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
            
        let i = self.merchant.iter().position(|x| x.user_id == user_id).expect("Merchant not found");
        self.merchant[i].total_orders = total_orders;
        self.merchant[i].orders_completed = orders_completed;
        self.merchant[i].percentaje_completion = (orders_completed as f64 / total_orders as f64) * 100.0;
        self.merchant[i].badge = badge.to_string();
        self.merchant[i].is_merchant = is_merchant;

        env::log_str(
            &json!({
                "type": "put_merchant",
                "params": {
                    "user_id": user_id.to_string(),
                    "total_orders": total_orders.to_string(),
                    "orders_completed": orders_completed.to_string(),
                    "percentaje_completion": self.merchant[i].percentaje_completion.to_string(),
                    "badge": badge.to_string(),
                    "is_merchant": is_merchant,
                }
            }).to_string(),
        );
    }


    /// Returns the Payment Method object loaded in contract
    pub fn get_payment_method(&self,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<PaymentMethodsObject> {
        if self.payment_method.len() > 0 {
            let start_index: u128 = from_index.map(From::from).unwrap_or_default();
            assert!(
                (self.payment_method.len() as u128) > start_index,
                "Out of bounds, please use a smaller from_index."
            );
            let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
            assert_ne!(limit, 0, "Cannot provide limit of 0.");

            self.payment_method.iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|x| x.clone()).collect()
        } else {
            [].to_vec()
        }
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
        , input5: String
    ) -> i128 {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        self.payment_method_id += 1;
        let data = PaymentMethodsObject {
            id: self.payment_method_id,
            payment_method: payment_method.clone(),
            input1: input1.clone(),
            input2: input2.clone(),
            input3: input3.clone(),
            input4: input4.clone(),
            input5: input5.clone(),
        };
        env::log_str(
            &json!({
                "type": "set_payment_method",
                "params": {
                    "id": self.payment_method_id.to_string(),
                    "payment_method": payment_method.clone(),
                    "input1": input1.clone(),
                    "input2": input2.clone(),
                    "input3": input3.clone(),
                    "input4": input4.clone(),
                    "input5": input5.clone(),
                }
            }).to_string(),
        );
        self.payment_method.push(data);
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
        , input5: String
    ) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
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
        env::log_str(
            &json!({
                "type": "put_payment_method",
                "params": {
                    "id": id.to_string(),
                    "payment_method": payment_method.to_string(),
                    "input1": input1.to_string(),
                    "input2": input2.to_string(),
                    "input3": input3.to_string(),
                    "input4": input4.to_string(),
                    "input5": input5.to_string(),
                }
            }).to_string(),
        );
        //self.merchant.get(0).unwrap().user_id.clone()
        //self.payment_method
    }

    /// delete the Payment Method object into the contract
    /// Params: id: i128
    pub fn delete_payment_method(&mut self, id: i128) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let mut index = self.payment_method.iter().position(|x| x.id == id).expect("Payment method does not exist");
        self.payment_method.remove(index);
            
        index = self.payment_method_user.iter().position(|x| x.payment_method_id == id).expect("Payment method does not exist");
        self.payment_method_user.remove(index);
            
        env::log_str(
            &json!({
                "type": "delete_payment_method",
                "params": {
                    "id": id.to_string(),
                }
            }).to_string(),
        );
        //self.merchant.get(0).unwrap().user_id.clone()
        //self.payment_method
    }
    
    /// Returns the Fiat Method object loaded in contract
    pub fn get_fiat_method(&self,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<FiatMethodsObject> {
        if self.fiat_method.len() > 0 {
            let start_index: u128 = from_index.map(From::from).unwrap_or_default();
            assert!(
                (self.fiat_method.len() as u128) > start_index,
                "Out of bounds, please use a smaller from_index."
            );
            let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
            assert_ne!(limit, 0, "Cannot provide limit of 0.");

            self.fiat_method.iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|x| x.clone()).collect()
        } else {
            [].to_vec()
        }
    }

    /// Set the Fiat Method object into the contract
    /// Params: fiat_method_id: String, flagcdn: String
    /// List of fiat methods, will be called by the user
    pub fn set_fiat_method(&mut self, fiat_method: String, flagcdn: String) -> i128 {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        self.fiat_method_id += 1;
        let data = FiatMethodsObject {
            id: self.fiat_method_id,
            fiat_method: fiat_method.clone(),
            flagcdn: flagcdn.clone(),
        };
        env::log_str(
            &json!({
                "type": "set_fiat_method",
                "params": {
                    "id": self.fiat_method_id.to_string(),
                    "fiat_method": fiat_method.clone(),
                    "flagcdn": flagcdn.clone(),
                }
            }).to_string(),
        );
        self.fiat_method.push(data);
        self.fiat_method_id
    }

    /// Put the Fiat Method object into the contract
    /// Params: id: i128, fiat_method: String, flagcdn: String
    pub fn put_fiat_method(&mut self, id: i128
        , fiat_method: String, flagcdn: String
    ) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let i = self.fiat_method.iter().position(|x| x.id == id).expect("Fiat method does not exist");
        self.fiat_method[i].fiat_method = fiat_method.to_string();
        self.fiat_method[i].flagcdn = flagcdn.to_string();
        
        env::log_str(
            &json!({
                "type": "put_fiat_method",
                "params": {
                    "id": id.to_string(),
                    "fiat_method": fiat_method.clone(),
                    "flagcdn": flagcdn.clone(),
                }
            }).to_string(),
        );
    }

    /// Delete the Fiat Method object into the contract
    /// Params: id: i128
    pub fn delete_fiat_method(&mut self, id: i128) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let i = self.fiat_method.iter().position(|x| x.id == id).expect("Fiat method does not exist");
        self.fiat_method.remove(i);
        
        env::log_str(
            &json!({
                "type": "delete_fiat_method",
                "params": {
                    "id": id.to_string(),
                }
            }).to_string(),
        );
    }


     /// Returns the users object loaded in contract
     pub fn get_payment_method_user(self, user_id: AccountId, method_id: Option<i128>) -> Vec<PaymentMethodUserObject> {
        if self.payment_method_user.len() > 0 {
            let mut result: Vec<PaymentMethodUserObject> = Vec::new();
            if self.payment_method_user.len() > 0 {
                for i in 0..self.payment_method_user.len() {
                    if method_id.is_some() {
                        if self.payment_method_user.get(i).unwrap().payment_method_id == method_id.unwrap() && self.payment_method_user.get(i).unwrap().user_id == user_id {
                            result.push(PaymentMethodUserObject {
                                user_id: self.payment_method_user[i].user_id.clone(),
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
                    } else {
                        if self.payment_method_user.get(i).unwrap().user_id == user_id {
                            result.push(PaymentMethodUserObject {
                                user_id: self.payment_method_user[i].user_id.clone(),
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
                }
                result
            } else {
                result
            }
        } else {
            [].to_vec()
        }
    }
    
    //Set the Payment Method User object into the contract
    pub fn set_payment_method_user(&mut self, payment_method_id: i128
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String
    ) -> String {
        let duplicado  = self.payment_method_user.iter().find(|x| x.payment_method_id == payment_method_id && x.user_id == env::signer_account_id());
        
        if duplicado.is_none() {
            let index2 = self.payment_method.iter().position(|x| x.id == payment_method_id).expect("Payment method does not exist");
            
            let data = PaymentMethodUserObject {
                user_id: env::signer_account_id(),
                payment_method_id: payment_method_id,
                payment_method: self.payment_method[index2].payment_method.to_string(),
                desc1: self.payment_method[index2].input1.to_string(),
                input1: input1.clone(),
                desc2: self.payment_method[index2].input2.to_string(),
                input2: input2.clone(),
                desc3: self.payment_method[index2].input3.to_string(),
                input3: input3.clone(),
                desc4: self.payment_method[index2].input4.to_string(),
                input4: input4.clone(),
                desc5: self.payment_method[index2].input5.to_string(),
                input5: input5.clone(),
            };

            env::log_str(
                &json!({
                    "type": "set_payment_method_user",
                    "params": {
                        "user_id": env::signer_account_id(),
                        "payment_method_id": payment_method_id.to_string(),
                        "payment_method": self.payment_method[index2].payment_method.to_string(),
                        "desc1": self.payment_method[index2].input1.to_string(),
                        "input1": input1.clone(),
                        "desc2": self.payment_method[index2].input2.to_string(),
                        "input2": input2.clone(),
                        "desc3": self.payment_method[index2].input3.to_string(),
                        "input3": input3.clone(),
                        "desc4": self.payment_method[index2].input4.to_string(),
                        "input4": input4.clone(),
                        "desc5": self.payment_method[index2].input5.to_string(),
                        "input5": input5.clone(),
                    }
                }).to_string(),
            );

            self.payment_method_user.push(data);
            
            payment_method_id.to_string()
        } else {
            env::panic_str("Repeated payment methods are not allowed");
        }
    }

    //Set the Payment Method User object into the contract
    pub fn set_payment_method_user_admin(&mut self, user_id: AccountId
        , payment_method_id: i128
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String
    ) -> String {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        let duplicado  = self.payment_method_user.iter().find(|x| x.payment_method_id == payment_method_id && x.user_id == user_id.clone());
        
        if duplicado.is_none() {
            let index2 = self.payment_method.iter().position(|x| x.id == payment_method_id).expect("Payment method does not exist");
            
            let data = PaymentMethodUserObject {
                user_id: user_id.clone(),
                payment_method_id: payment_method_id,
                payment_method: self.payment_method[index2].payment_method.to_string(),
                desc1: self.payment_method[index2].input1.to_string(),
                input1: input1.clone(),
                desc2: self.payment_method[index2].input2.to_string(),
                input2: input2.clone(),
                desc3: self.payment_method[index2].input3.to_string(),
                input3: input3.clone(),
                desc4: self.payment_method[index2].input4.to_string(),
                input4: input4.clone(),
                desc5: self.payment_method[index2].input5.to_string(),
                input5: input5.clone(),
            };

            env::log_str(
                &json!({
                    "type": "set_payment_method_user_admin",
                    "params": {
                        "user_id": user_id,
                        "payment_method_id": payment_method_id.to_string(),
                        "payment_method": self.payment_method[index2].payment_method.to_string(),
                        "desc1": self.payment_method[index2].input1.to_string(),
                        "input1": input1.clone(),
                        "desc2": self.payment_method[index2].input2.to_string(),
                        "input2": input2.clone(),
                        "desc3": self.payment_method[index2].input3.to_string(),
                        "input3": input3.clone(),
                        "desc4": self.payment_method[index2].input4.to_string(),
                        "input4": input4.clone(),
                        "desc5": self.payment_method[index2].input5.to_string(),
                        "input5": input5.clone(),
                    }
                }).to_string(),
            );

            self.payment_method_user.push(data);
            
            payment_method_id.to_string()
        } else {
            env::panic_str("Repeated payment methods are not allowed");
        }
    }

    /// put the Payment Method object into the contract
    pub fn put_payment_method_user(&mut self, payment_method_id: i128
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String
    ) {
        let i = self.payment_method_user.iter().position(|x| x.payment_method_id == payment_method_id && x.user_id == env::signer_account_id()).expect("payment method user does not exist");
            
        self.payment_method_user[i].input1 = input1.to_string();
        self.payment_method_user[i].input2 = input2.to_string();
        self.payment_method_user[i].input3 = input3.to_string();
        self.payment_method_user[i].input4 = input4.to_string();
        self.payment_method_user[i].input5 = input5.to_string();
            
        env::log_str(
            &json!({
                "type": "put_payment_method_user",
                "params": {
                    "user_id": env::signer_account_id(),
                    "payment_method_id": payment_method_id.to_string(),
                    "input1": input1.to_string(),
                    "input2": input2.to_string(),
                    "input3": input3.to_string(),
                    "input4": input4.to_string(),
                    "input5": input5.to_string(),
                }
            }).to_string(),
        );
    }

    /// delete the Payment Method user object into the contract
    pub fn delete_payment_method_user(&mut self, payment_method_id: i128) {
        let i = self.payment_method_user.iter().position(|x| x.payment_method_id == payment_method_id && x.user_id == env::signer_account_id()).expect("payment method user does not exist");    
        self.payment_method_user.remove(i);
            
        env::log_str(
            &json!({
                "type": "delete_payment_method_user",
                "params": {
                    "user_id": env::signer_account_id(),
                    "payment_method_id": payment_method_id.to_string(),
                }
            }).to_string(),
        );
    }


    pub fn get_order_sell(self,
        order_id: Option<i128>,
        offer_id: Option<i128>,
        owner_id: Option<AccountId>,
        signer_id: Option<AccountId>,
        status: Option<i8>,
        asset: Option<String>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        if self.orders_sell.len() > 0 {
            search_order(self.orders_sell, order_id, offer_id, owner_id, signer_id, status, asset, from_index, limit)
        } else {
            SearchOrderObject {
                total_index: 0,
                data: [].to_vec(),
            }
        }
    }


    pub fn get_order_buy(self,
        order_id: Option<i128>,
        offer_id: Option<i128>,
        owner_id: Option<AccountId>,
        signer_id: Option<AccountId>,
        status: Option<i8>,
        asset: Option<String>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        if self.orders_buy.len() > 0 {
            search_order(self.orders_buy, order_id, offer_id, owner_id, signer_id, status, asset, from_index, limit)
        } else {
            SearchOrderObject {
                total_index: 0,
                data: [].to_vec(),
            }
        }
    }
    
    pub fn get_order_history_sell(self,
        user_id: Option<AccountId>,
        order_id: Option<i128>,
        status: Option<i8>,
        asset: Option<String>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        if self.order_history_sell.len() > 0 {
            search_order_history(self.order_history_sell, user_id, order_id, status, asset, from_index, limit)
        } else {
            SearchOrderObject {
                total_index: 0,
                data: [].to_vec(),
            }
        }
    }

    pub fn get_order_history_buy(self,
        user_id: Option<AccountId>,
        order_id: Option<i128>,
        status: Option<i8>,
        asset: Option<String>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        if self.order_history_buy.len() > 0 {
            search_order_history(self.order_history_buy, user_id, order_id, status, asset, from_index, limit)
        } else {
            SearchOrderObject {
                total_index: 0,
                data: [].to_vec(),
            }
        }
    }

}


fn search_offer(data: Vec<OfferObject>,
    amount: Option<U128>,
    fiat_method: Option<i128>,
    payment_method: Option<i128>,
    is_merchant: Option<bool>,
    owner_id: Option<AccountId>,
    status: Option<i8>,
    offer_id: Option<i128>,
    asset: Option<String>,
    signer_id: Option<AccountId>,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> SearchOfferObject {

    
    let mut result: Vec<OfferObject> = data;

    let start_index: u128 = from_index.map(From::from).unwrap_or_default();
    assert!(
        (result.len() as u128) > start_index,
        "Out of bounds, please use a smaller from_index."
    );
    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    assert_ne!(limit, 0, "Cannot provide limit of 0.");

    if signer_id.is_some() {
        result = result.iter().filter(|x| x.owner_id != AccountId::new_unchecked(signer_id.as_ref().unwrap().to_string()))
                    .map(|r| r.clone()).collect();
    }
    if fiat_method.is_some() {
        result = result.iter().filter(|x| x.fiat_method == fiat_method.unwrap())
                    .map(|r| r.clone()).collect();
    }
    if payment_method.is_some() {
        result = result.iter().filter(|x| x.payment_method.iter().filter(|z| z.id.parse::<i128>().unwrap() == payment_method.unwrap()).count() > 0 )
                    .map(|r| r.clone()).collect();
    }
    if is_merchant.is_some() {
        result = result.iter().filter(|x| x.is_merchant == is_merchant.unwrap())
                    .map(|r| r.clone()).collect();
    }
    if owner_id.is_some() {
        result = result.iter().filter(|x| x.owner_id == AccountId::new_unchecked(owner_id.as_ref().unwrap().to_string()))
                    .map(|r| r.clone()).collect();
    }
    if status.is_some() {
        result = result.iter().filter(|x| x.status == status.unwrap())
                    .map(|r| r.clone()).collect();
    }
    if offer_id.is_some() {
        result = result.iter().filter(|x| x.offer_id == offer_id.unwrap())
                    .map(|r| r.clone()).collect();
    }
    if asset.is_some() {
        result = result.iter().filter(|x| x.asset == asset.as_ref().unwrap().to_string())
                    .map(|r| r.clone()).collect();
    }

    if amount.is_some() {
        result = result.iter().filter(|x| x.amount >= amount.unwrap().0)
                .map(|r| r.clone()).collect();
        /*if asset.is_some() {
            if asset.as_ref().unwrap().to_string() == "NEAR" {
                let monto: u128 = (amount.unwrap().0 * 1000000000000000000000000) as u128;
                result = result.iter().filter(|x| x.amount >= monto)
                        .map(|r| r.clone()).collect();
            } else {
                result = result.iter().filter(|x| x.amount >= amount.unwrap().0)
                        .map(|r| r.clone()).collect();
            }
        }*/
    }

    SearchOfferObject {
        total_index: result.len() as i128,
        data: result.iter()
        .skip(start_index as usize)
        .take(limit)
        .map(|r| r.clone()).collect(),
    }
}


fn search_order(data: Vec<OrderObject>,
    order_id: Option<i128>,
    offer_id: Option<i128>,
    owner_id: Option<AccountId>,
    signer_id: Option<AccountId>,
    status: Option<i8>,
    asset: Option<String>,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> SearchOrderObject {
    let mut result: Vec<OrderObject> = data;

    let start_index: u128 = from_index.map(From::from).unwrap_or_default();
    assert!(
        (result.len() as u128) > start_index,
        "Out of bounds, please use a smaller from_index."
    );
    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    assert_ne!(limit, 0, "Cannot provide limit of 0.");
    
    if order_id.is_some() {
        result = result.iter().filter(|x| x.order_id == order_id.unwrap())
                    .map(|r| r.clone()).collect();
    }
    
    if offer_id.is_some() {
        result = result.iter().filter(|x| x.offer_id == offer_id.unwrap())
                    .map(|r| r.clone()).collect();
    }

    if owner_id.is_some() {
        let user = owner_id.unwrap().clone();
        result = result.iter().filter(|x| x.owner_id == AccountId::new_unchecked(user.to_string()))
                    .map(|r| r.clone()).collect();
    }
    
    if signer_id.is_some() {
        let user = signer_id.unwrap().clone();
        result = result.iter().filter(|x| x.signer_id == AccountId::new_unchecked(user.to_string()))
                    .map(|r| r.clone()).collect();
    }
    
    if status.is_some() {
        result = result.iter().filter(|x| x.status == status.unwrap())
                    .map(|r| r.clone()).collect();
    }

    if asset.is_some() {
        result = result.iter().filter(|x| x.asset == asset.as_ref().unwrap().clone())
                    .map(|r| r.clone()).collect();
    }

    SearchOrderObject {
        total_index: result.len() as i128,
        data: result.iter()
        .skip(start_index as usize)
        .take(limit)
        .map(|r| r.clone()).collect(),
    }
}

fn search_order_history(data: Vec<OrderObject>,
    user_id: Option<AccountId>,
    order_id: Option<i128>,
    status: Option<i8>,
    asset: Option<String>,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> SearchOrderObject {
    let mut result: Vec<OrderObject> = data.iter()
    .map(|s| s.clone()).collect();

    let start_index: u128 = from_index.map(From::from).unwrap_or_default();
    assert!(
        (result.len() as u128) > start_index,
        "Out of bounds, please use a smaller from_index."
    );
    let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
    assert_ne!(limit, 0, "Cannot provide limit of 0.");

    if status.is_some() {
        result = result.iter().filter(|s| s.status == status.unwrap())
                .map(|s| s.clone()).collect();
    }

    if user_id.is_some() {
        let user = user_id.unwrap().clone();
        result = result.iter().filter(|s| s.owner_id == AccountId::new_unchecked(user.to_string()) || s.signer_id == AccountId::new_unchecked(user.to_string()))
                .map(|s| s.clone()).collect();
    }

    if order_id.is_some() {
        result = result.iter().filter(|s| s.order_id == order_id.unwrap())
                .map(|s| s.clone()).collect();
    } 

    if asset.is_some() {
        result = result.iter().filter(|s| s.asset == asset.as_ref().unwrap().clone())
                .map(|s| s.clone()).collect();
    }

    SearchOrderObject {
        total_index: result.len() as i128,
        data: result.iter().rev()
        .skip(start_index as usize)
        .take(limit)
        .map(|s| s.clone()).collect(),
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
        let account_id = "p2p-testnet.testnet".to_string();
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
        let account_id = "p2p-testnet.testnet".to_string();
        //contract.lock(account_id.to_string());
        //print!("Locked balance: {}", contract.get_locked_balance(account_id.to_string(), escrow_account_id));
    }
    
}

