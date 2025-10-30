use crate::config::Config;
use crate::state::AppState;
use crate::persistence::PointShape;
use eframe::egui;

pub fn draw_canvas(
    ui: &mut egui::Ui,
    state: &AppState,
    config: &Config,
) -> egui::Response {
    let (response, painter) = ui.allocate_painter(
        ui.available_size(),
        egui::Sense::click_and_drag(),
    );

    let bg = config.parse_color(&config.bg_color);
    painter.rect_filled(response.rect, 0.0, bg);

    if config.grid_enabled {
        draw_grid(&painter, &response.rect, config);
    }

    draw_points(&painter, state, config);

    response
}

fn draw_grid(painter: &egui::Painter, rect: &egui::Rect, config: &Config) {
    let grid_color = config.parse_color(&config.grid_color);
    let spacing = config.grid_spacing;

    let mut x = (rect.min.x / spacing).ceil() * spacing;
    while x < rect.max.x {
        painter.line_segment(
            [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
            egui::Stroke::new(1.0, grid_color),
        );
        x += spacing;
    }

    let mut y = (rect.min.y / spacing).ceil() * spacing;
    while y < rect.max.y {
        painter.line_segment(
            [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
            egui::Stroke::new(1.0, grid_color),
        );
        y += spacing;
    }
}

fn draw_points(painter: &egui::Painter, state: &AppState, config: &Config) {
    let point_color = config.parse_color(&config.point_color);
    let selected_color = config.parse_color(&config.selected_color);
    let selected_indices = state.selected_indices();

    for (i, pt) in state.points.iter().enumerate() {
        let pos = egui::pos2(pt.x, pt.y);
        let color = if selected_indices.contains(&i) || state.dragging == Some(i) {
            selected_color
        } else {
            point_color
        };

        match pt.shape {
            PointShape::Circle => {
                painter.circle_filled(pos, config.point_radius, color);
            }
            PointShape::Square => {
                let half = config.point_radius;
                let rect = egui::Rect::from_center_size(
                    pos,
                    egui::vec2(half * 2.0, half * 2.0),
                );
                painter.rect_filled(rect, 0.0, color);
            }
        }
    }

    if let (Some(start), Some(end)) = (state.box_select_start, state.box_select_end) {
        let box_color = config.parse_color(&config.selection_box_color);
        let rect = egui::Rect::from_two_pos(start, end);
        painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, box_color));
    }
}
