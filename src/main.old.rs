use eframe::egui::{self, Color32, Painter, Pos2, Rect, Stroke, Vec2};

const GRID_SIZE: usize = 20;
const DO_LERP: bool = false;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([700.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Field Lines & Isolines",
        options,
        Box::new(|_cc| Ok(Box::new(FieldApp))),
    )
}

struct FieldApp;

impl eframe::App for FieldApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());
            let rect = response.rect;

            // Draw field lines and isolines
            draw_field_lines(&painter, rect);
            draw_isolines(&painter, rect);

            let stroke = Stroke::new(0.2, Color32::WHITE);
            for x_i in 0..=GRID_SIZE {
                let x = (x_i as f32) * (rect.width() / GRID_SIZE as f32);
                painter.line_segment(
                    [Pos2::new(x as f32, 0.0), Pos2::new(x as f32, rect.height())],
                    stroke,
                );
            }
            for y_i in 0..=GRID_SIZE {
                let y = (y_i as f32) * (rect.height() / GRID_SIZE as f32);
                painter.line_segment(
                    [Pos2::new(0.0, y as f32), Pos2::new(rect.width(), y as f32)],
                    stroke,
                );
            }
        });
    }
}

// ==========================================
// 1. PHYSICAL FIELD (Electric Dipole)
// ==========================================

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

// Coordinate mapping: World [-3.0, 3.0] <-> Screen pixels
fn world_to_screen(w: Pos2, rect: Rect) -> Pos2 {
    let nx = (w.x + 3.0) / 6.0;
    let ny = (3.0 - w.y) / 6.0; // Flip Y for Cartesian coordinates
    Pos2::new(
        rect.min.x + nx * rect.width(),
        rect.min.y + ny * rect.height(),
    )
}

// ==========================================
// 2. FIELD LINES (Streamline Integration)
// ==========================================

fn draw_field_lines(painter: &Painter, rect: Rect) {
    let stroke = Stroke::new(1.2, Color32::from_rgb(255, 140, 0));
    let num_lines = 24;
    let step_size = 0.03;
    let max_steps = 250;

    // Seed rays evenly in a circle around the positive charge
    for i in 0..num_lines {
        let angle = (i as f32) * (std::f32::consts::TAU / num_lines as f32);
        let mut curr = CHARGE_POS + Vec2::new(angle.cos(), angle.sin()) * 0.15;

        let mut screen_points = vec![world_to_screen(curr, rect)];

        for _ in 0..max_steps {
            let e = electric_field(curr);
            if e.length_sq() < 1e-6 {
                break;
            }
            // Step forward in the direction of the field vector (Euler integration)
            let dir = e.normalized();
            curr += dir * step_size;

            screen_points.push(world_to_screen(curr, rect));

            // Stop if the line exits world bounds or reaches the negative charge sink
            if curr.x.abs() > 3.0 || curr.y.abs() > 3.0 || (curr - CHARGE_NEG).length() < 0.15 {
                break;
            }
        }

        // Draw the field line polyline
        for window in screen_points.windows(2) {
            painter.line_segment([window[0], window[1]], stroke);
        }
    }
}

// ==========================================
// 3. ISOLINES (Marching Squares + Lerp)
// ==========================================

// Precomputed edge connections for the 16 marching squares cases
// Edges: 0 = Bottom, 1 = Right, 2 = Top, 3 = Left
const LOOKUP_TABLE: [&[(usize, usize)]; 16] = [
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

fn draw_isolines(painter: &Painter, rect: Rect) {
    let isovalues = [
        -2.0, -1.2, -0.6, -0.2, 0.0, 0.3, 0.1, 0.4, 0.5, 0.2, 0.6, 1.2, 2.0,
    ];
    let stroke = Stroke::new(1.0, Color32::from_rgb(80, 160, 255));

    let step = 6.0 / (GRID_SIZE as f32);

    // 1. Precompute grid values
    let mut grid = vec![0.0; (GRID_SIZE + 1) * (GRID_SIZE + 1)];
    for j in 0..=GRID_SIZE {
        for i in 0..=GRID_SIZE {
            let wx = -3.0 + (i as f32) * step;
            let wy = -3.0 + (j as f32) * step;
            grid[j * (GRID_SIZE + 1) + i] = potential(Pos2::new(wx, wy));
        }
    }

    // Linear interpolation helper
    let lerp = |val_a: f32, val_b: f32, target: f32| -> f32 {
        if !DO_LERP {
            0.5
        } else {
            let diff = val_b - val_a;
            if diff.abs() < 1e-6 {
                0.5
            } else {
                ((target - val_a) / diff).clamp(0.0, 1.0)
            }
        }
    };

    // 2. March across all cells for each isovalue
    for &target in &isovalues {
        for j in 0..GRID_SIZE {
            for i in 0..GRID_SIZE {
                let x0 = -3.0 + (i as f32) * step;
                let y0 = -3.0 + (j as f32) * step;
                let x1 = x0 + step;
                let y1 = y0 + step;

                let row_stride = GRID_SIZE + 1;
                let v0 = grid[j * row_stride + i]; // Bottom-Left
                let v1 = grid[j * row_stride + (i + 1)]; // Bottom-Right
                let v2 = grid[(j + 1) * row_stride + (i + 1)]; // Top-Right
                let v3 = grid[(j + 1) * row_stride + i]; // Top-Left

                // Compute 4-bit index
                let mut cell_index = 0;
                if v0 >= target {
                    cell_index |= 1;
                }
                if v1 >= target {
                    cell_index |= 2;
                }
                if v2 >= target {
                    cell_index |= 4;
                }
                if v3 >= target {
                    cell_index |= 8;
                }

                for &(edge_a, edge_b) in LOOKUP_TABLE[cell_index] {
                    let get_edge_pos = |edge| match edge {
                        0 => Pos2::new(x0 + lerp(v0, v1, target) * step, y0), // Bottom
                        1 => Pos2::new(x1, y0 + lerp(v1, v2, target) * step), // Right
                        2 => Pos2::new(x0 + lerp(v3, v2, target) * step, y1), // Top
                        3 => Pos2::new(x0, y0 + lerp(v0, v3, target) * step), // Left
                        _ => unreachable!(),
                    };

                    let p1_world = get_edge_pos(edge_a);
                    let p2_world = get_edge_pos(edge_b);

                    painter.line_segment(
                        [
                            world_to_screen(p1_world, rect),
                            world_to_screen(p2_world, rect),
                        ],
                        stroke,
                    );
                }
            }
        }
    }
}
