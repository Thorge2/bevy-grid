use bevy::prelude::*;
use bevy_grid::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GridPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteBundle::default())
        .with_bundle(GridBundle::default());
}
