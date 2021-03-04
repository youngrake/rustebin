use anyhow::Result;
use diesel::PgConnection;

use super::entity::Paste;
use super::orm;

use crate::utils::is_url;

pub fn create_paste(paste: &mut Paste, connection: &PgConnection) -> Result<usize> {
    paste.is_url = Some(is_url(paste.body.clone()));
    orm::create_paste(paste, connection)
}

pub fn get_paste(id: String, connection: &PgConnection) -> Result<Paste> {
    orm::get_paste(id, connection)
}
