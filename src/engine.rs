use eframe::egui::{Painter, Pos2, Rect, Ui};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EngineKind {
    Software,
    Shaders,
}

pub struct RenderContext {
    pub world_rect: Rect,
    pub draw_isolines: bool,
    pub draw_field_lines: bool,
    pub draw_grid: bool,
    pub isolines_grid_size: u32,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            world_rect: Rect::from_two_pos(Pos2::new(-3.0, 3.0), Pos2::new(3.0, -3.0)),
            draw_isolines: true,
            draw_field_lines: true,
            draw_grid: true,
            isolines_grid_size: 60,
        }
    }
}

pub trait Engine {
    fn paint(&self, painter: &mut Painter);

    fn bottom_panel(&mut self, ui: &mut Ui);

    fn context(&self) -> &RenderContext;

    fn context_mut(&mut self) -> &mut RenderContext;
}
