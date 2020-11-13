use bevy::{
    prelude::*,
    render::pass::ClearColor,
};
use rand::prelude::*;

use snake_plugin::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::BLACK))
        .add_resource(GameState { 
            // difficulty: 25.0,
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
        .add_system(update_score.system())
        .add_event::<EventGrowTail>()
        .add_event::<EventMoveTail>()
        .add_event::<EventGameOver>()
        .add_event::<EventRestart>()
        .run();
}

// struct Cell {
//     position: Vec2,
//     transformation: Vec3,
// }

// #[derive(Default)]
// struct Grid {
//     cells: Vec<Cell>
// }

// Abstracted grid format for game logic
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

fn fruit_spawner(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game: Res<GameState>,
    fruit_query: Query<(Entity, &Fruit)>,
    snake_query: Query<&Snake>,
    tail_query: Query<&Tail>
){
    let cell_size = game.cell_size as f32;
    let mut rng = rand::thread_rng();    
    let mut rand_x: f32 = 0.0;
    let mut rand_y: f32 = 0.0;
    let max = (game.play_area / cell_size).round() - 2.0;

    for snake in snake_query.iter() {
        let mut known_positions: Vec<Vec2> = Vec::new();
        known_positions.push(snake.position);

        for segment in tail_query.iter() {
            known_positions.push(segment.position);
        }

        let mut overlaps = true;

        while overlaps {
            let rng_x: f32 = rng.gen();
            let rng_y: f32 = rng.gen();
            rand_x = (rng_x * max).round() * cell_size - (game.play_area / 2.0 - cell_size);
            rand_y = (rng_y * max).round() * cell_size - (game.play_area / 2.0 - cell_size);
            let mut found_overlap = false;

            for pos in known_positions.iter() {
                if rand_x == pos.x() && rand_y == pos.y(){
                    found_overlap = true;
                    break;
                }
            }

            if !found_overlap {
                overlaps = false;
            }
        }
    }

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
    }
}

fn setup(
    mut commands: Commands,
    game: Res<GameState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
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
            direction: snake_plugin::SnakeDirection::RIGHT,
            position: snake_pos,
            last_position: last_pos,
            movement_locked: false,
            next_move: snake_plugin::SnakeDirection::RIGHT
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
    println!("SNAKE!");

    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position: Rect {
                    left: Val::Percent(80.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                value: "Score".to_string(),
                font: asset_server.load("fonts/arcade.ttf"),
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .with(ScoreText);
}