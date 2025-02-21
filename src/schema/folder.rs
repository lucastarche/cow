#[derive(sqlx::FromRow)]
pub struct Folder {
    id: i64,
    name: String,
    parent: Option<i64>,
    description: String,
}
