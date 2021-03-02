use anyhow::Result;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::entity::User;
use super::orm;

pub fn create_user(user: &mut User, conn: &PgConnection) -> Result<usize> {
    // TODO: hashpassword
    orm::create_user(user, conn)
}

pub fn fetch_user(id: String, conn: &PgConnection) -> Result<User> {
    orm::find_user(id, conn)
}
