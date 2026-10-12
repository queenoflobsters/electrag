use eframe::Renderer;

use crate::engine::{Engine, RenderContext};

pub struct SoftwareEngine {
    ctx: RenderContext,
}

impl Default for SoftwareEngine {
    fn default() -> Self {
        Self {
            ctx: RenderContext::default(),
        }
    }
}

impl Engine for SoftwareEngine {
    fn paint(&self, painter: &mut eframe::egui::Painter) {}

    fn bottom_panel(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Salut à tous c'est le panneau d'en bas");
    }

    fn context(&self) -> &RenderContext {
        &self.ctx
    }

    fn context_mut(&mut self) -> &mut RenderContext {
        &mut self.ctx
    }
}
