use anyhow::Result;
use diesel::PgConnection;

use super::entity::Paste;
use super::orm;

pub fn create_paste(paste: &Paste, connection: &PgConnection) -> Result<usize> {
    // TODO: check if paste is url
    orm::create_paste(paste, connection)
}

pub fn get_paste(id: String, connection: &PgConnection) -> Result<Paste> {
    orm::get_paste(id, connection)
}
