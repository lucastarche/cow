use sqlx::{Pool, Sqlite, SqlitePool};

use crate::problem::{Problem, ProblemId};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(url: &str) -> anyhow::Result<Database> {
        let pool = SqlitePool::connect(url).await?;
        Ok(Database { pool })
    }

    pub async fn insert_problem(&self, problem: &Problem) -> anyhow::Result<i64> {
        let name = problem.name.to_owned().unwrap();
        let url = problem.url.to_owned().unwrap();
        let code = problem.code.to_owned().unwrap();
        let comment = problem.comment.to_owned().unwrap();

        let id = sqlx::query!(
            r#"
            INSERT INTO problems ( name, url, code, comment )
            VALUES ( ?1, ?2, ?3, ?4 )
            "#,
            name,
            url,
            code,
            comment
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn get_metadata_all_problems(&self) -> anyhow::Result<Vec<Problem>> {
        let recs = sqlx::query!(
            r#"
            SELECT rowid, name, url, date
            FROM problems
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let problems = recs
            .into_iter()
            .map(|r| Problem {
                id: r.rowid,
                name: r.name,
                url: r.url,
                date: r.date,
                ..Default::default()
            })
            .collect();

        Ok(problems)
    }

    pub async fn get_problem_by_id(&self, rowid: ProblemId) -> anyhow::Result<Problem> {
        let rec = sqlx::query!(
            r#"
            SELECT name, url, date, code, comment
            FROM problems
            WHERE rowid = $1
            "#,
            rowid
        )
        .fetch_one(&self.pool)
        .await?;

        let problem = Problem {
            id: rowid,
            name: rec.name,
            url: rec.url,
            date: rec.date,
            code: rec.code,
            comment: rec.comment,
            ..Default::default()
        };

        Ok(problem)
    }
}
