use bevy::prelude::*;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Grid::default());
        app.add_system(update_grid.system());
    }
}

#[derive(Bundle)]
pub struct GridBundle {
    pub position: Position,
    pub size: Size,
}
impl Default for GridBundle {
    fn default() -> Self {
        GridBundle {
            position: Position::default(),
            size: Size::default(),
        }
    }
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}
impl Size {
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl Default for Size {
    fn default() -> Self {
        Size {
            width: 1,
            height: 1,
        }
    }
}

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub mode: GridMode,
}
impl Default for Grid {
    fn default() -> Self {
        Grid {
            width: 10,
            height: 10,
            mode: GridMode::Fit,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum GridMode {
    Fit,
    FitX,
    FitY,
    Adjust,
}

fn update_grid(
    mut grid_query: Query<(&Position, &Size, &mut Transform, &mut Sprite)>,
    grid: Res<Grid>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    for (position, size, mut transform, mut sprite) in grid_query.iter_mut() {
        let step_x = window.width() / grid.width as f32;
        let step_y = window.height() / grid.height as f32;

        let width = step_x * size.width as f32;
        let height = step_y * size.height as f32;

        match grid.mode {
            GridMode::Adjust => {
                let width = (grid.width * size.width) as f32;
                let height = (grid.height * size.height) as f32;

                sprite.size = Vec2::new(width, height);
                transform.translation.x =
                    (grid.width as i32 * position.x) as f32 - window.width() / 2.0 + width / 2.0;
                transform.translation.y = (-(grid.height as i32) * position.y) as f32
                    + window.height() / 2.0
                    - height / 2.0;
            }

            GridMode::FitY => {
                sprite.size = Vec2::new(height, height);
                transform.translation.x =
                    step_y * position.x as f32 - window.width() / 2.0 + height / 2.0;
                transform.translation.y =
                    -step_y * position.y as f32 + window.height() / 2.0 - height / 2.0;
            }

            GridMode::FitX => {
                sprite.size = Vec2::new(width, width);

                transform.translation.x =
                    step_x * position.x as f32 - window.width() / 2.0 + width / 2.0;
                transform.translation.y =
                    -step_x * position.y as f32 + window.height() / 2.0 - width / 2.0;
            }

            // default mode is Fit
            _ => {
                sprite.size = Vec2::new(width, height);
                transform.translation.x =
                    step_x * position.x as f32 - window.width() / 2.0 + width / 2.0;
                transform.translation.y =
                    -step_y * position.y as f32 + window.height() / 2.0 - height / 2.0;
            }
        }
    }
}
