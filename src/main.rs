//! pts: A point canvas with JSON storage.
use eframe::egui;

mod config;
mod persistence;
mod state;
mod drawing;
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

impl eframe::App for PointDragger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::show_menu(ctx, &mut self.state);
        ui::show_tool_panel(ctx, &self.config, &mut self.state);
        ui::show_help_window(ctx, &mut self.state);
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

            if self.state.box_select_mode {
                if response.drag_started() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.state.box_select_start = Some(pos);
                        self.state.box_select_end = Some(pos);
                    }
                }

                if response.dragged() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.state.box_select_end = Some(pos);
                    }
                }

                if response.drag_stopped() {
                    if let (Some(start), Some(end)) = (self.state.box_select_start, self.state.box_select_end) {
                        let rect = egui::Rect::from_two_pos(start, end);
                        self.state.select_in_box(rect, self.config.point_radius);
                    }
                    self.state.box_select_start = None;
                    self.state.box_select_end = None;
                }
            } else if self.state.paintbrush_mode {
                if response.clicked() || response.dragged() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.state.paint_point(
                            pos,
                            self.config.point_radius,
                            self.config.move_step,
                            self.config.grid_spacing,
                            self.state.snap_to_grid,
                        );
                    }
                }

                if response.drag_stopped() {
                    self.state.last_paint_pos = None;
                    persistence::save_points(&self.state.points);
                }
            } else {
                if response.drag_started() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        if let Some(idx) = self.state.point_at_pos(pos, self.config.point_radius) {
                            let selected_indices = self.state.selected_indices();
                            if selected_indices.contains(&idx) {
                                self.state.dragging = Some(idx);
                            } else {
                                self.state.selection = crate::state::Selection::Single(idx);
                                self.state.dragging = Some(idx);
                            }
                        }
                    }
                }

                if response.dragged() {
                    if self.state.dragging.is_some() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            let selected = self.state.selected_indices();
                            if let Some(drag_idx) = self.state.dragging {
                                let old_pos = (self.state.points[drag_idx].x, self.state.points[drag_idx].y);
                                let quantized_x = state::AppState::quantize_position(pos.x, self.config.move_step);
                                let quantized_y = state::AppState::quantize_position(pos.y, self.config.move_step);
                                let dx = quantized_x - old_pos.0;
                                let dy = quantized_y - old_pos.1;

                                for idx in selected {
                                    self.state.points[idx].x += dx;
                                    self.state.points[idx].y += dy;
                                }

                                if self.state.snap_to_grid {
                                    self.state.snap_to_grid(self.config.grid_spacing, self.config.point_radius);
                                }
                            }
                        }
                    }
                }

                if response.drag_stopped() {
                    if self.state.dragging.is_some() {
                        persistence::save_points(&self.state.points);
                        self.state.dragging = None;
                    }
                }

                if response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        if let Some(idx) = self.state.point_at_pos(pos, self.config.point_radius) {
                            self.state.selection = crate::state::Selection::Single(idx);
                        } else {
                            self.state.selection = crate::state::Selection::None;
                        }
                    }
                }
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
