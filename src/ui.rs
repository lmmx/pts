use crate::config::Config;
use crate::persistence::{self, PointShape};
use crate::state::{AppState, InteractionMode, PendingMode};
use eframe::egui;

pub fn show_status_bar(ctx: &egui::Context, state: &AppState) {
    if let Some(status) = state.status_text() {
        egui::Area::new(egui::Id::new("status"))
            .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -10.0))
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(egui::Color32::from_black_alpha(180))
                    .inner_margin(8.0)
                    .rounding(4.0)
                    .show(ui, |ui| {
                        ui.set_min_width(80.0);
                        ui.label(egui::RichText::new(status).color(egui::Color32::WHITE).size(16.0));
                    });
            });
    }
}

pub fn show_menu(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    persistence::save_points(&state.points);
                    ui.close_menu();
                }
                if ui.button("Load").clicked() {
                    state.points = persistence::load_points();
                    ui.close_menu();
                }
                if ui.button("Reset").clicked() {
                    state.points = persistence::load_points();
                    ui.close_menu();
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.menu_button("Help", |ui| {
                if ui.button("Keyboard Shortcuts").clicked() {
                    state.show_help = !state.show_help;
                    ui.close_menu();
                }
            });
        });
    });
}

pub fn show_tool_panel(ctx: &egui::Context, config: &Config, _state: &mut AppState) {
    egui::SidePanel::left("tools").show(ctx, |ui| {
        ui.heading("Parameters");
        ui.separator();

        ui.label("Movement");
        ui.label(format!("Move Step: {} (Arrow)", config.move_step));
        ui.label(format!("Large Step: {} (Shift + Arrow)", config.move_step_large));
        ui.separator();

        ui.label("Appearance");
        ui.label(format!("Point Radius: {}", config.point_radius));
        ui.label(format!("Grid Spacing: {}", config.grid_spacing));
        ui.separator();

        ui.label("Colors");
        show_color_swatch(ui, "Background", &config.bg_color, config);
        show_color_swatch(ui, "Point", &config.point_color, config);
        show_color_swatch(ui, "Selected", &config.selected_color, config);
        show_color_swatch(ui, "Selection Box", &config.selection_box_color, config);
        show_color_swatch(ui, "Grid", &config.grid_color, config);
    });
}

fn show_color_swatch(ui: &mut egui::Ui, label: &str, hex: &str, _config: &Config) {
    ui.horizontal(|ui| {
        let color = Config::parse_colour(hex);
        ui.label(format!("{label}: "));
        let size = egui::vec2(16.0, 16.0);
        let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
        ui.painter().rect_filled(rect, 2.0, color);
        ui.label(hex);
    });
}

pub fn show_help_window(ctx: &egui::Context, state: &mut AppState) {
    if state.show_help {
        egui::Window::new("Keyboard Shortcuts")
            .open(&mut state.show_help)
            .show(ctx, |ui| {
                ui.heading("Interaction Modes");
                ui.label("B: Toggle box select");
                ui.label("  Arrow keys: Expand selection");
                ui.label("P: Toggle paintbrush mode");
                ui.label("  Click/drag: Paint points");

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.heading("Movement");
                ui.label("Arrow Keys: Move selected point(s)");
                ui.label("Shift + Arrow: Move by large step");

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.heading("Cloning");
                ui.label("C then C: Clone on top");
                ui.label("C then Arrow: Clone adjacent");

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.heading("Shapes");
                ui.label("S then C: Set shape to Circle");
                ui.label("S then S: Set shape to Square");
                ui.label("S then D: Set shape to Diamond");
                ui.label("S then H: Set shape to Semicircle");

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                ui.heading("Other");
                ui.label("D: Delete selected");
                ui.label("G: Toggle snap-to-grid");
                ui.label("V then G: Toggle grid visibility");
                ui.label("Ctrl + Scroll: Zoom");
                ui.label("Ctrl+S: Save");
                ui.label("Ctrl+O: Load");
                ui.label("Ctrl+R: Reset");
                ui.label("?: Show/hide help");
                ui.label("Q or Escape: Quit");
            });
    }
}

#[allow(clippy::too_many_lines)]
pub fn handle_keyboard(ctx: &egui::Context, state: &mut AppState, config: &mut Config) {
    let shift = ctx.input(|i| i.modifiers.shift);
    let step = if shift { config.move_step_large } else { config.move_step };

    if ctx.input(|i| i.key_pressed(egui::Key::G)) {
        if state.pending_mode == PendingMode::View {
            config.grid_enabled = !config.grid_enabled;
            state.pending_mode = PendingMode::None;
        } else {
            state.snap_to_grid = !state.snap_to_grid;
        }
    }

    if ctx.input(|i| i.key_pressed(egui::Key::V)) {
        state.pending_mode = PendingMode::View;
    } else if !ctx.input(|i| i.key_down(egui::Key::G)) {
        state.pending_mode = PendingMode::None;
    }

    if ctx.input(|i| i.key_pressed(egui::Key::Questionmark)) {
        state.show_help = !state.show_help;
    }

    if ctx.input(|i| i.key_pressed(egui::Key::Q) || i.key_pressed(egui::Key::Escape)) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
        persistence::save_points(&state.points);
    }

    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::O)) {
        state.points = persistence::load_points();
    }

    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::R)) {
        state.points = persistence::load_points();
    }

    if ctx.input(|i| i.key_pressed(egui::Key::X)) {
        state.delete_selected();
    }

    if ctx.input(|i| i.key_pressed(egui::Key::B)) {
        if state.interaction_mode == InteractionMode::BoxSelect {
            state.interaction_mode = InteractionMode::Normal;
        } else {
            state.interaction_mode = InteractionMode::BoxSelect;
            state.box_select_start = None;
            state.box_select_end = None;
        }
    }

    if ctx.input(|i| i.key_pressed(egui::Key::P)) {
        if state.interaction_mode == InteractionMode::Paintbrush {
            state.interaction_mode = InteractionMode::Normal;
        } else {
            state.interaction_mode = InteractionMode::Paintbrush;
            state.last_paint_pos = None;
        }
    }

    if state.interaction_mode == InteractionMode::BoxSelect {
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            state.expand_selection_box((-1.0, 0.0), config.point_radius);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            state.expand_selection_box((1.0, 0.0), config.point_radius);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            state.expand_selection_box((0.0, -1.0), config.point_radius);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            state.expand_selection_box((0.0, 1.0), config.point_radius);
        }
    } else if ctx.input(|i| i.key_pressed(egui::Key::S)) {
        if state.pending_mode == PendingMode::Shape {
            state.set_selected_shape(PointShape::Square);
            state.pending_mode = PendingMode::None;
        } else {
            state.pending_mode = PendingMode::Shape;
        }
    } else if state.pending_mode == PendingMode::Shape {
        if ctx.input(|i| i.key_pressed(egui::Key::C)) {
            state.set_selected_shape(PointShape::Circle);
            state.pending_mode = PendingMode::None;
        } else if ctx.input(|i| i.key_pressed(egui::Key::D)) {
            state.set_selected_shape(PointShape::Diamond);
            state.pending_mode = PendingMode::None;
        } else if ctx.input(|i| i.key_pressed(egui::Key::H)) {
            state.set_selected_shape(PointShape::Semicircle);
            state.pending_mode = PendingMode::None;
        }
    } else if ctx.input(|i| i.key_pressed(egui::Key::C)) {
        if state.pending_mode == PendingMode::Clone {
            state.clone_selected(0.0, 0.0);
            state.pending_mode = PendingMode::None;
        } else {
            state.pending_mode = PendingMode::Clone;
        }
    } else if state.pending_mode == PendingMode::Clone {
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            let (dx, dy) = state.convex_hull_offset((-1.0, 0.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_mode = PendingMode::None;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            let (dx, dy) = state.convex_hull_offset((1.0, 0.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_mode = PendingMode::None;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            let (dx, dy) = state.convex_hull_offset((0.0, -1.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_mode = PendingMode::None;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            let (dx, dy) = state.convex_hull_offset((0.0, 1.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_mode = PendingMode::None;
        }
    } else {
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            state.move_selected(-step, 0.0);
            if state.snap_to_grid {
                state.snap_to_grid(config.grid_spacing, config.point_radius);
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            state.move_selected(step, 0.0);
            if state.snap_to_grid {
                state.snap_to_grid(config.grid_spacing, config.point_radius);
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            state.move_selected(0.0, -step);
            if state.snap_to_grid {
                state.snap_to_grid(config.grid_spacing, config.point_radius);
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            state.move_selected(0.0, step);
            if state.snap_to_grid {
                state.snap_to_grid(config.grid_spacing, config.point_radius);
            }
        }
    }
}
