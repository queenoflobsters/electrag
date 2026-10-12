use eframe::egui::{self};

mod math;
mod ui;
mod engine;
mod software_engine;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("Electrag"),
        ..Default::default()
    };

    eframe::run_native(
        "Electrag",
        native_options,
        Box::new(|_cc| Ok(Box::new(ui::UserInterface::default()))),
    )
}
