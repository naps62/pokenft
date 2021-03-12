#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nft {
    use ink_storage::collections::{hashmap::Entry, HashMap};

    #[ink(storage)]
    pub struct NFT {
        owners: HashMap<TokenId, AccountId>,
        counts: HashMap<AccountId, TokenId>,
        approved: HashMap<TokenId, AccountId>,
        operators: HashMap<(AccountId, AccountId), bool>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotAllowed,
        InvalidAddress,
        TokenNotFound,
        TokenAlreadyExists,
        ValueNotFound,
        CannotRemove,
    }

    pub type TokenId = u64;
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct Minted {
        amount: TokenId,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transfer {
        id: TokenId,
        from: Option<AccountId>,
        to: Option<AccountId>,
    }

    #[ink(event)]
    pub struct Approval {
        id: TokenId,
        account: Option<AccountId>,
    }

    impl NFT {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owners: Default::default(),
                approved: Default::default(),
                counts: Default::default(),
                operators: Default::default(),
            }
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> TokenId {
            *self.counts.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<AccountId> {
            self.owners.get(&id).cloned()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            self.impl_transfer_from(&caller, &to, id)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, id: TokenId) -> Result<()> {
            self.impl_transfer_from(&from, &to, id)
        }

        #[ink(message)]
        pub fn approve(&mut self, approved: AccountId, id: TokenId) -> Result<()> {
            self.assert_exists(id)?;
            self.assert_valid_account(&approved)?;
            self.assert_owner_or_approved(id)?;

            self.approved.insert(id, approved);

            self.env().emit_event(Approval {
                id,
                account: Some(approved),
            });

            Ok(())
        }

        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approval: bool) -> Result<()> {
            self.assert_valid_account(&operator)?;

            let caller = self.env().caller();

            if operator == self.env().caller() {
                return Err(Error::NotAllowed);
            }

            self.operators
                .entry((caller, operator))
                .and_modify(|v| *v = approval)
                .or_insert(approval);

            Ok(())
        }

        #[ink(message)]
        pub fn clear_approval(&mut self, id: TokenId) -> Result<()> {
            self.assert_exists(id)?;
            self.assert_owner_or_approved(id)?;

            if !self.approved.contains_key(&id) {
                return Ok(());
            }

            match self.approved.take(&id) {
                Some(_) => {
                    self.env().emit_event(Approval { id, account: None });
                    Ok(())
                }
                None => Err(Error::CannotRemove),
            }
        }

        #[ink(message)]
        pub fn get_approved(&self, id: TokenId) -> Option<AccountId> {
            self.approved.get(&id).cloned()
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool {
            *self.operators.get(&(account, operator)).unwrap_or(&false)
        }

        #[ink(message)]
        pub fn mint(&mut self, id: TokenId) -> Result<()> {
            let owner = self.env().caller();

            self.add_token_to(&owner, id).unwrap();

            self.env().emit_event(Transfer {
                from: None,
                to: Some(owner),
                id,
            });

            Ok(())
        }

        fn assert_exists(&self, id: TokenId) -> Result<()> {
            if !self.exists(id) {
                return Err(Error::TokenNotFound);
            }

            Ok(())
        }

        fn assert_valid_account(&self, account: &AccountId) -> Result<()> {
            if *account == AccountId::from([0x0; 32]) {
                return Err(Error::InvalidAddress);
            }

            Ok(())
        }

        fn assert_owner_or_approved(&self, id: TokenId) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.owner_of(id);
            let current_approver = self.approved.get(&id);

            if !(owner == Some(caller)
                || current_approver == Some(&caller)
                || self.is_approved_for_all(owner.unwrap(), caller))
            {
                return Err(Error::NotAllowed);
            }

            Ok(())
        }

        fn exists(&self, id: TokenId) -> bool {
            self.owners.contains_key(&id)
        }

        fn impl_transfer_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            id: TokenId,
        ) -> Result<()> {
            self.assert_exists(id)?;
            self.assert_owner_or_approved(id)?;
            self.assert_valid_account(to)?;

            self.clear_approval(id)?;
            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                id,
            });

            Ok(())
        }

        fn remove_token_from(&mut self, from: &AccountId, id: TokenId) -> Result<()> {
            let entry = match self.owners.entry(id) {
                Entry::Vacant(_) => return Err(Error::TokenNotFound),
                Entry::Occupied(entry) => entry,
            };

            entry.remove_entry();
            self.decrease_count(from)?;

            Ok(())
        }

        fn add_token_to(&mut self, to: &AccountId, id: TokenId) -> Result<()> {
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

        use ink_env::test::{default_accounts, push_execution_context, recorded_events};
        use ink_env::{self, DefaultEnvironment};
        use ink_lang as ink;

        macro_rules! zero_account {
            () => {
                AccountId::from([0x0; 32])
            };
        }

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

        macro_rules! charlie {
            () => {
                default_accounts::<DefaultEnvironment>().unwrap().charlie
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

        fn get_event(idx: usize) -> Event {
            let raw_event = recorded_events()
                .nth(idx)
                .expect(&format!("No event found"));

            <Event as scale::Decode>::decode(&mut &raw_event.data[..])
                .expect("Invalid contract Event")
        }

        fn last_event() -> Event {
            get_event(recorded_events().count() - 1)
        }

        #[ink::test]
        fn mint() {
            let mut nft = NFT::new();
            nft.mint(10).unwrap();

            assert_eq!(nft.owners.get(&10), Some(alice!()).as_ref());

            if let Event::Transfer(Transfer { from, to, id }) = last_event() {
                assert_eq!(from, None);
                assert_eq!(to, Some(alice!()));
                assert_eq!(id, 10);
            } else {
                panic!("Expected to find Transfer event");
            };
        }

        #[ink::test]
        fn transfer() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();

            nft.transfer(bob!(), 0).unwrap();

            assert_eq!(nft.owner_of(0), Some(bob!()));

            if let Event::Transfer(Transfer { from, to, id }) = last_event() {
                assert_eq!(from, Some(alice!()));
                assert_eq!(to, Some(bob!()));
                assert_eq!(id, 0);
            } else {
                panic!("Expected to find Transfer event");
            };
        }

        #[ink::test]
        fn transfer_someone_elses_token() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();

            use_account!(bob!());

            let result = nft.transfer(alice!(), 0);

            assert_eq!(result, Err(Error::NotAllowed));
        }

        #[ink::test]
        fn approved_transfers_from() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();
            nft.approve(bob!(), 0).unwrap();

            use_account!(bob!());
            let result = nft.transfer_from(alice!(), charlie!(), 0);

            assert_eq!(result, Ok(()));
            assert_eq!(nft.owner_of(0).unwrap(), charlie!());
        }

        #[ink::test]
        fn unapproved_transfers_from() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();

            use_account!(bob!());
            let result = nft.transfer_from(alice!(), charlie!(), 0);

            assert_eq!(result, Err(Error::NotAllowed));
            assert_eq!(nft.owner_of(0).unwrap(), alice!());
        }

        #[ink::test]
        fn transfer_non_existing_token() {
            let mut nft = NFT::new();

            let result = nft.transfer(bob!(), 0);

            assert_eq!(result, Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn transfer_removes_approval() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();
            nft.approve(bob!(), 0).unwrap();

            nft.transfer(charlie!(), 0).unwrap();

            assert_eq!(nft.get_approved(0), None);
        }

        #[ink::test]
        fn balance_of() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();
            nft.mint(1).unwrap();

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
            nft.mint(0).unwrap();
            nft.mint(1).unwrap();

            assert_eq!(nft.owner_of(0), Some(alice!()));
            assert_eq!(nft.owner_of(1), Some(alice!()));
            assert_eq!(nft.owner_of(2), None);

            nft.transfer(bob!(), 0).unwrap();

            assert_eq!(nft.owner_of(0), Some(bob!()));
            assert_eq!(nft.owner_of(1), Some(alice!()));
            assert_eq!(nft.owner_of(2), None);
        }

        type Event = <NFT as ::ink_lang::BaseEvent>::Type;

        #[ink::test]
        fn approved() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();

            assert_eq!(nft.get_approved(0), None);

            nft.approve(bob!(), 0).unwrap();

            assert_eq!(nft.get_approved(0), Some(bob!()));

            if let Event::Approval(Approval { id, account }) = get_event(1) {
                assert_eq!(id, 0);
                assert_eq!(account, Some(bob!()));
            } else {
                panic!("Expected last event to be an Approval");
            }
        }

        #[ink::test]
        fn approve_0_address() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();

            assert_eq!(nft.approve(zero_account!(), 0), Err(Error::InvalidAddress));
        }

        #[ink::test]
        fn approve_while_not_owner() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();

            use_account!(bob!());

            assert_eq!(nft.approve(bob!(), 0), Err(Error::NotAllowed));
        }

        #[ink::test]
        fn approve_non_existing_token() {
            let mut nft = NFT::new();
            assert_eq!(nft.get_approved(0), None);

            let result = nft.approve(bob!(), 0);

            assert_eq!(result, Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn approve_while_approved() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();
            nft.approve(bob!(), 0).unwrap();

            use_account!(bob!());
            let result = nft.approve(alice!(), 0);

            assert_eq!(result, Ok(()));
        }

        #[ink::test]
        fn clear_approval() {
            let mut nft = NFT::new();

            nft.mint(0).unwrap();

            nft.approve(bob!(), 0).unwrap();
            assert_eq!(nft.get_approved(0), Some(bob!()));

            nft.clear_approval(0).unwrap();

            assert_eq!(nft.get_approved(0), None);
        }

        #[ink::test]
        fn approver_removes_itself() {
            let mut nft = NFT::new();

            nft.mint(0).unwrap();

            nft.approve(bob!(), 0).unwrap();
            assert_eq!(nft.get_approved(0), Some(bob!()));

            use_account!(bob!());
            nft.clear_approval(0).unwrap();

            assert_eq!(nft.get_approved(0), None);
        }

        #[ink::test]
        fn other_user_fails_to_remove_approver() {
            let mut nft = NFT::new();

            nft.mint(0).unwrap();

            nft.approve(bob!(), 0).unwrap();
            assert_eq!(nft.get_approved(0), Some(bob!()));

            use_account!(charlie!());
            let result = nft.clear_approval(0);

            assert_eq!(result, Err(Error::NotAllowed));
            assert_eq!(nft.get_approved(0), Some(bob!()));
        }

        #[ink::test]
        fn approved_for_all() {
            let mut nft = NFT::new();

            assert_eq!(nft.is_approved_for_all(alice!(), bob!()), false);

            nft.set_approval_for_all(bob!(), true).unwrap();

            assert_eq!(nft.is_approved_for_all(alice!(), bob!()), true);
        }

        #[ink::test]
        fn operator_can_transfer_from_owner() {
            let mut nft = NFT::new();
            nft.mint(0).unwrap();
            nft.set_approval_for_all(bob!(), true).unwrap();

            use_account!(bob!());
            nft.transfer_from(alice!(), bob!(), 0).unwrap();
        }
    }
}
