use crate::*;

#[near_bindgen]
impl NearP2P {
    /// confirmation order into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    // #[payable]

    pub fn delete_order(&mut self, offer_type: i8, order_id: i128) {
        assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
        if offer_type == 1{
            self.orders_sell.remove(&order_id).expect("Order Sell not found");
        } else if offer_type == 2 {
            self.orders_buy.remove(&order_id).expect("Order Sell not found");
        } else {
            env::panic_str("offer type no found");
        }
        
    }

    pub fn order_confirmation(&mut self, offer_type: i8, order_id: i128) {
        require!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR");
        let contract_ft: Option<AccountId>;
        let ft_token: String;
        let mut status: i8;
        if offer_type == 1 {
            let mut order = self.orders_sell.get(&order_id).expect("Order Sell not found");
            if order.owner_id == env::signer_account_id() {
                order.confirmation_owner_id = 1;
                if order.status == 1 {
                    order.status = 2;
                }
                self.orders_sell.insert(&order_id, &order);

                env::log_str(
                    &json!({
                        "type": "order_confirmation_owner",
                        "params": {
                            "offer_type": offer_type.to_string(),
                            "order_id": order_id.to_string(),
                            "confirmation_owner_id": "1".to_string(),
                            "status": order.status.to_string(),
                        }
                    }).to_string(),
                );
            } else if order.signer_id == env::signer_account_id() { 
                status = order.status;
                if order.status == 1 {
                    status = 2;
                }

                let offer = self.offers_sell.get(&order.offer_id).expect("Offer sell not found");

                #[warn(unused_assignments)]
                let contract_name = self.contract_list.get(&order.signer_id).expect("the user does not have a sub contract deployed");

                match offer.asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = offer.asset;
                    },
                };
                
                ext_subcontract::transfer(
                    order.owner_id.clone(),
                    U128(order.operation_amount),
                    U128(order.fee_deducted),
                    contract_ft,
                    false,
                    ft_token,
                    contract_name.contract.clone(),
                    2,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    status,
                    1,
                    /*ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                    self.orders_sell[i].signer_id.clone(),*/
                    order.clone(),
                    true,
                    order.confirmation_owner_id,
                    1,
                    order.confirmation_current,
                    env::current_account_id(),
                    0,
                    GAS_ON_CONFIRMATION,
                ));
            } else {
                env::panic_str("Server internar error, signer not found");
            }
        } else if offer_type == 2 {
            let mut order = self.orders_buy.get(&order_id).expect("Order buy not found");
            
            if order.signer_id == env::signer_account_id() {
                order.confirmation_signer_id = 1;
                if order.status == 1 {
                    order.status = 2;
                }

                self.orders_buy.insert(&order_id, &order);

                env::log_str(
                    &json!({
                        "type": "order_confirmation_signer",
                        "params": {
                            "offer_type": offer_type.to_string(),
                            "order_id": order_id.to_string(),
                            "confirmation_signer_id": "1".to_string(),
                            "status": order.status.to_string(),
                        }
                    }).to_string(),
                );
            } else if order.owner_id == env::signer_account_id() {
                status = order.status;
                if order.status == 1 {
                    status = 2;
                }

                let offer = self.offers_buy.get(&order.offer_id).expect("Offer buy not found");

                #[warn(unused_assignments)]
                let contract_name = self.contract_list.get(&order.owner_id).expect("the user does not have a sub contract deployed");
               
                match offer.asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = offer.asset;
                    },
                };
                
                ext_subcontract::transfer(
                    order.signer_id.clone(),
                    U128(order.operation_amount),
                    U128(order.fee_deducted),
                    contract_ft,
                    false,
                    ft_token,
                    contract_name.contract.clone(),
                    2,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    status,
                    2,
                    /*ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                    self.orders_buy[i].owner_id.clone(),*/
                    order.clone(),
                    true,
                    1,
                    order.confirmation_signer_id,
                    order.confirmation_current,
                    env::current_account_id(),
                    0,
                    GAS_ON_CONFIRMATION,
                ));
            } else {
                env::panic_str("Server internar error, signer not found");
            }
        }  else {
            env::panic_str("Invalid offer type");
        }
    }


    // #[payable]
    pub fn cancel_order(&mut self, offer_type: i8, order_id: i128) {
        assert_one_yocto();
        let contract_ft: Option<AccountId>;
        let ft_token: String;
        let mut status: i8;
        if offer_type == 1 {
            let mut order = self.orders_sell.get(&order_id).expect("Order Sell not found");
            
            if order.owner_id == env::signer_account_id() {
                let offer = self.offers_sell.get(&order.offer_id).expect("Offer Sell not found");
                
                status = order.status;
                if order.status == 1 || order.status == 2 {
                    status = 4;
                }

                #[warn(unused_assignments)]
                let contract_name = self.contract_list.get(&order.signer_id).expect("the user does not have a sub contract deployed");

                match offer.asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = offer.asset;
                    },
                };
                
                ext_subcontract::transfer(
                    order.signer_id.clone(),
                    U128(order.operation_amount),
                    U128(0),
                    contract_ft,
                    false,
                    ft_token,
                    contract_name.contract.clone(),
                    1,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    status,
                    1,
                    /*ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                    self.orders_sell[i].signer_id.clone(),*/
                    order.clone(),
                    false,
                    3,
                    order.confirmation_signer_id,
                    order.confirmation_current,
                    env::current_account_id(),
                    0,
                    GAS_ON_CONFIRMATION,
                ));
                
            } else if order.signer_id == env::signer_account_id() {
                order.confirmation_signer_id = 3;
                if order.status == 1 || order.status == 2 {
                    order.status = 4;
                }

                self.orders_sell.insert(&order_id, &order);

                env::log_str(
                    &json!({
                        "type": "cancel_order_signer",
                        "params": {
                            "offer_type": offer_type.to_string(),
                            "order_id": order_id.to_string(),
                            "confirmation_signer_id": "3".to_string(),
                            "status": order.status.to_string(),
                        }
                    }).to_string(),
                );
            } else {
                env::panic_str("Server internar error, signer not found");  
            }
        } else if offer_type == 2 {
            let mut order = self.orders_buy.get(&order_id).expect("Order buy not found");

            if order.owner_id == env::signer_account_id() {
                order.confirmation_owner_id = 3;
                if order.status == 1 || order.status == 2 {
                    order.status = 4;
                }

                self.orders_sell.insert(&order_id, &order);

                env::log_str(
                    &json!({
                        "type": "cancel_order_owner",
                        "params": {
                            "offer_type": offer_type.to_string(),
                            "order_id": order_id.to_string(),
                            "confirmation_owner_id": "3".to_string(),
                            "status": order.status.to_string(),
                        }
                    }).to_string(),
                );
            } else if order.signer_id == env::signer_account_id() {
                let offer = self.offers_buy.get(&order.offer_id).expect("Offer buy not found");
                
                status = order.status;
                if order.status == 1 || order.status == 2 {
                    status = 4;
                }

                #[warn(unused_assignments)]
                let contract = self.contract_list.get(&order.owner_id).expect("the user does not have a sub contract deployed");

                match offer.asset.as_str(){
                    "NEAR" => {
                        contract_ft = None;
                        ft_token = "NEAR".to_string();
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                        ft_token = offer.asset;
                    },
                };

                ext_subcontract::transfer(
                    order.owner_id.clone(),
                    U128(order.operation_amount),
                    U128(0),
                    contract_ft,
                    false,
                    ft_token,
                    contract.contract.clone(),
                    1,
                    GAS_FOR_TRANSFER,
                ).then(int_process::on_confirmation(
                    status,
                    2,
                    /*ContractList{contract: contract.contract.clone(), type_contract: contract.type_contract.clone()},
                    self.orders_buy[i].owner_id.clone(),*/
                    order.clone(),
                    false,
                    order.confirmation_owner_id,
                    3,
                    order.confirmation_current,
                    env::current_account_id(),
                    0,
                    GAS_ON_CONFIRMATION,
                ));

            } else {
                env::panic_str("Server internar error, signer not found");  
            }
        }  else {
            env::panic_str("Invalid offer type");
        }
    }


    #[private]
    pub fn on_confirmation(&mut self,
        status: i8,
        order_type: i8,
        /*data_contract: ContractList,
        signer_id: AccountId,*/
        mut order: OrderObject,
        confirmacion: bool,
        confirmation_owner_id: i8,
        confirmation_signer_id: i8,
        confirmation_current: i8,
    ) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("balance is None".as_ref());
        }

        order.status = status;
        order.confirmation_owner_id = confirmation_owner_id;
        order.confirmation_signer_id = confirmation_signer_id;
        order.confirmation_current = confirmation_current;

        if order_type == 1 {
            /*if confirmacion == true {
                self.orders_sell_completed(index);
            }*/
            self.orders_sell.remove(&order.order_id);
            
            env::log_str(
                &json!({
                    "type": "on_confirmation_sell",
                    "params": {
                        "offer_id": order.offer_id.to_string(),
                        "order_id": order.order_id.to_string(),
                        "owner_id": order.owner_id.clone(),
                        "asset": order.asset.clone(),
                        "signer_id": order.signer_id.clone(),
                        "exchange_rate": order.exchange_rate.clone(),
                        "operation_amount": order.operation_amount.to_string(),
                        "amount_delivered": order.amount_delivered.to_string(),
                        "fee_deducted": order.fee_deducted.to_string(),
                        "payment_method": order.payment_method.to_string(),
                        "fiat_method": order.fiat_method.to_string(),
                        "confirmation_owner_id": order.confirmation_owner_id.to_string(),
                        "confirmation_signer_id": order.confirmation_signer_id.to_string(),
                        "confirmation_current": order.confirmation_current.to_string(),
                        "referente": order.referente.clone(),
                        "porcentaje_referente": self.porcentaje_referente.to_string(),
                        "porcentaje_referido": self.porcentaje_referido.to_string(),
                        "time": order.time.to_string(),
                        "datetime": order.datetime.clone(),
                        "terms_conditions": order.terms_conditions.clone(),
                        "status": order.status.to_string(),
                        "confirmacion": confirmacion,
                    }
                }).to_string(),
            );

            /*if data_contract.type_contract == 2 {
                ext_subcontract::get_balance_block_total(
                    data_contract.contract.clone(),
                    0,
                    BASE_GAS,
                ).then(int_offer::on_delete_contract_user(
                    signer_id,
                    data_contract.contract,
                    env::current_account_id(),
                    0,
                    Gas(20_000_000_000_000),
                ));
            }*/ 
        } else if order_type == 2 {
            /*if confirmacion  == true {
                self.orders_buy_completed(index);
            }*/
            
            self.orders_buy.remove(&order.order_id);   

            env::log_str(
                &json!({
                    "type": "on_confirmation_buy",
                    "params": {
                        "offer_id": order.offer_id.to_string(),
                        "order_id": order.order_id.to_string(),
                        "owner_id": order.owner_id.clone(),
                        "asset": order.asset.clone(),
                        "signer_id": order.signer_id.clone(),
                        "exchange_rate": order.exchange_rate.clone(),
                        "operation_amount": order.operation_amount.to_string(),
                        "amount_delivered": order.amount_delivered.to_string(),
                        "fee_deducted": order.fee_deducted.to_string(),
                        "payment_method": order.payment_method.to_string(),
                        "fiat_method": order.fiat_method.to_string(),
                        "confirmation_owner_id": order.confirmation_owner_id.to_string(),
                        "confirmation_signer_id": order.confirmation_signer_id.to_string(),
                        "confirmation_current": order.confirmation_current.to_string(),
                        "referente": order.referente.clone(),
                        "porcentaje_referente": self.porcentaje_referente.to_string(),
                        "porcentaje_referido": self.porcentaje_referido.to_string(),
                        "time": order.time.to_string(),
                        "datetime": order.datetime.clone(),
                        "terms_conditions": order.terms_conditions.clone(),
                        "status": order.status.to_string(),
                        "confirmacion": confirmacion,
                    }
                }).to_string(),
            );
        }   
    }

    /*#[private]
    pub fn on_delete_contract_user(&mut self, signer_id: AccountId, sub_contract: AccountId) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error check balance blocked".as_ref());
        }
        let balance_block = near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128");
        //require!(balance_block <= 0, "You still have operations in progress, finish all the operations to be able to delete the contract");
        
        if balance_block <= 0 {
            ext_subcontract::delete_contract(
                sub_contract.clone(),
                0,
                Gas(5_000_000_000_000),
            ).then(int_offer::on_delete_contract_list_user(
                signer_id,
                env::current_account_id(),
                0,
                Gas(5_000_000_000_000),
            ));
            env::log_str("delete")
        } else {
            env::log_str("no delete")
        }

    }

    #[private]
    pub fn on_delete_contract_list_user(&mut self, signer_id: AccountId) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error al eliminar la cuenta".as_ref());
        }
        self.contract_list.remove(&signer_id);
    }*/

    /*#[private]
    fn orders_sell_completed(&mut self, index_order: usize) {
        let mut index = self.merchant.iter().position(|x| x.user_id == self.orders_sell[index_order].owner_id.clone()).expect("owner not merchant");
        self.merchant[index].orders_completed += 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
        index = self.merchant.iter().position(|x| x.user_id == self.orders_sell[index_order].signer_id.clone()).expect("owner not merchant");
        self.merchant[index].orders_completed += 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
    }

    #[private]
    fn orders_buy_completed(&mut self, index_order: usize) {
        let mut index = self.merchant.iter().position(|x| x.user_id == self.orders_buy[index_order].owner_id.clone()).expect("owner not merchant");
        self.merchant[index].orders_completed += 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
        index = self.merchant.iter().position(|x| x.user_id == self.orders_buy[index_order].signer_id.clone()).expect("owner not merchant");
        self.merchant[index].orders_completed += 1;
        self.merchant[index].percentaje_completion = (self.merchant[index].orders_completed as f64 / self.merchant[index].total_orders as f64) * 100.0;
    }*/
}