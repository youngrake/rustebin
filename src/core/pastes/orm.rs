use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use anyhow::Result;

use crate::schema::pastes;

use super::entity::Paste;

pub fn create_paste(paste: &Paste, conn: &PgConnection) -> Result<usize> {
    let rows_inserted = diesel::insert_into(pastes::table)
        .values(paste)
        .on_conflict_do_nothing()
        .execute(conn)?;

    Ok(rows_inserted)
}

pub fn get_paste(id: String, conn: &PgConnection) -> Result<Paste> {
    let paste = pastes::table.find(id).get_result::<Paste>(conn)?;

    Ok(paste)
}

pub fn update_paste(paste: &Paste, conn: &PgConnection) -> Result<Paste> {
    use crate::schema::pastes::dsl::*;
    let updated = diesel::update(pastes.filter(id.eq(paste.id.as_ref().unwrap())))
        .set(paste)
        .get_result(conn)?;

    Ok(updated)
}
