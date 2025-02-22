#[derive(sqlx::FromRow, Default)]
pub struct Folder {
    pub id: i64,
    pub name: String,
    pub parent: Option<i64>,
    pub description: String,
}
