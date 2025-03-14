use crate::*;

#[near_bindgen]
impl NearP2P {
    /// confirmation order into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    // #[payable]

    /*pub fn delete_order(&mut self, offer_type: i8, order_id: i128) {
        assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
        if offer_type == 1{
            self.orders_sell.remove(&order_id).expect("Order Sell not found");
        } else if offer_type == 2 {
            self.orders_buy.remove(&order_id).expect("Order Sell not found");
        } else {
            env::panic_str("offer type no found");
        }
        
    }*/

    #[payable]
    pub fn order_confirmation(&mut self, offer_type: i8, order_id: i128) {
        require!(env::attached_deposit() >= 3, "Requires attached deposit of at least 3 yoctoNEAR");
        let contract_ft: Option<AccountId>;
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
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                    },
                };

                
                ext_subcontract::transfer(
                    order.owner_id.clone(),
                    U128(order.amount_delivered),
                    U128(order.fee_deducted),
                    contract_ft.clone(),
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
                    contract_ft.clone(),
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
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                    },
                };
                
                ext_subcontract::transfer(
                    order.signer_id.clone(),
                    U128(order.amount_delivered),
                    U128(order.fee_deducted),
                    contract_ft.clone(),
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
                    contract_ft.clone(),
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


    #[payable]
    pub fn cancel_order(&mut self, offer_type: i8, order_id: i128) {
        //assert_one_yocto();
        require!(env::attached_deposit() >= 2, "Requires attached deposit of at least 2 yoctoNEAR");
        let contract_ft: Option<AccountId>;
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
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                    },
                };
                
                ext_subcontract::transfer(
                    order.signer_id.clone(),
                    U128(order.operation_amount),
                    U128(0),
                    contract_ft.clone(),
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
                    contract_ft.clone(),
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
                    },
                    _=> {
                        contract_ft = Some(self.ft_token_list.get(&offer.asset).expect("El asset subministrado en la oferta es incorrecto").contract); //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                    },
                };

                ext_subcontract::transfer(
                    order.owner_id.clone(),
                    U128(order.operation_amount),
                    U128(0),
                    contract_ft.clone(),
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
                    contract_ft.clone(),
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
        contract_ft: Option<AccountId>,
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
        
        let mut amount_referente: u128 = 0;
        let mut amount_referido: u128 = 0;
        if confirmacion  == true {
            //self.orders_buy_completed(index);
            let fee_referidos: u128 = (order.fee_deducted * 4_000u128) / 10_000u128;
            amount_referente = (fee_referidos * self.porcentaje_referente)/10_000u128;
            amount_referido = (fee_referidos * self.porcentaje_referido)/10_000u128;
        }

        if order_type == 1 {
            let data_sub_contract = self.contract_list.get(&order.signer_id); //.expect("the user does not have a sub contract deployed");
            
            if data_sub_contract.is_some() {
                let mut data_sub_contract_clon: ContractList = data_sub_contract.clone().unwrap();

                let balance_json_default: BalanceJson = BalanceJson{asset: "".to_string(), balance: 0u128};

                let balance_block: u128 = data_sub_contract_clon.balance_block.get(&format!("ORDER|SELL|{}", order.order_id).to_string()).or(Some(&balance_json_default)).unwrap().balance;
                
                data_sub_contract_clon.balance_block.insert(format!("ORDER|SELL|{}", order.order_id).to_string(), BalanceJson{
                    asset: order.asset.clone(), 
                    balance: balance_block - order.operation_amount,
                });
                
                self.contract_list.insert(&order.signer_id, &data_sub_contract_clon);
            }
            

            /* self.orders_sell.remove(&order.order_id);

            let oferta = self.offers_sell.get(&order.offer_id); // .expect("Offer sell not found");
            if oferta.is_some() {
                if oferta.unwrap().remaining_amount <= 0 {
                    self.offers_sell.remove(&order.offer_id);
                }
            } */

            let associated: Vec<&str> = order.terms_conditions.split("|").collect::<Vec<&str>>();
            if associated.len() > 1 {
                ext_distribution::set_fee(
                    associated[1].to_string(),
                    contract_ft,
                    U128(order.fee_deducted),
                    AccountId::new_unchecked(CONTRACT_DISTRIBUTION.to_string()),
                    0,
                    GAS_FOR_DISTRIBUCION,
                );
            }
            
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
                        "amount_referente": amount_referente.to_string(),
                        "amount_referido": amount_referido.to_string(),
                        "time": order.time.to_string(),
                        "datetime": order.datetime.clone(),
                        "terms_conditions": order.terms_conditions.clone(),
                        "status": status.to_string(),
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
            self.orders_buy.remove(&order.order_id);
            
            let data_sub_contract = self.contract_list.get(&order.owner_id);//.expect("the offer does not have a sub contract deployed");

            if data_sub_contract.is_some() {
                let mut data_sub_contract_clon: ContractList = data_sub_contract.clone().unwrap();

                let balance_block: u128 = data_sub_contract_clon.balance_block.get(&format!("OFFER|BUY|{}", order.offer_id).to_string()).unwrap().balance;
                
                data_sub_contract_clon.balance_block.insert(format!("OFFER|BUY|{}", order.offer_id).to_string(), BalanceJson{
                    asset: order.asset.clone(), 
                    balance: balance_block - (order.operation_amount + order.fee_deducted),
                });
                
                self.contract_list.insert(&order.owner_id, &data_sub_contract_clon);
            }
            
            /* let oferta = self.offers_buy.get(&order.offer_id); //.expect("Offer sell not found");
            
            if oferta.is_some() {
                if oferta.unwrap().remaining_amount <= 0 {
                    self.offers_buy.remove(&order.offer_id);
                }
            } */

            let associated: Vec<&str> = order.terms_conditions.split("|").collect::<Vec<&str>>();
            if associated.len() > 1 {
                ext_distribution::set_fee(
                    associated[1].to_string(),
                    contract_ft,
                    U128(order.fee_deducted),
                    AccountId::new_unchecked(CONTRACT_DISTRIBUTION.to_string()),
                    0,
                    GAS_FOR_DISTRIBUCION,
                );
            }

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
                        "amount_referente": amount_referente.to_string(),
                        "amount_referido": amount_referido.to_string(),
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
}
