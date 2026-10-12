use eframe::{
    egui::{Painter},
};

use crate::engine::{Engine, RenderContext};

pub struct SoftwareEngine {}

impl Default for SoftwareEngine {
    fn default() -> Self {
        Self {}
    }
}

impl SoftwareEngine {
    fn draw_grid(&self, painter: &mut Painter) {}

    fn draw_isolines(&self, painter: &mut Painter) {}

    fn draw_field_lines(&self, painter: &mut Painter) {}
}

impl Engine for SoftwareEngine {
    fn paint(&self, painter: &mut eframe::egui::Painter, ctx: &RenderContext) {
        if ctx.draw_grid {
            self.draw_grid(painter);
        }

        if ctx.draw_isolines {
            self.draw_isolines(painter);
        }

        if ctx.draw_field_lines {
            self.draw_field_lines(painter);
        }
    }
}
