# NearP2PBackend

Smart contratc that controls the workflow of a Peer to Peer DaPP. At the moment, are defined the next funtions calls:

**NETWORK:** Testnet

Created functions calls

**Lock Balance**

     near call cotractowner.your-account.testnet lock '{ "owner_id": "YOU-OWNERID.testnet" }'  --accountId youraccount.testnet --deposit attached-deposit
 
**Get Locked balance**

     near call cotractowner.your-account.testnet get_locked_balance '{ "owner_id": "YOU-OWNERID.testnet", "escrow_account_id": "ESCROW-ACCOUNT.testnet" }'  --accountId youraccount.testnet
     
**Get Offers**

    near call cotractowner.your-account.testnet get_offers --accountId youraccount.testnet
    
**Remove offers**

    near call cotractowner.your-account.testnet remove_offers '{ "order_id": 7 }' --accountId youraccount.testnet
    
**Set offers**

    near call cotractowner.your-account.testnet set_offers '{ "owner_id": "carla.testnet", "asset": "NEAR", "price": "15", "amount": "10", "min_limit": "20", "max_limit": "150", "order_type": 2, "payment_method": 1, "orders_completed": 0, "percentaje_completion": 0, "badge": "star-o" }' --accountId youraccount.testnet

