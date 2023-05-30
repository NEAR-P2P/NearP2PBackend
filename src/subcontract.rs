use crate::*;


#[near_bindgen]
impl NearP2P {
    //send Near
    #[payable]
    pub fn deposit(&mut self, sub_contract: AccountId) -> Promise {
        let attached_deposit = env::attached_deposit();
        //let sub_contract = self.contract_list.get(&sub_contract).expect("El usuario no cuenta con subcontract registrado");

        assert!(
            attached_deposit >= 1000000000000000000000,
            "Requires attached deposit of at least 0.001 NEAR",
        );
        //let sub_contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        let result = Promise::new(sub_contract).transfer(attached_deposit);

        result
    }

    pub fn listar_subcuenta(&mut self, account_id: AccountId, subcuenta: AccountId, type_contract: i8) {
        assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
        assert!(self.contract_list.get(&account_id).is_none(), "El usuario ya cuenta con un subcontract listado");
        self.contract_list.insert(&account_id, &ContractList{
            contract: subcuenta,
            type_contract: type_contract,
            balance_avalible: HashMap::new(),
            balance_block: HashMap::new(),
        });
    }

    /*#[payable]
    pub fn activar_subcuenta_usdc(&mut self, subaccount_id: AccountId) -> Promise {
        ext_usdc::storage_deposit(
            true,
            subaccount_id,
            AccountId::new_unchecked(CONTRACT_USDC.to_string()),
            100000000000000000000000,
            BASE_GAS,
        )
    }*/

    #[payable]
    pub fn activar_subcuenta_ft(&mut self, subaccount_id: AccountId, asset: String) -> Promise {
        assert!(
            env::attached_deposit() >= 100000000000000000000000,
            "Requires attached deposit of at least 100000000000000000000000 yoctoNEAR",
        );
        
        let contract_ft = self.ft_token_list.get(&asset).expect("The asset supplied in the offer is incorrect");

        let token_activos = self.activate_token_list.get(&subaccount_id).or(Some([].to_vec())).unwrap();

        assert!(token_activos.iter().find(|&x| x == &asset).is_none(), "The token is already active");

        ext_usdc::storage_deposit(
            true,
            subaccount_id,
            contract_ft.contract,
            100000000000000000000000,
            BASE_GAS,
        ).then(int_sub_contract::on_listar_token_activo(
            env::signer_account_id(),
            asset,
            env::current_account_id(),
            0,
            Gas(10_000_000_000_000)
        ))
    }


    #[payable]
    pub fn create_subcontract_merchant(&mut self) -> Promise {
        let attached_deposit = env::attached_deposit();
        
        let amount_despliegue: u128 = 1300000000000000000000000;
        
        assert!(
            attached_deposit >= amount_despliegue,
            "Requires attached deposit of at least {} yoctoNEAR",
            amount_despliegue
        );
        assert!(self.contract_list.get(&env::signer_account_id()).is_none(), "El usuario ya cuenta con un subcontract listado");

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
                AccountId::new_unchecked("vault.nearp2pdex.near".to_string()),
                amount_despliegue,
                subaccount_id.clone(),
                0,
                BASE_GAS,
            ));

            self.contract_list.insert(&env::signer_account_id(), &ContractList{ 
                contract: subaccount_id.clone(),
                type_contract: 1,
                balance_avalible: HashMap::new(),
                balance_block: HashMap::new(),
            });

        result
    }

    pub fn delete_user_contract_list(&mut self, user_id: AccountId) {
        assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");
        self.contract_list.remove(&user_id);
    }

    #[payable]
    pub fn create_subcontract_user(&mut self) -> Promise {
        require!(env::attached_deposit() >= 1, "You have to deposit a minimum 1 YoctoNear");
        assert!(self.contract_list.get(&env::signer_account_id()).is_none(), "El usuario ya cuenta con un subcontract listado");
        
        let amount_despliegue: u128 = 1300000000000000000000000;

        let signer: AccountId = AccountId::new_unchecked(env::signer_account_id().as_str().split('.').collect::<Vec<&str>>()[0].to_string());
        let subaccount_id: AccountId = AccountId::new_unchecked(
        format!("{}.{}", signer, env::current_account_id())
        );
        let result = Promise::new(subaccount_id.clone())
        .create_account()
        .transfer(amount_despliegue)
        .deploy_contract(CODE.to_vec())
        .then(ext_subcontract::new(
            env::current_account_id(),
            env::current_account_id(),
            AccountId::new_unchecked("vault.nearp2pdex.near".to_string()),
            amount_despliegue,
            subaccount_id.clone(),
            0,
            BASE_GAS,
        ));

        self.contract_list.insert(&env::signer_account_id(), &ContractList{ 
            contract: subaccount_id.clone(), 
            type_contract: 2,
            balance_avalible: HashMap::new(),
            balance_block: HashMap::new(),
        });
        

        result
    }
    
    #[private]
    pub fn on_listar_token_activo(&mut self, signer_id: AccountId, ft_token: String) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error activate ft_token".as_ref());
        }
        //let listar_token_activo = self.activate_token_list.get(&signer_id);

        /*if listar_token_activo.is_some() {
            let mut tokens: Vec<String> = listar_token_activo.unwrap().iter().map(|x| x.clone()).collect::<Vec<String>>();
            tokens.push(ft_token);
            self.activate_token_list.insert(signer_id, tokens);
        } else {*/
        self.activate_token_list.insert(&signer_id, &vec![ft_token]);
        //}
    }

    pub fn get_token_activo(self, user_id: AccountId) -> bool {
        let tokens = self.activate_token_list.get(&user_id);
        if tokens.is_none() {
            false
        } else {
            true
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

    pub fn get_balance_block(self, user_id: AccountId, asset: String) -> u128 {
        let list_balance_block = self.contract_list.get(&user_id).expect("El usuario no tiene contrato listado");
        let balance_block: u128 = sum_balance_contract_token(list_balance_block.balance_block, asset.clone()) + sum_balance_contract_token(list_balance_block.balance_avalible, asset.clone());
        
        balance_block
    }
    

    // #[payable]
    pub fn delete_contract(&mut self) {
        // let attached_deposit = env::attached_deposit();
        // assert!(
           // attached_deposit >= 1,
           // "you have to deposit a minimum of one yoctoNear"
        // );
        
        let data_sub_contract = self.contract_list.get(&env::signer_account_id()).expect("el usuario no cuenta con un subcontracto listado");

        let balance_block_total = sum_balance_contract(data_sub_contract.balance_block) + sum_balance_contract(data_sub_contract.balance_avalible);
        
        require!(balance_block_total <= 0, "You still have operations in progress, finish all the operations to be able to delete the contract");
        
        
        /*let contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        ext_subcontract::get_balance_block_total(
            contract.contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_contract(
            env::signer_account_id(),
            contract.contract.clone(),
            env::current_account_id(),
            0,
            Gas(140_000_000_000_000),
        ));*/

        ext_subcontract::get_balance_near(
            data_sub_contract.contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_withdraw_near(
            data_sub_contract.contract.clone(),
            env::signer_account_id(),
            env::current_account_id(),
            0,
            Gas(100_000_000_000_000),
        ));
    }

    
    pub fn delete_contract_admin(&mut self, user_id: AccountId) {
        assert!(self.owner_id == env::signer_account_id() || self.administrators.contains(&env::signer_account_id()), "Only administrator");

        let data_sub_contract = self.contract_list.get(&user_id).expect("el usuario no cuenta con un subcontracto listado");

        //let balance_block_total = sum_balance_contract(data_sub_contract.balance_block) + sum_balance_contract(data_sub_contract.balance_avalible);
        
        //require!(balance_block_total <= 0, "You still have operations in progress, finish all the operations to be able to delete the contract");

        ext_subcontract::get_balance_near(
            data_sub_contract.contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_withdraw_near(
            data_sub_contract.contract.clone(),
            user_id.clone(),
            env::current_account_id(),
            0,
            Gas(100_000_000_000_000),
        ));
    }

    /*#[private]
    pub fn on_delete_contract(&mut self, signer_id: AccountId, sub_contract: AccountId) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error check balance blocked".as_ref());
        }
        let balance_block = near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128");
        
        if !self.administrators.contains(&env::signer_account_id()) {
            require!(balance_block <= 0, "You still have operations in progress, finish all the operations to be able to delete the contract");
        }

        ext_subcontract::get_balance_near(
            "libre".to_string(),
            sub_contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_withdraw_near(
            sub_contract.clone(),
            signer_id.clone(),
            env::current_account_id(),
            0,
            Gas(100_000_000_000_000),
        ));
    }*/


    #[private]
    pub fn on_delete_withdraw_near(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId
    ) -> Promise {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error Balance NEAR".as_ref());
        }
        
        let amount_withdraw: u128 = near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128");
        //env::log_str(format!("{}",amount_withdraw).as_str());
        //require!(amount_withdraw > 0, "No balance available to withdraw");
        
        if amount_withdraw > 0 {
            ext_subcontract::transfer(
                signer_id.clone(),
                U128(amount_withdraw),
                U128(0),
                None,
                sub_contract.clone(),
                1,
                GAS_FOR_TRANSFER,
            );
        }

        ext_subcontract::delete_contract(
            sub_contract.clone(),
            0,
            BASE_GAS,
        ).then(int_sub_contract::on_delete_contract_list(
            signer_id,
            env::current_account_id(),
            0,
            Gas(25_000_000_000_000),
        ))
    }


    #[private]
    pub fn on_delete_contract_list(&mut self, signer_id: AccountId) {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
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
        let data_sub_contract = self.contract_list.get(&env::signer_account_id()).expect("El usuario no cuenta con subcontract listado");

        //let contract: AccountId = AccountId::new_unchecked("pruebaa.globaldv.testnet".to_string());
        let contract = self.contract_list.get(&env::signer_account_id()).expect("the user does not have contract deployed");
        let balance_avalible: u128 = sum_balance_contract_token(data_sub_contract.balance_avalible, ft_token.clone());
        let balance_block: u128 = sum_balance_contract_token(data_sub_contract.balance_block, ft_token.clone()); //U128(*data_sub_contract.balance_block.get(&ft_token).or(Some(&0u128)).unwrap());
        //let balance_block: U128 = U128(sum_balance_block_token(data_sub_contract.balance_block, ft_token.clone())); //U128( *data_sub_contract.balance_block.get(&"NEAR".to_string()).or(Some(&0u128)).unwrap() );
        match ft_token.as_ref() {
            "NEAR" => { 
                ext_subcontract::get_balance_near(
                    contract.contract.clone(),
                    0,
                    BASE_GAS,
                ).then(int_sub_contract::on_withdraw_near(
                    contract.contract.clone(),
                    env::signer_account_id(),
                    U128(balance_block + balance_avalible),
                    env::current_account_id(),
                    0,
                    GAS_ON_WITHDRAW_NEAR,
                ))
            },
            _=> {
                let contract_ft = self.ft_token_list.get(&ft_token).expect("El ft_token subministrado es incorrecto");

                ext_usdc::ft_balance_of(
                    contract.contract.to_string(),
                    contract_ft.contract.clone(), //AccountId::new_unchecked(CONTRACT_USDC.to_string()),
                    0,
                    Gas(30_000_000_000_000),
                ).then(int_sub_contract::on_withdraw_token_block(
                    contract.contract.clone(),
                    env::signer_account_id(),
                    contract_ft.contract.clone(),
                    U128(balance_block + balance_avalible),
                    env::current_account_id(),
                    0,
                    GAS_ON_WITHDRAW_TOKEN_BLOCK,
                ))
            }
        }
        
    }

    #[private]
    pub fn on_withdraw_near(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId,
        balance_block: U128
    ) -> Promise {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");
        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error Balance NEAR".as_ref());
        }
        
        let amount_withdraw: u128 = near_sdk::serde_json::from_slice::<u128>(&result.unwrap()).expect("u128");
        //env::log_str(format!("{}",amount_withdraw).as_str());
        let amount_withdraw_final: u128 = amount_withdraw - balance_block.0; 
        require!(amount_withdraw_final > 0, "No balance available to withdraw");
        
        ext_subcontract::transfer(
            signer_id,
            U128(amount_withdraw_final),
            U128(0),
            None,
            sub_contract,
            1,
            GAS_FOR_TRANSFER,
        )
    }

    #[private]
    pub fn on_withdraw_token_block(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId,
        contract_ft: AccountId,
        balance_block: U128,
    ) -> Promise {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");

        let result = promise_result_as_success();
        if result.is_none() {
            env::panic_str("Error balance token general".as_ref());
        }
        
        let balance_ft: U128 = near_sdk::serde_json::from_slice::<U128>(&result.unwrap()).expect("U128");
        let amount_withdraw: U128 = U128(balance_ft.0 - balance_block.0);

        require!(amount_withdraw.0 > 0, "No balance available to withdraw");

        int_sub_contract::on_withdraw_token(
            sub_contract,
            signer_id,
            contract_ft,
            amount_withdraw,
            env::current_account_id(),
            0,
            GAS_ON_WITHDRAW_TOKEN,
        )
    }

    #[private]
    pub fn on_withdraw_token(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId,
        contract_ft: AccountId,
        amount_withdraw: U128,
    ) -> Promise {
        require!(env::predecessor_account_id() == env::current_account_id(), "Only administrators");

        ext_subcontract::transfer(
            signer_id,
            amount_withdraw,
            U128(0u128),
            Some(contract_ft), //Some(AccountId::new_unchecked(CONTRACT_USDC.to_string())),
            sub_contract,
            1,
            GAS_FOR_TRANSFER,
        )

    }

}

pub fn sum_balance_contract(list_balance_block:  HashMap<String, BalanceJson>) -> u128 {
    let mut balance_bloqueado = 0;
    list_balance_block.iter().for_each(|(_k, v)| {
        balance_bloqueado += v.balance;
    });
    balance_bloqueado
}

pub fn sum_balance_contract_token(list_balance_block:  HashMap<String, BalanceJson>, asset: String) -> u128 {
    let mut balance_bloqueado: u128 = 0;
    list_balance_block.iter().for_each(|(_k, v)| {
        if v.asset == asset {
            balance_bloqueado += v.balance;
        }
    });
    balance_bloqueado
}
