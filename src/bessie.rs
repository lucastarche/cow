use eframe::{egui::CentralPanel, App};

use crate::farmer_john::FarmerJohn;

pub struct Bessie {
    john: FarmerJohn,
}

impl Bessie {
    pub fn new() -> Bessie {
        let john = FarmerJohn::new();
        Bessie { john }
    }
}

impl App for Bessie {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.john.process_messages();

        CentralPanel::default().show(ctx, |ui| {
            for i in self.john.get_subfolders_of(None) {
                ui.label(format!("{i}"));
            }
        });
    }
}
