#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    // -> Self or -> Bank is the same
    fn new() -> Self {
        Bank { accounts: vec![]}
    }
}


/*fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account //manually move value between diff owners.
}*/

// send account reference !!! 
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn main() {
    /*let account = Account::new(1, String::from("me"));

    let account_ref = &account; // account_ref is read-only variable (immutable)

    // account_ref.balance = 100; //I cannot change the value of a variable that has a reference.

    print_account(account_ref);

    println!("{:#?}", account);*/

    /*
        let mut account = Account::new(1, String::from("me"));

        //let account_ref = &mut account; //only 1 mut reference at a time.

        //account.balance = 10; //you can't modify the owner value if you create other mutable reference.

        change_account(&mut account);

        println!("{:#?}", account);
    */

    let account = Account::new(1, String::from("me"));

    let other_account = account;

    println!("{:#?}", account);

    let num = 5;

    let other_num = num;

    println!("{} {}", num, other_num);
}
