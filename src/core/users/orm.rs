use anyhow::Result;
use diesel::prelude::*;
use diesel::PgConnection;

use super::entity::User;
use crate::schema::users;

pub fn create_user(user: &User, conn: &PgConnection) -> Result<usize> {
    let rows_inserted = diesel::insert_into(users::table)
        .values(user)
        .on_conflict_do_nothing()
        .execute(conn)?;

    Ok(rows_inserted)
}

pub fn find_user(id: String, conn: &PgConnection) -> Result<User> {
    let user = users::table.find(id).get_result::<User>(conn)?;

    Ok(user)
}
