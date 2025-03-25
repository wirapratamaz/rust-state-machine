use std::collections::BTreeMap;

pub struct Pallet {
    // the current block number
    block_number: u64,

    // a mapping of account nonces
    nonces: BTreeMap<String, u64>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonces: BTreeMap::new(),
        }
    }

    // get the current block number
    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    // set the current block number
    
}
