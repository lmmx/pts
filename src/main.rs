//! pts: A point canvas with JSON storage.
#![allow(clippy::multiple_crate_versions)]
use eframe::egui;

mod config;
mod drawing;
mod persistence;
mod state;
mod ui;

struct PointDragger {
    state: state::AppState,
    config: config::Config,
}

impl PointDragger {
    fn new() -> Self {
        let config = config::Config::load();
        let points = persistence::load_points();
        Self {
            state: state::AppState::new(points),
            config,
        }
    }
}

fn handle_box_select_interaction(state: &mut state::AppState, config: &config::Config, response: &egui::Response) {
    if response.drag_started() {
        if let Some(pos) = response.interact_pointer_pos() {
            state.box_select_start = Some(pos);
            state.box_select_end = Some(pos);
        }
    }

    if response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            state.box_select_end = Some(pos);
        }
    }

    if response.drag_stopped() {
        if let (Some(start), Some(end)) = (state.box_select_start, state.box_select_end) {
            let rect = egui::Rect::from_two_pos(start, end);
            state.select_in_box(rect, config.point_radius);
        }
        state.box_select_start = None;
        state.box_select_end = None;
    }
}

fn handle_paintbrush_interaction(state: &mut state::AppState, config: &config::Config, response: &egui::Response) {
    if response.clicked() || response.dragged() {
        if let Some(pos) = response.interact_pointer_pos() {
            state.paint_point(pos, config.point_radius, config.move_step, config.grid_spacing, state.snap_to_grid);
        }
    }

    if response.drag_stopped() {
        state.last_paint_pos = None;
        persistence::save_points(&state.points);
    }
}

fn handle_normal_interaction(state: &mut state::AppState, config: &config::Config, response: &egui::Response) {
    if response.drag_started() {
        if let Some(pos) = response.interact_pointer_pos() {
            if let Some(idx) = state.point_at_pos(pos, config.point_radius) {
                let selected_indices = state.selected_indices();
                if selected_indices.contains(&idx) {
                    state.dragging = Some(idx);
                } else {
                    state.selection = state::Selection::Single(idx);
                    state.dragging = Some(idx);
                }
            }
        }
    }

    if response.dragged() && state.dragging.is_some() {
        if let Some(pos) = response.interact_pointer_pos() {
            let selected = state.selected_indices();
            if let Some(drag_idx) = state.dragging {
                let old_pos = (state.points[drag_idx].x, state.points[drag_idx].y);
                let quantized_x = state::AppState::quantize_position(pos.x, config.move_step);
                let quantized_y = state::AppState::quantize_position(pos.y, config.move_step);
                let dx = quantized_x - old_pos.0;
                let dy = quantized_y - old_pos.1;

                for idx in selected {
                    state.points[idx].x += dx;
                    state.points[idx].y += dy;
                }

                if state.snap_to_grid {
                    state.snap_to_grid(config.grid_spacing, config.point_radius);
                }
            }
        }
    }

    if response.drag_stopped() && state.dragging.is_some() {
        persistence::save_points(&state.points);
        state.dragging = None;
    }

    if response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            if let Some(idx) = state.point_at_pos(pos, config.point_radius) {
                state.selection = state::Selection::Single(idx);
            } else {
                state.selection = state::Selection::None;
            }
        }
    }
}

impl eframe::App for PointDragger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::show_menu(ctx, &mut self.state);
        ui::show_tool_panel(ctx, &self.config, &mut self.state);
        ui::show_help_window(ctx, &mut self.state);
        ui::show_status_bar(ctx, &self.state);
        ui::handle_keyboard(ctx, &mut self.state, &mut self.config);

        egui::CentralPanel::default().show(ctx, |ui| {
            let response = drawing::draw_canvas(ui, &self.state, &self.config);

            if ctx.input(|i| i.modifiers.ctrl) {
                let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
                if scroll_delta != 0.0 {
                    let zoom_delta = scroll_delta * 0.001;
                    self.state.zoom = (self.state.zoom + zoom_delta).clamp(0.1, 10.0);
                }
            }

            if self.state.interaction_mode == state::InteractionMode::BoxSelect {
                handle_box_select_interaction(&mut self.state, &self.config, &response);
            } else if self.state.interaction_mode == state::InteractionMode::Paintbrush {
                handle_paintbrush_interaction(&mut self.state, &self.config, &response);
            } else {
                handle_normal_interaction(&mut self.state, &self.config, &response);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pts",
        options,
        Box::new(|_cc| Ok(Box::new(PointDragger::new()))),
    )
}
