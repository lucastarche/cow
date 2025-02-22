#[derive(sqlx::FromRow, Default)]
pub struct Entry {
    pub parent: i64,
    pub date: String,
    pub description: String,
    pub code: String,
}
