/* TODO: You might need to update your imports. */
use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
	/// The current block number.
	/* TODO: Create a field `block_number` that stores a `u32`. */
	/// A map from an account to their nonce.
	/* TODO: Create a field `nonce` that is a `BTreeMap` from `String` to `u32`. */
	block_number: u32,
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self {
			block_number: 0,
			nonce: BTreeMap::new(),
		}
	}

	// Get the current block number.
	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	// Set the current block number.
	pub fn set_block_number(&mut self, block_number: u32) {
		self.block_number = block_number;
	}

	// if the account has no stored nonce, we return zero
	pub fn nonce(&self, who: &String) -> u32 {
		*self.nonce.get(who).unwrap_or(&0)
	}

	// increment the nonce of an account 'who'

	pub fn inc_nonce(&mut self, who: &String) {
		let next_nonce = self.nonce(who);
		self.nonce.insert(who.clone(), next_nonce);
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn block_number() {
        let mut system = super::Pallet::new();
        
        assert_eq!(system.block_number(), 0);
        system.set_block_number(1);
        assert_eq!(system.block_number(), 1);
    }

    #[test]
    fn nonces() {
        let mut system = super::Pallet::new();
        let alice = "alice".to_string();
        
        assert_eq!(system.nonce(&alice), 0);
        system.inc_nonce(&alice);
        assert_eq!(system.nonce(&alice), 1);
        system.inc_nonce(&alice);
        assert_eq!(system.nonce(&alice), 2);
    }
}