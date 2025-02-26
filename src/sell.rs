use crate::*;


#[near_bindgen]
impl NearP2P {
    /// Set the offer sell object into the contract
    /// Params: owner_id: String, asset: String, exchange_rate: String, amount: String
    /// min_limit: String, max_limit: String, payment_method_id: String, status: i8
    /// This is a list of offers for sellings operations, will be called by the user
    #[payable]
    pub fn set_offers_sell(&mut self
        , asset: String
        , exchange_rate: String
        , amount: U128
        , min_limit: U128
        , max_limit: U128
        , payment_method: Vec<PaymentMethodsOfferObject>
        , fiat_method: i128
        , time: i64
        , terms_conditions: String
        , extra: serde_json::Value // New parameter
    ) -> i128 {
        require!(env::attached_deposit() >= 1000000000000000000000, "you have to deposit a minimum 0.001 Near");
        let merchant = self.merchant.get(&env::signer_account_id()).expect("the user is not in the list of users");
        
        self.offer_sell_id += 1;
        
        let offer_sell_id = self.offer_sell_id;

        let data = OfferObject {
            offer_id: offer_sell_id,
            owner_id: env::signer_account_id(),
            asset: asset.clone(),
            exchange_rate: exchange_rate.clone(),
            amount: amount.0,
            remaining_amount: amount.0,
            min_limit: min_limit.0,
            max_limit: max_limit.0,
            payment_method: payment_method.clone(),
            fiat_method: fiat_method,
            is_merchant: merchant.is_merchant,
            time: time,
            terms_conditions: terms_conditions.clone(),
            status: 1,
            is_pause: false,
            extra: extra,
        };

        self.offers_sell.insert(&offer_sell_id, &data);

        //let fiat_method_string: String = fiat_method.to_string();

        env::log_str(
            &json!({
                "type": "set_offers_sell",
                "params": {
                    "offer_id": offer_sell_id.to_string(),
                    "owner_id": env::signer_account_id(),
                    "asset": asset.to_string(),
                    "exchange_rate": exchange_rate.to_string(),
                    "amount": amount,
                    "remaining_amount": amount,
                    "min_limit": min_limit,
                    "max_limit": max_limit,
                    "payment_method": payment_method.clone(),
                    "fiat_method": fiat_method.to_string(),
                    "is_merchant": merchant.is_merchant,
                    "time": time.to_string(),
                    "terms_conditions": terms_conditions.to_string(),
                    "status": "1".to_string(),
                    "is_pause": false,
                    "extra": extra,
                }
            }).to_string(),
        );
       
        self.offer_sell_id
    }


    #[payable]
    pub fn delete_offers_sell(&mut self, offer_id: i128) {
        assert_one_yocto();
        let offer = self.offers_sell.get(&offer_id).expect("Offer not found");
        
        assert!(offer.owner_id == env::signer_account_id(), "the user is not the creator of this offer");

        self.offers_sell.remove(&offer_id).expect("Offer not found");
        
        env::log_str(
            &json!({
                "type": "delete_offers_sell",
                "params": {
                    "offer_id": offer_id.to_string(),
                }
            }).to_string(),
        );
    }
}