#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: Account) {
    print!("{:#?}", account);
}
fn main() {
    // create a variable bank, and assign it a new Bank
    let bank = Bank::new();
    
    // create another variable and assign it to the bank (move the bank value to the other_bank)
    // this is an error. every value is owned by a single variable at a time
    // reassigning to another variable moves the value. you cannot acces the old value
    let other_bank = bank;

    println!("{:#?}", bank);
}
