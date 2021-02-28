use bevy::prelude::*;
use bevy_grid::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GridPlugin)
        .add_resource(Grid {
            mode: GridMode::Adjust,
            width: 80,
            height: 80,
        })
        .add_startup_system(setup.system())
        .add_system(move_player.system())
        .add_system(change_grid.system())
        .add_system(change_mode.system())
        .run();
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle::default())
        .with_bundle(GridBundle {
            size: bevy_grid::Size::new(2, 8),
            ..Default::default()
        });

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            ..Default::default()
        })
        .with_bundle(GridBundle {
            size: bevy_grid::Size::new(4, 2),
            position: Position::new(2, 6),
        });
}

fn move_player(mut player_query: Query<&mut Position>, keyboard_input: Res<Input<KeyCode>>) {
    for mut position in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::S) {
            position.y += 1;
        }
        if keyboard_input.just_pressed(KeyCode::W) {
            position.y -= 1;
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

fn change_mode(mut grid: ResMut<Grid>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::F) {
        grid.mode = GridMode::Fit;
    }

    if keyboard_input.just_pressed(KeyCode::X) {
        grid.mode = GridMode::FitX;
    }

    if keyboard_input.just_pressed(KeyCode::Y) {
        grid.mode = GridMode::FitY;
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        grid.mode = GridMode::Adjust;
    }
}
