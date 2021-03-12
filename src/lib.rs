#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nft {
    use ink_storage::collections::{hashmap::Entry, HashMap};

    #[ink(storage)]
    pub struct NFT {
        total_minted: u64,
        owners: HashMap<u64, AccountId>,
        counts: HashMap<AccountId, u64>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotAllowed,
        TokenNotFound,
        TokenAlreadyExists,
        ValueNotFound,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct Minted {
        amount: u64,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transfer {
        id: u64,
        from: Option<AccountId>,
        to: Option<AccountId>,
    }

    impl NFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                total_minted: 0,
                owners: Default::default(),
                counts: Default::default(),
            }
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u64 {
            *self.counts.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn owner_of(&self, id: u64) -> Option<AccountId> {
            self.owners.get(&id).cloned()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, id: u64) -> Result<()> {
            let caller = self.env().caller();
            self.impl_transfer_from(&caller, &to, id)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, _from: AccountId, _to: AccountId, _id: u64) -> Result<()> {
            panic!("not yet implemented")
        }

        #[ink(message)]
        pub fn approve(&mut self, _approved: AccountId, _id: u64) {
            panic!("not implemented");
        }

        #[ink(message)]
        pub fn approve_all(&mut self, _approved: AccountId) {
            panic!("not implemented");
        }

        #[ink(message)]
        pub fn get_approved(&self, _id: AccountId) -> Option<AccountId> {
            panic!("not implemented");
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, _address: AccountId, _operator: AccountId) -> bool {
            panic!("not implemented");
        }

        #[ink(message)]
        pub fn mint(&mut self, amount: u64) -> Result<()> {
            let owner = self.env().caller();

            for i in 0..amount {
                self.owners.insert(self.total_minted + i, owner);
                self.increase_count(&owner)?;
            }
            self.total_minted += amount;

            self.env().emit_event(Minted { amount, owner });

            Ok(())
        }

        #[ink(message)]
        pub fn get_total_minted(&mut self) -> u64 {
            self.total_minted
        }

        fn exists(&self, id: u64) -> bool {
            self.owners.contains_key(&id)
        }

        fn impl_transfer_from(&mut self, from: &AccountId, to: &AccountId, id: u64) -> Result<()> {
            if !self.exists(id) {
                return Err(Error::TokenNotFound);
            }

            if self.owners.get(&id) != Some(from) {
                return Err(Error::NotAllowed);
            }

            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                id,
            });

            Ok(())
        }

        fn remove_token_from(&mut self, from: &AccountId, id: u64) -> Result<()> {
            let entry = match self.owners.entry(id) {
                Entry::Vacant(_) => return Err(Error::TokenNotFound),
                Entry::Occupied(entry) => entry,
            };

            entry.remove_entry();
            self.decrease_count(from)?;

            Ok(())
        }

        fn add_token_to(&mut self, to: &AccountId, id: u64) -> Result<()> {
            let entry = match self.owners.entry(id) {
                Entry::Vacant(entry) => entry,
                Entry::Occupied(_) => return Err(Error::TokenAlreadyExists),
            };

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed);
            }

            entry.insert(*to);
            self.increase_count(to)?;

            Ok(())
        }

        fn decrease_count(&mut self, account: &AccountId) -> Result<()> {
            let count = self.counts.get_mut(account).ok_or(Error::ValueNotFound)?;
            *count -= 1;

            Ok(())
        }

        fn increase_count(&mut self, account: &AccountId) -> Result<()> {
            self.counts
                .entry(*account)
                .and_modify(|v| *v += 1)
                .or_insert(1);

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_env::test::{default_accounts, push_execution_context};
        use ink_env::{self, DefaultEnvironment};
        use ink_lang as ink;

        macro_rules! alice {
            () => {
                default_accounts::<DefaultEnvironment>().unwrap().alice
            };
        }

        macro_rules! bob {
            () => {
                default_accounts::<DefaultEnvironment>().unwrap().bob
            };
        }

        macro_rules! use_account {
            ($account:expr) => {
                let mut data =
                    ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4]));
                data.push_arg(&$account);

                push_execution_context::<DefaultEnvironment>(
                    $account,
                    ink_env::account_id::<ink_env::DefaultEnvironment>()
                        .unwrap_or([0x0; 32].into()),
                    1000000,
                    1000000,
                    data,
                );
            };
        }

        #[ink::test]
        fn default_total_amount() {
            let nft = NFT::new();

            assert_eq!(nft.total_minted, 0);
        }

        #[ink::test]
        fn mint() {
            let mut nft = NFT::new();
            nft.mint(10).unwrap();

            assert_eq!(nft.total_minted, 10);
            for i in 0..10 {
                assert_eq!(nft.owners.get(&i), Some(alice!()).as_ref());
            }
        }

        #[ink::test]
        fn transfer_from() {
            let mut nft = NFT::new();
            nft.mint(1).unwrap();

            nft.transfer(bob!(), 0).unwrap();

            assert_eq!(nft.owner_of(0), Some(bob!()));
        }

        #[ink::test]
        fn transfer_someone_elses_token() {
            let mut nft = NFT::new();
            nft.mint(1).unwrap();

            use_account!(bob!());

            let result = nft.transfer(alice!(), 0);

            assert_eq!(result, Err(Error::NotAllowed));
        }

        #[ink::test]
        fn transfer_non_existing_token() {
            let mut nft = NFT::new();

            let result = nft.transfer(bob!(), 0);

            assert_eq!(result, Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn balance_of() {
            let mut nft = NFT::new();
            nft.mint(2).unwrap();

            assert_eq!(nft.balance_of(alice!()), 2);
            assert_eq!(nft.balance_of(bob!()), 0);

            nft.transfer(bob!(), 0).unwrap();

            assert_eq!(nft.balance_of(alice!()), 1);
            assert_eq!(nft.balance_of(bob!()), 1);

            use_account!(bob!());

            nft.transfer(alice!(), 0).unwrap();

            assert_eq!(nft.balance_of(alice!()), 2);
            assert_eq!(nft.balance_of(bob!()), 0);
        }

        #[ink::test]
        fn owner_of() {
            let mut nft = NFT::new();
            nft.mint(2).unwrap();

            assert_eq!(nft.owner_of(0), Some(alice!()));
            assert_eq!(nft.owner_of(1), Some(alice!()));
            assert_eq!(nft.owner_of(2), None);

            nft.transfer(bob!(), 0).unwrap();

            assert_eq!(nft.owner_of(0), Some(bob!()));
            assert_eq!(nft.owner_of(1), Some(alice!()));
            assert_eq!(nft.owner_of(2), None);
        }
    }
}
