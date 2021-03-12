#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nft {
    use ink_storage::collections::HashMap;

    #[ink(storage)]
    pub struct NFT {
        owner: AccountId,
        total_minted: u64,
        owners: HashMap<u64, AccountId>,
        counts: HashMap<AccountId, u64>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        UnauthorizedAction,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct Minted {
        amount: u64,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transferred {
        id: u64,
        from: AccountId,
        to: AccountId,
    }

    impl NFT {
        #[ink(constructor)]
        pub fn new(init_value: u64) -> Self {
            let mut ret = Self {
                owner: Self::env().caller(),
                total_minted: 0,
                owners: Default::default(),
                counts: Default::default(),
            };

            ret.mint(init_value);

            ret
        }

        fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u64 {
            panic!("not implemented")
        }

        #[ink(message)]
        pub fn owner_of(&self, id: u64) -> Option<AccountId> {
            panic!("not implemented")
        }

        #[ink(message)]
        pub fn safe_transfer_from(&mut self, to: AccountId, id: u64) -> Result<()> {
            panic!("not implemented")
        }

        #[ink(message)]
        pub fn approve(&mut self, approved: AccountId, id: u64) {}

        #[ink(message)]
        pub fn approve_all(&mut self, approved: AccountId) {
            panic!("not implemented");
        }

        #[ink(message)]
        pub fn get_approved(&self, id: AccountId) -> Option<AccountId> {
            panic!("not implemented");
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, address: AccountId, operator: AccountId) -> bool {
            panic!("not implemented");
        }

        #[ink(message)]
        pub fn mint(&mut self, amount: u64) -> Result<()> {
            if !self.is_owner() {
                return Err(Error::UnauthorizedAction);
            }

            let owner = self.env().caller();

            let current_owner_count = *self.counts.get(&owner).unwrap_or(&0);

            for i in 0..amount {
                self.owners.insert(self.total_minted + i, owner);
            }
            self.total_minted += amount;
            self.counts.insert(owner, current_owner_count + amount);

            self.env().emit_event(Minted { amount, owner });

            Ok(())
        }

        #[ink(message)]
        pub fn get_total_minted(&mut self) -> u64 {
            self.total_minted
        }

        fn is_owner(&self) -> bool {
            self.env().caller() == self.owner
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_env::test::{default_accounts, push_execution_context};
        use ink_env::{self, DefaultEnvironment};
        use ink_lang as ink;

        #[ink::test]
        fn default_total_amount() {
            let nft = NFT::default();

            assert_eq!(nft.total_minted, 0);
        }

        #[ink::test]
        fn default_owner() {
            let nft = NFT::default();

            let accounts = default_accounts::<DefaultEnvironment>().unwrap();

            assert_eq!(nft.owner, accounts.alice);
        }

        #[ink::test]
        fn initial_mint() {
            let nft = NFT::new(10);

            let accounts = default_accounts::<DefaultEnvironment>().unwrap();

            assert_eq!(nft.total_minted, 10);
            for i in 0..10 {
                assert_eq!(nft.owners.get(&i), Some(accounts.alice).as_ref());
            }
        }

        #[ink::test]
        fn mint_by_someone_else_fails() {
            let mut nft = NFT::new(0);

            let accounts = default_accounts::<DefaultEnvironment>().unwrap();

            // Get contract address.
            let callee =
                ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());

            // Create call.
            let mut data = ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); // balance_of

            data.push_arg(&accounts.bob);

            push_execution_context::<DefaultEnvironment>(
                accounts.bob,
                callee,
                1000000,
                1000000,
                data,
            );

            let result = nft.mint(1);

            assert_eq!(result, Err(Error::UnauthorizedAction));
        }
    }
}
