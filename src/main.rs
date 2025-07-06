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
        Bank {accounts: vec![]}
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account { id: id, balance: 0, holder: holder }
    }
}


fn print_account(account: Account) {
    println!("Account: {:#?}", account);
}
fn main() {
    let mut bank = Bank::new();
    let account1 = Account::new(1, String::from("Alice"));
    // let account2 = Account::new(2, String::from("Bob"));
    // bank.accounts.push(account1);
    // bank.accounts.push(account2);
    // println!("{:#?}", bank);
    // println!("Account 1: {:#?}", account1);
    print_account(account1);
    print_account(account1);
}
