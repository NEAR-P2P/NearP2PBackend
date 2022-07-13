use crate::*;


#[near_bindgen]
impl NearP2P {
    pub fn bloquear(&mut self, contract_name: AccountId) -> Promise {
        //let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&env::signer_account_id()).expect("the user does not have a sub contract deployed").to_string());
        ext_subcontract::block_balance_token(
            AccountId::new_unchecked(CONTRACT_USDC.to_string()),
            "USDC".to_string(),
            U128(1000000),
            contract_name,
            0,
            Gas(30_000_000_000_000),
        )
    }

    #[payable]
    pub fn transferir(&mut self, sub_contract: AccountId) -> Promise {
        ext_subcontract::transfer(
            env::signer_account_id(),
            U128(1000000),
            U128(100),
            Some(AccountId::new_unchecked(CONTRACT_USDC.to_string())),
            true,
            "USDC".to_string(),
            sub_contract,
            2,
            Gas(30_000_000_000_000),
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
        /*ext_subcontract::transfer(
            env::signer_account_id(),
            U128(1000000000000000000000000u128),
            U128(1000000000000),
            Some(AccountId::new_unchecked(CONTRACT_USDC.to_string())),
            false,
            "USDC".to_string(),
            sub_contract,
            2,
            Gas(8_000_000_000_000),
        )*/
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
        require!(attached_deposit >= 1, "you have to deposit a minimum of one yoctoNear");

        if offer_type == 1 {
            #[warn(unused_assignments)]
            let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&env::signer_account_id()).expect("the user does not have a sub contract deployed").to_string());
              
            let offer: usize = self.offers_sell.iter().position(|x| x.offer_id == offer_id).expect("Offer sell not found");
            
            require!(self.offers_sell[offer].owner_id != env::signer_account_id(), "you can not accept your own offer");
            
            match self.offers_sell[offer].asset.as_str() {
                "NEAR" => {
                    ext_subcontract::block_balance_near(
                        amount,
                        contract_name,
                        0,
                        GAS_FOR_BLOCK,
                    ).then(
                        int_offer::on_accept_offer_sell(
                            offer
                            , amount
                            , payment_method
                            , datetime
                            , rate
                            , env::current_account_id()
                            , 0
                            , BASE_GAS
                    ));
                }, 
                "USDC" => {
                    ext_subcontract::block_balance_token(
                        AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                        "USDC".to_string(),
                        amount,
                        contract_name,
                        0,
                        GAS_FOR_BLOCK,
                    ).then(
                        int_offer::on_accept_offer_sell(
                            offer
                            , amount
                            , payment_method
                            , datetime
                            , rate
                            , env::current_account_id()
                            , 0
                            , BASE_GAS
                    ));
                },
                _=> env::panic_str("The requested asset does not exist")
            };
        } else if offer_type == 2 {
            let offer: usize = self.offers_buy.iter().position(|x| x.offer_id == offer_id).expect("Offer buy not found");
            
            require!(self.offers_buy[offer].owner_id != env::signer_account_id(), "you can not accept your own offer");
            require!(self.offers_buy[offer].remaining_amount >= amount.0, "the quantity is greater than the offer buy amount");
                
            let remaining: u128 = self.offers_buy[offer].remaining_amount - amount.0;
            if remaining <= 0 {
                self.offers_buy[offer].status = 2;
            }

            if self.offers_buy[offer].max_limit > remaining {
                self.offers_buy[offer].max_limit = remaining;
            }
            if self.offers_buy[offer].min_limit > remaining {
                match self.offers_buy[offer].asset.as_str() {
                    "NEAR" => self.offers_buy[offer].min_limit = 1000000000000000000000000,
                    _=> self.offers_buy[offer].min_limit = 1000000,
                };
            }

            let fee: u128 = (amount.0 * FEE_TRANSACTION_NEAR) / 100000;
            //let fee_deducted = amount - fee;
            self.offers_buy[offer].remaining_amount = remaining;
            self.order_buy_id += 1;
            let data = OrderObject {
                offer_id: offer_id,
                order_id: self.order_buy_id,
                owner_id: self.offers_buy[offer].owner_id.clone(),
                asset: self.offers_buy[offer].asset.clone(),
                signer_id: env::signer_account_id(),
                exchange_rate: rate.to_string(),
                operation_amount: amount.0,
                amount_delivered: amount.0 - fee,
                fee_deducted: fee,
                payment_method: payment_method,
                fiat_method: self.offers_buy[offer].fiat_method,
                confirmation_owner_id: 0,
                confirmation_signer_id: 0,
                confirmation_current: 0,
                time: self.offers_buy[offer].time,
                datetime: datetime,
                terms_conditions: self.offers_buy[offer].terms_conditions.to_string(),
                status: 1,
            };
            self.orders_buy.push(data);
            //actualizar total ordenes owner_id
            let mut index = self.merchant.iter().position(|x| x.user_id == self.offers_buy[offer].owner_id.clone()).expect("owner not merchant");
            self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
            self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
            index = self.merchant.iter().position(|x| x.user_id == env::signer_account_id().clone()).expect("owner not merchant");
            self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
            self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;

            env::log_str("Offer buy accepted");
        
        }   else {
            env::panic_str("Invalid offer type");
        }
    }

    #[private]
    pub fn on_accept_offer_sell(&mut self, offer: usize
        , amount: U128
        , payment_method: i128
        , datetime: String
        , rate: f64
    ) { 
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        require!(result.is_none(), "balance is None");
        
        let valid: bool = near_sdk::serde_json::from_slice::<bool>(&result.unwrap()).expect("bool");
        require!(valid, "No balance");

        let remaining: u128 = self.offers_sell[offer].remaining_amount - amount.0;
        if remaining <= 0 {
            self.offers_sell[offer].status = 2;
        }
        
        if self.offers_sell[offer].max_limit > remaining {
            self.offers_sell[offer].max_limit = remaining;
        }
        if self.offers_sell[offer].min_limit > remaining {
            match self.offers_sell[offer].asset.as_str() {
                "NEAR" => self.offers_sell[offer].min_limit = 1000000000000000000000000,
                _=> self.offers_sell[offer].min_limit = 1000000,
            };
        }
        
        let fee: u128 = (amount.0 * FEE_TRANSACTION_NEAR) / 100000;

        self.offers_sell[offer].remaining_amount = remaining;
        self.order_sell_id += 1;
        let data = OrderObject {
            offer_id: self.offers_sell[offer].offer_id,
            order_id: self.order_sell_id,
            owner_id: self.offers_sell[offer].owner_id.clone(),
            asset: self.offers_sell[offer].asset.clone(),
            signer_id: env::signer_account_id(),
            exchange_rate: rate.to_string(),
            operation_amount: amount.0,
            amount_delivered: amount.0 - fee,
            fee_deducted: fee,
            payment_method: payment_method,
            fiat_method: self.offers_sell[offer].fiat_method,
            confirmation_owner_id: 0,
            confirmation_signer_id: 0,
            confirmation_current: 0,
            time: self.offers_sell[offer].time,
            datetime: datetime,
            terms_conditions: self.offers_sell[offer].terms_conditions.to_string(),
            status: 1,
        };
        self.orders_sell.push(data);
        //actualizar total ordenes owner_id
        let mut index = self.merchant.iter().position(|x| x.user_id == self.offers_sell[offer].owner_id.clone()).expect("owner not merchant");
        self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
        index = self.merchant.iter().position(|x| x.user_id == env::signer_account_id().clone()).expect("owner not merchant");
        self.merchant[index].total_orders = self.merchant[index].total_orders + 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
        
            
        env::log_str("Offer sell accepted");
    }
}