use crate::*;
use crate::subcontract::sum_balance_contract_token;

#[near_bindgen]
impl NearP2P {
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
        , extra: String // New parameter
    ) -> Promise {
        require!(env::attached_deposit() >= 1000000000000000000000, "you have to deposit a minimum 0.001 Near");
        
        let merchant = self.merchant.get(&env::signer_account_id()).expect("the user is not in the list of users");
        
        #[warn(unused_assignments)]
        let contract_name = self.contract_list.get(&env::signer_account_id()).expect("the user does not have a sub contract deployed");
        
        require!(contract_name.type_contract != 2, "must have a deployed a merchant contract");

        if asset == "NEAR".to_string() {
            ext_subcontract::get_balance_near(
                contract_name.contract.clone(),
                0,
                BASE_GAS,
            ).then(int_buy::on_set_offers_buy(
                merchant.is_merchant
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
                , extra
                , env::current_account_id()
                , 0
                , Gas(30_000_000_000_000)
            ))
        } else {
            let contract_ft = self.ft_token_list.get(&asset).expect("El ft_token subministrado en la oferta es incorrecto");

            ext_usdc::ft_balance_of(
                contract_name.contract.to_string(),
                contract_ft.contract, //AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                0,
                Gas(30_000_000_000_000),
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
                , extra
                , env::current_account_id()
                , 0
                , Gas(30_000_000_000_000)
            ))
        }
    }

    #[private]
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
        , extra: String
    ) -> i128 {
        assert!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error Balance".as_ref());
        }

        let balance_of: U128; 
        if asset.clone() == "NEAR" {
            balance_of = U128(near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128"));
        } else {
            balance_of = near_sdk::serde_json::from_slice::<U128>(&result.unwrap()).expect("U128");
        }

        assert!(balance_of.0 > 0, "El subcontrato no tiene balance");

        let mut data_sub_contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have a sub contract deployed");
        let balance_block: u128 = sum_balance_contract_token(data_sub_contract.balance_avalible.clone(), asset.clone().to_string()); // *data_sub_contract.balance_block.get(&ft_token).or(Some(&0u128)).unwrap();
        let balance_avalible: u128 = balance_of.0 - balance_block;

        assert!(balance_avalible > 0, "no hay saldo libre en el subcontrato");

        let fee: u128 = (amount.0 * FEE_TRANSACTION_NEAR) / 10000;
        let amount_offer: u128 = amount.0 + fee;

        //env::log_str(&format!("balance_of: {} - balance_block: {} - amount: {} - fee: {} - balance_avalible: {} - amount_ofer: {} - ",  balance_of.0, balance_block, amount.0, fee, balance_avalible, amount_offer).to_string());

        assert!((balance_avalible - amount_offer) as f64 >= 0.0, "el balance en la subcuenta es menor al amount + el fee suministrado");

        
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
            is_pause: false
        };

        self.offers_buy.insert(&offer_buy_id, &data);

        data_sub_contract.balance_avalible.insert(format!("OFFER|BUY|{}", offer_buy_id).to_string(), BalanceJson{asset: asset.clone(), balance: (amount.0 + fee)});
        data_sub_contract.balance_block.insert(format!("OFFER|BUY|{}", offer_buy_id).to_string(), BalanceJson{asset: asset.clone(), balance: 0u128});

        self.contract_list.insert(&env::signer_account_id(), &data_sub_contract);
        // }
        env::log_str(
            &json!({
                "type": "set_offers_buy",
                "params": {
                    "offer_id": offer_buy_id.to_string(),
                    "owner_id": owner_id.clone(),
                    "asset": asset,
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
                    "is_pause": false,
                    "extra": extra
                }
            }).to_string(),
        );

        self.offer_buy_id
        
    }
    
    #[payable]
    pub fn delete_offers_buy(&mut self, offer_id: i128) {
        let offer = self.offers_buy.get(&offer_id).expect("Offer not found");
        
        assert!(offer.owner_id == env::signer_account_id(), "the user is not the creator of this offer");

        #[warn(unused_assignments)]
        let contract_name = self.contract_list.get(&offer.owner_id.clone()).expect("the user does not have a sub contract deployed");
        require!(contract_name.type_contract != 2, "must have a contract as a deployed merchant");

        let balance_block: u128 = contract_name.balance_block.get(&format!("OFFER|BUY|{}", offer_id).to_string()).expect("la offerta no esta registrada en la lista del subcontrato del usuario").balance;

        assert!(balance_block <= 0, "Aun tiene ordenes pendientes por culminar");

        let balance_avalible: u128 = contract_name.balance_avalible.get(&format!("OFFER|BUY|{}", offer_id).to_string()).expect("la offerta no esta registrada en la lista del subcontrato del usuario").balance;

        let contract_ft: Option<AccountId>;
        
        if offer.asset == "NEAR".to_string() {
            contract_ft = None;
        } else {
            contract_ft = Some(self.ft_token_list.get(&offer.asset.clone()).expect("El ft_token subministrado en la oferta es incorrecto").contract);
        }   
        
        ext_subcontract::transfer(
            offer.owner_id.clone(),
            U128(balance_avalible),
            U128(0u128),
            contract_ft,
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
        
        let mut data_sub_contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have a sub contract deployed");
        data_sub_contract.balance_avalible.remove(&format!("OFFER|BUY|{}", offer_buy_id).to_string());
        data_sub_contract.balance_block.remove(&format!("OFFER|BUY|{}", offer_buy_id).to_string());

        self.contract_list.insert(&env::signer_account_id(), &data_sub_contract);

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