//! Mouse interaction handlers for different modes.

use eframe::egui;
use crate::{config, persistence, state};

pub fn box_select(state: &mut state::AppState, config: &config::Config, response: &egui::Response) {
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

pub fn paintbrush(state: &mut state::AppState, config: &config::Config, response: &egui::Response) {
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

pub fn normal(state: &mut state::AppState, config: &config::Config, response: &egui::Response) {
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
