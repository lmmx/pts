use crate::config::Config;
use crate::persistence::{self, PointShape};
use crate::state::AppState;
use eframe::egui;

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

fn show_color_swatch(ui: &mut egui::Ui, label: &str, hex: &str, config: &Config) {
    ui.horizontal(|ui| {
        let color = config.parse_color(hex);
        ui.label(format!("{}: ", label));
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
                ui.label("B: Toggle box select");
                ui.label("  In box mode: Arrow keys expand selection");
                ui.label("P: Toggle paintbrush mode");
                ui.label("  In paintbrush: Click/drag to paint points");
                ui.label("Arrow Keys: Move selected point(s)");
                ui.label("Shift + Arrow: Move by large step");
                ui.label("C then C: Clone on top");
                ui.label("C then Arrow: Clone adjacent");
                ui.label("S then C: Set shape to Circle");
                ui.label("S then S: Set shape to Square");
                ui.label("D: Delete selected");
                ui.label("G: Toggle snap-to-grid");
                ui.label("V then G: Toggle grid visibility");
                ui.label("Ctrl + Scroll: Zoom");
                ui.label("?: Show help");
                ui.label("Ctrl+S: Save");
                ui.label("Ctrl+O: Load");
                ui.label("Ctrl+R: Reset");
                ui.label("Q or Escape: Quit");
            });
    }
}

pub fn handle_keyboard(ctx: &egui::Context, state: &mut AppState, config: &mut Config) {
    let shift = ctx.input(|i| i.modifiers.shift);
    let step = if shift { config.move_step_large } else { config.move_step };

    if ctx.input(|i| i.key_pressed(egui::Key::G)) {
        if state.pending_view {
            config.grid_enabled = !config.grid_enabled;
            state.pending_view = false;
        } else {
            state.snap_to_grid = !state.snap_to_grid;
        }
    }

    if ctx.input(|i| i.key_pressed(egui::Key::V)) {
        state.pending_view = true;
    } else if !ctx.input(|i| i.key_down(egui::Key::G)) {
        state.pending_view = false;
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

    if ctx.input(|i| i.key_pressed(egui::Key::D)) {
        state.delete_selected();
    }

    if ctx.input(|i| i.key_pressed(egui::Key::B)) {
        state.box_select_mode = !state.box_select_mode;
        if !state.box_select_mode {
            state.box_select_start = None;
            state.box_select_end = None;
        }
    }

    if ctx.input(|i| i.key_pressed(egui::Key::P)) {
        state.paintbrush_mode = !state.paintbrush_mode;
        state.last_paint_pos = None;
    }

    if state.box_select_mode {
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
        if state.pending_shape {
            state.set_selected_shape(PointShape::Square);
            state.pending_shape = false;
        } else {
            state.pending_shape = true;
        }
    } else if state.pending_shape {
        if ctx.input(|i| i.key_pressed(egui::Key::C)) {
            state.set_selected_shape(PointShape::Circle);
            state.pending_shape = false;
        }
    } else if ctx.input(|i| i.key_pressed(egui::Key::C)) {
        if state.pending_clone {
            state.clone_selected(0.0, 0.0);
            state.pending_clone = false;
        } else {
            state.pending_clone = true;
        }
    } else if state.pending_clone {
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            let (dx, dy) = state.convex_hull_offset((-1.0, 0.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_clone = false;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            let (dx, dy) = state.convex_hull_offset((1.0, 0.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_clone = false;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            let (dx, dy) = state.convex_hull_offset((0.0, -1.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_clone = false;
        } else if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            let (dx, dy) = state.convex_hull_offset((0.0, 1.0), config.point_radius);
            state.clone_selected(dx, dy);
            state.pending_clone = false;
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
