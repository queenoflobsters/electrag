use eframe::egui::{Pos2, Vec2};

const CHARGE_POS: Pos2 = Pos2::new(-1.2, 0.0); // + charge
const CHARGE_NEG: Pos2 = Pos2::new(1.2, 0.0); // - charge
const SOFTENING: f32 = 0.05; // Prevents division by zero

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
