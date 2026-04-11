use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}

	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("Claim already exists.")
		}

		self.claims.insert(claim, caller);

		Ok(())
	}

	/// Revoke an existing claim on some content.
	/// This function should only succeed if the caller is the owner of an existing claim.
	/// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let claim_to_revoke = self.claims.get(&claim);

		match claim_to_revoke {
			None => Err("Can't revoke unexistent claim."),
			Some(owner) => {
				if owner != &caller {
					return Err("Caller is not the owner of the claim.");
				}
				self.claims.remove(&claim);
				Ok(())
			},
		}
	}
}

pub enum Call<T: Config> {
	// GetClaim { claim: T::Content },
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;
	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			// Call::GetClaim { claim } => self.get_claim(claim).ok_or(""),
			Call::CreateClaim { claim } => self.create_claim(caller, claim),
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
		}
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = String;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut claim_storage = super::Pallet::<TestConfig>::new();

		let alice = "alice".to_string();
		let bob = "bob".to_string();
		let a_claim = "A first claim.".to_string();
		let another_claim = "A second claim.".to_string();

		// Check initial state: empty claims.
		assert_eq!(claim_storage.claims.clone().into_keys().len(), 0);

		// Check searching an unexisting clain returns None

		assert_eq!(claim_storage.get_claim(&a_claim), None);

		// Check succesfully adding an unexisting clain
		assert_eq!(claim_storage.create_claim(alice.clone(), a_claim.clone()), Ok(()));

		// Check it was actually inserted:
		assert_eq!(claim_storage.get_claim(&a_claim), Some(&alice));

		// Check it throws if we add a claim already existing
		assert_eq!(
			claim_storage.create_claim(alice.clone(), a_claim.clone()),
			Err("Claim already exists.")
		);

		// Check removing throws if claim is removed by not-owner
		assert_eq!(
			claim_storage.revoke_claim(bob.clone(), a_claim.clone()),
			Err("Caller is not the owner of the claim.")
		);

		// Check removing throws if unexistent claim is removed
		assert_eq!(
			claim_storage.revoke_claim(alice.clone(), another_claim.clone()),
			Err("Can't revoke unexistent claim.")
		);

		// Check owner can remove a claim
		assert_eq!(claim_storage.revoke_claim(alice.clone(), a_claim.clone()), Ok(()));
	}
}
