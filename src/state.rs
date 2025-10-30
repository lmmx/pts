use eframe::egui;
use crate::persistence::{Point, PointShape};
use facet::Facet;

#[derive(Clone, Facet)]
#[repr(u8)]
pub enum Selection {
    None,
    Single(usize),
    Multiple(Vec<usize>),
}

pub struct AppState {
    pub points: Vec<Point>,
    pub selection: Selection,
    pub dragging: Option<usize>,
    pub pending_clone: bool,
    pub pending_shape: bool,
    pub pending_view: bool,
    pub show_help: bool,
    pub next_id: u64,
    pub box_select_start: Option<egui::Pos2>,
    pub box_select_end: Option<egui::Pos2>,
    pub box_select_mode: bool,
    pub snap_to_grid: bool,
    pub zoom: f32,
    pub paintbrush_mode: bool,
    pub last_paint_pos: Option<egui::Pos2>,
}

impl AppState {
    pub fn new(points: Vec<Point>) -> Self {
        let next_id = points.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        let selection = if points.is_empty() {
            Selection::None
        } else {
            Selection::Single(0)
        };
        Self {
            points,
            selection,
            dragging: None,
            pending_clone: false,
            pending_shape: false,
            pending_view: false,
            show_help: false,
            next_id,
            box_select_start: None,
            box_select_end: None,
            box_select_mode: false,
            snap_to_grid: false,
            zoom: 1.0,
            paintbrush_mode: false,
            last_paint_pos: None,
        }
    }

    pub fn point_at_pos(&self, pos: egui::Pos2, radius: f32) -> Option<usize> {
        self.points.iter().position(|pt| {
            let dx = pos.x - pt.x;
            let dy = pos.y - pt.y;
            (dx * dx + dy * dy).sqrt() < radius * 2.0
        })
    }

    pub fn selected_indices(&self) -> Vec<usize> {
        match &self.selection {
            Selection::None => vec![],
            Selection::Single(idx) => vec![*idx],
            Selection::Multiple(indices) => indices.clone(),
        }
    }

    pub fn move_selected(&mut self, dx: f32, dy: f32) {
        for idx in self.selected_indices() {
            self.points[idx].x += dx;
            self.points[idx].y += dy;
        }
    }

    pub fn snap_to_grid(&mut self, grid_spacing: f32, radius: f32) {
        for idx in self.selected_indices() {
            let pt = &mut self.points[idx];
            let left = pt.x - radius;
            let right = pt.x + radius;
            let top = pt.y - radius;
            let bottom = pt.y + radius;
            
            let left_snap = (left / grid_spacing).round() * grid_spacing;
            let right_snap = (right / grid_spacing).round() * grid_spacing;
            let top_snap = (top / grid_spacing).round() * grid_spacing;
            let bottom_snap = (bottom / grid_spacing).round() * grid_spacing;
            
            let left_dist = (left - left_snap).abs();
            let right_dist = (right - right_snap).abs();
            let top_dist = (top - top_snap).abs();
            let bottom_dist = (bottom - bottom_snap).abs();
            
            if left_dist < right_dist {
                pt.x = left_snap + radius;
            } else {
                pt.x = right_snap - radius;
            }
            
            if top_dist < bottom_dist {
                pt.y = top_snap + radius;
            } else {
                pt.y = bottom_snap - radius;
            }
        }
    }

    pub fn quantize_position(pos: f32, step: f32) -> f32 {
        (pos / step).round() * step
    }

    pub fn clone_selected(&mut self, dx: f32, dy: f32) {
        let indices = self.selected_indices();
        let mut new_points = Vec::new();
        
        for idx in indices {
            let pt = &self.points[idx];
            new_points.push(Point {
                id: self.next_id,
                x: pt.x + dx,
                y: pt.y + dy,
                shape: pt.shape.clone(),
            });
            self.next_id += 1;
        }
        
        let start_idx = self.points.len();
        self.points.extend(new_points);
        
        if self.points.len() - start_idx == 1 {
            self.selection = Selection::Single(start_idx);
        } else {
            self.selection = Selection::Multiple((start_idx..self.points.len()).collect());
        }
    }

    pub fn set_selected_shape(&mut self, shape: PointShape) {
        for idx in self.selected_indices() {
            self.points[idx].shape = shape.clone();
        }
    }

    pub fn delete_selected(&mut self) {
        let indices = self.selected_indices();
        if indices.is_empty() {
            return;
        }
        
        let mut indices_sorted = indices.clone();
        indices_sorted.sort_by(|a, b| b.cmp(a));
        
        for idx in indices_sorted {
            self.points.remove(idx);
        }
        
        if self.points.is_empty() {
            self.selection = Selection::None;
        } else {
            let max_id = self.points.iter().map(|p| p.id).max().unwrap();
            let max_idx = self.points.iter().position(|p| p.id == max_id).unwrap();
            self.selection = Selection::Single(max_idx);
        }
    }

    pub fn point_in_box(&self, idx: usize, rect: egui::Rect, radius: f32) -> bool {
        let pt = &self.points[idx];
        match pt.shape {
            PointShape::Circle => {
                rect.contains(egui::pos2(pt.x - radius, pt.y - radius)) &&
                rect.contains(egui::pos2(pt.x + radius, pt.y + radius)) &&
                rect.contains(egui::pos2(pt.x - radius, pt.y + radius)) &&
                rect.contains(egui::pos2(pt.x + radius, pt.y - radius))
            }
            PointShape::Square => {
                rect.contains(egui::pos2(pt.x - radius, pt.y - radius)) &&
                rect.contains(egui::pos2(pt.x + radius, pt.y + radius)) &&
                rect.contains(egui::pos2(pt.x - radius, pt.y + radius)) &&
                rect.contains(egui::pos2(pt.x + radius, pt.y - radius))
            }
        }
    }

    pub fn select_in_box(&mut self, rect: egui::Rect, radius: f32) {
        let mut selected = Vec::new();
        for (idx, _) in self.points.iter().enumerate() {
            if self.point_in_box(idx, rect, radius) {
                selected.push(idx);
            }
        }
        
        self.selection = if selected.is_empty() {
            Selection::None
        } else if selected.len() == 1 {
            Selection::Single(selected[0])
        } else {
            Selection::Multiple(selected)
        };
    }

    pub fn convex_hull_offset(&self, direction: (f32, f32), radius: f32) -> (f32, f32) {
        let indices = self.selected_indices();
        if indices.is_empty() {
            return (0.0, 0.0);
        }
        
        let (dx, dy) = direction;
        
        if dx.abs() > 0.0 {
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            for idx in &indices {
                let pt = &self.points[*idx];
                min_x = min_x.min(pt.x - radius);
                max_x = max_x.max(pt.x + radius);
            }
            let width = max_x - min_x;
            (dx * width, 0.0)
        } else {
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;
            for idx in &indices {
                let pt = &self.points[*idx];
                min_y = min_y.min(pt.y - radius);
                max_y = max_y.max(pt.y + radius);
            }
            let height = max_y - min_y;
            (0.0, dy * height)
        }
    }

    pub fn expand_selection_box(&mut self, direction: (f32, f32), radius: f32) {
        let current = self.selected_indices();
        if current.is_empty() {
            return;
        }

        let mut candidates = Vec::new();
        for idx in current {
            let pt = &self.points[idx];
            let search_pos = egui::pos2(
                pt.x + direction.0 * radius * 2.0,
                pt.y + direction.1 * radius * 2.0,
            );
            
            for (i, other) in self.points.iter().enumerate() {
                let dist_sq = (other.x - search_pos.x).powi(2) + (other.y - search_pos.y).powi(2);
                if dist_sq < (radius * 2.5).powi(2) {
                    candidates.push(i);
                }
            }
        }

        let mut all_selected = self.selected_indices();
        for c in candidates {
            if !all_selected.contains(&c) {
                all_selected.push(c);
            }
        }
        
        self.selection = if all_selected.len() == 1 {
            Selection::Single(all_selected[0])
        } else {
            Selection::Multiple(all_selected)
        };
    }

    pub fn status_text(&self) -> Option<String> {
        if self.paintbrush_mode {
            Some("Paintbrush".to_string())
        } else if self.box_select_mode {
            Some("Box Select".to_string())
        } else if self.pending_clone {
            Some("Clone mode".to_string())
        } else if self.pending_shape {
            Some("Shape mode".to_string())
        } else if self.snap_to_grid {
            Some("Snap to Grid".to_string())
        } else {
            None
        }
    }

    pub fn get_paint_shape(&self) -> PointShape {
        match &self.selection {
            Selection::Single(idx) => self.points[*idx].shape.clone(),
            Selection::Multiple(indices) => {
                if let Some(idx) = indices.first() {
                    self.points[*idx].shape.clone()
                } else {
                    PointShape::Circle
                }
            }
            Selection::None => PointShape::Circle,
        }
    }

    pub fn paint_point(&mut self, pos: egui::Pos2, radius: f32, move_step: f32, grid_spacing: f32, snap: bool) {
        let quantized_x = Self::quantize_position(pos.x, move_step);
        let quantized_y = Self::quantize_position(pos.y, move_step);
        
        if let Some(last_pos) = self.last_paint_pos {
            let dx = (quantized_x - last_pos.x).abs();
            let dy = (quantized_y - last_pos.y).abs();
            
            if dx < radius * 2.0 && dy < radius * 2.0 {
                return;
            }
        }
        
        let shape = self.get_paint_shape();
        let mut new_point = Point {
            id: self.next_id,
            x: quantized_x,
            y: quantized_y,
            shape,
        };
        
        self.next_id += 1;
        self.points.push(new_point.clone());
        
        if snap {
            let idx = self.points.len() - 1;
            let temp_selection = self.selection.clone();
            self.selection = Selection::Single(idx);
            self.snap_to_grid(grid_spacing, radius);
            self.selection = temp_selection;
        }
        
        self.last_paint_pos = Some(egui::pos2(quantized_x, quantized_y));
    }
}
