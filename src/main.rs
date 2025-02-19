use eframe::egui::{self, SidePanel};
use node_graph::NodeGraph;
use sqlx::SqlitePool;
use std::env;

mod node_graph;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let cow_app = CowApp::new().await?;
    if let Err(e) = eframe::run_native(
        "COW",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(cow_app))),
    ) {
        println!("{e:#?}");
    }

    Ok(())
}

struct CowApp {
    pool: SqlitePool,
    graph: NodeGraph,
}

impl CowApp {
    async fn new() -> anyhow::Result<CowApp> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
        let graph = NodeGraph::new(&pool).await?;

        Ok(CowApp { pool, graph })
    }
}

impl eframe::App for CowApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        SidePanel::left("left-panel")
            .default_width(100.0)
            .show(&ctx, |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    for i in self.graph.get_adjacent_nodes(0) {
                        if let Some(node) = self.graph.get_node(*i) {
                            ui.selectable_label(false, node.name.clone());
                        }
                    }
                })
            });
    }
}
