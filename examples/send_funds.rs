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

    print!("Please enter your password > ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    match io::stdin().read_line(&mut password) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    password = password.trim().to_string();
    io::stdout().flush().unwrap();

    print!("Please enter your the recipient's username > ");
    io::stdout().flush().unwrap();
    let mut recipient_name = String::new();
    match io::stdin().read_line(&mut recipient_name) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    recipient_name = recipient_name.trim().to_lowercase();
    io::stdout().flush().unwrap();

    print!("Please enter the amount you want to send > ");
    io::stdout().flush().unwrap();
    let mut amount_string = String::new();
    match io::stdin().read_line(&mut amount_string) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let amount = match amount_string.trim().parse::<u32>() {
        Ok(amt) => amt,
        Err(e) => panic!("{}", e),
    };

    let user = match CCashUser::new(&name, &password) {
        Ok(user) => user,
        Err(error) => panic!("{}", error),
    };

    let mut session = CCashSession::new(&instance_url);
    session.establish_connection().await.expect("{}");
    println!(
        "Sent {} to {}. {} now has {} CSH",
        amount,
        &recipient_name,
        user.get_username(),
        methods::send_funds(&session, &user, &recipient_name, amount)
            .await
            .expect("{}"),
    );
    Ok(())
}
