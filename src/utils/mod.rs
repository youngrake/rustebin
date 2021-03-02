pub mod db;
pub mod errors;

pub fn generate_id() -> String {
    use nanoid::nanoid;

    nanoid!()
}
