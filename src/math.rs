use eframe::egui::{Pos2, Rect, Vec2};

pub const CHARGE_POS: Pos2 = Pos2::new(-1.2, 0.0); // + charge
pub const CHARGE_NEG: Pos2 = Pos2::new(1.2, 0.0); // - charge
pub const SOFTENING: f32 = 0.05; // Prevents division by zero

// Precomputed edge connections for the 16 marching squares cases
// Edges: 0 = Bottom, 1 = Right, 2 = Top, 3 = Left
pub const EDGES: [&[(usize, usize)]; 16] = [
    &[],               // 0
    &[(0, 3)],         // 1
    &[(0, 1)],         // 2
    &[(1, 3)],         // 3
    &[(1, 2)],         // 4
    &[(0, 3), (1, 2)], // 5 (saddle)
    &[(0, 2)],         // 6
    &[(2, 3)],         // 7
    &[(2, 3)],         // 8
    &[(0, 2)],         // 9
    &[(0, 1), (2, 3)], // 10 (saddle)
    &[(1, 2)],         // 11
    &[(1, 3)],         // 12
    &[(0, 1)],         // 13
    &[(0, 3)],         // 14
    &[],               // 15
];
/// Scalar potential: V(x, y) = 1/r_pos - 1/r_neg
pub fn potential(p: Pos2) -> f32 {
    let r1 = (p - CHARGE_POS).length().hypot(SOFTENING);
    let r2 = (p - CHARGE_NEG).length().hypot(SOFTENING);
    (1.0 / r1) - (1.0 / r2)
}

/// Vector field: E = -grad(V)
pub fn electric_field(p: Pos2) -> Vec2 {
    let d1 = p - CHARGE_POS;
    let r1 = d1.length().hypot(SOFTENING);
    let d2 = p - CHARGE_NEG;
    let r2 = d2.length().hypot(SOFTENING);

    // E = q * r_vec / r^3
    let e1 = d1 / (r1 * r1 * r1);
    let e2 = d2 / (r2 * r2 * r2);
    e1 - e2
}

pub fn world_to_screen(world_rect: Rect, screen_rect: Rect, pos: Pos2) -> Pos2 {
    let nx = (pos.x - world_rect.min.x) / world_rect.width();
    // Invert Y: 0.0 at world top (max.y), 1.0 at world bottom (min.y)
    let ny = (world_rect.max.y - pos.y) / world_rect.height();

    let screen_x = screen_rect.min.x + nx * screen_rect.width();
    let screen_y = screen_rect.min.y + ny * screen_rect.height();

    Pos2::new(screen_x, screen_y)
}
