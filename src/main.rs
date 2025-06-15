use bevy::prelude::*;
use game::{
    handle_player_input,
    move_snake,
    project_positions,
    spawn_camera,
    spawn_snake,
    wrap_snake_position,
};
fn main() {
    let mut app=App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_snake,
        ))
        .add_systems(Update, (
            move_snake,
            handle_player_input.after(move_snake),
            project_positions.after(handle_player_input),
            wrap_snake_position,
        ));
    app.run();
}
