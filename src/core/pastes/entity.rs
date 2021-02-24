use crate::schema::pastes;

#[table_name = "pastes"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Paste {
    id: String,
    owner: Option<String>,
    is_url: Option<bool>,
    body: Option<String>,
}
