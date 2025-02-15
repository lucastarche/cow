use std::{
    env::{self},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use sqlx::SqlitePool;
use tokio::fs::read_to_string;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add {
        name: String,
        url: String,
        code_path: PathBuf,
        comment_path: PathBuf,
    },
    Get {
        id: i64,
    },
}

#[derive(Default, Debug)]
struct ProblemMetadata {
    name: String,
    url: String,
    date: String,
}

#[derive(Default, Debug)]
struct Problem {
    metadata: ProblemMetadata,
    comment: String,
    code: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let args = Args::parse();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::Add {
            name,
            url,
            code_path,
            comment_path,
        }) => {
            let metadata = ProblemMetadata {
                name,
                url,
                ..Default::default()
            };

            let code = read_to_string(code_path).await?;
            let comment = read_to_string(comment_path).await?;
            let problem = Problem {
                metadata,
                code,
                comment,
            };

            add_problem(&pool, &problem).await?;
        }
        Some(Command::Get { id }) => {
            query_problem_by_id(&pool, id).await?;
        }
        None => {
            query_metadata_all_problems(&pool).await?;
        }
    }

    Ok(())
}

async fn add_problem(pool: &SqlitePool, problem: &Problem) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO problems ( name, url, code, comment )
        VALUES ( ?1, ?2, ?3, ?4 )
        "#,
        problem.metadata.name,
        problem.metadata.url,
        problem.code,
        problem.comment
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn query_metadata_all_problems(pool: &SqlitePool) -> anyhow::Result<Vec<ProblemMetadata>> {
    let recs = sqlx::query!(
        r#"
        SELECT rowid, name, url, date
        FROM problems
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "{} | {} | {} | {}",
            rec.rowid,
            rec.name.unwrap_or_default(),
            rec.date.unwrap_or_default(),
            rec.url.unwrap_or_default()
        );
    }

    Ok(vec![])
}

async fn query_problem_by_id(pool: &SqlitePool, id: i64) -> anyhow::Result<()> {
    let rec = sqlx::query!(
        r#"
        SELECT rowid, name, url, date, code, comment
        FROM problems
        WHERE rowid = ?1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    println!("{}", rec.name.unwrap_or_default());
    println!("{}", rec.url.unwrap_or_default());
    println!("{}", rec.date.unwrap_or_default());

    println!("\n{}", rec.comment.unwrap_or_default());
    println!("\n{}", rec.code.unwrap_or_default());

    Ok(())
}
