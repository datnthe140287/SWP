use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    near_bindgen,
    Promise,
    AccountId, 
    collections::{LookupMap},
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenStorage {
    pub balances: LookupMap<AccountId, u128>,
    pub allowances: LookupMap<(AccountId, AccountId), u128>,
}

impl Default for TokenStorage {
    fn default() -> Self {
        TokenStorage {
            balances: LookupMap::new(b"b".to_vec()),
            allowances: LookupMap::new(b"a".to_vec()),
        }
    }
}

#[near_bindgen]
#[derive(Default,BorshDeserialize, BorshSerialize)]
pub struct Token {
    // token name
    pub name: String,
    // token symbol
    pub symbol: String,
    // token decimals
    pub decimals: u8,
    // total supply of tokens
    pub total_supply: u128,
    // storage for account balances and allowances
    pub storage: TokenStorage,
}

#[near_bindgen]
impl Token {
    // Initializes the contract with the given name, symbol, decimals and total supply
    #[init]
    pub fn new(name: String, symbol: String, decimals: u8, total_supply: u128) -> Self {
        let mut storage = TokenStorage::default();
        let account_id = env::predecessor_account_id();
        storage.balances.insert(&account_id, &total_supply);
        Self {
            name,
            symbol,
            decimals,
            total_supply,
            storage,
        }
    }

    // Transfers tokens from the sender to the recipient
    pub fn transfer(&mut self, recipient_id: AccountId, amount: u128) -> bool {
        let sender_id = env::predecessor_account_id();
        let sender_balance = self.storage.balances.get(&sender_id).unwrap_or(0);
        assert!(
            sender_balance >= amount,
            "Not enough balance to transfer."
        );
        self.storage.balances.insert(&sender_id, &(sender_balance - amount));
        let recipient_balance = self.storage.balances.get(&recipient_id).unwrap_or(0);
        self.storage.balances.insert(&recipient_id, &(recipient_balance + amount));
        true
    }

    // Returns the balance of the account with the given account ID
    pub fn balance_of(&self, account_id: AccountId) -> u128 {
        self.storage.balances.get(&account_id).unwrap_or(0)
    }

    // Approves a spender to transfer a certain amount of tokens from the sender's account
    pub fn approve(&mut self, spender_id: AccountId, amount: u128) {
        let owner_id = env::predecessor_account_id();
        self.storage
            .allowances
            .insert(&(owner_id, spender_id), &amount);
    }

    // Returns the amount of tokens that the spender is allowed to transfer from the owner's account
    pub fn allowance(&self, owner_id: AccountId, spender_id: AccountId) -> u128 {
        self.storage.allowances.get(&(owner_id, spender_id)).unwrap_or(0)
    }
}
