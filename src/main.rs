use node_graph::NodeGraph;
use sqlx::SqlitePool;
use std::env;

mod node_graph;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let graph = NodeGraph::new(&pool).await?;
    graph.print_to_depth(3);

    Ok(())
}
