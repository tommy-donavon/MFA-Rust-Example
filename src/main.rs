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
use dotenv::dotenv;
use crate::db::establish_connection;
use crate::models::code::{NewUserCode, UserCode};
use crate::models::user::User;
use crate::utils::{hash, send_email};

fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    let database_pool = establish_connection(true, true, Some(Duration::from_secs(30)))?;
    let mut provided_email = String::new();
    println!("Please Enter Your Email: ");
    io::stdin().read_line(&mut provided_email)?;
    println!("Please Enter Your Password: ");
    let mut provided_password = String::new();
    io::stdin().read_line(&mut provided_password)?;
    let u = User{
        email: provided_email.trim().to_owned(),
        password: hash(provided_password.trim())
    };
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
    Ok(())
}

