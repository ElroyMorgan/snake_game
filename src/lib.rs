use  bevy::{prelude::*};

#[derive(Component,Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

type ID=usize;

#[derive(Component,PartialEq, Eq, PartialOrd, Ord)]
#[require(Position,Velocity(Vec2::new(0., 0.)))]
pub struct Snake;

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeTail(ID);

#[derive(Component,Default)]
pub struct Velocity(Vec2);

#[derive(Component)]
#[require(Position)]
pub struct Food;


#[allow(dead_code)]
pub fn spawn_camera(mut commands:Commands){
    commands.spawn(Camera2d);
}

pub fn spawn_snake(
    mut commands:Commands,
    mut mesh:ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<ColorMaterial>>,
){
    let shape=[
        mesh.add(Circle::new(20.)),
        mesh.add(Rectangle::new(40.,40.)),
        mesh.add(Rectangle::new(40.,40.)),
    ];
    let color=Color::hsl(1., 0.5, 0.75);
    let color1=Color::hsl(120., 0.5, 0.75);
    for (i, shape) in shape.into_iter().enumerate() {
        let position_x = -50_f32 * i as f32;  
        if i==0 {
            commands.spawn((
                Snake,
                SnakeHead,
                Position(Vec2::new(position_x, 0.)),
                Velocity(Vec2::new(50., 0.)),
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
            ));
        }else {
            commands.spawn((
                Snake,
                SnakeTail(i),
                Position(Vec2::new(position_x, 0.)),
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color1)),
            ));
        }
        
    }
}

pub fn project_positions(
    mut query:Query<(&Position,&mut Transform)>,    
){
    for (position,mut transform) in query.iter_mut(){
        transform.translation=position.0.extend(0.);
    }
}
pub fn move_snake_head(
    snake: Query<(&mut Position,&Velocity), With<SnakeHead>>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_secs_f64() as f32;
        if *timer >= 0.1 {
            for (mut position, velocity) in snake {
                position.0 += velocity.0;
            }
            *timer = 0.0;
        }
}

pub fn wrap_snake_position(
    mut snake: Query<&mut Position, With<SnakeHead>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let width = window.width()/2.;
        let height = window.height()/2.;
                if let Ok(mut position)=snake.single_mut(){
                    if position.0.x <= -width{
                        position.0.x=width-40.;
                    }
                    if position.0.x >= width{
                        position.0.x=-width+40.;
                    }
                    if position.0.y <= -height{
                        position.0.y=height-40.;
                    }
                    if position.0.y >= height{
                        position.0.y=-height+40.;
                    }
                }
        }
}

pub fn handle_player_input(
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut snake:Query<&mut Velocity, With<SnakeHead>>,
){
    if let Ok(mut velocity) = snake.single_mut() {
        let current_dir = velocity.0.normalize_or_zero();
        
        if keyboard_input.just_pressed(KeyCode::ArrowUp) && current_dir.y == 0.0 {
            velocity.0 = Vec2::new(0., velocity.0.length());
        } else if keyboard_input.just_pressed(KeyCode::ArrowDown) && current_dir.y == 0.0 {
            velocity.0 = Vec2::new(0., -velocity.0.length());
        } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) && current_dir.x == 0.0 {
            velocity.0 = Vec2::new(-velocity.0.length(), 0.);
        } else if keyboard_input.just_pressed(KeyCode::ArrowRight) && current_dir.x == 0.0 {
            velocity.0 = Vec2::new(velocity.0.length(), 0.);
        }
    }
}

pub fn move_snake_body(
    mut query: Query<(&mut Position, &SnakeTail), Without<SnakeHead>>,
    head: Query<&Position, With<SnakeHead>>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {

    *timer += time.delta_secs_f64() as f32;
        if *timer >= 0.1 {
            let mut entities: Vec<_> = query.iter_mut().collect();
    entities.sort_by_key(|(_, snake)| snake.0);
    
    // 先保存所有位置的快照
    let positions: Vec<Vec2> = entities.iter().map(|(pos, _)| pos.0).collect();
    
    // 然后更新位置（跳过头部）
    for i in 1..entities.len() {
        entities[i].0.0 = positions[i-1];
    }
    // 最后更新头部位置
    if let Ok(head_pos) = head.single() {
        entities[0].0.0 = head_pos.0;
    }
            *timer = 0.0;
        }  
}

pub fn test_position(
    query: Query<&Position, With<Snake>>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
*timer += time.delta_secs_f64() as f32;
    if *timer >= 1.0 {
        let positions: Vec<Vec2> = query.iter().map(|position| position.0).collect();
        println!("{:?}", positions);
        *timer = 0.0;
    }
}