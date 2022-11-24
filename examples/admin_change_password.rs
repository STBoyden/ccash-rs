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

    print!("Please enter the username you want to change the password for > ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    name = name.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter the new password for {} > ", name);
    io::stdout().flush().unwrap();
    let mut new_password = String::new();
    match io::stdin().read_line(&mut new_password) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    new_password = new_password.trim().to_string();
    io::stdout().flush().unwrap();

    let admin_user = match CCashUser::new(&admin_name, &admin_password) {
        Ok(admin_user) => admin_user,
        Err(error) => panic!("{}", error),
    };

    let mut user = match CCashUser::new(&name, &"") {
        Ok(user) => user,
        Err(error) => panic!("{}", error),
    };

    let mut session = CCashSession::new(&instance_url);
    session.establish_connection().await.expect("{}");
    // let old_password = user
    if methods::admin::change_password(
        &mut session,
        &admin_user,
        &mut user,
        &new_password,
    )
    .await
    .unwrap_or_default()
    {
        println!("Changed password to {} for {}", new_password, name);
    } else {
        println!("Could not change password to {} for {}", new_password, name);
    }
    Ok(())
}
