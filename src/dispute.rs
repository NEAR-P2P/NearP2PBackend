use crate::*;

#[near_bindgen]
impl NearP2P {
    pub fn set_disputer(&mut self, disputer: AccountId) -> AccountId {
        assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
        self.disputer = disputer;
        self.disputer.clone()
    }

    /// dispute order into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    #[payable]
    pub fn order_dispute(&mut self, offer_type: i8, order_id: i128) {
        require!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR");
        // assert_one_yocto();
        if offer_type == 1 {
            let mut order = self.orders_sell.get(&order_id).expect("Order Sell not found");
            if order.status != 3 {
                if order.owner_id == env::signer_account_id() {
                    order.status = 3;
                    order.confirmation_owner_id = 2;

                    env::log_str(
                        &json!({
                            "type": "order_dispute_owner",
                            "params": {
                                "offer_type": offer_type.to_string(),
                                "order_id": order_id.to_string(),
                                "confirmation_owner_id": "2".to_string(),
                                "status": order.status.to_string(),
                            }
                        }).to_string(),
                    );
                } else if order.signer_id == env::signer_account_id() {
                    order.status = 3;
                    order.confirmation_signer_id = 2;

                    env::log_str(
                        &json!({
                            "type": "order_dispute_signer",
                            "params": {
                                "offer_type": offer_type.to_string(),
                                "order_id": order_id.to_string(),
                                "confirmation_signer_id": "2".to_string(),
                                "status": order.status.to_string(),
                            }
                        }).to_string(),
                    );
                } else {
                    env::panic_str("Server internar error, signer not found");  
                }

                self.orders_sell.insert(&order_id, &order);

            } else {
                env::panic_str("The sales order is already in dispute");
            }
        } else if offer_type == 2 {
            let mut order = self.orders_buy.get(&order_id).expect("Order buy not found");
            if order.status != 3 {
                if order.owner_id == env::signer_account_id() {
                    order.status = 3;
                    order.confirmation_owner_id = 2;
                    env::log_str(
                        &json!({
                            "type": "order_dispute_owner",
                            "params": {
                                "offer_type": offer_type.to_string(),
                                "order_id": order_id.to_string(),
                                "confirmation_owner_id": "2".to_string(),
                                "status": order.status.to_string(),
                            }
                        }).to_string(),
                    );
                } else if order.signer_id == env::signer_account_id() {
                    order.status = 3;
                    order.confirmation_signer_id = 2;

                    env::log_str(
                        &json!({
                            "type": "order_dispute_signer",
                            "params": {
                                "offer_type": offer_type.to_string(),
                                "order_id": order_id.to_string(),
                                "confirmation_signer_id": "2".to_string(),
                                "status": order.status.to_string(),
                            }
                        }).to_string(),
                    );
                } else {
                    env::panic_str("Server internar error, signer not found");  
                }
                self.orders_buy.insert(&order_id, &order);
            } else {
                env::panic_str("The sales order is already in dispute");
            }
        }  else {
            env::panic_str("Invalid offer type");
        }
    }
    
    pub fn dispute(&mut self, offer_type: i8, order_id: i128, token: String) {
        if KEY_TOKEN == token {
            if offer_type == 1 {
                let mut order = self.orders_sell.get(&order_id).expect("Order Sell not found");
                if order.status != 3 {
                    order.status = 3;
                    order.confirmation_owner_id = 2;
                    order.confirmation_signer_id = 2;

                    self.orders_sell.insert(&order_id, &order);

                    env::log_str("Order sell in dispute");
                } else {
                    env::panic_str("The sales order is already in dispute");
                }
            } else if offer_type == 2 {
                let mut order = self.orders_buy.get(&order_id).expect("Order buy not found");
                if order.status != 3 {
                    order.status = 3;
                    order.confirmation_owner_id = 2;
                    order.confirmation_signer_id = 2;

                    self.orders_buy.insert(&order_id, &order);

                    env::log_str("Order buy in dispute");
                } else {
                    env::panic_str("The sales order is already in dispute");
                }
            }  else {
                env::panic_str("Invalid offer type");
            }
        } else {
            env::panic_str("Invalid Key_token");
        }
    }

    pub fn resolve_dispute(&mut self,
        confirmation: bool,
        offer_type: i8,
        order_id: i128,
    ) {
        require!(self.disputer == env::signer_account_id(), "Only disputer");
        let contract_ft: Option<AccountId>;
        let mut status: i8;
        match confirmation {
            true => {
                match offer_type {
                    1 => {
                        let order = self.orders_sell.get(&order_id).expect("Order Sell not found");
                        
                        status = order.status;
                        if order.status == 1 {
                            status = 2;
                        }
                        
                        /* let offer = self.offers_sell.get(&order.offer_id).expect("Offer sell not found");
        
                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.signer_id).expect("the user does not have a sub contract deployed");
        
                        match offer.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&offer.asset.clone()).expect("El ft_token subministrado en la oferta es incorrecto").contract);//Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                            },
                        }; */
                        
        
                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.signer_id).expect("the user does not have a sub contract deployed");
        
                        match order.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&order.asset.clone()).expect("El ft_token subministrado en la orden es incorrecto").contract);//Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                            },
                        };


                        ext_subcontract::transfer(
                            order.owner_id.clone(),
                            U128(order.operation_amount),
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
                            false,
                            order.confirmation_owner_id,
                            order.confirmation_signer_id,
                            1,
                            contract_ft.clone(),
                            env::current_account_id(),
                            0,
                            GAS_ON_CONFIRMATION,
                        ));
                
                    }, 
                    2 => {
                        let order = self.orders_buy.get(&order_id).expect("Order buy not found");
                        
                        status = order.status;
                        if order.status == 1 {
                            status = 2;
                        }
        
                        /* let offer = self.offers_buy.get(&order.offer_id).expect("Offer buy not found");
        
                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.owner_id).expect("the user does not have a sub contract deployed");
                        
                        match offer.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&offer.asset.clone()).expect("El ft_token subministrado en la oferta es incorrecto").contract);
                            },
                        }; */


                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.owner_id).expect("the user does not have a sub contract deployed");
        
                        match order.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&order.asset.clone()).expect("El ft_token subministrado en la orden es incorrecto").contract);//Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                            },
                        };
                        
                        ext_subcontract::transfer(
                            order.signer_id.clone(),
                            U128(order.operation_amount),
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
                            false,
                            order.confirmation_owner_id,
                            order.confirmation_signer_id,
                            1,
                            contract_ft.clone(),
                            env::current_account_id(),
                            0,
                            GAS_ON_CONFIRMATION,
                        ));
                    },
                    _=> env::panic_str("Invalid offer type"),
                }
            },
            _=> {
                match offer_type {
                    1 => {
                        let order = self.orders_sell.get(&order_id).expect("Order Sell not found");
                        
                        // let offer = self.offers_sell.get(&order.offer_id).expect("Offer Sell not found");
                        status = order.status;
                        if order.status == 1 || order.status == 2 {
                            status = 4;
                        }

                        /* #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.signer_id).expect("the user does not have a sub contract deployed");

                        match offer.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&offer.asset.clone()).expect("El ft_token subministrado en la oferta es incorrecto").contract);
                            },
                        }; */


                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.signer_id).expect("the user does not have a sub contract deployed");
        
                        match order.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&order.asset.clone()).expect("El ft_token subministrado en la orden es incorrecto").contract);//Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
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
                            order.confirmation_owner_id,
                            order.confirmation_signer_id,
                            3,
                            contract_ft.clone(),
                            env::current_account_id(),
                            0,
                            GAS_ON_CONFIRMATION,
                        ));
                    },
                    2 => {
                        let order = self.orders_buy.get(&order_id).expect("Order buy not found");

                        // let offer = self.offers_buy.get(&order.offer_id).expect("Offer buy not found");
                        status = order.status;
                        if order.status == 1 || order.status == 2 {
                            status = 4;
                        }

                        /* #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.owner_id).expect("the user does not have a sub contract deployed");

                        match offer.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&offer.asset.clone()).expect("El ft_token subministrado en la oferta es incorrecto").contract);
                            },
                        }; */

                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&order.owner_id).expect("the user does not have a sub contract deployed");
        
                        match order.asset.as_str(){
                            "NEAR" => {
                                contract_ft = None;
                            },
                            _=> {
                                contract_ft = Some(self.ft_token_list.get(&order.asset.clone()).expect("El ft_token subministrado en la orden es incorrecto").contract);//Some(AccountId::new_unchecked(CONTRACT_USDC.to_string()));
                            },
                        };

                        ext_subcontract::transfer(
                            order.owner_id.clone(),
                            U128(order.operation_amount),
                            U128(0),
                            contract_ft.clone(),
                            contract_name.contract.clone(),
                            1,
                            GAS_FOR_TRANSFER,
                        ).then(int_process::on_confirmation(
                            status,
                            2,
                            /*ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                            self.orders_buy[i].owner_id.clone(),*/
                            order.clone(),
                            false,
                            order.confirmation_owner_id,
                            order.confirmation_signer_id,
                            3,
                            contract_ft.clone(),
                            env::current_account_id(),
                            0,
                            GAS_ON_CONFIRMATION,
                        ));
                    },
                    _=> env::panic_str("Invalid offer type"),
                }
            },
        }
        
    }
}