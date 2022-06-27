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
use near_sdk::{env, near_bindgen, AccountId, Promise, assert_one_yocto, ext_contract, Gas, promise_result_as_success, Balance}; // json_types::U128, 
use near_sdk::json_types::U128;
use std::collections::HashMap;

//near_sdk::setup_alloc!();

const YOCTO_NEAR: u128 = 1000000000000000000000000;
const KEY_TOKEN: &str = "qbogcyqiqO7Utwqm3VgKhxrmQIc0ROjj";
const FEE_TRANSACTION: f64 = 0.003;

//const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_TRANSFER: Gas = Gas(40_000_000_000_000);
const BASE_GAS_TOKEN: Gas = Gas(3_000_000_000_000);
const CONTRACT_USDC: &str = "usdc.fakes.testnet";

const INITIAL_BALANCE: Balance = 2_50_000_000_000_000_000_000_000; // 1e24yN, 0.25N
const CODE: &[u8] = include_bytes!("./wasm/subcontract-p2-p.wasm");
/////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition///////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////


#[ext_contract(ext_usdc)]
trait ExtTranferUsdc {
    fn ft_transfer(&mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>
    );

    fn ft_balance_of(self, account_id: String);
}

#[ext_contract(ext_subcontract)]
trait ExtTranferUsdc {
    fn transfer(&mut self,
        ft_token: String,
        receiver_id: AccountId,
        operation_amount: u128,
        fee_deducted: u128,
    );
}

#[ext_contract(ext_internal)]
trait ExtNftDos {
    fn on_ft_balance_of(&mut self);

    fn on_confirmation(&mut self,
        order_id: i128,
        status: i8,
        order_type: i8,
    );
}

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
    is_active: bool,
}


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PaymentMethodsOfferObject {
    id: i128,
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
    amount: f64,
    remaining_amount: f64,
    min_limit: f64,
    max_limit: f64,
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
    signer_id: AccountId,
    exchange_rate: String,
    operation_amount: f64,
    fee_deducted: f64,
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


#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
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

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Objects Definition////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////
/// 

/*
Near P2P Struct
*/
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
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

    pub contract_list: HashMap<AccountId, AccountId>,
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
            vault: AccountId::new_unchecked("vault.p2p-testnet.testnet".to_string()),
            administrators: vec![
                AccountId::new_unchecked("info.testnet".to_string()),
                AccountId::new_unchecked("gperez.testnet".to_string()),
                        ],
            contract_list: HashMap::new(),
        }
    }
}

/// Implementing Struct
#[near_bindgen]
impl NearP2P {
    pub fn prueba_balance(&mut self, account_id: String) -> Promise {
        let nft_contract: AccountId = CONTRACT_USDC.parse().unwrap();
        let gas_internal: Gas = Gas(1_000_000_000_000);
        ext_usdc::ft_balance_of(
            account_id,
            nft_contract,
            0,
            BASE_GAS_TOKEN,
        )
        .then(ext_internal::on_ft_balance_of(
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
    }


    #[payable]
    pub fn create_subcontract(&mut self) -> Promise {
        assert!(
            env::attached_deposit() >= 1,
            "Requires attached deposit of at least 1 yoctoNEAR",
        );
        
        let subaccount_id = AccountId::new_unchecked(
          format!("{}.{}", env::signer_account_id(), env::current_account_id())
        );
        let result = Promise::new(subaccount_id.clone())
            .create_account()
            //.add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec());
        
        self.contract_list.insert(env::signer_account_id(), subaccount_id);

        result
    }

    pub fn get_subcontract(self, user_id: AccountId) -> bool {
        if self.contract_list.get(&user_id).is_some() {
            true
        } else {
            false
        }
    }

    
    pub fn delete_contract(&mut self) {
        let subcontract = self.contract_list.get(&env::signer_account_id());
        if subcontract.is_some() {
            env::panic_str("El usuario no cuenta con un contrato desplegado")
        }
        Promise::new(AccountId::from(env::current_account_id())).delete_account(AccountId::new_unchecked(subcontract.unwrap().to_string()));
    }
   
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
            .map(|x| UserObject {
                user_id: x.user_id.to_string(),
                name: x.name.to_string(),
                last_name: x.last_name.to_string(),
                phone: x.phone.to_string(),
                email: x.email.to_string(),
                country: x.country.to_string(),
                mediator: x.mediator,
                is_active: x.is_active,
            }).collect()
        } else {
            self.users.iter()
            .skip(start_index as usize)
            .take(limit)
            .map(|x| UserObject {
                user_id: x.user_id.to_string(),
                name: x.name.to_string(),
                last_name: x.last_name.to_string(),
                phone: x.phone.to_string(),
                email: x.email.to_string(),
                country: x.country.to_string(),
                mediator: x.mediator,
                is_active: x.is_active,
            }).collect()
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
        country: String) -> String {
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
        env::log_str("User Created");
        env::signer_account_id().to_string().to_string()
    }
    
    /// Set the users object into the contract
    /// Params: user_id: String, name: String
    /// name: String, last_name: String, phone: String, email: String, country: String
    pub fn put_user(&mut self, name: String
        , last_name: String
        , phone: String
        , email: String
        , country: String) {
        for i in 0..self.users.len() {
            if self.users[i].user_id == env::signer_account_id().to_string() {
                self.users[i].name = name.to_string();
                self.users[i].last_name = last_name.to_string();
                self.users[i].phone = phone.to_string();
                self.users[i].email = email.to_string();
                self.users[i].country = country.to_string();
                self.users[i].mediator = self.users[i].mediator;
                self.users[i].is_active = self.users[i].is_active;
            }
        }
        env::log_str("User Updated");
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
        let i = self.users.iter().position(|x| x.user_id == user_id.to_string()).expect("the user is not in the list of users");
        self.users[i].name = name.to_string();
        self.users[i].last_name = last_name.to_string();
        self.users[i].phone = phone.to_string();
        self.users[i].email = email.to_string();
        self.users[i].country = country.to_string();
        self.users[i].mediator = mediator;
        self.users[i].is_active = is_active;
                            
        env::log_str("User Updated");
    }

    /// Returns the order object loaded in contract
    /// Params: campo: String, valor: String
    pub fn get_offers_sell(self, amount: Option<f64>,
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
        search_offer(self.offers_sell, amount, fiat_method, payment_method, is_merchant, owner_id, status, offer_id, asset, signer_id, from_index, limit)
    }


    /// Set the offer sell object into the contract
    /// Params: owner_id: String, asset: String, exchange_rate: String, amount: String
    /// min_limit: String, max_limit: String, payment_method_id: String, status: i8
    /// This is a list of offers for sellings operations, will be called by the user
    #[payable]
    pub fn set_offers_sell(&mut self, owner_id: AccountId
        , asset: String
        , exchange_rate: String
        , amount: f64
        , min_limit: f64
        , max_limit: f64
        , payment_method: Vec<PaymentMethodsOfferObject>
        , fiat_method: i128
        , time: i64
        , terms_conditions: String
    ) -> i128 {
        self.offer_sell_id += 1;
        let index = self.merchant.iter().position(|x| x.user_id == owner_id).expect("the user is not in the list of users");
        
        let data = OfferObject {
            offer_id: self.offer_sell_id,
            owner_id: owner_id,
            asset: String::from(asset),
            exchange_rate: String::from(exchange_rate),
            amount: amount,
            remaining_amount: amount,
            min_limit: min_limit,
            max_limit: max_limit,
            payment_method: payment_method,
            fiat_method: fiat_method,
            is_merchant: self.merchant[index].is_merchant,
            time: time,
            terms_conditions: terms_conditions,
            status: 1,
        };
        self.offers_sell.push(data);
        env::log_str("Offer Created");
        self.offer_sell_id
    }


    #[payable]
    pub fn put_offers_sell(&mut self, offer_id: i128
        , asset: Option<String>
        , exchange_rate: Option<String>
        , remaining_amount: Option<f64>
        , min_limit: Option<f64>
        , max_limit: Option<f64>
        , payment_method: Option<Vec<PaymentMethodsOfferObject>>
        , fiat_method: Option<i128>
        , time: Option<i64>
        , terms_conditions: Option<String>
    ) -> OfferObject {
        let offer = self.offers_sell.iter().position(|x| x.offer_id == offer_id && x.owner_id == env::signer_account_id()).expect("Offer not found");
        if asset.is_some() {
            self.offers_sell[offer].asset = asset.unwrap();
        }
        if exchange_rate.is_some() {
            self.offers_sell[offer].exchange_rate = exchange_rate.unwrap();
        }
        if remaining_amount.is_some() {
            self.offers_sell[offer].remaining_amount = remaining_amount.unwrap();
        }
        if min_limit.is_some() {
            self.offers_sell[offer].min_limit = min_limit.unwrap();
        }
        if max_limit.is_some() {
            self.offers_sell[offer].max_limit = max_limit.unwrap();
        }
        if payment_method.is_some() {
            self.offers_sell[offer].payment_method = payment_method.unwrap().iter().map(|x| PaymentMethodsOfferObject {id: x.id, payment_method: x.payment_method.clone()}).collect();
        }
        if fiat_method.is_some() {
            self.offers_sell[offer].fiat_method = fiat_method.unwrap();
        }
        if time.is_some() {
            self.offers_sell[offer].time = time.unwrap();
        }
        if terms_conditions.is_some() {
            self.offers_sell[offer].terms_conditions = terms_conditions.unwrap();
        }
        
        env::log_str("Offer updated");
        OfferObject {
            offer_id: offer_id,
            owner_id: self.offers_sell[offer].owner_id.clone(),
            asset: String::from(self.offers_sell[offer].asset.clone()),
            exchange_rate: String::from(self.offers_sell[offer].exchange_rate.clone()),
            amount: self.offers_sell[offer].amount,
            remaining_amount: self.offers_sell[offer].remaining_amount,
            min_limit: self.offers_sell[offer].min_limit,
            max_limit: self.offers_sell[offer].max_limit,
            payment_method: self.offers_sell[offer].payment_method.iter().map(|x| PaymentMethodsOfferObject {id: x.id, payment_method: x.payment_method.clone()}).collect(),
            fiat_method: self.offers_sell[offer].fiat_method,
            is_merchant: self.offers_sell[offer].is_merchant,
            time: self.offers_sell[offer].time,
            terms_conditions: String::from(self.offers_sell[offer].terms_conditions.clone()),
            status: self.offers_sell[offer].status,
        }
    }


    pub fn delete_offers_sell(&mut self, offer_id: i128) {
        let offer = self.offers_sell.iter().position(|x| x.offer_id == offer_id && x.owner_id == env::signer_account_id()).expect("Offer not found");
        self.offers_sell.remove(offer);
        env::log_str("Offer Buy Delete");
    }


    /// Returns the order object loaded in contract
    /// Params: campo: String, valor: String
    pub fn get_offers_buy(self, amount: Option<f64>,
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
        search_offer(self.offers_buy, amount, fiat_method, payment_method, is_merchant, owner_id, status, offer_id, asset, signer_id, from_index, limit)
    }


    /// Set the offer buy object into the contract
    /// Params: owner_id: String, asset: String, exchange_rate: String, amount: String
    /// min_limit: String, max_limit: String, payment_method_id: String, status: i8
    /// This is a list of offers for buying operations, will be called by the user
    #[payable]
    pub fn set_offers_buy(&mut self, owner_id: AccountId
        , asset: String
        , exchange_rate: String
        , amount: f64
        , min_limit: f64
        , max_limit: f64
        , payment_method: Vec<PaymentMethodsOfferObject>
        , fiat_method: i128
        , time: i64
        , terms_conditions: String
    ) -> i128{
        let attached_deposit = env::attached_deposit();
        assert!(
            (attached_deposit as f64 / YOCTO_NEAR as f64) as f64 >= amount,
            "the deposit attached is less than the quantity supplied : {}",
            amount
        );
        self.offer_buy_id += 1;
        let index = self.merchant.iter().position(|x| x.user_id == owner_id).expect("the user is not in the list of users");

        let data = OfferObject {
            offer_id: self.offer_buy_id,
            owner_id: owner_id,
            asset: String::from(asset),
            exchange_rate: String::from(exchange_rate),
            amount: amount,
            remaining_amount: amount,
            min_limit: min_limit,
            max_limit: max_limit,
            payment_method: payment_method,
            fiat_method: fiat_method,
            is_merchant: self.merchant[index].is_merchant,
            time: time,
            terms_conditions: terms_conditions,
            status: 1,
        };
        self.offers_buy.push(data);
        env::log_str("Offer Created");
        self.offer_buy_id
    }

    #[payable]
    pub fn put_offers_buy(&mut self, offer_id: i128
        , asset: Option<String>
        , exchange_rate: Option<String>
        , remaining_amount: Option<f64>
        , min_limit: Option<f64>
        , max_limit: Option<f64>
        , payment_method: Option<Vec<PaymentMethodsOfferObject>>
        , fiat_method: Option<i128>
        , time: Option<i64>
        , terms_conditions: Option<String>
    ) -> OfferObject {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1,
            "you have to deposit a minimum of one yoctoNear"
        );
        let offer = self.offers_buy.iter().position(|x| x.offer_id == offer_id && x.owner_id == env::signer_account_id()).expect("Offer not found");
        if asset.is_some() {
            self.offers_buy[offer].asset = asset.unwrap();
        }
        if exchange_rate.is_some() {
            self.offers_buy[offer].exchange_rate = exchange_rate.unwrap();
        }
        if remaining_amount.is_some() {
            if remaining_amount.unwrap() < self.offers_buy[offer].remaining_amount {
                let diff_return = self.offers_buy[offer].remaining_amount - remaining_amount.unwrap();

                #[warn(unused_assignments)]
                let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.offers_buy[offer].owner_id.clone()).expect("the user does not have a sub contract deployed").to_string());
                
                let ft_token: String;
                let fee_deducted: u128;
                let operation_amount: u128;
                if self.offers_buy[offer].asset == "USDC".to_string() {
                    ft_token = "USDC".to_string();
                    fee_deducted = 0;
                    operation_amount = (diff_return as f64) as u128;
                } else {
                    ft_token = "NEAR".to_string();
                    fee_deducted = 0;
                    operation_amount = (diff_return * YOCTO_NEAR as f64) as u128;
                }   
                
                ext_subcontract::transfer(
                    ft_token,
                    self.offers_buy[offer].owner_id.clone(),
                    operation_amount,
                    fee_deducted,
                    contract_name,
                    0,
                    GAS_FOR_TRANSFER,
                );

            } else if remaining_amount.unwrap() > self.offers_buy[offer].remaining_amount {
                assert!(
                    remaining_amount.unwrap() <= self.offers_buy[offer].amount,
                    "the remaining amount is greater than the original amount of the offer, original amount {}, remaining amount {}.",
                    self.offers_buy[offer].amount, remaining_amount.unwrap()
                );
                let diff_pay = self.offers_buy[offer].remaining_amount - remaining_amount.unwrap();
                assert!(
                    (attached_deposit as f64 / YOCTO_NEAR as f64) as f64 >= diff_pay,
                    "the deposit attached is less than the remaining supplied : {}",
                    diff_pay
                );  
            }
            self.offers_buy[offer].remaining_amount = remaining_amount.unwrap();
        }
        if min_limit.is_some() {
            self.offers_buy[offer].min_limit = min_limit.unwrap();
        }
        if max_limit.is_some() {
            self.offers_buy[offer].max_limit = max_limit.unwrap();
        }
        if payment_method.is_some() {
            self.offers_buy[offer].payment_method = payment_method.unwrap().iter().map(|x| PaymentMethodsOfferObject {id: x.id, payment_method: x.payment_method.clone()}).collect();
        }
        if fiat_method.is_some() {
            self.offers_buy[offer].fiat_method = fiat_method.unwrap();
        }
        if time.is_some() {
            self.offers_buy[offer].time = time.unwrap();
        }
        if terms_conditions.is_some() {
            self.offers_buy[offer].terms_conditions = terms_conditions.unwrap();
        }
        
        env::log_str("Offer updated");
        OfferObject {
            offer_id: offer_id,
            owner_id: self.offers_buy[offer].owner_id.clone(),
            asset: String::from(self.offers_buy[offer].asset.clone()),
            exchange_rate: String::from(self.offers_buy[offer].exchange_rate.clone()),
            amount: self.offers_buy[offer].amount,
            remaining_amount: self.offers_buy[offer].remaining_amount,
            min_limit: self.offers_buy[offer].min_limit,
            max_limit: self.offers_buy[offer].max_limit,
            payment_method: self.offers_buy[offer].payment_method.iter().map(|x| PaymentMethodsOfferObject {id: x.id, payment_method: x.payment_method.clone()}).collect(),
            fiat_method: self.offers_buy[offer].fiat_method,
            is_merchant: self.offers_buy[offer].is_merchant,
            time: self.offers_buy[offer].time,
            terms_conditions: String::from(self.offers_buy[offer].terms_conditions.clone()),
            status: self.offers_buy[offer].status,
        }
    }
    

    pub fn delete_offers_buy(&mut self, offer_id: i128) {
        let offer = self.offers_buy.iter().position(|x| x.offer_id == offer_id && x.owner_id == env::signer_account_id()).expect("Offer not found");
        #[warn(unused_assignments)]
        let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.offers_buy[offer].owner_id.clone()).expect("the user does not have a sub contract deployed").to_string());
        
        let ft_token: String;
        let fee_deducted: u128;
        let operation_amount: u128;
        if self.offers_buy[offer].asset == "USDC".to_string() {
            ft_token = "USDC".to_string();
            fee_deducted = 0;
            operation_amount = (self.offers_buy[offer].remaining_amount as f64) as u128;
        } else {
            ft_token = "NEAR".to_string();
            fee_deducted = 0;
            operation_amount = (self.offers_buy[offer].remaining_amount * YOCTO_NEAR as f64) as u128;
        }   
        
        ext_subcontract::transfer(
            ft_token,
            self.offers_buy[offer].owner_id.clone(),
            operation_amount,
            fee_deducted,
            contract_name,
            0,
            GAS_FOR_TRANSFER,
        );
        
        self.offers_buy.remove(offer);
        env::log_str("Offer Buy Delete");
    }


    /// Returns the merchant object loaded in contract
    pub fn get_merchant(self,
        user_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<MerchantObject> {
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

        env::log_str("Merchant Updated");
    }


    /// Returns the Payment Method object loaded in contract
    pub fn get_payment_method(&self,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<PaymentMethodsObject> {
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
            payment_method: payment_method,
            input1: input1,
            input2: input2,
            input3: input3,
            input4: input4,
            input5: input5,
        };
        self.payment_method.push(data);
        env::log_str("Payment Method Created");
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
        env::log_str("Payment Method Update");
        //self.merchant.get(0).unwrap().user_id.clone()
        //self.payment_method
    }

    /// delete the Payment Method object into the contract
    /// Params: id: i128
    pub fn delete_payment_method(&mut self, id: i128) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        for i in 0..self.payment_method.len() {
            if self.payment_method.get(i).unwrap().id == id {
                self.payment_method.remove(i);
                break;
            }
        }
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == id {
                self.payment_method_user.remove(i);
                break;
            }
        }
        env::log_str("Payment Method Delete");
        //self.merchant.get(0).unwrap().user_id.clone()
        //self.payment_method
    }
    
    /// Returns the Fiat Method object loaded in contract
    pub fn get_fiat_method(&self,
        from_index: Option<U128>,
        limit: Option<u64>
    ) -> Vec<FiatMethodsObject> {
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
    }

    /// Set the Fiat Method object into the contract
    /// Params: fiat_method_id: String, flagcdn: String
    /// List of fiat methods, will be called by the user
    pub fn set_fiat_method(&mut self, fiat_method: String, flagcdn: String) -> i128 {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        self.fiat_method_id += 1;
        let data = FiatMethodsObject {
            id: self.fiat_method_id,
            fiat_method: fiat_method,
            flagcdn: flagcdn,
        };
        self.fiat_method.push(data);
        env::log_str("Fiat Method Created");
        self.fiat_method_id
    }

    /// Put the Fiat Method object into the contract
    /// Params: id: i128, fiat_method: String, flagcdn: String
    pub fn put_fiat_method(&mut self, id: i128
        , fiat_method: String, flagcdn: String
    ) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        for i in 0..self.fiat_method.len() {
            if self.fiat_method.get(i).unwrap().id == id {
                self.fiat_method[i].fiat_method = fiat_method.to_string();
                self.fiat_method[i].flagcdn = flagcdn.to_string();
            }
        }
        env::log_str("Fiat Method Update");
    }

    /// Delete the Fiat Method object into the contract
    /// Params: id: i128
    pub fn delete_fiat_method(&mut self, id: i128) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        for i in 0..self.fiat_method.len() {
            if self.fiat_method.get(i).unwrap().id == id {
                self.fiat_method.remove(i);
                break;
            }
        }
        env::log_str("Fiat Method Delete");
    }


     /// Returns the users object loaded in contract
     pub fn get_payment_method_user(self, user_id: AccountId, method_id: Option<i128>) -> Vec<PaymentMethodUserObject> {
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
    }
    
    //Set the Payment Method User object into the contract
    pub fn set_payment_method_user(&mut self, payment_method_id: i128
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String) -> String {
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == payment_method_id && self.payment_method_user.get(i).unwrap().user_id == env::signer_account_id() {
                env::panic_str("Repeated payment methods are not allowed");
            }
        }
        for i in 0..self.payment_method.len() {
            if self.payment_method[i].id == payment_method_id {
                let data = PaymentMethodUserObject {
                    user_id: env::signer_account_id(),
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
                env::log_str("Payment Method User Created");
                return "".to_string();
            }
        }
        env::panic_str("the payment method provided does not exist");
    }

    /// put the Payment Method object into the contract
    pub fn put_payment_method_user(&mut self, payment_method_id: i128
        , input1: String
        , input2: String
        , input3: String
        , input4: String
        , input5: String) {
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == payment_method_id && self.payment_method_user.get(i).unwrap().user_id == env::signer_account_id() {
                self.payment_method_user[i].input1 = input1.to_string();
                self.payment_method_user[i].input2 = input2.to_string();
                self.payment_method_user[i].input3 = input3.to_string();
                self.payment_method_user[i].input4 = input4.to_string();
                self.payment_method_user[i].input5 = input5.to_string();
                break;
            }
        }
        env::log_str("Payment Method User Update");
    }

    /// delete the Payment Method user object into the contract
    pub fn delete_payment_method_user(&mut self, payment_method_id: i128) {
        for i in 0..self.payment_method_user.len() {
            if self.payment_method_user.get(i).unwrap().payment_method_id == payment_method_id && self.payment_method_user.get(i).unwrap().user_id == env::signer_account_id() {
                self.payment_method_user.remove(i);
                break;
            }
        }
        env::log_str("Payment Method User Delete");
    }


    


    /// accept offer into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    #[payable]
    pub fn accept_offer(&mut self, offer_type: i8
        , offer_id: i128
        , amount: f64
        , payment_method: i128
        , datetime: String
        , rate: f64
    ) -> String {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1,
            "you have to deposit a minimum of one yoctoNear"
        );


        if offer_type == 1 {
            for i in 0..self.offers_sell.len() {
                if self.offers_sell.get(i).unwrap().offer_id == offer_id {
                    if self.offers_sell[i].owner_id == env::signer_account_id() {
                        env::panic_str("you can not accept your own offer");
                    }
                    //if (self.offers_sell[i].remaining_amount * YOCTO_NEAR as f64) as f64 >= attached_deposit as f64 {
                    if (self.offers_sell[i].remaining_amount * YOCTO_NEAR as f64) as f64 >= amount as f64 {
                        ////////////////////////////////////////////////////////////////////
                        /* colocar aqui el bloqueo de saldo del owner_id  cuando sea venta */
                        ////////////////////////////////////////////////////////////////////
                        //let remaining: f64 = self.offers_sell[i].remaining_amount  - (attached_deposit as f64 / YOCTO_NEAR as f64) as f64;
                        let remaining: f64 = self.offers_sell[i].remaining_amount  - (amount / YOCTO_NEAR as f64) as f64;
                        if remaining <= 0.0 {
                            self.offers_sell[i].status = 2;
                        }
                        
                        if self.offers_sell[i].max_limit > remaining {
                            self.offers_sell[i].max_limit = remaining;
                        }
                        if self.offers_sell[i].min_limit > remaining {
                            self.offers_sell[i].min_limit = 1.0;
                        }
                        
                        let fee = (amount as f64 / YOCTO_NEAR as f64) as f64 * FEE_TRANSACTION;
                        let fee_deducted = (amount as f64 / YOCTO_NEAR as f64) as f64 - fee;
                        self.offers_sell[i].remaining_amount = remaining;
                        self.order_sell_id += 1;
                        let data = OrderObject {
                            offer_id: offer_id,
                            order_id: self.order_sell_id,
                            owner_id: self.offers_sell[i].owner_id.clone(),
                            signer_id: env::signer_account_id(),
                            exchange_rate: rate.to_string(), // self.offers_sell[i].exchange_rate.to_string(),
                            operation_amount: (amount as f64 / YOCTO_NEAR as f64) as f64,
                            fee_deducted: fee_deducted,
                            payment_method: payment_method,
                            fiat_method: self.offers_sell[i].fiat_method,
                            confirmation_owner_id: 0,
                            confirmation_signer_id: 0,
                            confirmation_current: 0,
                            time: self.offers_sell[i].time,
                            datetime: datetime,
                            terms_conditions: self.offers_sell[i].terms_conditions.to_string(),
                            status: 1,
                        };
                        self.orders_sell.push(data);
                        //actualizar total ordenes owner_id
                        let mut index = self.merchant.iter().position(|x| x.user_id == self.offers_sell[i].owner_id.clone()).expect("owner not merchant");
                        self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
                        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
                        index = self.merchant.iter().position(|x| x.user_id == env::signer_account_id().clone()).expect("owner not merchant");
                        self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
                        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
                         
                            
                        env::log_str("Offer sell accepted");
                        // let msg: String = format!("Offer sell accepted - remaining: {} - Attached: {} - Amount: {}", self.offers_buy[i].remaining_amount, attached_deposit, amount.0);
                        let msg: String = "Offer sell accepted".to_string();
                        return String::from(msg);
                    } else {
                        // let error: String = format!("the quantity is greater than the offer sell amount - Remaining: {} - Attached: {}", self.offers_buy[i].remaining_amount, attached_deposit);
                        // nv::panic(error.as_ref());
                        env::panic_str("the quantity is greater than the offer sell amount");
                    }
                }
            }
            return String::from("Offer sell not found");
        } else if offer_type == 2 {
            for i in 0..self.offers_buy.len() {
                if self.offers_buy.get(i).unwrap().offer_id == offer_id {
                    if self.offers_buy[i].owner_id == env::signer_account_id() {
                        env::panic_str("you can not accept your own offer");
                    }
                    if self.offers_buy[i].remaining_amount >= amount  {
                        ////////////////////////////////////////////////////////////////////////
                        /* colocar aqui el bloqueo de saldo del owner_id  cuando sea compra */
                        ///////////////////////////////////////////////////////////////////////
                        let remaining: f64 = self.offers_buy[i].remaining_amount - amount;
                        if remaining <= 0.0 {
                            self.offers_buy[i].status = 2;
                        }

                        if self.offers_buy[i].max_limit > remaining {
                            self.offers_buy[i].max_limit = remaining;
                        }
                        if self.offers_buy[i].min_limit > remaining {
                            self.offers_buy[i].min_limit = 1.0;
                        }

                        let fee = amount * FEE_TRANSACTION;
                        let fee_deducted = amount - fee;

                        self.offers_buy[i].remaining_amount = remaining;
                        self.order_buy_id += 1;
                        let data = OrderObject {
                            offer_id: offer_id,
                            order_id: self.order_buy_id,
                            owner_id: self.offers_buy[i].owner_id.clone(),
                            signer_id: env::signer_account_id(),
                            exchange_rate: rate.to_string(), //self.offers_buy[i].exchange_rate.to_string(),
                            operation_amount: amount,
                            fee_deducted: fee_deducted,
                            payment_method: payment_method,
                            fiat_method: self.offers_buy[i].fiat_method,
                            confirmation_owner_id: 0,
                            confirmation_signer_id: 0,
                            confirmation_current: 0,
                            time: self.offers_buy[i].time,
                            datetime: datetime,
                            terms_conditions: self.offers_buy[i].terms_conditions.to_string(),
                            status: 1,
                        };
                        self.orders_buy.push(data);
                        //actualizar total ordenes owner_id
                        let mut index = self.merchant.iter().position(|x| x.user_id == self.offers_buy[i].owner_id.clone()).expect("owner not merchant");
                        self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
                        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
                        index = self.merchant.iter().position(|x| x.user_id == env::signer_account_id().clone()).expect("owner not merchant");
                        self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
                        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;

                        env::log_str("Offer buy accepted");
                        // let msg: String = format!("Offer buy accepted - remaining: {} - Amount: {} - Amount: ", self.offers_buy[i].remaining_amount, amount.0);
                        let msg: String = "Offer buy accepted".to_string();
                        return String::from(msg);
                    } else {
                        // let error: String = format!("the quantity is greater than the offer buy amount - Remaining: {} - Amount: {}", self.offers_buy[i].remaining_amount, amount.0);
                        // env::panic_str(error.as_ref());
                        env::panic_str("the quantity is greater than the offer buy amount");
                    }
                }
            }
            env::panic_str("Offer buy not found");
        }   else {
            env::panic_str("Invalid offer type");
        }
    }


    pub fn get_order_sell(self,
        order_id: Option<i128>,
        offer_id: Option<i128>,
        owner_id: Option<AccountId>,
        signer_id: Option<AccountId>,
        status: Option<i8>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        search_order(self.orders_sell, order_id, offer_id, owner_id, signer_id, status, from_index, limit)
    }


    pub fn get_order_buy(self,
        order_id: Option<i128>,
        offer_id: Option<i128>,
        owner_id: Option<AccountId>,
        signer_id: Option<AccountId>,
        status: Option<i8>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        search_order(self.orders_buy, order_id, offer_id, owner_id, signer_id, status, from_index, limit)
    }
    
    pub fn get_order_history_sell(self,
        user_id: Option<AccountId>,
        order_id: Option<i128>,
        status: Option<i8>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        search_order_history(self.order_history_sell, user_id, order_id, status, from_index, limit)
    }

    pub fn get_order_history_buy(self,
        user_id: Option<AccountId>,
        order_id: Option<i128>,
        status: Option<i8>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> SearchOrderObject {
        search_order_history(self.order_history_buy, user_id, order_id, status, from_index, limit)
    }
    
    

    /// confirmation order into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    #[payable]
    pub fn order_confirmation(&mut self, offer_type: i8, order_id: i128) {
        assert_one_yocto();
        let ft_token: String;
        let fee_deducted: u128;
        let operation_amount: u128;
        if offer_type == 1 {
            let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
            if self.orders_sell[i].owner_id == env::signer_account_id() {
                self.orders_sell[i].confirmation_owner_id = 1;
                if self.orders_sell[i].status == 1 {
                    self.orders_sell[i].status = 2;
                }
                env::log_str("Order sell Confirmation");
            } else if self.orders_sell[i].signer_id == env::signer_account_id() {
                self.orders_sell[i].confirmation_signer_id = 1;
                if self.orders_sell[i].status == 1 {
                    self.orders_sell[i].status = 2;
                }

                let mut index = self.merchant.iter().position(|x| x.user_id == self.orders_sell[i].owner_id.clone()).expect("owner not merchant");
                self.merchant[index].orders_completed = self.merchant[index].orders_completed + 1;
                self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
                index = self.merchant.iter().position(|x| x.user_id == self.orders_sell[i].signer_id.clone()).expect("owner not merchant");
                self.merchant[index].orders_completed = self.merchant[index].orders_completed + 1;
                self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;

                let index_offer = self.offers_sell.iter().position(|x| x.offer_id == self.orders_sell[i].offer_id).expect("Offer sell not found");

                #[warn(unused_assignments)]
                let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.orders_sell[i].signer_id).expect("the user does not have a sub contract deployed").to_string());
                
                if self.offers_sell[index_offer].asset == "USDC".to_string() {
                    ft_token = "USDC".to_string();
                    fee_deducted = 0;
                    operation_amount = self.orders_sell[i].operation_amount as u128;
                } else {
                    ft_token = "NEAR".to_string();
                    fee_deducted = ((self.orders_sell[i].operation_amount * FEE_TRANSACTION) * YOCTO_NEAR as f64) as u128;
                    operation_amount = (self.orders_sell[i].operation_amount * YOCTO_NEAR as f64) as u128;
                }   
                
                ext_subcontract::transfer(
                    ft_token,
                    self.orders_sell[i].owner_id.clone(),
                    operation_amount,
                    fee_deducted,
                    contract_name,
                    0,
                    GAS_FOR_TRANSFER,
                ).then(ext_internal::on_confirmation(
                    self.orders_sell[i].order_id,
                    2,
                    1,
                    env::current_account_id(),
                    0,
                    GAS_FOR_TRANSFER,
                ));
            } else {
                env::panic_str("Server internar error, signer not found");
            }
        } else if offer_type == 2 {
            let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");
            if self.orders_buy[i].signer_id == env::signer_account_id() {
                self.orders_buy[i].confirmation_signer_id = 1;
                if self.orders_buy[i].status == 1 {
                    self.orders_buy[i].status = 2;
                }
                env::log_str("Order buy Confirmation");
            } else if self.orders_buy[i].owner_id == env::signer_account_id() {
                self.orders_buy[i].confirmation_owner_id = 1;
                if self.orders_buy[i].status == 1 {
                    self.orders_buy[i].status = 2;
                }

                let mut index = self.merchant.iter().position(|x| x.user_id == self.orders_buy[i].owner_id.clone()).expect("owner not merchant");
                self.merchant[index].orders_completed = self.merchant[index].orders_completed + 1;
                self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
                index = self.merchant.iter().position(|x| x.user_id == self.orders_buy[i].signer_id.clone()).expect("owner not merchant");
                self.merchant[index].orders_completed = self.merchant[index].orders_completed + 1;
                self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
                
                //let fee_deducted = ((self.orders_buy[i].operation_amount * FEE_TRANSACTION) * YOCTO_NEAR as f64) as u128;
                //let operation_amount = (self.orders_buy[i].operation_amount * YOCTO_NEAR as f64) as u128;

                let index_offer = self.offers_buy.iter().position(|x| x.offer_id == self.orders_buy[i].offer_id).expect("Offer buy not found");

                #[warn(unused_assignments)]
                let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.orders_buy[i].owner_id).expect("the user does not have a sub contract deployed").to_string());
                
                if self.offers_buy[index_offer].asset == "USDC".to_string() {
                    ft_token = "USDC".to_string();
                    fee_deducted = 0;
                    operation_amount = self.orders_buy[i].operation_amount as u128;
                } else {
                    ft_token = "NEAR".to_string();
                    fee_deducted = ((self.orders_buy[i].operation_amount * FEE_TRANSACTION) * YOCTO_NEAR as f64) as u128;
                    operation_amount = (self.orders_buy[i].operation_amount * YOCTO_NEAR as f64) as u128;
                }   
                
                ext_subcontract::transfer(
                    ft_token,
                    self.orders_buy[i].signer_id.clone(),
                    operation_amount,
                    fee_deducted,
                    contract_name,
                    0,
                    GAS_FOR_TRANSFER,
                ).then(ext_internal::on_confirmation(
                    self.orders_buy[i].order_id,
                    2,
                    2,
                    env::current_account_id(),
                    0,
                    GAS_FOR_TRANSFER,
                ));
            } else {
                env::panic_str("Server internar error, signer not found");
            }
        }  else {
            env::panic_str("Invalid offer type");
        }
    }


    /// dispute order into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    #[payable]
    pub fn order_dispute(&mut self, offer_type: i8, order_id: i128) {
        assert_one_yocto();
        if offer_type == 1 {
            let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
            if self.orders_sell[i].status != 3 {
                if self.orders_sell[i].owner_id == env::signer_account_id() {
                    self.orders_sell[i].status = 3;
                    self.orders_sell[i].confirmation_owner_id = 2;
                    env::log_str("Order sell in dispute");
                } else if self.orders_sell[i].signer_id == env::signer_account_id() {
                    self.orders_sell[i].status = 3;
                    self.orders_sell[i].confirmation_signer_id = 2;
                    env::log_str("Order sell in dispute");
                } else {
                    env::panic_str("Server internar error, signer not found");  
                }
            } else {
                env::panic_str("The sales order is already in dispute");
            }
        } else if offer_type == 2 {
            let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");
            if self.orders_buy[i].status != 3 {
                if self.orders_buy[i].owner_id == env::signer_account_id() {
                    self.orders_buy[i].status = 3;
                    self.orders_buy[i].confirmation_owner_id = 2;
                    env::log_str("Order buy in dispute");
                } else if self.orders_buy[i].signer_id == env::signer_account_id() {
                    self.orders_buy[i].status = 3;
                    self.orders_buy[i].confirmation_signer_id = 2;
                    env::log_str("Order buy in dispute");
                } else {
                    env::panic_str("Server internar error, signer not found");  
                }
            } else {
                env::panic_str("The sales order is already in dispute");
            }
        }  else {
            env::panic_str("Invalid offer type");
        }
    }


    pub fn dispute(&mut self, offer_type: i8, order_id: i128, token: String) {
        if KEY_TOKEN == token {
            if offer_type == 1 {
                let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
                if self.orders_sell[i].status != 3 {
                    self.orders_sell[i].status = 3;
                    self.orders_sell[i].confirmation_owner_id = 2;
                    self.orders_sell[i].confirmation_signer_id = 2;
                    env::log_str("Order sell in dispute");
                } else {
                    env::panic_str("The sales order is already in dispute");
                }
            } else if offer_type == 2 {
                let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");
                if self.orders_buy[i].status != 3 {
                    self.orders_buy[i].status = 3;
                    self.orders_buy[i].confirmation_owner_id = 2;
                    self.orders_buy[i].confirmation_signer_id = 2;
                    env::log_str("Order buy in dispute");
                } else {
                    env::panic_str("The sales order is already in dispute");
                }
            }  else {
                env::panic_str("Invalid offer type");
            }
        } else {
            env::panic_str("Invalid Key_token");
        }
    }


    #[payable]
    pub fn cancel_order(&mut self, offer_type: i8, order_id: i128) {
        assert_one_yocto();
        let ft_token: String;
        let fee_deducted: u128;
        let operation_amount: u128;
        if offer_type == 1 {
            let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
            
            if self.orders_sell[i].owner_id == env::signer_account_id() {
                let j = self.offers_sell.iter().position(|x| x.offer_id == self.orders_sell[i].offer_id).expect("Offer Sell not found");
                self.orders_sell[i].confirmation_owner_id = 3;
                if self.orders_sell[i].status == 1 || self.orders_sell[i].status == 2 {
                    self.orders_sell[i].status = 4;
                }

                #[warn(unused_assignments)]
                let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.orders_sell[i].signer_id).expect("the user does not have a sub contract deployed").to_string());
                
                if self.offers_sell[j].asset == "USDC".to_string() {
                    ft_token = "USDC".to_string();
                    fee_deducted = 0;
                    operation_amount = self.orders_sell[i].operation_amount as u128;
                } else {
                    ft_token = "NEAR".to_string();
                    fee_deducted = 0;
                    operation_amount = (self.orders_sell[i].operation_amount * YOCTO_NEAR as f64) as u128;
                }   
                
                ext_subcontract::transfer(
                    ft_token,
                    self.orders_sell[i].signer_id.clone(),
                    operation_amount,
                    fee_deducted,
                    contract_name,
                    0,
                    GAS_FOR_TRANSFER,
                ).then(ext_internal::on_confirmation(
                    self.orders_sell[i].order_id,
                    4,
                    1,
                    env::current_account_id(),
                    0,
                    GAS_FOR_TRANSFER,
                ));
                
            } else if self.orders_sell[i].signer_id == env::signer_account_id() {
                self.orders_sell[i].confirmation_signer_id = 3;
                if self.orders_sell[i].status == 1 || self.orders_sell[i].status == 2 {
                    self.orders_sell[i].status = 4;
                }
                env::log_str("cancellation request sent");
            } else {
                env::panic_str("Server internar error, signer not found");  
            }
        } else if offer_type == 2 {
            let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");

            if self.orders_buy[i].owner_id == env::signer_account_id() {
                self.orders_buy[i].confirmation_owner_id = 3;
                if self.orders_buy[i].status == 1 || self.orders_buy[i].status == 2 {
                    self.orders_buy[i].status = 4;
                }
                env::log_str("cancellation request sent");
            } else if self.orders_buy[i].signer_id == env::signer_account_id() {
                let j = self.offers_buy.iter().position(|x| x.offer_id == self.orders_buy[i].offer_id).expect("Offer buy not found");
                self.orders_buy[i].confirmation_signer_id = 3;
                if self.orders_buy[i].status == 1 || self.orders_buy[i].status == 2 {
                    self.orders_buy[i].status = 4;
                }

                #[warn(unused_assignments)]
                let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.orders_buy[i].owner_id).expect("the user does not have a sub contract deployed").to_string());
                
                if self.offers_buy[j].asset == "USDC".to_string() {
                    ft_token = "USDC".to_string();
                    fee_deducted = 0;
                    operation_amount = self.orders_buy[i].operation_amount as u128;
                } else {
                    ft_token = "NEAR".to_string();
                    fee_deducted = 0;
                    operation_amount = (self.orders_buy[i].operation_amount * YOCTO_NEAR as f64) as u128;
                }   
                
                ext_subcontract::transfer(
                    ft_token,
                    self.orders_buy[i].owner_id.clone(),
                    operation_amount,
                    fee_deducted,
                    contract_name,
                    0,
                    GAS_FOR_TRANSFER,
                ).then(ext_internal::on_confirmation(
                    self.orders_buy[i].order_id,
                    4,
                    2,
                    env::current_account_id(),
                    0,
                    GAS_FOR_TRANSFER,
                ));

            } else {
                env::panic_str("Server internar error, signer not found");  
            }
        }  else {
            env::panic_str("Invalid offer type");
        }
    }


    #[private]
    pub fn on_confirmation(&mut self, order_id: i128, status: i8, order_type: i8) {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("balance is None".as_ref());
        }

        let arreglo;
        if order_type == 1 {
            arreglo = self.orders_sell.clone();
        } else if order_type == 2 {
            arreglo = self.orders_buy.clone();
        } else {
            env::panic_str("order type incorret");
        }

        let index = arreglo.iter().position(|x| x.order_id == order_id).expect("Order not found");

        let data = OrderObject {
            offer_id: arreglo[index].offer_id,
            order_id: arreglo[index].order_id,
            owner_id: arreglo[index].owner_id.clone(),
            signer_id: arreglo[index].signer_id.clone(),
            exchange_rate: arreglo[index].exchange_rate.to_string(),
            operation_amount: arreglo[index].operation_amount,
            fee_deducted: arreglo[index].fee_deducted,
            payment_method: arreglo[index].payment_method,
            fiat_method: arreglo[index].fiat_method,
            confirmation_owner_id: arreglo[index].confirmation_owner_id,
            confirmation_signer_id: arreglo[index].confirmation_signer_id,
            confirmation_current: arreglo[index].confirmation_current,
            time: arreglo[index].time,
            datetime: arreglo[index].datetime.to_string(),
            terms_conditions: arreglo[index].terms_conditions.to_string(),
            status: status,
        };


        if order_type == 1 {
            self.order_history_sell.push(data);
            if status == 4 {
                let j = self.offers_sell.iter().position(|x| x.offer_id == arreglo[index].offer_id).expect("Offer Sell not found");
                self.offers_sell[j].remaining_amount = self.offers_sell[j].remaining_amount + arreglo[index].operation_amount;
                self.offers_sell[j].status = 1;
                env::log_str("Order sell canceled");
            } else {
                env::log_str("Order sell Completed");
            }
            self.orders_sell.remove(index);
        } else if order_type == 2 {
            self.order_history_buy.push(data);
            if status == 4 {
                let j = self.offers_buy.iter().position(|x| x.offer_id == arreglo[index].offer_id).expect("Offer Sell not found");
                self.offers_buy[j].remaining_amount = self.offers_buy[j].remaining_amount + arreglo[index].operation_amount;
                self.offers_buy[j].status = 1;
                env::log_str("Order Buy canceled");
            } else {
                env::log_str("Order Buy Completed");
            }
            self.orders_buy.remove(index);   
        }   
    }

}


fn search_offer(data: Vec<OfferObject>,
    amount: Option<f64>,
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
    if amount.is_some() {
        result = result.iter().filter(|x| x.amount >= amount.unwrap())
                    .map(|r| r.clone()).collect();
    }
    if fiat_method.is_some() {
        result = result.iter().filter(|x| x.fiat_method == fiat_method.unwrap())
                    .map(|r| r.clone()).collect();
    }
    if payment_method.is_some() {
        result = result.iter().filter(|x| x.payment_method.iter().filter(|z| z.id == payment_method.unwrap()).count() > 0 )
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

    if user_id.is_some() {
        let user = user_id.unwrap().clone();
        result = data.iter().filter(|s| s.owner_id == AccountId::new_unchecked(user.to_string()) || s.signer_id == AccountId::new_unchecked(user.to_string()))
                .map(|s| s.clone()).collect();
    }

    if order_id.is_some() {
        result = data.iter().filter(|s| s.order_id == order_id.unwrap())
                .map(|s| s.clone()).collect();
    }

    if status.is_some() {
        result = data.iter().filter(|s| s.status == status.unwrap())
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

