use crate::*;
use crate::subcontract::sum_balance_contract_token;


#[near_bindgen]
impl NearP2P {
    pub fn pause_play_offer(&mut self, offer_type: i8, offer_id: i128) {
        let signer_id = env::signer_account_id();
        
        if offer_type == 1 {
            let mut offer = self.offers_sell.get(&offer_id).expect("Offer sell not found");
            
            require!(offer.owner_id == signer_id, "only the offer creator can pause or resume");

            offer.is_pause = !offer.is_pause;

            self.offers_sell.insert(&offer_id, &offer);

            env::log_str(
                &json!({
                    "type": "pause_play_offer",
                    "params": {
                        "offer_type": offer_type.to_string(),
                        "offer_id": offer_id.to_string(),
                        "is_pause": offer.is_pause.clone(),
                    }
                }).to_string(),
            );
        } else if offer_type == 2 {
            let mut offer = self.offers_buy.get(&offer_id).expect("Offer sell not found");
            
            require!(offer.owner_id == signer_id, "only the offer creator can pause or resume");

            offer.is_pause = !offer.is_pause;

            self.offers_buy.insert(&offer_id, &offer);

            env::log_str(
                &json!({
                    "type": "pause_play_offer",
                    "params": {
                        "offer_type": offer_type.to_string(),
                        "offer_id": offer_id.to_string(),
                        "is_pause": offer.is_pause.clone(),
                    }
                }).to_string(),
            );
        } else {
            env::panic_str("invalid offer type");
        }
    }
    
    /// accept offer into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    #[payable]
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
            require!(offer.is_pause == false, "the offer is currently on pause");


            #[warn(unused_assignments)]
            let contract_name = self.contract_list.get(&signer_id).expect("the user does not have a sub contract deployed");
            
            match offer.asset.as_str() {
                "NEAR" => {
                    ext_subcontract::get_balance_near(
                        contract_name.contract.clone(),
                        0,
                        BASE_GAS,
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
                    
                    ext_usdc::ft_balance_of(
                        contract_name.contract.to_string(),
                        contract_ft.contract, //AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                        0,
                        Gas(30_000_000_000_000),
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
                }
            };
        } else if offer_type == 2 {
            require!(attached_deposit >= 1, "you have to deposit a minimum of one YoctoNear");

            let mut offer = self.offers_buy.get(&offer_id).expect("Offer buy not found");
            
            require!(offer.owner_id != env::signer_account_id(), "you can not accept your own offer");
            require!(offer.is_pause == false, "the offer is currently on pause");
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

            let fee: u128 = (amount.0 * FEE_TRANSACTION_NEAR) / 10000;
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
                amount_delivered: amount.0,
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

            let mut data_sub_contract = self.contract_list.get(&offer.owner_id.clone()).expect("the offer have a sub contract deployed");
            
            let balance_avalible: u128 = data_sub_contract.balance_avalible.get(&format!("OFFER|BUY|{}", offer_id).to_string()).unwrap().balance;
            
            data_sub_contract.balance_avalible.insert(format!("OFFER|BUY|{}", offer_id).to_string(), BalanceJson{
                asset: offer.asset.clone(), 
                balance: balance_avalible - (amount.0 + fee),
            });
            
            let balance_block: u128 = data_sub_contract.balance_block.get(&format!("OFFER|BUY|{}", offer_id).to_string()).unwrap().balance;
            
            data_sub_contract.balance_block.insert(format!("OFFER|BUY|{}", offer_id).to_string(), BalanceJson{
                asset: offer.asset.clone(), 
                balance: balance_block + (amount.0 + fee)
            });
    
            self.contract_list.insert(&offer.owner_id.clone(), &data_sub_contract);

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
                        "amount_delivered": amount,
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
            
        }   else {
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
        if result.is_none() {
            env::panic_str("Error Balance".as_ref());
        }

        let balance_of: U128; 
        if offer.asset.clone() == "NEAR" {
            balance_of = U128(near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128"));
        } else {
            balance_of = near_sdk::serde_json::from_slice::<U128>(&result.unwrap()).expect("U128");
        }

        let mut data_sub_contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have a sub contract deployed");
        let balance_block: u128 = sum_balance_contract_token(data_sub_contract.balance_block.clone(), offer.asset.clone()) + sum_balance_contract_token(data_sub_contract.balance_avalible.clone(), offer.asset.clone()); // *data_sub_contract.balance_block.get(&ft_token).or(Some(&0u128)).unwrap();
        let balance_avalible: u128 = balance_of.0 - balance_block;


        assert!(balance_avalible - amount.0 > 0, "el balance en la subcuenta es menor al amount + el fee suministrado");


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
        
        let fee: u128 = (amount.0 * FEE_TRANSACTION_NEAR) / 10000;

        offer.remaining_amount = remaining;

        self.offers_sell.insert(&offer.offer_id, &offer);
        let amount_delivered: U128 = U128(amount.0 - fee);
        
        self.order_sell_id += 1;
        let data = OrderObject {
            offer_id: offer.offer_id,
            order_id: self.order_sell_id,
            owner_id: offer.owner_id.clone(),
            asset: offer.asset.clone(),
            signer_id: env::signer_account_id(),
            exchange_rate: rate.to_string(),
            operation_amount: amount.0,
            amount_delivered: amount_delivered.0,
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
       
        self.orders_sell.insert(&self.order_sell_id, &data);
       
        data_sub_contract.balance_avalible.insert(format!("ORDER|SELL|{}", self.order_sell_id).to_string(), BalanceJson{asset: offer.asset.clone(), balance: 0});
        data_sub_contract.balance_block.insert(format!("ORDER|SELL|{}", self.order_sell_id).to_string(), BalanceJson{asset: offer.asset.clone(), balance: amount.0});

        self.contract_list.insert(&env::signer_account_id(), &data_sub_contract);

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
    }
}