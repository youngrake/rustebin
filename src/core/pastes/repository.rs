use anyhow::Result;
use diesel::PgConnection;

use super::entity::Paste;
use super::orm;

use crate::utils::is_url;

pub fn create_paste(paste: &mut Paste, conn: &PgConnection) -> Result<usize> {
    paste.is_url = Some(is_url(paste.body.clone()));
    orm::create_paste(paste, conn)
}

pub fn get_paste(id: String, connection: &PgConnection) -> Result<Paste> {
    orm::get_paste(id, connection)
}

pub fn update_paste(paste: &mut Paste, conn: &PgConnection) -> Result<Paste> {
    paste.is_url = Some(is_url(paste.body.clone()));

    orm::update_paste(paste, conn)
}
