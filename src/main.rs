use eframe::egui::{self, Color32, Painter, Pos2, Rect, Stroke, Vec2};

use eframe::egui::FontId;

const CHARGE_POS: Pos2 = Pos2::new(-1.2, 0.0); // + charge
const CHARGE_NEG: Pos2 = Pos2::new(1.2, 0.0); // - charge
const SOFTENING: f32 = 0.05; // Prevents division by zero

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Electrag"),
        ..Default::default()
    };

    eframe::run_native(
        "Electrag",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    // Add any application state here
    draw_crosshair: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ====================================================================
        // 1. LEFT PANEL (PRIORITY OVER BOTTOM)
        // Registered FIRST, so it stretches the full vertical height of the window.
        // ====================================================================
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(220.0)
            .min_width(120.0)
            .max_width(450.0)
            .show(ctx, |ui| {
                ui.heading("Left Panel");
                ui.label("This panel takes priority and claims the full window height.");
                ui.separator();

                ui.checkbox(&mut self.draw_crosshair, "Show Mouse Crosshair");
            });

        // ====================================================================
        // 2. BOTTOM PANEL
        // Registered SECOND, so it only occupies the area to the right of the left panel.
        // ====================================================================
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(140.0)
            .min_height(120.0)
            .max_height(350.0)
            .show(ctx, |ui| {
                ui.heading("Bottom Panel");
                ui.label(
                    "This panel spans only the space remaining to the right of the left panel.",
                );
            });

        // ====================================================================
        // 3. CENTER PANEL (PAINTER CANVAS)
        // Allocates the remaining screen area and exposes the low-level Painter.
        // ====================================================================
        egui::CentralPanel::default().show(ctx, |ui| {
            // Allocate the entire available area as an interactive drawing surface
            let available_size = ui.available_size();
            let (response, painter) = ui.allocate_painter(available_size, egui::Sense::hover());
            let canvas_rect = response.rect;
        });
    }
}

/// Scalar potential: V(x, y) = 1/r_pos - 1/r_neg
fn potential(p: Pos2) -> f32 {
    let r1 = (p - CHARGE_POS).length().hypot(SOFTENING);
    let r2 = (p - CHARGE_NEG).length().hypot(SOFTENING);
    (1.0 / r1) - (1.0 / r2)
}

/// Vector field: E = -grad(V)
fn electric_field(p: Pos2) -> Vec2 {
    let d1 = p - CHARGE_POS;
    let r1 = d1.length().hypot(SOFTENING);
    let d2 = p - CHARGE_NEG;
    let r2 = d2.length().hypot(SOFTENING);

    // E = q * r_vec / r^3
    let e1 = d1 / (r1 * r1 * r1);
    let e2 = d2 / (r2 * r2 * r2);
    e1 - e2
}
