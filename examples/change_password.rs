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

    print!("Please enter your username > ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    match io::stdin().read_line(&mut name) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    name = name.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter your current password > ");
    io::stdout().flush().unwrap();
    let mut current_password = String::new();
    match io::stdin().read_line(&mut current_password) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    current_password = current_password.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter your new password > ");
    io::stdout().flush().unwrap();
    let mut new_password = String::new();
    match io::stdin().read_line(&mut new_password) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    new_password = new_password.trim().to_string();
    io::stdout().flush().unwrap();

    let mut user = match CCashUser::new(&name, &current_password) {
        Ok(user) => user,
        Err(error) => panic!("{}", error),
    };

    let mut session = CCashSession::new(&instance_url);
    session.establish_connection().await.expect("{}");
    // let old_password = user
    if methods::change_password(&mut session, &mut user, &new_password)
        .await
        .unwrap_or_default()
    {
        println!("Changed password to {}", new_password);
    } else {
        println!("Could not change password to {}", new_password);
    }
    Ok(())
}
