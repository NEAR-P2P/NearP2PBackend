use crate::*;

#[near_bindgen]
impl NearP2P {
    /// confirmation order into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    #[payable]
    pub fn order_confirmation(&mut self, offer_type: i8, order_id: i128) {
        require!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR");
        let contract_ft: Option<AccountId>;
        let ft_token: String;
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

                match self.offers_sell[index_offer].asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = "USDC".to_string();
                    },
                };
                
                ext_subcontract::transfer(
                    self.orders_sell[i].owner_id.clone(),
                    U128(self.orders_sell[i].operation_amount),
                    U128(self.orders_sell[i].fee_deducted),
                    contract_ft,
                    false,
                    ft_token,
                    contract_name,
                    2,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    self.orders_sell[i].order_id,
                    2,
                    1,
                    env::current_account_id(),
                    0,
                    BASE_GAS,
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

                let index_offer = self.offers_buy.iter().position(|x| x.offer_id == self.orders_buy[i].offer_id).expect("Offer buy not found");

                #[warn(unused_assignments)]
                let contract_name: AccountId = AccountId::new_unchecked(self.contract_list.get(&self.orders_buy[i].owner_id).expect("the user does not have a sub contract deployed").to_string());
               
                match self.offers_buy[index_offer].asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = "USDC".to_string();
                    },
                };
                
                ext_subcontract::transfer(
                    self.orders_buy[i].signer_id.clone(),
                    U128(self.orders_buy[i].operation_amount),
                        U128(self.orders_sell[i].fee_deducted),
                    contract_ft,
                    false,
                    ft_token,
                    contract_name,
                    2,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    self.orders_buy[i].order_id,
                    2,
                    2,
                    env::current_account_id(),
                    0,
                    BASE_GAS,
                ));
            } else {
                env::panic_str("Server internar error, signer not found");
            }
        }  else {
            env::panic_str("Invalid offer type");
        }
    }


    #[payable]
    pub fn cancel_order(&mut self, offer_type: i8, order_id: i128) {
        assert_one_yocto();
        let contract_ft: Option<AccountId>;
        let ft_token: String;
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

                match self.offers_sell[j].asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = "USDC".to_string();
                    },
                };
                
                ext_subcontract::transfer(
                    self.orders_sell[i].signer_id.clone(),
                    U128(self.orders_sell[i].operation_amount),
                    U128(0u128),
                    contract_ft,
                    false,
                    ft_token,
                    contract_name,
                    1,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    self.orders_sell[i].order_id,
                    4,
                    1,
                    env::current_account_id(),
                    0,
                    BASE_GAS,
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

                match self.offers_buy[j].asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = "USDC".to_string();
                    },
                };

                ext_subcontract::transfer(
                    self.orders_buy[i].owner_id.clone(),
                    U128(self.orders_buy[i].operation_amount),
                    U128(0u128),
                    contract_ft,
                    false,
                    ft_token,
                    contract_name,
                    1,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    self.orders_buy[i].order_id,
                    4,
                    2,
                    env::current_account_id(),
                    0,
                    BASE_GAS,
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
            asset: arreglo[index].asset.clone(),
            signer_id: arreglo[index].signer_id.clone(),
            exchange_rate: arreglo[index].exchange_rate.to_string(),
            operation_amount: arreglo[index].operation_amount,
            amount_delivered: arreglo[index].amount_delivered,
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