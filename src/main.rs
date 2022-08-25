#[macro_use]
extern crate diesel;
extern crate sha2;
extern crate base64;
extern crate lettre;
extern crate rand;

mod models;
mod db;
mod utils;

use std::{io, time::Duration};
use std::error::Error;
use std::fmt::{Display, Formatter};
use dotenv::dotenv;
use crate::db::establish_connection;
use crate::models::code::{NewUserCode, UserCode};
use crate::models::user::User;
use crate::utils::{hash, send_email};

#[derive(Debug)]
struct Menu {
    options: Vec<String>,
}

impl Menu {
    fn new(mut options:Vec<String>) -> Menu{
        options.push("Exit".to_owned());
        Menu {
            options
        }
    }
}

impl Display for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Please Select from the following options: \n").expect("could not write to standard output");
        for (mut count, item) in self.options.iter().enumerate() {
            // let p_count = count + 1;
            count += 1;
            write!(f, "{count}) {item}\n").expect("could not write to standard output");
        };
        write!(f, "Your selection: ")
    }
}



fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    let menu_options = vec!["Create Account".to_owned(), "Login".to_owned()];
    let menu = Menu::new(menu_options);
    loop {
        println!("{menu}");
        let mut user_option = String::new();
        io::stdin().read_line(&mut user_option)?;
        match user_option.trim() {
            "1" => {
                let database_pool = establish_connection(true, true, Some(Duration::from_secs(30)))?;
                let mut provided_email = String::new();
                println!("Please Enter Your Email: ");
                io::stdin().read_line(&mut provided_email)?;
                println!("Please Enter Your Password: ");
                let mut provided_password = String::new();
                io::stdin().read_line(&mut provided_password)?;
                let u = User::new(provided_email.trim(), hash(provided_password.trim()).as_str());
                match u.save(database_pool.clone()) {
                    Err(_) => println!("Could not save account with that information"),
                    Ok(_) => println!("Account saved"),
                };
            }
            "2" => {
                let database_pool = establish_connection(true, true, Some(Duration::from_secs(30)))?;
                let mut provided_email = String::new();
                println!("Please Enter Your Email: ");
                io::stdin().read_line(&mut provided_email)?;
                println!("Please Enter Your Password: ");
                let mut provided_password = String::new();
                io::stdin().read_line(&mut provided_password)?;
                let u = User::new(provided_email.trim(), hash(provided_password.trim()).as_str());
                let first_factor:bool = u.verify(database_pool.clone())?;
                if !first_factor {
                    println!("Invalid Credentials");
                    return Ok(());
                }
                let code = NewUserCode{
                    code: utils::gen_rand_num(),
                    user_email: u.email.clone()
                };
                code.save(database_pool.clone())?;
                send_email(u.email.clone().as_str(), code.code.as_str())?;
                println!("Please enter in your security code: ");
                let mut provided_code = String::new();
                io::stdin().read_line(&mut provided_code)?;
                let f_code = UserCode::get_code(u.email.clone().as_str(), database_pool.clone())?;
                if f_code.code.eq(&provided_code.trim()){
                    println!("You are in!")
                }else {
                    println!("Invalid code")
                }
                UserCode::delete_code(u.email.as_str(), database_pool)?;
            }
            "3" => {
                println!("Goodbye");
                break;
            }
            _ => {
                println!("Invalid option.");
                continue;
            }
        }
    }

    Ok(())
}

