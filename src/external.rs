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
        vault: AccountId
    );

    fn transfer(&mut self,
        receiver_id: AccountId,
        operation_amount: U128,
        fee_deducted: U128,
        contract_ft: Option<AccountId>,
        retiro: bool,
        ft_token: String,
    );

    fn get_balance_near(self, balance_block: bool) -> Balance;

    fn get_balance_block_token(self, ft_token: String) -> Balance;

    fn delete_contract(&mut self);

    fn block_balance_near(&mut self, amount: U128) -> bool;

    fn block_balance_token(&mut self,
        contract_ft: AccountId,
        ft_token: String,
        amount: U128
    ) -> bool;

}