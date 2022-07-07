use crate::*;

#[ext_contract(ext_usdc)]
trait ExtTranferUsdc {
    fn ft_transfer(&mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>
    );

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
        operation_amount: u128,
        fee_deducted: u128,
        contract_ft: Option<AccountId>,
    );

    fn get_balance_near(self) -> Balance;

    fn delete_contract(&mut self);

    fn block_balance_near(&mut self, amount: U128) -> bool;

    fn block_balance_token(&mut self,
        contract_ft: AccountId,
        ft_token: String,
        amount: U128
    ) -> bool;


}