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

fn print_account(account: &Account) {
    print!("{:#?}", account);
}

// I can uodate the value using ref mut
fn change_account(account: &mut Account) {
    account.balance = 10;

}


fn main() {

    // some types of values are copied instead of moved. 
    // numbers, bool, char, arrays, tuples, references are types that are copied and not moved
   let num = 5;

   let other_num = num;
 
   println!("{:#?}", num)

}
