use std::error::Error;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::models::schema::user_code::dsl::*;
use crate::models::schema::user_code;
use crate::db::Pool;

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

