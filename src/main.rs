use bevy::prelude::*;
use snake_game::{
    handle_player_input,
    move_snake_head,
    project_positions,
    spawn_camera,
    spawn_snake,
    wrap_snake_position,
    test_position,
    move_snake_body,
};
fn main() {
    let mut app=App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_snake,
        ))
        // 确保系统执行顺序
        .add_systems(Update, (
            handle_player_input,
            wrap_snake_position,
            move_snake_body,
            move_snake_head.after(move_snake_body),
            project_positions.after(move_snake_head),
            test_position,
        ).chain());
    app.run();
}
