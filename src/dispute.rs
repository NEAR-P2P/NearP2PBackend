use crate::*;

#[near_bindgen]
impl NearP2P {
    pub fn set_disputer(&mut self, disputer: AccountId) -> AccountId {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");
        self.disputer = disputer;
        self.disputer.clone()
    }

    /// dispute order into the contract
    /// Params: offer_type: 1 = sell, 2 = buy
    #[payable]
    pub fn order_dispute(&mut self, offer_type: i8, order_id: i128) {
        assert_one_yocto();
        if offer_type == 1 {
            let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
            if self.orders_sell[i].status != 3 {
                if self.orders_sell[i].owner_id == env::signer_account_id() {
                    self.orders_sell[i].status = 3;
                    self.orders_sell[i].confirmation_owner_id = 2;
                    env::log_str(
                        &json!({
                            "type": "order_dispute_owner",
                            "params": {
                                "offer_type": offer_type,
                                "order_id": order_id,
                                "confirmation_owner_id": 2,
                                "status": self.orders_sell[i].status,
                            }
                        }).to_string(),
                    );
                } else if self.orders_sell[i].signer_id == env::signer_account_id() {
                    self.orders_sell[i].status = 3;
                    self.orders_sell[i].confirmation_signer_id = 2;
                    env::log_str(
                        &json!({
                            "type": "order_dispute_signer",
                            "params": {
                                "offer_type": offer_type,
                                "order_id": order_id,
                                "confirmation_signer_id": 2,
                                "status": self.orders_sell[i].status,
                            }
                        }).to_string(),
                    );
                } else {
                    env::panic_str("Server internar error, signer not found");  
                }
            } else {
                env::panic_str("The sales order is already in dispute");
            }
        } else if offer_type == 2 {
            let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");
            if self.orders_buy[i].status != 3 {
                if self.orders_buy[i].owner_id == env::signer_account_id() {
                    self.orders_buy[i].status = 3;
                    self.orders_buy[i].confirmation_owner_id = 2;
                    env::log_str(
                        &json!({
                            "type": "order_dispute_owner",
                            "params": {
                                "offer_type": offer_type,
                                "order_id": order_id,
                                "confirmation_owner_id": 2,
                                "status": self.orders_buy[i].status,
                            }
                        }).to_string(),
                    );
                } else if self.orders_buy[i].signer_id == env::signer_account_id() {
                    self.orders_buy[i].status = 3;
                    self.orders_buy[i].confirmation_signer_id = 2;
                    env::log_str(
                        &json!({
                            "type": "order_dispute_signer",
                            "params": {
                                "offer_type": offer_type,
                                "order_id": order_id,
                                "confirmation_signer_id": 2,
                                "status": self.orders_buy[i].status,
                            }
                        }).to_string(),
                    );
                } else {
                    env::panic_str("Server internar error, signer not found");  
                }
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
                let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
                if self.orders_sell[i].status != 3 {
                    self.orders_sell[i].status = 3;
                    self.orders_sell[i].confirmation_owner_id = 2;
                    self.orders_sell[i].confirmation_signer_id = 2;
                    env::log_str("Order sell in dispute");
                } else {
                    env::panic_str("The sales order is already in dispute");
                }
            } else if offer_type == 2 {
                let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");
                if self.orders_buy[i].status != 3 {
                    self.orders_buy[i].status = 3;
                    self.orders_buy[i].confirmation_owner_id = 2;
                    self.orders_buy[i].confirmation_signer_id = 2;
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
        order_id: i128
    ) {
        require!(self.disputer == env::signer_account_id(), "Only disputer");
        let contract_ft: Option<AccountId>;
        let ft_token: String;
        let mut status: i8;
        match confirmation {
            true => {
                match offer_type {
                    1 => {
                        let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
                        
                        status = self.orders_sell[i].status;
                        if self.orders_sell[i].status == 1 {
                            status = 2;
                        }
                        
                        let index_offer = self.offers_sell.iter().position(|x| x.offer_id == self.orders_sell[i].offer_id).expect("Offer sell not found");
        
                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&self.orders_sell[i].signer_id).expect("the user does not have a sub contract deployed");
        
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
                            contract_name.contract.clone(),
                            2,
                            GAS_FOR_TRANSFER,
                        ).then(int_process::on_confirmation(
                            self.orders_sell[i].order_id,
                            status,
                            1,
                            ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                            self.orders_sell[i].signer_id.clone(),
                            i,
                            false,
                            self.orders_sell[i].confirmation_owner_id,
                            self.orders_sell[i].confirmation_signer_id,
                            1,
                            env::current_account_id(),
                            0,
                            BASE_GAS,
                        ));
                
                    }, 
                    2 => {
                        let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");
                        
                        status = self.orders_buy[i].status;
                        if self.orders_buy[i].status == 1 {
                            status = 2;
                        }
        
                        let index_offer = self.offers_buy.iter().position(|x| x.offer_id == self.orders_buy[i].offer_id).expect("Offer buy not found");
        
                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&self.orders_buy[i].owner_id).expect("the user does not have a sub contract deployed");
                        
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
                            U128(self.orders_buy[i].fee_deducted),
                            contract_ft,
                            false,
                            ft_token,
                            contract_name.contract.clone(),
                            2,
                            GAS_FOR_TRANSFER,
                        ).then(int_process::on_confirmation(
                            self.orders_buy[i].order_id,
                            status,
                            2,
                            ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                            self.orders_buy[i].owner_id.clone(),
                            i,
                            false,
                            self.orders_buy[i].confirmation_owner_id,
                            self.orders_buy[i].confirmation_signer_id,
                            1,
                            env::current_account_id(),
                            0,
                            BASE_GAS,
                        ));
                    },
                    _=> env::panic_str("Invalid offer type"),
                }
            },
            _=> {
                match offer_type {
                    1 => {
                        let i = self.orders_sell.iter().position(|x| x.order_id == order_id).expect("Order Sell not found");
                        
                        let j = self.offers_sell.iter().position(|x| x.offer_id == self.orders_sell[i].offer_id).expect("Offer Sell not found");
                        status = self.orders_sell[i].status;
                        if self.orders_sell[i].status == 1 || self.orders_sell[i].status == 2 {
                            status = 4;
                        }

                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&self.orders_sell[i].signer_id).expect("the user does not have a sub contract deployed");

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
                            U128(0),
                            contract_ft,
                            false,
                            ft_token,
                            contract_name.contract.clone(),
                            1,
                            GAS_FOR_TRANSFER,
                        ).then(int_process::on_confirmation(
                            self.orders_sell[i].order_id,
                            status,
                            1,
                            ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                            self.orders_sell[i].signer_id.clone(),
                            i,
                            false,
                            self.orders_sell[i].confirmation_owner_id,
                            self.orders_sell[i].confirmation_signer_id,
                            3,
                            env::current_account_id(),
                            0,
                            GAS_ON_CONFIRMATION,
                        ));
                    },
                    2 => {
                        let i = self.orders_buy.iter().position(|x| x.order_id == order_id).expect("Order buy not found");

                        let j = self.offers_buy.iter().position(|x| x.offer_id == self.orders_buy[i].offer_id).expect("Offer buy not found");
                        status = self.orders_buy[i].status;
                        if self.orders_buy[i].status == 1 || self.orders_buy[i].status == 2 {
                            status = 4;
                        }

                        #[warn(unused_assignments)]
                        let contract_name = self.contract_list.get(&self.orders_buy[i].owner_id).expect("the user does not have a sub contract deployed");

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
                            U128(0),
                            contract_ft,
                            false,
                            ft_token,
                            contract_name.contract.clone(),
                            1,
                            GAS_FOR_TRANSFER,
                        ).then(int_process::on_confirmation(
                            self.orders_buy[i].order_id,
                            status,
                            2,
                            ContractList{contract: contract_name.contract.clone(), type_contract: contract_name.type_contract.clone()},
                            self.orders_buy[i].owner_id.clone(),
                            i,
                            false,
                            self.orders_buy[i].confirmation_owner_id,
                            self.orders_buy[i].confirmation_signer_id,
                            3,
                            env::current_account_id(),
                            0,
                            BASE_GAS,
                        ));
                    },
                    _=> env::panic_str("Invalid offer type"),
                }
            },
        }
        
    }
}