#![allow(clippy::multiple_crate_versions)]
use eframe::egui;
use pts::{config, drawing, interactions, persistence, state, ui};

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
                interactions::box_select(&mut self.state, &self.config, &response);
            } else if self.state.interaction_mode == state::InteractionMode::Paintbrush {
                interactions::paintbrush(&mut self.state, &self.config, &response);
            } else {
                interactions::normal(&mut self.state, &self.config, &response);
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
