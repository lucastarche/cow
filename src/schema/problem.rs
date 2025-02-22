#[derive(sqlx::FromRow, Default)]
pub struct Problem {
    pub id: i64,
    pub name: String,
    pub description: String,
}
