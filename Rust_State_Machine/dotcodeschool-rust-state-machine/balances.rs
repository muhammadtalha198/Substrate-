#[warn(dead_code)]

use std::collections::BTreeMap;

pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self {
			balances: BTreeMap::new(),
		}
	}

    pub fn set_balance(&mut self, who: &str, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    pub fn get_balance(&self, who: &str) -> Option<&u128> {
        self.balances.get(who).unwrap_or(&0)
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
}
