use  bevy::prelude::*;
#[derive(Component,Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component)]
#[require(Position,Velocity(Vec2::new(0., 0.)))]
pub struct Snake;

#[derive(Component,Default)]
pub struct Velocity(Vec2);

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
        mesh.add(Rectangle::new(20.,40.)),
        mesh.add(Rectangle::new(20.,40.)),
    ];
    let color=Color::hsl(1., 0.5, 0.75);
    let mut position = Vec2::new(20., 20.);
    
    for (i, shape) in shape.into_iter().enumerate() {
        // 由于 Vec2 不能直接乘以 usize，将 usize 转换为 f32 后再进行乘法运算
        position += position * i as f32;
        commands.spawn((
            Snake,
            Position(position),
            Velocity(Vec2::new(1., 0.)),
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
        ));
    }
}

pub fn project_positions(
    mut query:Query<(&Position,&mut Transform)>,    
){
    for (position,mut transform) in query.iter_mut(){
        transform.translation=position.0.extend(0.);
    }
}
pub fn move_snake(mut snake:Query<(&mut Position,&Velocity),With<Snake>>){
    if let Ok((mut position,velocity))=snake.single_mut(){
        position.0 += velocity.0;
    }
}

pub fn wrap_snake_position(
    mut snake: Query<&mut Position, With<Snake>>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let width = window.width()/2.;
        let height = window.height()/2.;
        
        if let Ok(mut position) = snake.single_mut() {
            // 水平边界检测
            if position.0.x > width {
                position.0.x = -width;
            } else if position.0.x < -width {
                position.0.x = width;
            }
            
            // 垂直边界检测
            if position.0.y > height {
                position.0.y = -height;
            } else if position.0.y < -height {
                position.0.y = height;
            }
        }
    }
}

pub fn handle_player_input(
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut snake:Query<&mut Velocity,With<Snake>>,
){
    if let Ok(mut velocity)=snake.single_mut(){
        if keyboard_input.just_pressed(KeyCode::ArrowDown){
            println!("向下");
            velocity.0 = Vec2::new(0., -velocity.0.length());
            //position.0.y-=10.;
            println!("{:?}",velocity.0);
        }else if keyboard_input.just_pressed(KeyCode::ArrowUp){
            println!("向上");
            velocity.0 = Vec2::new(0., velocity.0.length());
            println!("{:?}",velocity.0);
        }else if keyboard_input.just_pressed(KeyCode::ArrowLeft){
            println!("向左");
            velocity.0 = Vec2::new(-velocity.0.length(),0. );
            println!("{:?}",velocity.0);
        }else if keyboard_input.just_pressed(KeyCode::ArrowRight){
            println!("向右");
            velocity.0 = Vec2::new(velocity.0.length(),0. );
            println!("{:?}",velocity.0);
        }
    }
}