use crate::*;

#[ext_contract(int_sub_contract)]
trait IntSubContract {
    fn on_ft_balance_of(&mut self);

    fn on_delete_contract(&mut self,
        signer_id: AccountId,
        sub_contract: AccountId
    );
    
    fn on_delete_contract_list(&mut self,
        signer_id: AccountId
    );

    fn on_withdraw_near(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId
    ) -> Promise;

    fn on_withdraw_token_block(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId,
        ft_token: String,
    ) -> Promise;

    fn on_withdraw_token(&mut self,
        sub_contract: AccountId,
        signer_id: AccountId,
        ft_token: String,
        balance_general: U128,
    ) -> Promise;

    fn on_listar_token_activo(&mut self, signer_id: AccountId, ft_token: String);
}


#[ext_contract(int_buy)]
trait IntBuy {
    fn on_set_offers_buy(&mut self, index: usize
        , owner_id: AccountId
        , asset: String
        , exchange_rate: String
        , amount:  U128
        , min_limit: U128
        , max_limit: U128
        , payment_method: Vec<PaymentMethodsOfferObject>
        , fiat_method: i128
        , time: i64
        , terms_conditions: String
    ) -> i128;

    /*fn on_put_offers_buy(&mut self, offer_id: i128
        , offer: usize
        , asset: Option<String>
        , exchange_rate: Option<String>
        , remaining_amount: Option<U128>
        , min_limit: Option<U128>
        , max_limit: Option<U128>
        , payment_method: Option<Vec<PaymentMethodsOfferObject>>
        , fiat_method: Option<i128>
        , time: Option<i64>
        , terms_conditions: Option<String>
    );*/

    fn on_delete_offers_buy(&mut self, offer: usize);
}

#[ext_contract(int_offer)]
trait IntOffer {
    fn on_accept_offer_sell(&mut self, offer: usize
        , amount: U128
        , payment_method: i128
        , datetime: String
        , rate: f64
    );

    fn on_delete_contract_user(&mut self, signer_id: AccountId, sub_contract: AccountId);

    fn on_delete_contract_list_user(&mut self, signer_id: AccountId);
}

#[ext_contract(int_process)]
trait IntProcess {
    fn on_confirmation(&mut self,
        order_id: i128,
        status: i8,
        order_type: i8,
        data_contract: ContractList,
        signer_id: AccountId,
    );
}