pub mod db;
pub mod errors;

pub fn generate_id() -> String {
    use uuid::Uuid;

    Uuid::new_v4().to_string()
}
