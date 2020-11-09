use bevy::{
    prelude::*,
    render::pass::ClearColor,
};
use rand::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(GameState { 
            difficulty: 25.0,
            score: 0,
            playing: true, 
            play_area: 600.0,
            cell_size: 25.0
        })
        .add_resource( Grid {
            cells: Vec::new()
        })
        .add_startup_system(setup.system())
        .add_startup_system(grid_init.system())
        .add_system(snake_movement.system())
        .add_system(snake_collision.system())
        .add_system(fruit_spawner.system())
        .run();
}

struct Snake {
    head_size: f32,
    position: Vec2,
    direction: SnakeDirection,
    movement_locked: bool,
}

#[derive(Default)]
struct GameState{
    difficulty: f64,
    score: usize,
    playing: bool,
    play_area: f32,
    cell_size: f64,
}
struct Fruit {
    blink_state: bool,
    position: Vec2,
}

struct Tail {
    position: u8,
}


struct Cell {
    position: Vec2,
    transformation: Vec3,
}

#[derive(Default)]
struct Grid {
    cells: Vec<Cell>
}

enum SnakeDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn grid_init(
    game: Res<GameState>,
    mut grid: ResMut<Grid>,
){
    let play_area = game.play_area as f64;
    let grid_width = (play_area / game.cell_size / 2.0).round();
    let mut x = (0.0 - grid_width).round();
    let mut y = x;

    while x <= grid_width && y <= grid_width{
        grid.cells.push(Cell { 
                position: Vec2::new(
                    x as f32,
                    y as f32),
                transformation: Vec3::new(
                    (x * game.cell_size).round() as f32,
                    (y * game.cell_size).round() as f32,
                    0.0)
        });
        if x >= grid_width {
            x = (0.0 - grid_width).round();
            y += 1.0;
        }
        x += 1.0;
    }
    for cell in grid.cells.iter() {
        println!("{} & {}", cell.position, cell.transformation);
    }
}

enum Collider {
    Solid,
    Snake,
    Fruit,
}

fn snake_pos_to_translation(snake_pos: Vec2, c_size: f64) -> Vec3 {
    return Vec3::new((snake_pos.x() * c_size as f32).floor(), (snake_pos.y() * c_size as f32).floor(), 0.0);
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
                SnakeDirection::UP => snake.position = Vec2::new(snake.position.x(), snake.position.y() + 1.0),
                SnakeDirection::LEFT => snake.position = Vec2::new(snake.position.x() - 1.0, snake.position.y()),
                SnakeDirection::RIGHT => snake.position = Vec2::new(snake.position.x() + 1.0, snake.position.y()),
                SnakeDirection::DOWN => snake.position = Vec2::new(snake.position.x(), snake.position.y() - 1.0),
            }
            transform.translation = snake_pos_to_translation(snake.position, game.cell_size);
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
    let cell_size = game.cell_size as f32;
    let mut rng = rand::thread_rng();
    let rng_x: f32 = rng.gen();
    let rng_y: f32 = rng.gen();
    let max = (game.play_area / cell_size).round() - 1.0;
    let rand_x = (rng_x * max).round() * cell_size - (game.play_area / 2.0 - cell_size);
    let rand_y = (rng_y * max).round() * cell_size - (game.play_area / 2.0 - cell_size);

    if fruit_query.iter().len() == 0 {
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(rand_x, rand_y, 0.0)),
                sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                ..Default::default()
            })
            .with(Fruit {
                blink_state: false,
                position: Vec2::new((rand_x / cell_size).round(), (rand_y / cell_size).round()),
            })
            .with(Collider::Fruit);
    }
}

fn snake_collision(
    mut commands: Commands,
    mut game: ResMut<GameState>,
    mut snake_query: Query<(Entity, &mut Snake)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
    fruit_query: Query<(Entity, &Fruit)>,
){
    for (snake_entity, snake) in snake_query.iter_mut() {
        for (_, collider, collider_transform) in collider_query.iter() {
            match collider {
                Collider::Snake => {
                    let grid_max = (game.play_area / game.cell_size as f32 / 2.0).round();
                    if snake.position.x().abs() == grid_max || snake.position.y().abs() == grid_max{
                        commands.despawn(snake_entity);  
                    }
                },
                Collider::Fruit => {
                    let fruit_x = (collider_transform.translation.x() / game.cell_size as f32).round();
                    let fruit_y = (collider_transform.translation.y() / game.cell_size as f32).round();
                    if fruit_x == snake.position.x() && fruit_y == snake.position.y(){
                        for (fruit_entity, _) in fruit_query.iter() {
                            commands.despawn(fruit_entity);
                            game.score += 1;
                        }
                    }
                    println!("NOM! SCORE: {}", game.score);
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
    let cell_size = game.cell_size as f32;
    let snake_pos = Vec2::new(0.0, -6.0);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, snake_pos.y() * game.cell_size as f32, 0.0)),
            sprite: Sprite::new(Vec2::new(cell_size, cell_size)),
            ..Default::default()
        })
        .with(Snake { 
            head_size: cell_size,
            direction: SnakeDirection::RIGHT,
            position: snake_pos,
            movement_locked: false
        })
        .with(Collider::Snake);
        let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
        let wall_thickness = cell_size;
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