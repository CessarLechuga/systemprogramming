#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount { balance: initial_balance.max(0.0) }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    // Bonus challenge: apply_interest method
    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance *= 1.0 + rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);

        // Test with negative initial balance
        let account = BankAccount::new(-50.0);
        assert_eq!(account.balance(), 0.0);
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);

        // Test depositing negative amount
        account.deposit(-25.0);
        assert_eq!(account.balance(), 150.0);
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0);
        assert_eq!(account.balance(), 50.0);

        // Test withdrawing more than balance
        account.withdraw(75.0);
        assert_eq!(account.balance(), 50.0);

        // Test withdrawing negative amount
        account.withdraw(-25.0);
        assert_eq!(account.balance(), 50.0);
    }

    #[test]
    fn test_balance() {
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.05);
        assert!((account.balance() - 105.0).abs() < 1e-10);

        // Test with negative interest rate
        account.apply_interest(-0.05);
        assert!((account.balance() - 105.0).abs() < 1e-10);
    }
}