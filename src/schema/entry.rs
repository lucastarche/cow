#[derive(sqlx::FromRow)]
pub struct Entry {
    parent: i64,
    date: String,
    description: String,
    code: String,
}
