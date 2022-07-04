use crate::*;

#[near_bindgen]
impl NearP2P {
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
                    env::log_str("Order sell in dispute");
                } else if self.orders_sell[i].signer_id == env::signer_account_id() {
                    self.orders_sell[i].status = 3;
                    self.orders_sell[i].confirmation_signer_id = 2;
                    env::log_str("Order sell in dispute");
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
                    env::log_str("Order buy in dispute");
                } else if self.orders_buy[i].signer_id == env::signer_account_id() {
                    self.orders_buy[i].status = 3;
                    self.orders_buy[i].confirmation_signer_id = 2;
                    env::log_str("Order buy in dispute");
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
}