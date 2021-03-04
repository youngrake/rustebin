use anyhow::Result;

use diesel::result::Error;
use diesel::PgConnection;

use super::entity::User;
use super::orm;

pub fn create_user(user: &mut User, conn: &PgConnection) -> Result<usize> {
    // TODO: hashpassword
    orm::create_user(user, conn)
}

pub fn find_or_create(id: String, conn: &PgConnection) -> Result<User> {
    let user = match orm::find_user(id.clone(), conn) {
        Ok(u) => u,
        Err(err) => match err.downcast_ref::<Error>() {
            Some(Error::NotFound) => {
                let new_user = User {
                    id: id.clone(),
                    username: None,
                    password: None,
                    activated: Some(false),
                };

                orm::create_user(&new_user, conn)?;
                new_user
            }

            _ => return Err(err),
        },
    };

    Ok(user)
}

pub fn fetch_user(id: String, conn: &PgConnection) -> Result<User> {
    orm::find_user(id, conn)
}

pub fn update_user(user: &mut User, conn: &PgConnection) -> Result<User> {
    let mut user = orm::update_user(user, conn)?;
    user.password = None;

    Ok(user)
}
