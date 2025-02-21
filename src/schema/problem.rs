#[derive(sqlx::FromRow)]
pub struct Problem {
    id: i64,
    name: String,
    description: String,
}
