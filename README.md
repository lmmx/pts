# Pts

A point canvas with JSON storage

## “Points”

Pts is a GUI for creating, manipulating, and arranging points on a canvas.

Points can be:

- different shapes and switched from one to the other with <kbd>S</kbd> + the shape key:
  - <kbd>C</kbd> for circle
  - <kbd>S</kbd> for square
- cloned,
  - in-place with <kbd>C</kbd>×2
  - in the dirrection of an arrow key with <kbd>C</kbd>+{<kbd>←</kbd><kbd>↑</kbd><kbd>→</kbd><kbd>↓</kbd>}
- locked to a grid of lines, toggled with <kbd>G</kbd>
- selected one at a time, or multiple at a time by either:
  - <kbd>B</kbd>ox selection by holding the mouse to drag a bounding box
  - flood fill (also in <kbd>B</kbd> mode) in a particular direction (<kbd>←</kbd><kbd>↑</kbd><kbd>→</kbd><kbd>↓</kbd>),
- saved to JSON (`points.json`) and re-loaded.

## Usage

- Select single or multiple points with your mouse
- Draw new points by holding the mouse down in paintbrush mode (<kbd>P</kbd> toggles)
- Drag points with mouse (<kbd>G</kbd> toggles snapping to grid)
- Keyboard-driven workflow

## Unimplemented

- Configure settings via tool TOML in Cargo.toml
- Publish to crates.io

## Installation

crates.io is still TODO

```sh
cargo install --path .
```

## Configuration

Create `config.toml` in the working directory:

```toml
bg_color = "#FFFFFF"
point_color = "#000000"
selected_color = "#FF0000"
selection_box_color = "#0000FF"
grid_enabled = true
grid_spacing = 50.0
grid_color = "#CCCCCC"
point_radius = 8.0
move_step = 1.0
move_step_large = 8.0
```

## Controls

### Selection
- Click point: Select single point
- Click empty: Deselect all
- <kbd>B</kbd>: Toggle box select mode
- Arrow keys (in box mode): Expand selection to adjacent points
- Drag box: Select all points entirely within box

### Movement
- Arrow keys: Move selected points by `move_step`
- <kbd>Shift</kbd> + <kbd>Arrow</kbd>: Move by `move_step_large`
- Mouse drag: Move selected points (quantized to `move_step`)

### Cloning
- <kbd>C</kbd> then <kbd>C</kbd>: Clone selected points on top
- <kbd>C</kbd> then <kbd>Arrow</kbd>: Clone adjacent (offset by bounding box size)

### Shapes
- <kbd>S</kbd> then <kbd></kbd>S: Set selected points to square
- <kbd>S</kbd> then <kbd></kbd>C: Set selected points to circle

### View
- <kbd>G</kbd>: Toggle snap-to-grid mode
- <kbd>V</kbd> then <kbd>G</kbd>: Toggle grid visibility
- <kbd>Ctrl</kbd> + <kbd>Scroll</kbd>: Zoom (0.1x to 10x)

### Other
- <kbd></kbd>D: Delete selected points
- <kbd>?: Show</kbd> help window
- <kbd>Ctrl</kbd> + <kbd>S</kbd>: Save points to `points.json`
- <kbd>Ctrl</kbd> + <kbd>O</kbd>: Load points from file
- <kbd>Ctrl</kbd> + <kbd>R</kbd>: Reset to defaults
- <kbd>Q</kbd> or <kbd>Esc</kbd>: Quit

## File Format

Points are saved to `points.json` in the working directory:
```json
{
  "points": [
    {"id": 1, "x": 100.0, "y": 100.0, "shape": "Circle"},
    {"id": 2, "x": 200.0, "y": 200.0, "shape": "Square"}
  ]
}
```

## Snap to Grid

When snap-to-grid mode is enabled (<kbd>G</kbd>), point boundaries snap to the nearest grid lines. The closest edge of each point aligns with grid spacing.

## Multi-Selection Behavior

- Dragging or arrow moving a selected point moves all selected points together
- Cloning creates copies of all selected points
- Shape changes apply to all selected points
- Delete removes all selected points, then selects most recently created remaining point
  - ...often not very well: TOFIX!

## Ingredients

Made with:

- [**egui**][egui] (for the easy GUI, naturally)
- [**facet**][facet] (for [JSON][facet-json] de/serialisation and application [TOML][facet-toml] config loading)

[egui]: https://github.com/emilk/egui
[facet]: https://github.com/facet-rs/facet
[facet-json]: https://github.com/facet-rs/facet-json
[facet-toml]: https://github.com/facet-rs/facet-toml

## Licensing

pts is [MIT licensed](https://github.com/lmmx/pts/blob/master/LICENSE), a permissive open source license.
