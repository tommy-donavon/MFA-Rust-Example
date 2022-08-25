#[macro_use]
extern crate diesel;
extern crate sha2;
extern crate base64;
extern crate lettre;
extern crate rand;

mod schema;
mod models;
mod db;
mod utils;

use std::{env,io,time::Duration};
use std::error::Error;
use dotenv::dotenv;
use diesel::{sqlite::SqliteConnection, r2d2::{self, ConnectionManager}};
use crate::db::ConnectionOptions;
use crate::models::code::{NewUserCode, UserCode};
use crate::models::user::User;
use crate::utils::{hash, send_email};
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();
    let connection_options:Box<ConnectionOptions> = Box::new(ConnectionOptions {
        enable_wal: true,
        enable_foreign_keys: true,
        busy_timeout: Some(Duration::from_secs(30)),
    });
    let database_url:String = env::var("DATABASE_URL")?;
    let database_pool:Pool = Pool::builder()
        .max_size(16)
        .connection_customizer(connection_options)
        .build(ConnectionManager::<SqliteConnection>::new(database_url))?;

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
    println!("{}",u.password);
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

