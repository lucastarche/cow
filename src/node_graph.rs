use std::collections::HashMap;

use sqlx::SqlitePool;

pub struct NodeGraph {
    index_to_node: HashMap<i64, Node>,
    outgoing_edges: HashMap<i64, Vec<i64>>,
}

pub struct Node {
    pub name: String,
    pub url: String,
    pub comment: String,
}

impl NodeGraph {
    pub async fn new(pool: &SqlitePool) -> anyhow::Result<NodeGraph> {
        let nodes = sqlx::query!("SELECT * FROM nodes").fetch_all(pool).await?;
        let edges = sqlx::query!("SELECT * FROM edges").fetch_all(pool).await?;

        let mut index_to_node = HashMap::new();
        let mut outgoing_edges: HashMap<i64, Vec<i64>> = HashMap::new();

        for node in nodes {
            index_to_node.insert(
                node.id,
                Node {
                    name: node.name.unwrap_or_default(),
                    url: node.url.unwrap_or_default(),
                    comment: node.comment.unwrap_or_default(),
                },
            );
        }

        for edge in edges {
            outgoing_edges
                .entry(edge.source)
                .or_default()
                .push(edge.dest);
        }

        Ok(NodeGraph {
            index_to_node,
            outgoing_edges,
        })
    }

    pub fn get_node(&self, id: i64) -> Option<&Node> {
        self.index_to_node.get(&id)
    }

    pub fn get_adjacent_nodes(&self, id: i64) -> impl Iterator<Item = &i64> {
        self.outgoing_edges
            .get(&id)
            .into_iter()
            .flat_map(|v| v.iter())
    }
}
