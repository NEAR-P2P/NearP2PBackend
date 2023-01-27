use crate::*;


#[near_bindgen]
impl NearP2P {
    /*pub fn desplegar(&mut self) {
        let signer: AccountId = AccountId::new_unchecked(env::signer_account_id().as_str().split('.').collect::<Vec<&str>>()[0].to_string());
        let subaccount_id: AccountId = AccountId::new_unchecked(
        format!("2{}.{}", signer, env::current_account_id())
        );
        Promise::new(subaccount_id.clone())
            .create_account()
            .transfer(1600000000000000000000000)
            .deploy_contract(CODE.to_vec())
            .then(ext_subcontract::new(
                env::current_account_id(),
                env::current_account_id(),
                AccountId::new_unchecked("vault.nearp2pdex.near".to_string()),
                subaccount_id.clone(),
                0,
                BASE_GAS,
            ));

            ext_usdc::storage_deposit(
                true,
                subaccount_id.clone(),
                AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                100000000000000000000000,
                BASE_GAS,
            );
    }

  

    #[payable]
    pub fn transferir(&mut self, sub_contract: AccountId) -> Promise {
        ext_subcontract::transfer(
            env::signer_account_id(),
            U128(1000000000000000000),                            
            U128(0),
            None,
            true,
            "NEAR".to_string(),
            sub_contract,
            1,
            Gas(3_000_000_000_000),
        )
    }

    #[payable]
    pub fn transferir2(&mut self, sub_contract: AccountId) -> Promise {
        ext_usdc::ft_transfer(
            sub_contract,
            U128(1000000),
            None,
            AccountId::new_unchecked(CONTRACT_USDC.to_string()),
            1,
            BASE_GAS,
        )
    }*/

    
    /// accept offer into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    pub fn accept_offer(&mut self, offer_type: i8
        , offer_id: i128
        , amount: U128
        , payment_method: i128
        , datetime: String
        , rate: f64
    ) {
        let attached_deposit = env::attached_deposit();
        let result_referente = self.referidos.get(&env::signer_account_id());
        let mut referente: Option<AccountId> = None;
        
        if result_referente.is_some() {
            referente = result_referente.expect("error").referente.clone();
        }

        if offer_type == 1 {
            require!(attached_deposit >= 1, "you have to deposit a minimum one YoctoNEAR");

            let offer = self.offers_sell.get(&offer_id).expect("Offer sell not found");
            let signer_id = env::signer_account_id();
            require!(offer.owner_id != signer_id, "you can not accept your own offer");


            #[warn(unused_assignments)]
            let contract_name = self.contract_list.get(&signer_id).expect("the user does not have a sub contract deployed");
            
            match offer.asset.as_str() {
                "NEAR" => {
                    ext_subcontract::block_balance_near(
                        amount,
                        contract_name.contract.clone(),
                        0,
                        GAS_FOR_BLOCK,
                    ).then(
                        int_offer::on_accept_offer_sell(
                            offer
                            , amount
                            , payment_method
                            , datetime
                            , rate
                            , referente.clone()
                            , env::current_account_id()
                            , 0
                            , GAS_ON_ACCEPT_OFFER_SELL
                    ));
                }, 
                _=> {
                    let contract_ft = self.ft_token_list.get(&offer.asset).expect("El ft_token subministrado en la oferta es incorrecto");
                    
                    ext_subcontract::block_balance_token(
                        contract_ft.contract,
                        offer.asset.clone(),
                        amount,
                        contract_name.contract.clone(),
                        0,
                        GAS_FOR_BLOCK,
                    ).then(
                        int_offer::on_accept_offer_sell(
                            offer.clone()
                            , amount
                            , payment_method
                            , datetime
                            , rate
                            , referente.clone()
                            , env::current_account_id()
                            , 0
                            , GAS_ON_ACCEPT_OFFER_SELL
                    ));
                },
                _=> env::panic_str("The requested asset does not exist")
            };
        } else if offer_type == 2 {
            require!(attached_deposit >= 1, "you have to deposit a minimum of one YoctoNear");

            let mut offer = self.offers_buy.get(&offer_id).expect("Offer buy not found");
            
            require!(offer.owner_id != env::signer_account_id(), "you can not accept your own offer");
            require!(offer.remaining_amount >= amount.0, "the quantity is greater than the offer buy amount");
            
                
            let remaining: u128 = offer.remaining_amount - amount.0;
            if remaining <= 0 {
                offer.status = 2;
            }

            if offer.max_limit > remaining {
                offer.max_limit = remaining;
            }
            if offer.min_limit > remaining {
                match offer.asset.as_str() {
                    "NEAR" => offer.min_limit = 1000000000000000000000000,
                    _=> {
                        let contract_ft = self.ft_token_list.get(&offer.asset).expect("El ft_token subministrado en la oferta es incorrecto");
                        offer.min_limit = contract_ft.min_limit; //1000000
                    }
                };
            }

            let fee: u128 = (amount.0 * FEE_TRANSACTION_NEAR) / 100000;
            //let fee_deducted = amount - fee;
            offer.remaining_amount = remaining;

            self.offers_buy.insert(&offer_id, &offer.clone());

            self.order_buy_id += 1;
            let data = OrderObject {
                offer_id: offer_id,
                order_id: self.order_buy_id,
                owner_id: offer.owner_id.clone(),
                asset: offer.asset.clone(),
                signer_id: env::signer_account_id(),
                exchange_rate: rate.to_string(),
                operation_amount: amount.0,
                amount_delivered: amount.0 - fee,
                fee_deducted: fee,
                payment_method: payment_method,
                fiat_method: offer.fiat_method,
                confirmation_owner_id: 0,
                confirmation_signer_id: 0,
                confirmation_current: 0,
                referente: referente.clone(),
                time: offer.time,
                datetime: datetime.clone(),
                terms_conditions: offer.terms_conditions.clone(),
                status: 1,
            };

            self.orders_buy.insert(&self.order_buy_id, &data);

            let amount_delivered: U128 = U128(amount.0 - fee);

            env::log_str(
                &json!({
                    "type": "accept_offer_buy",
                    "params": {
                        "offer_id": offer_id.to_string(),
                        "order_id": self.order_buy_id.to_string(),
                        "owner_id": offer.owner_id.clone(),
                        "asset": offer.asset.clone(),
                        "signer_id": env::signer_account_id(),
                        "exchange_rate": rate.to_string(),
                        "operation_amount": amount,
                        "amount_delivered": amount_delivered,
                        "fee_deducted": U128(fee),
                        "payment_method": payment_method.to_string(),
                        "fiat_method": offer.fiat_method.to_string(),
                        "confirmation_owner_id": "0".to_string(),
                        "confirmation_signer_id": "0".to_string(),
                        "confirmation_current": "0".to_string(),
                        "referente": referente.clone(),
                        "time": offer.time.to_string(),
                        "datetime": datetime.clone(),
                        "terms_conditions": offer.terms_conditions.clone(),
                        "status": "1".to_string(),
                    }
                }).to_string(),
            );
            
            //actualizar total ordenes owner_id
            /*let mut index = self.merchant.iter().position(|x| x.user_id == self.offers_buy[offer].owner_id.clone()).expect("owner not merchant");
            self.merchant[index].total_orders += 1;
            self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
            index = self.merchant.iter().position(|x| x.user_id == env::signer_account_id().clone()).expect("owner not merchant");
            self.merchant[index].total_orders += 1;
            self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;*/

        }   else {
            //require!(attached_deposit >= 1, "you have to deposit a minimum of one YoctoNear");
            env::panic_str("Invalid offer type");
        }
    }


    #[private]
    pub fn on_accept_offer_sell(&mut self, mut offer: OfferObject
        , amount: U128
        , payment_method: i128
        , datetime: String
        , rate: f64
        , referente: Option<AccountId>
    ) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
       
        
        let valid: bool = near_sdk::serde_json::from_slice::<bool>(&result.unwrap()).expect("bool");
        require!(valid, "No balance");

        let remaining: u128 = offer.remaining_amount - amount.0;
        if remaining <= 0 {
            offer.status = 2;
        }
        
        if offer.max_limit > remaining {
            offer.max_limit = remaining;
        }
        if offer.min_limit > remaining {
            match offer.asset.as_str() {
                "NEAR" => offer.min_limit = 1000000000000000000000000,
                _=> offer.min_limit = 1000000,
            };
        }
        
        let fee: u128 = (amount.0 * FEE_TRANSACTION_NEAR) / 100000;

        offer.remaining_amount = remaining;

        self.offers_sell.insert(&offer.offer_id, &offer);
        
        
        self.order_sell_id += 1;
        let data = OrderObject {
            offer_id: offer.offer_id,
            order_id: self.order_sell_id,
            owner_id: offer.owner_id.clone(),
            asset: offer.asset.clone(),
            signer_id: env::signer_account_id(),
            exchange_rate: rate.to_string(),
            operation_amount: amount.0,
            amount_delivered: amount.0 - fee,
            fee_deducted: fee,
            payment_method: payment_method,
            fiat_method: offer.fiat_method,
            confirmation_owner_id: 0,
            confirmation_signer_id: 0,
            confirmation_current: 0,
            referente: referente.clone(),
            time: offer.time,
            datetime: datetime.clone(),
            terms_conditions: offer.terms_conditions.to_string(),
            status: 1,
        };
        
        let amount_delivered: U128 = U128(amount.0 - fee);
       
        self.orders_sell.insert(&self.order_sell_id, &data);
       
        env::log_str(
            &json!({
                "type": "accept_offer_sell",
                "params": {
                    "offer_id": offer.offer_id.to_string(),
                    "order_id": self.order_sell_id.to_string(),
                    "owner_id": offer.owner_id.clone(),
                    "asset": offer.asset.clone(),
                    "signer_id": env::signer_account_id(),
                    "exchange_rate": rate.to_string(),
                    "operation_amount": amount,
                    "amount_delivered": amount_delivered,
                    "fee_deducted": U128(fee),
                    "payment_method": payment_method.to_string(),
                    "fiat_method": offer.fiat_method.to_string(),
                    "confirmation_owner_id": "0".to_string(),
                    "confirmation_signer_id": "0".to_string(),
                    "confirmation_current": "0".to_string(),
                    "referente": referente.clone(),
                    "time": offer.time.to_string(),
                    "datetime": datetime.clone(),
                    "terms_conditions": offer.terms_conditions.clone(),
                    "status": "1".to_string(),
                }
            }).to_string(),
        );

        //actualizar total ordenes owner_id
        /*let mut index = self.merchant.iter().position(|x| x.user_id == self.offers_sell[offer].owner_id.clone()).expect("owner not merchant");
        self.merchant[index].total_orders += 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
        index = self.merchant.iter().position(|x| x.user_id == env::signer_account_id().clone()).expect("owner not merchant");
        self.merchant[index].total_orders += 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;*/
    }
}