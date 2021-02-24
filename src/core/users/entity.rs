use crate::schema::users;

#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub activated: Option<bool>,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            username: None,
            password: None,
            activated: None,
        }
    }
}
