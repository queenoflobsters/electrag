use eframe::{
    Renderer,
    egui::{self, Painter, Ui},
};

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

impl SoftwareEngine {
    fn left_column(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Isolines render grid size :");
            ui.add(egui::DragValue::new(&mut self.ctx.isolines_grid_size));
        });
    }

    fn right_column(&mut self, ui: &mut Ui) {}

    fn draw_grid(&self, painter: &mut Painter) {}

    fn draw_isolines(&self, painter: &mut Painter) {}

    fn draw_field_lines(&self, painter: &mut Painter) {}
}

impl Engine for SoftwareEngine {
    fn paint(&self, painter: &mut eframe::egui::Painter) {
        if self.ctx.draw_grid {
            self.draw_grid(painter);
        }

        if self.ctx.draw_isolines {
            self.draw_isolines(painter);
        }

        if self.ctx.draw_field_lines {
            self.draw_field_lines(painter);
        }
    }

    fn bottom_panel(&mut self, ui: &mut eframe::egui::Ui) {
        ui.columns(2, |col| {
            col[0].vertical(|ui| self.left_column(ui));
            col[1].vertical(|ui| self.right_column(ui))
        });
    }

    fn context(&self) -> &RenderContext {
        &self.ctx
    }

    fn context_mut(&mut self) -> &mut RenderContext {
        &mut self.ctx
    }
}
