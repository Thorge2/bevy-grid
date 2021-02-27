use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GridPlugin)
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup.system())
        .add_system(move_player.system())
        .add_system(change_grid.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .with_bundle(GridBundle {
            size: GridSize::new(2, 2),
            ..Default::default()
        });
}

fn move_player(mut player_query: Query<&mut GridPosition>, keyboard_input: Res<Input<KeyCode>>) {
    for mut position in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::S) {
            position.y -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::W) {
            position.y += 1;
        }
        if keyboard_input.just_pressed(KeyCode::A) {
            position.x -= 1;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            position.x += 1;
        }
    }
}

fn change_grid(mut grid: ResMut<Grid>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::P) {
        grid.height += 1;
        grid.width += 1;
    }

    if keyboard_input.just_pressed(KeyCode::M) {
        grid.height -= 1;
        grid.width -= 1;
    }
}

struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Grid::default());
        app.add_system(update_grid.system());
    }
}

#[derive(Bundle)]
struct GridBundle {
    pub position: GridPosition,
    pub size: GridSize,
}
impl Default for GridBundle {
    fn default() -> Self {
        GridBundle {
            position: GridPosition::default(),
            size: GridSize::default(),
        }
    }
}

struct GridPosition {
    x: i32,
    y: i32,
}
impl GridPosition {
    #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Default for GridPosition {
    fn default() -> Self {
        GridPosition { x: 0, y: 0 }
    }
}

struct GridSize {
    width: u32,
    height: u32,
}
impl GridSize {
    #[allow(dead_code)]
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl Default for GridSize {
    fn default() -> Self {
        GridSize {
            width: 1,
            height: 1,
        }
    }
}

struct Grid {
    width: u32,
    height: u32,
}
impl Default for Grid {
    fn default() -> Self {
        Grid {
            width: 10,
            height: 10,
        }
    }
}

fn update_grid(
    mut grid_query: Query<(&GridPosition, &GridSize, &mut Transform, &mut Sprite)>,
    grid: Res<Grid>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    for (position, size, mut transform, mut sprite) in grid_query.iter_mut() {
        let step_x = window.width() / grid.width as f32;
        let step_y = window.height() / grid.height as f32;

        let width = step_x * size.width as f32;
        let height = step_y * size.height as f32;

        sprite.size = Vec2::new(width, height);

        transform.translation.x = step_x * position.x as f32 - window.width() / 2.0 + width / 2.0;
        transform.translation.y = step_y * position.y as f32 - window.height() / 2.0 + height / 2.0;
    }
}
