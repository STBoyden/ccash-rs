#![allow(unused_assignments)]

use ccash_rs::*;
use std::io::{self, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    print!("Please enter the instance URL > ");
    io::stdout().flush().unwrap();
    let mut instance_url = String::new();
    match io::stdin().read_line(&mut instance_url) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    instance_url = instance_url.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter the admin username > ");
    io::stdout().flush().unwrap();
    let mut admin_name = String::new();
    match io::stdin().read_line(&mut admin_name) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    admin_name = admin_name.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter the admin password > ");
    io::stdout().flush().unwrap();
    let mut admin_password = String::new();
    match io::stdin().read_line(&mut admin_password) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    admin_password = admin_password.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter the username you want to set the balance for > ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    name = name.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter the new balance for {name} > ");
    io::stdout().flush().unwrap();
    let mut new_balance_str = String::new();
    match io::stdin().read_line(&mut new_balance_str) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let new_balance = new_balance_str.trim().to_string().parse::<u32>().unwrap();
    io::stdout().flush().unwrap();

    let admin_user = match CCashUser::new(&admin_name, &admin_password) {
        Ok(admin_user) => admin_user,
        Err(error) => panic!("{}", error),
    };

    let mut session = CCashSession::new(&instance_url);
    session.establish_connection().await.expect("{}");
    match methods::admin::set_balance(&session, &admin_user, &name, new_balance).await {
        Ok(_) => println!("Set balance for {name} to {new_balance}"),
        Err(e) => println!("Could not change balance to {new_balance} for {name}: {e}"),
    }
    Ok(())
}
