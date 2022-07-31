use crate::*;


#[near_bindgen]
impl NearP2P {
    //send Near
    #[payable]
    pub fn deposit(&mut self) -> Promise {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1,
            "Requires attached deposit of at least 1 yoctoNEAR",
        );
        let sub_contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        Promise::new(AccountId::new_unchecked(sub_contract.contract.to_string())).transfer(attached_deposit)
    }

    pub fn listar_subcuenta(&mut self, account_id: AccountId, subcuenta: AccountId, type_contract: i8) {
        self.contract_list.insert(account_id, ContractList{ contract: subcuenta, type_contract: type_contract});
    }

    #[payable]
    pub fn activar_subcuenta_usdc(&mut self, subaccount_id: AccountId) -> Promise {
        ext_usdc::storage_deposit(
            true,
            subaccount_id,
            AccountId::new_unchecked(CONTRACT_USDC.to_string()),
            100000000000000000000000,
            BASE_GAS,
        )
    }

    #[payable]
    pub fn create_subcontract_merchant(&mut self) -> Promise {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1490000000000000000000000,
            "Requires attached deposit of at least 1490000000000000000000000 yoctoNEAR",
        );
        let signer: AccountId = AccountId::new_unchecked(env::signer_account_id().as_str().split('.').collect::<Vec<&str>>()[0].to_string());
        let subaccount_id: AccountId = AccountId::new_unchecked(
        format!("{}.{}", signer, env::current_account_id())
        );
        let result = Promise::new(subaccount_id.clone())
            .create_account()
            .transfer(env::attached_deposit())
            .deploy_contract(CODE.to_vec())
            .then(ext_subcontract::new(
                env::signer_account_id(),
                env::current_account_id(),
                AccountId::new_unchecked("v.nearp2p.testnet".to_string()),
                subaccount_id.clone(),
                0,
                BASE_GAS,
            ));

            self.contract_list.insert(env::signer_account_id(), ContractList{ contract: subaccount_id.clone(), type_contract: 1 });

            let verificar_token = self.activate_token_list.get(&env::signer_account_id());

            //if verificar_token.is_none() || verificar_token.unwrap().get(verificar_token.unwrap().iter().position(|x| *x == "USDC".to_string()).unwrap() as usize).is_none() {
            if verificar_token.is_none() {
                ext_usdc::storage_deposit(
                    true,
                    subaccount_id,
                    AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                    100000000000000000000000,
                    BASE_GAS,
                ).then(int_sub_contract::on_listar_token_activo(
                    env::signer_account_id(),
                    "USDC".to_string(),
                    env::current_account_id(),
                    0,
                    BASE_GAS
                ));
            } else {
                Promise::new(env::signer_account_id()).transfer(100000000000000000000000);
            }  

        result
    }

    #[payable]
    pub fn create_subcontract_user(&mut self) -> Promise {
        require!(env::attached_deposit() >= 1, "you have to deposit a minimum 1 YoctoNear");
        let signer: AccountId = AccountId::new_unchecked(env::signer_account_id().as_str().split('.').collect::<Vec<&str>>()[0].to_string());
        let subaccount_id: AccountId = AccountId::new_unchecked(
        format!("{}.{}", signer, env::current_account_id())
        );
        let result = Promise::new(subaccount_id.clone())
        .create_account()
        .transfer(1490000000000000000000000)
        .deploy_contract(CODE.to_vec())
        .then(ext_subcontract::new(
            env::current_account_id(),
            env::current_account_id(),
            AccountId::new_unchecked("v.nearp2p.testnet".to_string()),
            subaccount_id.clone(),
            0,
            BASE_GAS,
        ));

        self.contract_list.insert(env::signer_account_id(), ContractList{ contract: subaccount_id.clone(), type_contract: 2 });
        
        let verificar_token = self.activate_token_list.get(&env::signer_account_id());

        //if verificar_token.is_none() || verificar_token.unwrap().get(verificar_token.unwrap().iter().position(|x| *x == "USDC".to_string()).unwrap() as usize).is_none() {
        if verificar_token.is_none() {
            ext_usdc::storage_deposit(
                true,
                subaccount_id,
                AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                100000000000000000000000,
                BASE_GAS,
            ).then(int_sub_contract::on_listar_token_activo(
                env::signer_account_id(),
                "USDC".to_string(),
                env::current_account_id(),
                0,
                BASE_GAS
            ));
        } else {
            Promise::new(env::signer_account_id()).transfer(100000000000000000000000);
        }

        result
    }
    
    #[private]
    pub fn on_listar_token_activo(&mut self, signer_id: AccountId, ft_token: String) {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error activate ft_token".as_ref());
        }
        let listar_token_activo = self.activate_token_list.get(&signer_id);

        if listar_token_activo.is_some() {
            let mut tokens: Vec<String> = listar_token_activo.unwrap().iter().map(|x| x.clone()).collect::<Vec<String>>();
            tokens.push(ft_token);
            self.activate_token_list.insert(signer_id, tokens);
        } else {
            self.activate_token_list.insert(signer_id, vec![ft_token]);
        }
    }

    pub fn get_token_activo(self, user_id: AccountId) -> bool {
        let tokens = self.activate_token_list.get(&user_id);
        if tokens.is_none() {
            false
        } else {
            if tokens.unwrap().len() > 0 {
                true
            } else {
                false
            }
        }  
    }

    /*pub fn deploy_subcontract(&mut self) -> Promise {
        let contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        Promise::new(AccountId::new_unchecked(contract.to_string())).add_full_access_key(env::signer_account_pk()).deploy_contract(CODE.to_vec())
        .then(Promise::new(AccountId::new_unchecked(contract.to_string())).delete_key(env::signer_account_pk()))
    }*/
    pub fn get_subcontract(self, user_id: AccountId) -> bool {
        let contract = self.contract_list.get(&user_id);
        if contract.is_some() {
            true
        } else if contract.is_none() {
            false
        } else {
            false
        }
    }

    pub fn get_subcontract_type(self, user_id: AccountId) -> i8 {
        let contract = self.contract_list.get(&user_id);
        if contract.is_some() {
            contract.unwrap().type_contract
        } else if contract.is_none() {
            0
        } else {
            0
        }
    }


    #[payable]
    pub fn delete_contract(&mut self) {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1,
            "you have to deposit a minimum of one yoctoNear"
        );

        let contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        ext_subcontract::get_balance_block_total(
            contract.contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_contract(
            env::signer_account_id(),
            contract.contract.clone(),
            env::current_account_id(),
            0,
            Gas(30_000_000_000_000),
        ));
    }

    
    pub fn delete_contract_admin(&mut self, user_id: AccountId) {
        self.administrators.iter().find(|&x| x == &env::signer_account_id()).expect("Only administrators");

        let contract = self.contract_list.get(&user_id).expect("the user does not have contract deployed");
        ext_subcontract::get_balance_block_total(
            contract.contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_contract(
            env::signer_account_id(),
            contract.contract.clone(),
            env::current_account_id(),
            0,
            Gas(30_000_000_000_000),
        ));
    }

    #[private]
    pub fn on_delete_contract(&mut self, signer_id: AccountId, sub_contract: AccountId) {
        /*let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error check balance blocked".as_ref());
        }
        let balance_block = near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128");
        require!(balance_block <= 0, "You still have operations in progress, finish all the operations to be able to delete the contract");
        */
        ext_subcontract::delete_contract(
            sub_contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_contract_list(
            signer_id,
            env::current_account_id(),
            0,
            BASE_GAS,
        ));

        
    }

    #[private]
    pub fn on_delete_contract_list(&mut self, signer_id: AccountId) {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error al eliminar la cuenta".as_ref());
        }
        self.contract_list.remove(&signer_id);
    }


    #[payable]
    pub fn withdraw(&mut self, ft_token: String) -> Promise {
        assert!(
            env::attached_deposit() >= 1,
            "Requires attached deposit of at least 1 yoctoNEAR",
        );
        //let contract: AccountId = AccountId::new_unchecked("pruebaa.globaldv.testnet".to_string());
        let contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        match ft_token.as_ref() {
            "NEAR" => { 
                ext_subcontract::get_balance_near(
                    "libre".to_string(),
                    contract.contract.clone(),
                    0,
                    BASE_GAS,
                ).then(int_sub_contract::on_withdraw_near(
                    contract.contract.clone(),
                    env::signer_account_id(),
                    env::current_account_id(),
                    0,
                    GAS_ON_WITHDRAW_NEAR,
                ))
            },
            "USDC" => {
                ext_usdc::ft_balance_of(
                    contract.contract.to_string(),
                    AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                    0,
                    BASE_GAS,
                ).then(int_sub_contract::on_withdraw_token_block(
                    contract.contract.clone(),
                    env::signer_account_id(),
                    "USDC".to_string(),
                    env::current_account_id(),
                    0,
                    GAS_ON_WITHDRAW_TOKEN_BLOCK,
                ))
            },
            _=> env::panic_str("ft_token not found")

        }
    }

    #[private]
    pub fn on_withdraw_near(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId
    ) -> Promise {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error Balance NEAR".as_ref());
        }
        
        let amount_withdraw: u128 = near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128");
        env::log_str(format!("{}",amount_withdraw).as_str());
        require!(amount_withdraw > 0, "No balance available to withdraw");
        
        ext_subcontract::transfer(
            signer_id,
            U128(amount_withdraw),
            U128(0),
            None,
            true,
            "NEAR".to_string(),
            sub_contract,
            1,
            GAS_FOR_TRANSFER,
        )
    }

    #[private]
    pub fn on_withdraw_token_block(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId,
        ft_token: String,
    ) -> Promise {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error balance token general".as_ref());
        }
        
        let balannce_general: U128 = near_sdk::serde_json::from_slice::<U128>(&result.unwrap()).expect("U128");
        
        ext_subcontract::get_balance_block_token(
            ft_token.clone(),
            sub_contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_withdraw_token(
            sub_contract,
            signer_id,
            ft_token,
            balannce_general,
            env::current_account_id(),
            0,
            GAS_ON_WITHDRAW_TOKEN,
        ))
    }

    #[private]
    pub fn on_withdraw_token(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId,
        ft_token: String,
        balance_general: U128,
    ) -> Promise {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error withdraw token".as_ref());
        }
        
        let balannce_block: u128 = near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128");
        let amount_withdraw: u128 = balance_general.0 - balannce_block;
        
        require!(amount_withdraw > 0, "No balance available to withdraw");

        ext_subcontract::transfer(
            signer_id,
            U128(amount_withdraw),
            U128(0u128),
            Some(AccountId::new_unchecked(CONTRACT_USDC.to_string())),
            true,
            ft_token,
            sub_contract,
            1,
            GAS_FOR_TRANSFER,
        )

    }

}