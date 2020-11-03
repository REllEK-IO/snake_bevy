use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup.system())
        .add_system(snake_movement.system())
        .run();
}

struct Snake {
    speed: f32,
    direction: SnakeDirection
}

enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

enum Collider {
    Solid,
    Snake,
}

fn snake_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Snake, &mut Transform)>,
) {
    for (mut snake, mut transform) in query.iter_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;
        let timer = (time.seconds_since_startup * 100.00).floor().round();
        if timer % 50.0 == 0.0 {
            println!("The value of x is: {}", timer);
            match snake.direction {
                SnakeDirection::UP => transform.translation += Vec3::new(0.0, 25.0, 0.0),
                SnakeDirection::LEFT => transform.translation += Vec3::new(-25.0, 0.0, 0.0),
                SnakeDirection::RIGHT => transform.translation += Vec3::new(25.0, 0.0, 0.0),
                SnakeDirection::DOWN => transform.translation += Vec3::new(0.0, -25.0, 0.0),
                _ => println!("SNAKE!!!!!!"),
            }
        }
        if keyboard_input.pressed(KeyCode::Left) {
            snake.direction = SnakeDirection::LEFT;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            snake.direction = SnakeDirection::RIGHT;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            snake.direction = SnakeDirection::DOWN;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            snake.direction = SnakeDirection::UP;
        }

        transform.translation += Vec3::new(direction_x, direction_y, 0.0);
        // Vec3::new(time.delta_seconds, direction_x, direction_y)
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(25.0, 25.0)),
            ..Default::default()
        })
        .with(Snake { speed: 500.0, direction: SnakeDirection::RIGHT })
        .with(Collider::Snake);
        let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
        let wall_thickness = 10.0;
        let bounds = Vec2::new(900.0, 600.0);
    
    commands
        // left
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // right
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // bottom
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x() + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // top
        .spawn(SpriteComponents {
            material: wall_material,
            transform: Transform::from_translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x() + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid);
}