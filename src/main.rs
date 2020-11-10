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
        .add_resource(GameTimer(Timer::from_seconds(0.25, true)))
        // .add_resource( Grid {
        //     cells: Vec::new()
        // })
        .add_startup_system(setup.system())
        // .add_startup_system(grid_init.system())
        .add_system(restart.system())
        .add_system(game_over.system())
        .add_system(fruit_spawner.system())
        .add_system(snake_movement.system())
        .add_system(snake_collision.system())
        .add_system(grow_tail_listener.system())
        .add_system(move_tail_listener.system())
        .add_event::<EventGrowTail>()
        .add_event::<EventMoveTail>()
        .add_event::<EventGameOver>()
        .add_event::<EventRestart>()
        .run();
}
struct GameTimer(Timer);
struct Snake {
    position: Vec2,
    last_position: Vec2,
    direction: SnakeDirection,
    movement_locked: bool,
    next_move: SnakeDirection,
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
}

struct Tail {
    position: Vec2,
}

struct EventGrowTail {
}
struct EventMoveTail {
    position: Vec2,
}

struct EventGameOver{}
struct EventRestart{}

// struct Cell {
//     position: Vec2,
//     transformation: Vec3,
// }

// #[derive(Default)]
// struct Grid {
//     cells: Vec<Cell>
// }

#[derive(Copy, Clone)]
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
    Tail
}

// fn grid_init(
//     game: Res<GameState>,
//     mut grid: ResMut<Grid>,
// ){
//     let play_area = game.play_area as f64;
//     let grid_width = (play_area / game.cell_size / 2.0).round();
//     let mut x = (0.0 - grid_width).round();
//     let mut y = x;

//     while x <= grid_width && y <= grid_width{
//         grid.cells.push(Cell { 
//                 position: Vec2::new(
//                     x as f32,
//                     y as f32),
//                 transformation: Vec3::new(
//                     (x * game.cell_size).round() as f32,
//                     (y * game.cell_size).round() as f32,
//                     0.0)
//         });
//         if x >= grid_width {
//             x = (0.0 - grid_width).round();
//             y += 1.0;
//         }
//         x += 1.0;
//     }
//     for cell in grid.cells.iter() {
//         println!("{} & {}", cell.position, cell.transformation);
//     }
// }


fn snake_pos_to_translation(snake_pos: Vec2, c_size: f64) -> Vec3 {
    return Vec3::new((snake_pos.x() * c_size as f32).floor(), (snake_pos.y() * c_size as f32).floor(), 0.0);
}

fn snake_movement(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut move_tail: ResMut<Events<EventMoveTail>>,
    game: Res<GameState>,
    mut query: Query<(&mut Snake, &mut Transform)>,
    // mut tail_query: Query<(&mut Tail, &mut Transform)>,
){
    timer.0.tick(time.delta_seconds);
    if timer.0.finished && game.playing {
        for (mut snake, mut transform) in query.iter_mut() {
        
            snake.last_position = snake.position;
            match snake.direction {
                SnakeDirection::UP => snake.position = Vec2::new(snake.position.x(), snake.position.y() + 1.0),
                SnakeDirection::LEFT => snake.position = Vec2::new(snake.position.x() - 1.0, snake.position.y()),
                SnakeDirection::RIGHT => snake.position = Vec2::new(snake.position.x() + 1.0, snake.position.y()),
                SnakeDirection::DOWN => snake.position = Vec2::new(snake.position.x(), snake.position.y() - 1.0),
            }
            transform.translation = snake_pos_to_translation(snake.position, game.cell_size);
            move_tail.send(EventMoveTail{ position: snake.last_position });
            snake.movement_locked = false;
        }
    }
    for (mut snake, _) in query.iter_mut() {
        if  keyboard_input.pressed(KeyCode::Left) {
            match snake.direction {
                SnakeDirection::RIGHT => (),
                _ => {
                        snake.next_move = SnakeDirection::LEFT;
                    }
            }
        }

        if  keyboard_input.pressed(KeyCode::Right) {
            match snake.direction {
                SnakeDirection::LEFT => (),
                _ => {
                        snake.next_move = SnakeDirection::RIGHT;
                    }
            }
        }

        if  keyboard_input.pressed(KeyCode::Down) {
            match snake.direction {
                SnakeDirection::UP => (),
                _ => {
                        snake.next_move = SnakeDirection::DOWN;
                    }
            }
        }

        if  keyboard_input.pressed(KeyCode::Up) {
            match snake.direction {
                SnakeDirection::DOWN => (),
                _ => {
                        snake.next_move = SnakeDirection::UP;
                    }
            }
        }

        if !snake.movement_locked {
            snake.direction = snake.next_move;
            snake.movement_locked = true;
        }
    }
}

fn move_tail_listener(
    mut move_reader: Local<EventReader<EventMoveTail>>,
    move_event: Res<Events<EventMoveTail>>,
    mut tail_query: Query<(&mut Tail, &mut Transform)>,
){
    for move_event in move_reader.iter(&move_event){
        let mut last_pos = move_event.position;
        for (mut segment, mut segment_transform) in tail_query.iter_mut(){
            let next_pos = segment.position;
            segment.position = last_pos;
            last_pos = next_pos;
            segment_transform.translation = Vec3::new(segment.position.x() * 25.0, segment.position.y() * 25.0, 0.0);
        }
    }
}

fn grow_tail_listener(
    mut commands: Commands,
    mut grow_reader: Local<EventReader<EventGrowTail>>,
    game: Res<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grow_event: Res<Events<EventGrowTail>>,
    snake_query: Query<&Snake>,
) {
    for _ in grow_reader.iter(&grow_event){
        let cell_size = game.cell_size as f32;
        for snake in snake_query.iter(){
            // let mut last_pos = snake.position.clone();
            // for  mut segment in tail_query.iter_mut(){
            //     let next_pos = segment.position.clone();
            //     segment.position = last_pos;
            //     last_pos = next_pos;
            // }
            // println!("{}", last_pos);
            commands.spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(
                    snake.last_position.x() * cell_size,
                    snake.last_position.y()  * cell_size,
                    0.0
                )),
                    sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                    ..Default::default()
                })
                .with(Tail{
                    position: snake.last_position,
                })
                .with(Collider::Tail);
        }
    }
}

fn fruit_spawner(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game: Res<GameState>,
    fruit_query: Query<(Entity, &Fruit)>,
){
    let cell_size = game.cell_size as f32;
    let mut rng = rand::thread_rng();
    let rng_x: f32 = rng.gen();
    let rng_y: f32 = rng.gen();
    let max = (game.play_area / cell_size).round() - 2.0;
    let rand_x = (rng_x * max).round() * cell_size - (game.play_area / 2.0 - cell_size);
    let rand_y = (rng_y * max).round() * cell_size - (game.play_area / 2.0 - cell_size);

    if fruit_query.iter().len() == 0 && game.playing{
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                transform: Transform::from_translation(Vec3::new(rand_x, rand_y, 0.0)),
                sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                ..Default::default()
            })
            .with(Fruit {})
            .with(Collider::Fruit);
        println!("{} {}", rand_x, rand_y);
    }
}

fn snake_collision(
    mut commands: Commands,
    timer: Res<GameTimer>,
    mut game: ResMut<GameState>,
    mut grow_tail: ResMut<Events<EventGrowTail>>,
    mut game_over: ResMut<Events<EventGameOver>>,
    mut snake_query: Query<(Entity, &mut Snake)>,
    tail_query: Query<(Entity, &Tail)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
    fruit_query: Query<(Entity, &Fruit)>,
){
    let mut hit = false;
    if timer.0.finished && game.playing {
        for (_, snake) in snake_query.iter_mut() {
            for (_, collider, collider_transform) in collider_query.iter() {
                match collider {
                    Collider::Snake => {
                        let grid_max = (game.play_area / game.cell_size as f32 / 2.0).round();
                        if snake.position.x().abs() == grid_max || snake.position.y().abs() == grid_max{
                            hit = true;
                        }
                    },
                    Collider::Tail => {
                        for (_, tail_segment) in tail_query.iter(){
                            if snake.position.x() == tail_segment.position.x() && snake.position.y() == tail_segment.position.y() {
                                hit = true;
                            }
                        }
                    },
                    Collider::Fruit => {
                        let fruit_x = (collider_transform.translation.x() / game.cell_size as f32).round();
                        let fruit_y = (collider_transform.translation.y() / game.cell_size as f32).round();
                        if fruit_x == snake.position.x() && fruit_y == snake.position.y(){
                            game.score += 1;
                            grow_tail.send(EventGrowTail{});
                            for (fruit_entity, _) in fruit_query.iter() {
                                commands.despawn(fruit_entity);
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    if hit {
        game_over.send(EventGameOver{});
    }
}


fn game_over (
    mut commands: Commands,
    mut game_over_reader: Local<EventReader<EventGameOver>>,
    game_over_event: Res<Events<EventGameOver>>,
    mut game: ResMut<GameState>,
    mut restart: ResMut<Events<EventRestart>>,
    snake_query: Query<(Entity, &Snake)>,
    tail_query: Query<(Entity, &Tail)>,
    fruit_query: Query<(Entity, &Fruit)>
) {
    for _ in game_over_reader.iter(&game_over_event) {
        println!("GAME OVER");
        game.playing = false;
        for (snake_entity, _) in snake_query.iter() {
            commands.despawn_recursive(snake_entity);
        }
        for (tail_entity, _) in tail_query.iter() {
            commands.despawn_recursive(tail_entity);
        }
        for (fruit_entity, _) in fruit_query.iter() {
            commands.despawn_recursive(fruit_entity);
        }
        restart.send(EventRestart {});
    }
} 

fn restart (
    mut commands: Commands,
    mut restart_reader: Local<EventReader<EventRestart>>,
    restart_event: Res<Events<EventRestart>>,
    mut game: ResMut<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !game.playing {
        for _ in restart_reader.iter(&restart_event) {
            println!("RESTART");
            let cell_size = game.cell_size as f32;
            let snake_pos = Vec2::new(0.0, -6.0);
            let last_pos = Vec2::new(-1.0, -6.0);
            commands
                .spawn(Camera2dComponents::default())
                .spawn(UiCameraComponents::default())
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                    transform: Transform::from_translation(Vec3::new(0.0, snake_pos.y() * game.cell_size as f32, 0.0)),
                    sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                    ..Default::default()
                })
                .with(Snake { 
                    direction: SnakeDirection::RIGHT,
                    position: snake_pos,
                    last_position: last_pos,
                    movement_locked: false,
                    next_move: SnakeDirection::RIGHT
                })
                .with(Collider::Snake);
            game.playing = true;
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
    let last_pos = Vec2::new(-1.0, -6.0);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, snake_pos.y() * game.cell_size as f32, 0.0)),
            sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
            ..Default::default()
        })
        .with(Snake { 
            direction: SnakeDirection::RIGHT,
            position: snake_pos,
            last_position: last_pos,
            movement_locked: false,
            next_move: SnakeDirection::RIGHT
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