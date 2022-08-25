use std::error::Error;
use diesel::{select, dsl::exists, QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::schema::user::dsl::*;
use crate::schema::user;
use crate::schema::user_code::dsl::*;
use crate::schema::user_code;
use crate::Pool;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "user"]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Debug)]
#[table_name = "user_code"]
pub struct NewUserCode {
    pub code: String,
    pub user_email: String,
}

#[derive(Queryable, Debug)]
pub struct UserCode {
    pub id: i32,
    pub code: String,
    pub user_email: String,
}

impl User {
    pub fn save(&self, pool: Pool) -> Result<(), Box<dyn Error>> {
        let connection = pool.get()?;
        diesel::insert_into(user::table).values(self).execute(&connection)?;
        Ok(())
    }
    pub fn verify(&self, pool: Pool) -> Result<bool, Box<dyn Error>> {
        let connection = pool.get()?;
        let result = select(
            exists(
                user.filter(
                    email.eq(&self.email).and(password.eq(&self.password))
                )
            )
        ).get_result::<bool>(&connection)?;
        Ok(result)
    }
}

impl NewUserCode {
    pub fn save(&self, pool: Pool) -> Result<(), Box<dyn Error>> {
        let connection = pool.get()?;
        diesel::insert_into(user_code::table).values(self).execute(&connection)?;
        Ok(())
    }
}

impl UserCode {
    pub fn get_code(q_email: &str, pool: Pool) -> Result<UserCode, Box<dyn Error>> {
        let connection = pool.get()?;
        Ok(user_code.
            filter(
                user_email.eq(q_email)
            ).first::<UserCode>(&connection)?)
    }
    pub fn delete_code(q_email: &str, pool: Pool) -> Result<(), Box<dyn Error>> {
        let connection = pool.get()?;
        diesel::delete(user_code.filter(user_email.eq(q_email))).execute(&connection)?;
        Ok(())
    }
}

