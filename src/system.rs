use std::{collections::BTreeMap, ops::AddAssign};
use num::traits::{Zero, One};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + Copy + AddAssign + One;
    type Nonce: AddAssign + Default + One;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,

	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl <T: Config> Pallet<T>{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		*self.nonce.entry(who.clone()).or_default() += T::Nonce::one();
	}
}

#[cfg(test)]
mod test {
    use super::*;
    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u16;
        type Nonce = u8;
    }

	#[test]
	fn init_system() {
		let mut system: Pallet<TestConfig> = Pallet::new();

		system.inc_block_number();

		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);

		assert_eq!(system.nonce.get("alice"), Some(&1));

		assert_eq!(system.nonce.get("bob"), None);
	}
}
