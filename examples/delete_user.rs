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

    let user = match CCashUser::new(&name, &password) {
        Ok(user) => user,
        Err(error) => panic!("{}", error),
    };

    let mut session = CCashSession::new(&instance_url);
    session.establish_connection().await.expect("{}");
    if methods::delete_user(&session, &user).await.is_ok() {
        println!("{} deleted!", &name);
    } else {
        println!("{} could not be deleted.", &name);
    }
    Ok(())
}
