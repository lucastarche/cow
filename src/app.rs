use crate::node_graph::NodeGraph;

use eframe::egui::{
    self, Align, Context, CursorIcon, InnerResponse, Layout, Sense, SidePanel, TopBottomPanel, Ui,
    UiBuilder,
};
use sqlx::SqlitePool;
use std::env;

pub struct CowApp {
    pool: SqlitePool,
    graph: NodeGraph,
    path: Vec<i64>,
}

impl CowApp {
    pub async fn new() -> anyhow::Result<CowApp> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
        let graph = NodeGraph::new(&pool).await?;

        Ok(CowApp {
            pool,
            graph,
            path: vec![0],
        })
    }

    fn render_left_sidebar(ctx: &Context, id: String, f: impl FnOnce(&mut Ui) -> ()) {
        SidePanel::left(id).default_width(150.0).show(ctx, |ui| {
            ui.scope_builder(
                UiBuilder::new()
                    .sense(Sense::click())
                    .layout(Layout::top_down_justified(Align::LEFT)),
                |ui| {
                    f(ui);

                    // Llenar el final de la barra lateral para poder detectar clicks
                    ui.allocate_space(ui.available_size());
                },
            );
        });
    }

    fn render_left_sidebars(&mut self, ctx: &Context) {
        let mut action: Option<(usize, i64)> = None;

        for (d, u) in self.path.iter().enumerate() {
            let next_node = self.path.get(d + 1);
            Self::render_left_sidebar(ctx, format!("left-panel-{d}"), |ui| {
                for v in self.graph.get_adjacent_nodes(*u) {
                    // FIXME: no hacer unwrap(), pensar como manejarlo
                    let node = self.graph.get_node(*v).unwrap();
                    let checked = Some(v) == next_node;

                    let mut label = ui.selectable_label(checked, node.name.clone());
                    label = label.on_hover_cursor(CursorIcon::PointingHand);

                    if label.clicked() {
                        action = Some((d + 1, *v));
                    }
                }

                // TODO: Añadir menu para crear / vincular nodo
                // ui.response().context_menu(|ui| {
                //     let _ = ui.button("Test");
                // });
            });
        }

        if let Some((d, u)) = action {
            self.path.truncate(d);
            self.path.push(u);
        }
    }
}

impl eframe::App for CowApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // TODO: Sacar esto y arreglar breadcrumbs
        TopBottomPanel::top("top-panel").show(&ctx, |ui| {
            ui.horizontal(|ui| {
                let mut is_first = true;
                for i in 1..=3 {
                    if let Some(node) = self.graph.get_node(i) {
                        if !is_first {
                            ui.label(">");
                        }
                        is_first = false;

                        let mut label = ui.label(node.name.clone());
                        label = label.on_hover_cursor(egui::CursorIcon::PointingHand);
                        if label.hovered() {
                            label = label.highlight();
                        }

                        if label.clicked() {
                            println!("oh hi");
                        }
                    }
                }
            });
        });

        self.render_left_sidebars(ctx);
    }
}
