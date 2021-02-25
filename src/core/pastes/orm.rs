use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

use anyhow::Result;

use crate::schema::pastes;

use super::entity::Paste;

pub fn create_paste(paste: &Paste, connection: &PgConnection) -> Result<usize> {
    let rows_inserted = diesel::insert_into(pastes::table)
        .values(paste)
        .on_conflict_do_nothing()
        .execute(connection)?;

    Ok(rows_inserted)
}

pub fn get_paste(id: String, connection: &PgConnection) -> Result<Paste> {
    let paste = pastes::table.find(id).get_result::<Paste>(connection)?;

    Ok(paste)
}
