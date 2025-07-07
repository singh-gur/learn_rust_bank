#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|a| a.balance).sum()
    }

    fn account_summary(&self) -> Vec<String> {
        self.accounts.iter().map(|a| a.summary()).collect()
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id: id,
            balance: 0,
            holder: holder,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        if amount > self.balance {
            panic!("Insufficient funds for withdrawal");
        } else {
            self.balance -= amount;
            self.balance
        }
    }

    fn summary(&self) -> String {
        format!(
            "Account ID: {}, Holder: {}, Balance: {}",
            self.id, self.holder, self.balance
        )
    }
}

fn main() {
    let mut account1 = Account::new(1, String::from("Alice"));
    let mut account2 = Account::new(2, String::from("Dan"));

    account1.deposit(100);
    account2.deposit(200);
    account2.withdraw(32);
    let mut bank = Bank::new();
    bank.add_account(account1);
    bank.add_account(account2);

    println!("Total balance in bank: {}", bank.total_balance());
    println!("Account summaries: {:#?}", bank.account_summary());
}
