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

    pub fn print_to_depth(&self, depth: u64) {
        let mut path = vec![];
        self.print_to_depth_(&mut path, 0, depth);
    }

    fn print_to_depth_(&self, path: &mut Vec<String>, node_index: i64, depth: u64) {
        let node = &self.index_to_node[&node_index];

        path.push(node.name.clone());

        println!("{}: {} {}", path.join("/"), node.url, node.comment);

        if depth > 0 && self.outgoing_edges.contains_key(&node_index) {
            for next in self.outgoing_edges[&node_index].iter() {
                self.print_to_depth_(path, *next, depth - 1);
            }
        }

        path.pop();
    }
}
