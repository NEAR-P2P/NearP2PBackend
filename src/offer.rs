use crate::*;

#[near_bindgen]
impl NearP2P {
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
            let contract = self.contract_list.get(&env::signer_account_id());
            if contract.is_some() {
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
            } else if contract.is_none() {
                env::panic_str("no tiene un contrato desplegado parta aceptar la oferta de venta");
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
}