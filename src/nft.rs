use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

#[near_bindgen]
#[derive(Default, PanicOnDefault)]
pub struct NFT {
    owner_id: AccountId,
    token_id: u64,
    metadata: String,
    approved_account_ids: Vec<AccountId>,
    token_owner: Option<AccountId>,
}

#[near_bindgen]
impl NFT {
    #[init]
    pub fn new(owner_id: AccountId, token_id: u64, metadata: String) -> Self {
        Self {
            owner_id,
            token_id,
            metadata,
            approved_account_ids: Vec::new(),
            token_owner: None,
        }
    }

    pub fn get_metadata(&self) -> String {
        self.metadata.clone()
    }

    pub fn transfer(&mut self, new_owner_id: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.token_owner.unwrap(),
            "Only the token owner can transfer the token"
        );
        self.token_owner = Some(new_owner_id);
    }

    pub fn approve(&mut self, account_id: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.token_owner.unwrap(),
            "Only the token owner can approve another account to transfer the token"
        );
        self.approved_account_ids.push(account_id);
    }

    pub fn get_approved_account_ids(&self) -> Vec<AccountId> {
        self.approved_account_ids.clone()
    }

    pub fn mint(&mut self, account_id: AccountId) -> Promise {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the contract owner can mint new tokens"
        );
        self.token_owner = Some(account_id);
        Promise::new(account_id).transfer(env::attached_deposit())
    }

    pub fn burn(&mut self) -> Promise {
        assert_eq!(
            env::predecessor_account_id(),
            self.token_owner.unwrap(),
            "Only the token owner can burn the token"
        );
        self.token_owner = None;
        Promise::new(env::predecessor_account_id()).transfer(env::account_balance())
    }
}

