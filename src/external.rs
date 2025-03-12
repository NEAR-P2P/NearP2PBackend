use crate::*;

#[ext_contract(ext_usdc)]
trait ExtTranferUsdc {
    fn ft_transfer(&mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>
    );

    fn storage_deposit(&mut self, registration_only: bool,
        account_id: AccountId);

    fn ft_balance_of(self, account_id: String);
}

#[ext_contract(ext_subcontract)]
trait ExtSubContract {
    fn new(
        owner_id: AccountId,
        user_admin: AccountId, 
        vault: AccountId,
        consumo_storage_near_subcontract: u128,
    );

    fn transfer(&mut self,
        receiver_id: AccountId,
        operation_amount: U128,
        fee_deducted: U128,
        contract_ft: Option<AccountId>,
    );

    fn get_balance_near(self) -> u128;

    //fn get_balance_block_token(self, ft_token: String) -> Balance;

    //fn get_balance_block_total(self) -> Balance;

    fn delete_contract(&mut self);

    //fn block_balance_near(&mut self, amount: U128) -> bool;

    /*fn block_balance_token(&mut self,
        contract_ft: AccountId,
        ft_token: String,
        amount: U128
    ) -> bool;*/

}

#[ext_contract(ext_distribution)]
trait ExtSubContract {
    fn set_fee(&mut self,
        id: String,
        token: Option<AccountId>,
        amount: U128,
    );
}