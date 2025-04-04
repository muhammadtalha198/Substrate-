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

    /// Set the balance of an account `who` to `amount`.    
    pub fn set_balance(&mut self, who: &str, amount: u128) {
        self.balances.insert(who.to_string(), amount);
    }

    /// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
    pub fn get_balance(&self, who: &str) -> Option<&u128> {
        self.balances.get(who).unwrap_or(&0)
    }

    
    /// /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
    pub fn transfer(&mut self, from: &str, to: &str, amount: u128) -> Result<(), String> {
        let from_balance = self.get_balance(from).unwrap();
        let to_balance = self.get_balance(to).unwrap();

        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        self.balances.insert(from.to_string(), from_balance - amount);
        self.balances.insert(to.to_string(), to_balance + amount);

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
    fn transfer_balance(){
        let mut balances = super::Pallet::new();

        balances.set_balance(&"alice".to_string(), 100);
        balances.set_balance(&"bob".to_string(), 0);

        assert!(balances.transfer(&"alice".to_string(), &"bob".to_string(), 100).is_ok());  
        assert_eq!(balances.balance(&"alice".to_string()), 0);
    }
}
