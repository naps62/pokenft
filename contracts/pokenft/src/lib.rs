#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod pokenft {
    // use crate::rng;
    use ink_storage::collections::{hashmap::Entry, HashMap};

    #[ink(storage)]
    pub struct PokeNFT {
        owners: HashMap<Seed, (AccountId, PokemonId)>,
        counts: HashMap<AccountId, u32>,
        approved: HashMap<Seed, AccountId>,
        operators: HashMap<(AccountId, AccountId), bool>,
    }

    #[derive(Debug, scale::Encode, scale::Decode, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InvalidSeed,
        NotOwner,
        NotAllowed,
        InvalidAddress,
        TokenNotFound,
        TokenAlreadyExists,
        ValueNotFound,
        CannotRemove,
    }

    pub type Seed = [u8; 32];
    pub type PokemonId = u32;
    pub type Result<T> = core::result::Result<T, Error>;

    #[ink(event)]
    pub struct Minted {
        amount: PokemonId,
        owner: AccountId,
    }

    #[ink(event)]
    pub struct Transfer {
        seed: Seed,
        from: Option<AccountId>,
        to: Option<AccountId>,
    }

    #[ink(event)]
    pub struct Approval {
        seed: Seed,
        account: Option<AccountId>,
    }

    mod rng {
        use rand::prelude::*;
        use rand_chacha::ChaChaRng;

        // indexes are pokemon IDs. values are weights within entire population.
        // e.g. #0 is Bulbasaur, which has 376 specimen within a total population of 10 million
        pub const POKEMON_LIST: [u32; 151] = [
            376, 251, 188, 376, 251, 188, 376, 251, 188, 187967, 9398, 4699, 187967, 9398, 4699,
            187967, 187967, 187967, 187967, 187967, 187967, 187967, 187967, 187967, 376, 342,
            187967, 187967, 187967, 187967, 187967, 187967, 187967, 31328, 93983, 23496, 37593,
            18797, 187967, 18797, 187967, 187967, 187967, 93983, 187967, 187967, 93983, 187967,
            93983, 187967, 18797, 187967, 93983, 187967, 187967, 187967, 37593, 18797, 9398, 37593,
            18797, 9398, 37593, 18797, 9398, 37593, 18797, 9398, 37593, 18797, 9398, 187967, 93983,
            187967, 93983, 46992, 46992, 31328, 187967, 93983, 187967, 62656, 62656, 187967, 62656,
            187967, 62656, 187967, 62656, 31328, 18797, 37593, 18797, 9398, 187967, 187967, 46992,
            187967, 62656, 187967, 46992, 23496, 18797, 46992, 26852, 6266, 6266, 9398, 18797,
            9398, 18797, 9398, 9398, 9398, 9398, 7519, 4699, 7519, 4699, 7519, 4699, 3759, 3759,
            3759, 3759, 3759, 3759, 4699, 3759, 3759, 3759, 3759, 3759, 1880, 1880, 1880, 1880,
            1213, 964, 1213, 964, 964, 964, 188, 188, 188, 627, 470, 372, 31, 20,
        ];

        pub fn sample(seed: super::Seed) -> super::Result<u32> {
            let mut rng = ChaChaRng::from_seed(seed);
            let mut r: u32 = rng.gen_range(0..10_000_000);

            let mut result = 0;
            for idx in 0..POKEMON_LIST.len() {
                let population = POKEMON_LIST[idx];

                if population >= r {
                    result = idx + 1;
                    break;
                } else {
                    r = r.saturating_sub(population);
                }
            }

            Ok(result as u32)
        }
    }

    impl PokeNFT {
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
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            *self.counts.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn owner_of(&self, seed: Seed) -> Option<AccountId> {
            self.owners.get(&seed).cloned().map(|(account, _)| account)
        }

        #[ink(message)]
        pub fn pokemon_of(&self, seed: Seed) -> Option<PokemonId> {
            self.owners.get(&seed).cloned().map(|(_, pokemon)| pokemon)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, seed: Seed) -> Result<()> {
            let caller = self.env().caller();
            self.impl_transfer_from(&caller, &to, seed)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, seed: Seed) -> Result<()> {
            self.impl_transfer_from(&from, &to, seed)
        }

        #[ink(message)]
        pub fn approve(&mut self, approved: AccountId, seed: Seed) -> Result<()> {
            self.assert_exists(seed)?;
            self.assert_valid_account(&approved)?;
            self.assert_owner_or_approved(seed)?;

            self.approved.insert(seed, approved);

            self.env().emit_event(Approval {
                seed,
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
        pub fn clear_approval(&mut self, seed: Seed) -> Result<()> {
            self.assert_exists(seed)?;
            self.assert_owner_or_approved(seed)?;

            if !self.approved.contains_key(&seed) {
                return Ok(());
            }

            match self.approved.take(&seed) {
                Some(_) => {
                    self.env().emit_event(Approval {
                        seed,
                        account: None,
                    });
                    Ok(())
                }
                None => Err(Error::CannotRemove),
            }
        }

        #[ink(message)]
        pub fn get_approved(&self, seed: Seed) -> Option<AccountId> {
            self.approved.get(&seed).cloned()
        }

        #[ink(message)]
        pub fn is_approved_for_all(&self, account: AccountId, operator: AccountId) -> bool {
            *self.operators.get(&(account, operator)).unwrap_or(&false)
        }

        #[ink(message)]
        pub fn mint(&mut self, seed: Seed) -> Result<()> {
            let owner = self.env().caller();

            self.add_token_to(&owner, seed)?;

            self.env().emit_event(Transfer {
                from: None,
                to: Some(owner),
                seed,
            });

            Ok(())
        }

        fn assert_exists(&self, seed: Seed) -> Result<()> {
            if !self.exists(seed) {
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

        fn assert_owner_or_approved(&self, seed: Seed) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.owner_of(seed);
            let current_approver = self.approved.get(&seed);

            if !(owner == Some(caller)
                || current_approver == Some(&caller)
                || self.is_approved_for_all(owner.unwrap(), caller))
            {
                return Err(Error::NotAllowed);
            }

            Ok(())
        }

        fn exists(&self, seed: Seed) -> bool {
            self.owners.contains_key(&seed)
        }

        fn impl_transfer_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            seed: Seed,
        ) -> Result<()> {
            self.assert_exists(seed)?;
            self.assert_owner_or_approved(seed)?;
            self.assert_valid_account(to)?;

            self.clear_approval(seed)?;
            self.remove_token_from(from, seed)?;
            self.add_token_to(to, seed)?;

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                seed,
            });

            Ok(())
        }

        fn remove_token_from(&mut self, from: &AccountId, seed: Seed) -> Result<()> {
            let entry = match self.owners.entry(seed) {
                Entry::Vacant(_) => return Err(Error::TokenNotFound),
                Entry::Occupied(entry) => entry,
            };

            entry.remove_entry();
            self.decrease_count(from)?;

            Ok(())
        }

        fn add_token_to(&mut self, to: &AccountId, seed: Seed) -> Result<()> {
            let id = rng::sample(seed).map_err(|_| Error::InvalidSeed)?;

            let entry = match self.owners.entry(seed) {
                Entry::Vacant(entry) => entry,
                Entry::Occupied(_) => return Err(Error::TokenAlreadyExists),
            };

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed);
            }

            entry.insert((*to, id));
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

        macro_rules! seed {
            ($seed:expr) => {
                [$seed; 32]
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
            let mut nft = PokeNFT::new();
            nft.mint(seed!(10)).unwrap();

            assert_eq!(nft.owners.get(&seed!(10)), Some(&(alice!(), 16)));

            if let Event::Transfer(Transfer { from, to, seed }) = last_event() {
                assert_eq!(from, None);
                assert_eq!(to, Some(alice!()));
                assert_eq!(seed, seed!(10));
            } else {
                panic!("Expected to find Transfer event");
            };
        }

        #[ink::test]
        fn transfer() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();

            nft.transfer(bob!(), seed!(0)).unwrap();

            assert_eq!(nft.owner_of(seed!(0)), Some(bob!()));

            if let Event::Transfer(Transfer { from, to, seed }) = last_event() {
                assert_eq!(from, Some(alice!()));
                assert_eq!(to, Some(bob!()));
                assert_eq!(seed, seed!(0));
            } else {
                panic!("Expected to find Transfer event");
            };
        }

        #[ink::test]
        fn transfer_someone_elses_token() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();

            use_account!(bob!());

            let result = nft.transfer(alice!(), seed!(0));

            assert_eq!(result, Err(Error::NotAllowed));
        }

        #[ink::test]
        fn approved_transfers_from() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();
            nft.approve(bob!(), seed!(0)).unwrap();

            use_account!(bob!());
            let result = nft.transfer_from(alice!(), charlie!(), seed!(0));

            assert_eq!(result, Ok(()));
            assert_eq!(nft.owner_of(seed!(0)).unwrap(), charlie!());
        }

        #[ink::test]
        fn unapproved_transfers_from() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();

            use_account!(bob!());
            let result = nft.transfer_from(alice!(), charlie!(), seed!(0));

            assert_eq!(result, Err(Error::NotAllowed));
            assert_eq!(nft.owner_of(seed!(0)).unwrap(), alice!());
        }

        #[ink::test]
        fn transfer_non_existing_token() {
            let mut nft = PokeNFT::new();

            let result = nft.transfer(bob!(), seed!(0));

            assert_eq!(result, Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn transfer_removes_approval() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();
            nft.approve(bob!(), seed!(0)).unwrap();

            nft.transfer(charlie!(), seed!(0)).unwrap();

            assert_eq!(nft.get_approved(seed!(0)), None);
        }

        #[ink::test]
        fn balance_of() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();
            nft.mint(seed!(1)).unwrap();

            assert_eq!(nft.balance_of(alice!()), 2);
            assert_eq!(nft.balance_of(bob!()), 0);

            nft.transfer(bob!(), seed!(0)).unwrap();

            assert_eq!(nft.balance_of(alice!()), 1);
            assert_eq!(nft.balance_of(bob!()), 1);

            use_account!(bob!());

            nft.transfer(alice!(), seed!(0)).unwrap();

            assert_eq!(nft.balance_of(alice!()), 2);
            assert_eq!(nft.balance_of(bob!()), 0);
        }

        #[ink::test]
        fn owner_of() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();
            nft.mint(seed!(1)).unwrap();

            assert_eq!(nft.owner_of(seed!(0)), Some(alice!()));
            assert_eq!(nft.owner_of(seed!(1)), Some(alice!()));
            assert_eq!(nft.owner_of(seed!(2)), None);

            nft.transfer(bob!(), seed!(0)).unwrap();

            assert_eq!(nft.owner_of(seed!(0)), Some(bob!()));
            assert_eq!(nft.owner_of(seed!(1)), Some(alice!()));
            assert_eq!(nft.owner_of(seed!(2)), None);
        }

        #[ink::test]
        fn pokemon_of() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();
            nft.mint(seed!(1)).unwrap();

            assert_eq!(nft.pokemon_of(seed!(0)), Some(72));
            assert_eq!(nft.pokemon_of(seed!(1)), Some(19));
            assert_eq!(nft.pokemon_of(seed!(2)), None);
        }

        type Event = <PokeNFT as ::ink_lang::BaseEvent>::Type;

        #[ink::test]
        fn approved() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();

            assert_eq!(nft.get_approved(seed!(0)), None);

            nft.approve(bob!(), seed!(0)).unwrap();

            assert_eq!(nft.get_approved(seed!(0)), Some(bob!()));

            if let Event::Approval(Approval { seed, account }) = get_event(1) {
                assert_eq!(seed, seed!(0));
                assert_eq!(account, Some(bob!()));
            } else {
                panic!("Expected last event to be an Approval");
            }
        }

        #[ink::test]
        fn approve_zero_address() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();

            assert_eq!(
                nft.approve(zero_account!(), seed!(0)),
                Err(Error::InvalidAddress)
            );
        }

        #[ink::test]
        fn approve_while_not_owner() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();

            use_account!(bob!());

            assert_eq!(nft.approve(bob!(), seed!(0)), Err(Error::NotAllowed));
        }

        #[ink::test]
        fn approve_non_existing_token() {
            let mut nft = PokeNFT::new();
            assert_eq!(nft.get_approved(seed!(0)), None);

            let result = nft.approve(bob!(), seed!(0));

            assert_eq!(result, Err(Error::TokenNotFound));
        }

        #[ink::test]
        fn approve_while_approved() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();
            nft.approve(bob!(), seed!(0)).unwrap();

            use_account!(bob!());
            let result = nft.approve(alice!(), seed!(0));

            assert_eq!(result, Ok(()));
        }

        #[ink::test]
        fn clear_approval() {
            let mut nft = PokeNFT::new();

            nft.mint(seed!(0)).unwrap();

            nft.approve(bob!(), seed!(0)).unwrap();
            assert_eq!(nft.get_approved(seed!(0)), Some(bob!()));

            nft.clear_approval(seed!(0)).unwrap();

            assert_eq!(nft.get_approved(seed!(0)), None);
        }

        #[ink::test]
        fn approver_removes_itself() {
            let mut nft = PokeNFT::new();

            nft.mint(seed!(0)).unwrap();

            nft.approve(bob!(), seed!(0)).unwrap();
            assert_eq!(nft.get_approved(seed!(0)), Some(bob!()));

            use_account!(bob!());
            nft.clear_approval(seed!(0)).unwrap();

            assert_eq!(nft.get_approved(seed!(0)), None);
        }

        #[ink::test]
        fn other_user_fails_to_remove_approver() {
            let mut nft = PokeNFT::new();

            nft.mint(seed!(0)).unwrap();

            nft.approve(bob!(), seed!(0)).unwrap();
            assert_eq!(nft.get_approved(seed!(0)), Some(bob!()));

            use_account!(charlie!());
            let result = nft.clear_approval(seed!(0));

            assert_eq!(result, Err(Error::NotAllowed));
            assert_eq!(nft.get_approved(seed!(0)), Some(bob!()));
        }

        #[ink::test]
        fn approved_for_all() {
            let mut nft = PokeNFT::new();

            assert_eq!(nft.is_approved_for_all(alice!(), bob!()), false);

            nft.set_approval_for_all(bob!(), true).unwrap();

            assert_eq!(nft.is_approved_for_all(alice!(), bob!()), true);
        }

        #[ink::test]
        fn operator_can_transfer_from_owner() {
            let mut nft = PokeNFT::new();
            nft.mint(seed!(0)).unwrap();
            nft.set_approval_for_all(bob!(), true).unwrap();

            use_account!(bob!());
            nft.transfer_from(alice!(), bob!(), seed!(0)).unwrap();
        }
    }
}
