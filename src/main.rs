use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use rand::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(GameState { 
            difficulty: 50.0,
            score: 0,
            playing: true, 
            play_area: 600.0,
        })
        .add_startup_system(setup.system())
        .add_system(snake_movement.system())
        .add_system(snake_collision.system())
        .add_system(fruit_spawner.system())
        .run();
}

struct Snake {
    head_size: f32,
    direction: SnakeDirection,
    movement_locked: bool,
}

#[derive(Default)]
struct GameState{
    difficulty: f64,
    score: usize,
    playing: bool,
    play_area: f32,
}
struct Fruit {
    blink_state: bool,
}

struct Tail {
    position: u8,
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
    Fruit,
}

fn snake_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    game: Res<GameState>,
    mut query: Query<(&mut Snake, &mut Transform)>,
){
    for (mut snake, mut transform) in query.iter_mut() {
        let timer = (time.seconds_since_startup * 100.00).floor().round();
        if timer % game.difficulty == 0.0 && game.playing {
            match snake.direction {
                SnakeDirection::UP => transform.translation += Vec3::new(0.0, snake.head_size, 0.0),
                SnakeDirection::LEFT => transform.translation += Vec3::new(-1.0 * snake.head_size, 0.0, 0.0),
                SnakeDirection::RIGHT => transform.translation += Vec3::new(snake.head_size, 0.0, 0.0),
                SnakeDirection::DOWN => transform.translation += Vec3::new(0.0, -1.0 * snake.head_size, 0.0),
                _ => println!("SNAKE!!!!!!"),
            }
            snake.movement_locked = false;
        }

        if keyboard_input.pressed(KeyCode::Left) && !snake.movement_locked {
            match snake.direction {
                SnakeDirection::RIGHT => (),
                _ => {
                        snake.direction = SnakeDirection::LEFT;
                        snake.movement_locked = true;
                    }
            }
        }

        if keyboard_input.pressed(KeyCode::Right) && !snake.movement_locked {
            match snake.direction {
                SnakeDirection::LEFT => (),
                _ => {
                        snake.direction = SnakeDirection::RIGHT;
                        snake.movement_locked = true;
                    }
            }
        }

        if keyboard_input.pressed(KeyCode::Down) && !snake.movement_locked {
            match snake.direction {
                SnakeDirection::UP => (),
                _ => {
                        snake.direction = SnakeDirection::DOWN;
                        snake.movement_locked = true;
                    }
            }
        }

        if keyboard_input.pressed(KeyCode::Up) && !snake.movement_locked {
            match snake.direction {
                SnakeDirection::DOWN => (),
                _ => {
                        snake.direction = SnakeDirection::UP;
                        snake.movement_locked = true;
                    }
            }
        }
    }
}

// fn snake_tail(
//     game: Ref<GameState>,
//     snake_query: Query<(&Snake, &Transform, &Sprite)>,
// ){

// }

fn fruit_spawner(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game: Res<GameState>,
    fruit_query: Query<&Fruit>,
){
    let mut rng = rand::thread_rng();
    let rng_x: f32 = rng.gen();
    let rng_y: f32 = rng.gen();
    let max = (game.play_area / 25.0).floor().round();
    let rand_x = (rng_x * max).floor().round() * 25.0 - (game.play_area / 2.0 - 25.0);
    let rand_y = (rng_y * max).floor().round() * 25.0 - (game.play_area / 2.0 - 25.0);

    if fruit_query.iter().len() == 0 {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(rand_x, rand_y, 0.0)),
                sprite: Sprite::new(Vec2::new(25.0, 25.0)),
                ..Default::default()
            })
            .with(Fruit { blink_state: false} )
            .with(Collider::Fruit);
    }
}

fn snake_collision(
    mut commands: Commands,
    mut game: ResMut<GameState>,
    mut snake_query: Query<(Entity, &mut Snake, &Transform, &Sprite)>,
    collider_query: Query<(Entity, &Collider, &Transform, &Sprite)>,
    mut fruit_query: Query<(Entity, &Fruit)>,
){
    for (snake_entity, mut snake, snake_transform, snake_sprite) in snake_query.iter_mut() {
        let mut snake_offset = snake_transform.translation.clone();
        if snake_transform.translation.x() > 0.0 {
            snake_offset += Vec3::new(-1.0, 0.0, 0.0);
        }
        if snake_transform.translation.x() < 0.0 {
            snake_offset += Vec3::new(1.0, 0.0, 0.0);
        }
        if snake_transform.translation.y() > 0.0 {
            snake_offset += Vec3::new(0.0, -1.0, 0.0);
        }
        if snake_transform.translation.y() < 0.0 {
            snake_offset += Vec3::new(0.0,1.0, 0.0);
        }

        // Need an inclusive collider snake offset is a hack
        for (collider_entity, collider, collider_transform, collider_sprite) in collider_query.iter() {
            let collision = collide(
                snake_offset,
                snake_sprite.size,
                collider_transform.translation,
                collider_sprite.size
            );
            match collider {
                Collider::Solid => {
                    match collision {
                        None => (),
                        _ => {
                            // Collides with wall or solid, despawns snake head
                            commands.despawn(snake_entity);
                        },
                    }
                },
                Collider::Fruit => {
                    match collision {
                        None => (),
                        _ => {
                            for (fruit_entity, fruit) in fruit_query.iter() {
                                commands.despawn(fruit_entity);
                                game.score += 1;
                            }
                            println!("NOM! SCORE: {}", game.score);
                        },
                    }
                }
                _ => (),
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    game: Res<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){  
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            sprite: Sprite::new(Vec2::new(25.0, 25.0)),
            ..Default::default()
        })
        .with(Snake { head_size: 25.0, direction: SnakeDirection::RIGHT, movement_locked: false })
        .with(Collider::Snake);
        let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
        let wall_thickness = 25.0;
        let bounds = Vec2::new(game.play_area, game.play_area);
    
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