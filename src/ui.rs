use eframe::egui::{self, Color32, Ui};

use crate::{
    engine::{self, Engine, EngineKind, RenderContext},
    software_engine::SoftwareEngine,
};

pub struct UserInterface {
    engine_kind: EngineKind,
    engine: Box<dyn Engine>,
    context: RenderContext,
}

impl Default for UserInterface {
    fn default() -> Self {
        Self {
            engine_kind: EngineKind::Software,
            engine: Box::new(SoftwareEngine::default()),
            context: RenderContext::default(),
        }
    }
}

impl UserInterface {
    fn construct_engine(&mut self) {
        match self.engine_kind {
            EngineKind::Software => self.engine = Box::new(SoftwareEngine::default()),
            EngineKind::Shaders => todo!(),
        }
    }

    fn side_panel(&mut self, ui: &mut Ui) {
        ui.heading("General");

        self.general_settings(ui);

    }

    fn general_settings(&mut self, ui:&mut Ui) {
        let previous_kind = self.engine_kind;

        egui::ComboBox::from_label("Choose Engine")
            .selected_text(format!("{:?}", self.engine_kind))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.engine_kind, EngineKind::Software, "Software");
                ui.selectable_value(
                    &mut self.engine_kind,
                    EngineKind::Shaders,
                    "Graphics Shaders",
                );
            });

        if previous_kind != self.engine_kind {
            self.construct_engine();
        }


        ui.checkbox(&mut self.context.draw_isolines, "Draw Isolines");
        ui.checkbox(&mut self.context.draw_field_lines, "Draw Field Lines");
        ui.checkbox(&mut self.context.draw_grid, "Draw Grid");
    }

    fn canvas(&self, ui: &mut Ui) {
        // Allocate the entire available area as an interactive drawing surface
        let available_size = ui.available_size();
        let (response, painter) =
            ui.allocate_painter(available_size, egui::Sense::click_and_drag());

        painter.rect_filled(response.rect, 0.0, Color32::from_gray(16));
    }
}

impl eframe::App for UserInterface {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(220.0)
            .min_width(120.0)
            .max_width(450.0)
            .show(ctx, |ui| self.side_panel(ui));

        egui::CentralPanel::default().show(ctx, |ui| self.canvas(ui));
    }
}
