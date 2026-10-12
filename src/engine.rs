use eframe::egui::{Painter, Ui};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EngineKind {
    Software,
    Shaders,
}

pub struct RenderContext {
    pub draw_isolines: bool,
    pub draw_field_lines: bool,
    pub draw_grid: bool,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            draw_isolines: true,
            draw_field_lines: true,
            draw_grid: true,
        }
    }
}

pub trait Engine {
    fn paint(&self, painter: &mut Painter);

    fn bottom_panel(&mut self, ui: &mut Ui);

    fn context(&self) -> &RenderContext;

    fn context_mut(&mut self) -> &mut RenderContext;
}
