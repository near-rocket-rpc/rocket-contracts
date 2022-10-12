use events::Event;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env,
    json_types::U128,
    near_bindgen, require, AccountId, Balance, PanicOnDefault, PromiseOrValue,
};

mod events;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Escrow {
    manager_account_id: AccountId,
    balances: UnorderedMap<AccountId, Balance>,
}

#[near_bindgen]
impl Escrow {
    #[init]
    pub fn new(manager_account_id: AccountId) -> Self {
        Self {
            manager_account_id,
            balances: UnorderedMap::new(b"b"),
        }
    }

    pub fn get_balance(&self, account_id: AccountId) -> U128 {
        U128(self.balances.get(&account_id).unwrap_or(0))
    }

    pub fn charge(&mut self, account_id: AccountId, amount: U128) {
        require!(
            env::predecessor_account_id() == self.manager_account_id,
            "Only manager can call charge"
        );

        let balance = self.balances.get(&account_id).unwrap_or(0);
        require!(balance >= amount.0, "No enough balance");

        self.balances.insert(&account_id, &(balance - amount.0));

        Event::Charge {
            account_id: &account_id,
            amount: &amount,
        }
        .emit();
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Escrow {
    fn ft_on_transfer(
        &mut self,
        sender: AccountId,
        amount: U128,
        _msg: String,
    ) -> PromiseOrValue<U128> {
        let balance = self.balances.get(&sender).unwrap_or(0);
        let new_balance = balance + amount.0;
        self.balances.insert(&sender, &new_balance);

        Event::Deposit {
            account_id: &sender,
            amount: &amount,
        }
        .emit();

        PromiseOrValue::Value(U128(0))
    }
}
