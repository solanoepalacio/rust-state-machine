use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance: u128 =
			caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;

		let new_to_balance: u128 = to_balance.checked_add(amount).ok_or("To many funds.")?;

		self.set_balance(&caller, new_caller_balance);
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();

		let invalid_transfer = balances.transfer("alice".to_string(), "bob".to_string(), 100);

		assert!(invalid_transfer.is_err());
		assert_eq!(invalid_transfer.unwrap_err(), "Not enough funds.");

		balances.set_balance(&"alice".to_string(), 100);
		let valid_transfer = balances.transfer("alice".to_string(), "bob".to_string(), 50);

		assert!(valid_transfer.is_ok());

		assert_eq!(balances.balance(&"alice".to_string()), 50);

		assert_eq!(balances.balance(&"bob".to_string()), 50);
	}
}
