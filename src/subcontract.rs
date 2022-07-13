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
        Promise::new(AccountId::new_unchecked(sub_contract.to_string())).transfer(attached_deposit)
    }

    pub fn listar_subcuenta(&mut self, account_id: AccountId, subcuenta: AccountId) {
        self.contract_list.insert(account_id, subcuenta);
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
    pub fn create_subcontract(&mut self) -> Promise {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1600000000000000000000000,
            "Requires attached deposit of at least 1600000000000000000000000 yoctoNEAR",
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

            ext_usdc::storage_deposit(
                true,
                subaccount_id.clone(),
                AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                100000000000000000000000,
                BASE_GAS,
            );
        
        self.contract_list.insert(env::signer_account_id(), subaccount_id);

        result
    }
    
    pub fn deploy_subcontract(&mut self) -> Promise {
        let contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        Promise::new(AccountId::new_unchecked(contract.to_string())).add_full_access_key(env::signer_account_pk()).deploy_contract(CODE.to_vec())
        .then(Promise::new(AccountId::new_unchecked(contract.to_string())).delete_key(env::signer_account_pk()))
    }

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

    #[payable]
    pub fn delete_contract(&mut self) {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1,
            "you have to deposit a minimum of one yoctoNear"
        );
        let contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        ext_subcontract::delete_contract(
            contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_contract(
            env::signer_account_id(),
            env::current_account_id(),
            0,
            BASE_GAS,
        ));
    }

    #[private]
    pub fn on_delete_contract(&mut self, account_id: AccountId) {
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error al eliminar la cuenta".as_ref());
        }
        self.contract_list.remove(&account_id);
    }

    #[payable]
    pub fn withdraw(&mut self, ft_token: String) -> Promise {
        assert!(
            env::attached_deposit() >= 1,
            "Requires attached deposit of at least 1 yoctoNEAR",
        );
        let contract: AccountId = AccountId::new_unchecked("pruebaa.globaldv.testnet".to_string());//AccountId::new_unchecked(self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed").to_string());
        match ft_token.as_ref() {
            "NEAR" => { 
                ext_subcontract::get_balance_near(
                    true,
                    contract.clone(),
                    0,
                    BASE_GAS,
                ).then(int_sub_contract::on_withdraw_near(
                    contract,
                    env::signer_account_id(),
                    env::current_account_id(),
                    0,
                    GAS_ON_WITHDRAW_NEAR,
                ))
            },
            "USDC" => {
                ext_usdc::ft_balance_of(
                    contract.to_string(),
                    AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                    0,
                    BASE_GAS,
                ).then(int_sub_contract::on_withdraw_token_block(
                    contract,
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
        /*if result.is_none() {
            env::panic_str("Error Balance NEAR".as_ref());
        }*/
        
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
            env::panic_str("Error Balance block".as_ref());
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