mod database;
mod problem;

use std::{
    env::{self},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use database::Database;
use tokio::fs::read_to_string;

use problem::Problem;

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

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let args = Args::parse();
    let db = Database::new(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::Add {
            name,
            url,
            code_path,
            comment_path,
        }) => {
            let code = read_to_string(code_path).await?;
            let comment = read_to_string(comment_path).await?;
            let problem = Problem {
                name: Some(name),
                url: Some(url),
                code: Some(code),
                comment: Some(comment),
                ..Default::default()
            };

            db.insert_problem(&problem).await?;
        }
        Some(Command::Get { id }) => {
            let problem = db.get_problem_by_id(id).await?;
            println!("{:#?}", problem);
        }
        None => {
            let v = db.get_metadata_all_problems().await?;
            for problem in v {
                println!(
                    "{} | {} | {} | {}",
                    problem.id,
                    problem.name.unwrap(),
                    problem.date.unwrap(),
                    problem.url.unwrap()
                )
            }
        }
    }

    Ok(())
}
