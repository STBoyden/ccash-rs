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

    let admin_user = match CCashUser::new(&admin_name, &admin_password) {
        Ok(user) => user,
        Err(error) => panic!("{}", error),
    };

    print!("Please enter the user's username > ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    match io::stdin().read_line(&mut username) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    username = username.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter the user's password > ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    match io::stdin().read_line(&mut password) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    password = password.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter the user's initial balance (>=0) > ");
    io::stdout().flush().unwrap();
    let mut initial_balance_str = String::new();
    match io::stdin().read_line(&mut initial_balance_str) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let initial_balance = initial_balance_str
        .trim()
        .parse::<u32>()
        .unwrap_or_default();
    io::stdout().flush().unwrap();

    let new_user = match CCashUser::new(&username, &password) {
        Ok(user) => user,
        Err(error) => panic!("{}", error),
    };

    let mut session = CCashSession::new(&instance_url);
    session.establish_connection().await.expect("{}");
    if methods::admin::add_user(&session, &admin_user, &new_user, initial_balance)
        .await
        .expect("{}")
    {
        println!(
            "{} created with a balance of {}!",
            &username, initial_balance
        );
    } else {
        println!("{} could not be created.", &username);
    }
    Ok(())
}
