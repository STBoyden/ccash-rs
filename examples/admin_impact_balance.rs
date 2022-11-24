#![allow(unused_assignments)]

use ccash_rs::*;
use std::io::{self, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    print!("Please enter the username you want to impact the balance for > ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    name = name.trim().to_string();
    io::stdout().flush().unwrap();

    print!(
        "Please enter how much to change the balance for {} (+/-)> ",
        name
    );
    io::stdout().flush().unwrap();
    let mut balance_modifier_str = String::new();
    match io::stdin().read_line(&mut balance_modifier_str) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let balance_modifier = balance_modifier_str
        .trim()
        .to_string()
        .parse::<i64>()
        .unwrap();
    io::stdout().flush().unwrap();

    let admin_user = match CCashUser::new(&admin_name, &admin_password) {
        Ok(admin_user) => admin_user,
        Err(error) => panic!("{}", error),
    };

    let mut session = CCashSession::new(&instance_url);
    session.establish_connection().await.expect("{}");
    match methods::admin::impact_balance(
        &mut session,
        &admin_user,
        &name,
        balance_modifier,
    )
    .await
    {
        Ok(_) => println!("Impacted balance for {} by {}", name, balance_modifier),
        Err(e) => println!(
            "Could not impact balance by {} for {}: {}",
            balance_modifier, name, e
        ),
    }
    Ok(())
}
