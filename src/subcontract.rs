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

    pub fn delete(&mut self, account_id: AccountId) {
        self.contract_list.remove(&account_id);
    }

    #[payable]
    pub fn create_subcontract(&mut self) -> Promise {
        let attached_deposit = env::attached_deposit();
        assert!(
            attached_deposit >= 1140000000000000000000000,
            "Requires attached deposit of at least 1140000000000000000000000 yoctoNEAR",
        );
        let signer: AccountId = AccountId::new_unchecked(env::signer_account_id().as_str().split('.').collect::<Vec<&str>>()[0].to_string());
        let subaccount_id = AccountId::new_unchecked(
        format!("{}.{}", signer, env::current_account_id())
        );
        let result = Promise::new(subaccount_id.clone())
            .create_account()
            .transfer(env::attached_deposit())
            .deploy_contract(CODE.to_vec())
            .then(ext_subcontract::new(
                AccountId::new_unchecked("nearp2p.testnet".to_string()),
                env::current_account_id(), 
                AccountId::new_unchecked("v.nearp2p.testnet".to_string()),
                subaccount_id.clone(),
                0,
                BASE_GAS,
            ));
        
        self.contract_list.insert(env::predecessor_account_id(), subaccount_id);

        result
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
}