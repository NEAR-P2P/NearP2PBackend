use crate::*;


#[near_bindgen]
impl NearP2P {
    /// Returns the order object loaded in contract
    /// Params: campo: String, valor: String
    pub fn get_offers_sell(self, amount: Option<U128>,
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
        if self.offers_sell.len() > 0 {
            search_offer(self.offers_sell, amount, fiat_method, payment_method, is_merchant, owner_id, status, offer_id, asset, signer_id, from_index, limit)
        } else {
            SearchOfferObject {
                total_index: 0,
                data: [].to_vec(),
            }
        }
    }


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
    ) -> i128 {
        require!(env::attached_deposit() >= 100000000000000000000000, "you have to deposit a minimum 0.1 Near");
        let index = self.merchant.iter().position(|x| x.user_id == env::signer_account_id()).expect("the user is not in the list of users");
        
        self.offer_sell_id += 1;
        
        let offer_sell_id: String = self.offer_sell_id.to_string();

        let data = OfferObject {
            offer_id: offer_sell_id.parse::<i128>().unwrap(),
            owner_id: env::signer_account_id(),
            asset: asset.clone(),
            exchange_rate: exchange_rate.clone(),
            amount: amount.0,
            remaining_amount: amount.0,
            min_limit: min_limit.0,
            max_limit: max_limit.0,
            payment_method: payment_method.clone(),
            fiat_method: fiat_method,
            is_merchant: self.merchant[index].is_merchant,
            time: time,
            terms_conditions: terms_conditions.clone(),
            status: 1,
        };

        self.offers_sell.push(data);

        //let fiat_method_string: String = fiat_method.to_string();

        env::log_str(
            &json!({
                "type": "set_offers_sell",
                "params": {
                    "offer_id": offer_sell_id,
                    "owner_id": env::signer_account_id(),
                    "asset": asset.to_string(),
                    "exchange_rate": exchange_rate.to_string(),
                    "amount": amount,
                    "remaining_amount": amount,
                    "min_limit": min_limit,
                    "max_limit": max_limit,
                    "payment_method": payment_method.clone(),
                    "fiat_method": fiat_method.to_string(),
                    "is_merchant": self.merchant[index].is_merchant,
                    "time": time.to_string(),
                    "terms_conditions": terms_conditions.to_string(),
                    "status": "1".to_string(),
                }
            }).to_string(),
        );
       
        self.offer_sell_id
    }


    /*#[payable]
    pub fn put_offers_sell(&mut self, offer_id: i128
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
        let offer = self.offers_sell.iter().position(|x| x.offer_id == offer_id && x.owner_id == env::signer_account_id()).expect("Offer not found");
        if asset.is_some() {
            self.offers_sell[offer].asset = asset.unwrap();
        }
        if exchange_rate.is_some() {
            self.offers_sell[offer].exchange_rate = exchange_rate.unwrap();
        }
        if remaining_amount.is_some() {
            self.offers_sell[offer].remaining_amount = remaining_amount.unwrap().0;
        }
        if min_limit.is_some() {
            self.offers_sell[offer].min_limit = min_limit.unwrap().0;
        }
        if max_limit.is_some() {
            self.offers_sell[offer].max_limit = max_limit.unwrap().0;
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
    }*/


    pub fn delete_offers_sell(&mut self, offer_id: i128) {
        let offer = self.offers_sell.iter().position(|x| x.offer_id == offer_id && x.owner_id == env::signer_account_id()).expect("Offer not found");
        self.offers_sell.remove(offer);
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