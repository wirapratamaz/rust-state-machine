use std::collections::BTreeMap;

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
pub struct Pallet {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		// Get the balance of account `caller`
		let caller_balance = self.balance(&caller);

		// Get the balance of account `to`
		let to_balance = self.balance(&to);

		// use safe math to calculate a `new_caller_balance`
		let new_caller_balance = caller_balance
			.checked_sub(amount)
			.ok_or("Not enough funds.")?;

		// use safe math to calculate a `new_to_balance`
		let new_to_balance = to_balance
			.checked_add(amount)
			.ok_or("Overflow")?;

		// Insert the new balance of `caller`
		self.balances.insert(caller, new_caller_balance);


		// Insert the new balance of `to`
		self.balances.insert(to, new_to_balance);

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
		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/

		let mut balances = super::Pallet::new();

		// initial setup
		balances.set_balance(&"alice".to_string(), 100);

		let result = balances.transfer(
			"alice".to_string(),
			"bob".to_string(),
			50,
		);

		assert_eq!(result, Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 50);
		
	}
}
