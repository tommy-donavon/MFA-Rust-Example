use std::error::Error;
use diesel::{select, dsl::exists, QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use crate::models::schema::user::dsl::*;
use crate::models::schema::user;
use crate::db::Pool;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "user"]
pub struct User {
    pub email: String,
    pub password: String,
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