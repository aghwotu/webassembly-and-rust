struct BankAccount {
    balance: i32,
    verified: bool,
}

fn main() {
    let my_account = BankAccount {
        balance: 34,
        verified: true,
    };

    println!("{:?}", my_account.balance);
    println!("{:?}", my_account.verified);
}
