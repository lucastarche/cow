use eframe::{
    egui::{Align, Context, Layout, Sense, SidePanel, Ui, UiBuilder},
    App,
};

use crate::farmer_john::FarmerJohn;

pub struct Bessie {
    john: FarmerJohn,
}

impl Bessie {
    pub fn new() -> Bessie {
        let john = FarmerJohn::new();
        Bessie { john }
    }

    fn render_left_panel(ctx: &Context, id: String, f: impl FnOnce(&mut Ui) -> ()) {
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

    fn render_left_panels(&mut self, ctx: &Context) {
        Self::render_left_panel(ctx, format!("left-panel-0"), |ui| {
            // FIXME: puede evitarse esta copia de alguna forma? Rust a veces es medio tonto
            let v = self.john.get_subfolders_of(None).clone();
            for i in v.into_iter() {
                let folder = self.john.get_folder(i);
                ui.label(folder.name.clone());
            }
        });
    }
}

impl App for Bessie {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.john.process_messages();
        self.render_left_panels(ctx);
    }
}
