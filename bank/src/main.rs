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

fn print_account(account: Account) -> Account {
    print!("{:#?}", account);
    account
}

fn print_holder(holder: String) {
    println!("{}", holder);
}
fn main() {
   let mut account = Account::new(
    1,
    String::from("me")
   );
   
   //manually move values back and forth btw diff owners.
   // this is tedious.
   account = print_account(account);
   account = print_account(account);

   println!("{:#?}", account)

}
