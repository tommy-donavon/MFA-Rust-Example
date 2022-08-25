use std::fmt::{Display, Formatter};
use diesel::{select, dsl::exists, QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use diesel::r2d2::PoolError as R2D2Err;
use diesel::result::Error as ResultErr;
use crate::schema::user::dsl::*;
use crate::schema::user_code::dsl::*;
use crate::schema::user_code;
use crate::Pool;

#[derive(Queryable, Debug)]
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

#[derive(Debug)]
pub enum VerifyErr {
    R2D2Error(R2D2Err),
    ResultError(ResultErr),
}

impl Display for VerifyErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyErr::R2D2Error(r2d2_error) =>
                write!(f, "{}", r2d2_error),
            VerifyErr::ResultError(result_error) =>
                write!(f, "{}", result_error)
        }
    }
}

impl std::error::Error for VerifyErr {}

impl User {
    pub fn verify(&self, pool: Pool) -> Result<bool, VerifyErr> {
        let connection = match pool.get() {
            Ok(v) => v,
            Err(e) => return Err(VerifyErr::R2D2Error(e)),
        };
        let result = match select(
            exists(
                user.filter(
                    email.eq(&self.email).and(password.eq(&self.password))
                )
            )
        ).get_result::<bool>(&connection) {
            Ok(v) => v,
            Err(e) => return Err(VerifyErr::ResultError(e)),
        };
        Ok(result)
    }
}

impl NewUserCode {
    pub fn save(&self, pool: Pool) {
        let connection = pool.get().unwrap();
        diesel::insert_into(user_code::table).values(self).execute(&connection).unwrap();
    }
    pub fn get_code(q_email: &str, pool: Pool) -> Result<String, VerifyErr> {
        let connection = match pool.get() {
            Ok(v) => v,
            Err(e) => return Err(VerifyErr::R2D2Error(e)),
        };
        match user_code.filter(user_email.eq(q_email)).first::<UserCode>(&connection) {
            Ok(v) => Ok(v.code),
            Err(e) => Err(VerifyErr::ResultError(e))
        }
    }
    pub fn delete_code(q_email: &str, pool:Pool) -> Result<(), VerifyErr>{
        let connection = match pool.get() {
            Ok(v) => v,
            Err(e) => return Err(VerifyErr::R2D2Error(e)),
        };
        match diesel::delete(user_code.filter(user_email.eq(q_email))).execute(&connection){
            Ok(_) => Ok(()),
            Err(e) => Err(VerifyErr::ResultError(e))
        }
    }
}