use crate::*;


#[near_bindgen]
impl NearP2P {
    /// Returns the order object loaded in contract
    /// Params: campo: String, valor: String
    /*pub fn get_offers_buy(self, amount: Option<U128>,
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
        if self.offers_buy.len() > 0 {
            search_offer(self.offers_buy, amount, fiat_method, payment_method, is_merchant, owner_id, status, offer_id, asset, signer_id, from_index, limit)
        } else {
            SearchOfferObject {
                total_index: 0,
                data: [].to_vec(),
            }
        }
    }*/


    /// Set the offer buy object into the contract
    /// Params: owner_id: String, asset: String, exchange_rate: String, amount: String
    /// min_limit: String, max_limit: String, payment_method_id: String, status: i8
    /// This is a list of offers for buying operations, will be called by the user
    #[payable]
    pub fn set_offers_buy(&mut self
        , asset: String
        , exchange_rate: String
        , amount: U128
        , min_limit: U128
        , max_limit: U128
        , payment_method: Vec<PaymentMethodsOfferObject>
        , fiat_method: i128
        , time: i64
        , terms_conditions: String
    ) -> Promise {
        require!(env::attached_deposit() >= 100000000000000000000000, "you have to deposit a minimum 0.1 Near");
        
        let merchant = self.merchant.get(&env::signer_account_id()).expect("the user is not in the list of users");
        
        #[warn(unused_assignments)]
        let contract_name = self.contract_list.get(&env::signer_account_id()).expect("the user does not have a sub contract deployed");
        
        require!(contract_name.type_contract != 2, "must have a deployed a merchant contract");

        if asset == "NEAR".to_string() {
            ext_subcontract::block_balance_near(
                amount,
                contract_name.contract.clone(),
                0,
                GAS_FOR_BLOCK,
            ).then(
                int_buy::on_set_offers_buy(merchant.is_merchant
                , env::signer_account_id()
                , asset
                , exchange_rate
                , amount
                , min_limit
                , max_limit
                , payment_method
                , fiat_method
                , time
                , terms_conditions
                , env::current_account_id()
                , 0
                , Gas(15_000_000_000_000)
            ))
        } else {
            let contract_ft = self.ft_token_list.get(&asset).expect("El ft_token subministrado en la oferta es incorrecto");

            ext_subcontract::block_balance_token(
                contract_ft.contract,
                asset.clone(),
                amount,
                contract_name.contract.clone(),
                0,
                GAS_FOR_BLOCK,
            ).then(
                int_buy::on_set_offers_buy(merchant.is_merchant
                , env::signer_account_id()
                , asset
                , exchange_rate
                , amount
                , min_limit
                , max_limit
                , payment_method
                , fiat_method
                , time
                , terms_conditions
                , env::current_account_id()
                , 0
                , Gas(15_000_000_000_000)
            ))
        }
    }

    pub fn on_set_offers_buy(&mut self, merchant: bool
        , owner_id: AccountId
        , asset: String
        , exchange_rate: String
        , amount: U128
        , min_limit: U128
        , max_limit: U128
        , payment_method: Vec<PaymentMethodsOfferObject>
        , fiat_method: i128
        , time: i64
        , terms_conditions: String
    ) -> i128 {
        assert!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error bloquear balance token".as_ref());
        }

        
        
        if near_sdk::serde_json::from_slice::<bool>(&result.unwrap()).expect("bool") { 
            self.offer_buy_id += 1;
            let offer_buy_id = self.offer_buy_id;

            let data = OfferObject {
                offer_id: offer_buy_id,
                owner_id: owner_id.clone(),
                asset: asset.clone(),
                exchange_rate: exchange_rate.clone(),
                amount: amount.0,
                remaining_amount: amount.0,
                min_limit: min_limit.0,
                max_limit: max_limit.0,
                payment_method: payment_method.clone(),
                fiat_method: fiat_method,
                is_merchant: merchant,
                time: time,
                terms_conditions: terms_conditions.clone(),
                status: 1,
            };

            self.offers_buy.insert(&offer_buy_id, &data);

            env::log_str(
                &json!({
                    "type": "set_offers_buy",
                    "params": {
                        "offer_id": offer_buy_id.to_string(),
                        "owner_id": owner_id.clone(),
                        "asset": asset.clone(),
                        "exchange_rate": exchange_rate.clone(),
                        "amount": amount,
                        "remaining_amount": amount,
                        "min_limit": min_limit,
                        "max_limit": max_limit,
                        "payment_method": payment_method.clone(),
                        "fiat_method": fiat_method.to_string(),
                        "is_merchant": merchant,
                        "time": time.to_string(),
                        "terms_conditions": terms_conditions.clone(),
                        "status": "1".to_string(),
                    }
                }).to_string(),
            );

            self.offer_buy_id

        } else {
            env::panic_str("el balance en la subcuenta es menor al amount suministrado")
        }
        
    }

    /*#[warn(dead_code)]
    #[payable]
    pub fn put_offers_buy(&mut self, offer_id: i128
        , asset: Option<String>
        , exchange_rate: Option<String>
        , remaining_amount: Option<U128>
        , min_limit: Option<U128>
        , max_limit: Option<U128>
        , payment_method: Option<Vec<PaymentMethodsOfferObject>>
        , fiat_method: Option<i128>
        , time: Option<i64>
        , terms_conditions: Option<String>
    ) {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1,
            "you have to deposit a minimum of one yoctoNear"
        );

        let offer = self.offers_buy.iter().position(|x| x.offer_id == offer_id && x.owner_id == env::signer_account_id()).expect("Offer not found");
        
        if remaining_amount.is_some() {
            if remaining_amount.unwrap().0 < self.offers_buy[offer].remaining_amount {
                let diff_return = self.offers_buy[offer].remaining_amount - remaining_amount.unwrap().0;

                #[warn(unused_assignments)]
                let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.offers_buy[offer].owner_id.clone()).expect("the user does not have a sub contract deployed").to_string());
                
                let contract_ft: Option<AccountId>;
                let fee_deducted: u128;
                let operation_amount: u128;
                if self.offers_buy[offer].asset == "USDC".to_string() {
                    contract_ft = Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                    fee_deducted = 0;
                    operation_amount = diff_return;
                } else {
                    contract_ft = None;
                    fee_deducted = 0;
                    operation_amount = diff_return;
                }   
                
                ext_subcontract::transfer(
                    self.offers_buy[offer].owner_id.clone(),
                    operation_amount,
                    fee_deducted,
                    contract_ft,
                    contract_name,
                    1,
                    GAS_FOR_TRANSFER,
                ).then(
                    int_buy::on_put_offers_buy(offer_id
                    , offer
                    , asset
                    , exchange_rate
                    , remaining_amount
                    , min_limit
                    , max_limit
                    , payment_method
                    , fiat_method
                    , time
                    , terms_conditions
                    , env::current_account_id()
                    , 0
                    , BASE_GAS
                ));

            } else if remaining_amount.unwrap().0 > self.offers_buy[offer].remaining_amount {
                assert!(
                    remaining_amount.unwrap().0 <= self.offers_buy[offer].amount,
                    "the remaining amount is greater than the original amount of the offer, original amount {}, remaining amount {}.",
                    self.offers_buy[offer].amount, remaining_amount.unwrap().0
                );  
            }
        } else {
            self.offers_buy_internal(offer_id
                , offer
                , asset
                , exchange_rate
                , Some(U128(self.offers_buy[offer].remaining_amount))
                , min_limit
                , max_limit
                , payment_method
                , fiat_method
                , time
                , terms_conditions
            );
        }
    }
    

    #[private]
    fn on_put_offers_buy(&mut self, offer_id: i128
        , offer: usize
        , asset: Option<String>
        , exchange_rate: Option<String>
        , remaining_amount: Option<U128>
        , min_limit: Option<U128>
        , max_limit: Option<U128>
        , payment_method: Option<Vec<PaymentMethodsOfferObject>>
        , fiat_method: Option<i128>
        , time: Option<i64>
        , terms_conditions: Option<String>
    ) {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error al devolver el saldo restantes".as_ref());
        }
        self.offers_buy_internal(offer_id
            , offer
            , asset
            , exchange_rate
            , remaining_amount
            , min_limit
            , max_limit
            , payment_method
            , fiat_method
            , time
            , terms_conditions
        );
    }

    #[private]
    fn offers_buy_internal(&mut self, offer_id: i128
        , offer: usize
        , asset: Option<String>
        , exchange_rate: Option<String>
        , remaining_amount: Option<U128>
        , min_limit: Option<U128>
        , max_limit: Option<U128>
        , payment_method: Option<Vec<PaymentMethodsOfferObject>>
        , fiat_method: Option<i128>
        , time: Option<i64>
        , terms_conditions: Option<String>
    ) -> OfferObject {
        if asset.is_some() {
            self.offers_buy[offer].asset = asset.unwrap();
        }
        if exchange_rate.is_some() {
            self.offers_buy[offer].exchange_rate = exchange_rate.unwrap();
        }
        if remaining_amount.is_some() {
            self.offers_buy[offer].remaining_amount = remaining_amount.unwrap().0;
        }
        if min_limit.is_some() {
            self.offers_buy[offer].min_limit = min_limit.unwrap().0;
        }
        if max_limit.is_some() {
            self.offers_buy[offer].max_limit = max_limit.unwrap().0;
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
    }*/
    
    #[payable]
    pub fn delete_offers_buy(&mut self, offer_id: i128) {
        let offer = self.offers_buy.get(&offer_id).expect("Offer not found");
        
        assert!(offer.owner_id == env::signer_account_id(), "the user is not the creator of this offer");

        #[warn(unused_assignments)]
        let contract_name = self.contract_list.get(&offer.owner_id.clone()).expect("the user does not have a sub contract deployed");
        require!(contract_name.type_contract != 2, "must have a contract as a deployed merchant");

        let contract_ft: Option<AccountId>;
        let ft_token: String;
        
        if offer.asset == "USDC".to_string() {
            contract_ft = Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
            ft_token = "USDC".to_string();
        } else {
            contract_ft = None;
            ft_token = "NEAR".to_string();
        }   
        
        ext_subcontract::transfer(
            offer.owner_id.clone(),
            U128(offer.remaining_amount),
            U128(0u128),
            contract_ft,
            false,
            ft_token,
            contract_name.contract.clone(),
            1,
            GAS_FOR_TRANSFER,
        ).then(int_buy::on_delete_offers_buy(
            offer_id,
            env::current_account_id(),
            0,
            Gas(80_000_000_000_000),
        ));
    }

    #[private]
    pub fn on_delete_offers_buy(&mut self, offer_buy_id: i128) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error al eliminar".as_ref());
        }
        
        self.offers_buy.remove(&offer_buy_id);
        
        env::log_str(
            &json!({
                "type": "delete_offers_buy",
                "params": {
                    "offer_id": offer_buy_id.to_string(),
                }
            }).to_string(),
        );
    }

}