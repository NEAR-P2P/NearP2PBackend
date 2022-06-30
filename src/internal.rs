use crate::*;

#[ext_contract(ext_internal)]
trait ExtNftDos {
    fn on_ft_balance_of(&mut self);

    fn on_confirmation(&mut self,
        order_id: i128,
        status: i8,
        order_type: i8,
    );

    fn on_delete_contract(&mut self,
        account_id: AccountId
    );

}