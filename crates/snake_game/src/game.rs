pub mod game_data {
    use bevy::prelude::*;
    #[derive(Default)]
    pub struct GameState{
        // difficulty: f64, Todo with ui
        pub score: usize,
        pub playing: bool,
        pub play_area: f32,
        pub cell_size: f64,
        pub prev_scores: Vec<usize>,
    }
    pub struct GameTimer(pub Timer);
    pub struct EventGameOver;
    pub struct EventRestart;
    pub struct EventUpdateScores;
    pub struct ScoreText;
    pub struct PrevScoreText;
}

pub mod game_functions {
    use bevy::prelude::*;
    use super::game_data::*;
    use crate::snake::snake_data::*;

    pub fn game_over (
        mut commands: Commands,
        mut game_over_event: EventReader<EventGameOver>,
        mut game: ResMut<GameState>,
        mut restart: EventWriter<EventRestart>,
        snake_query: Query<(Entity, &Snake)>,
        tail_query: Query<(Entity, &Tail)>,
        fruit_query: Query<(Entity, &Fruit)>
    ) {
        for _ in game_over_event.iter() {
            println!("GAME OVER");
            let mut new_scores: Vec<usize> = Vec::new();
            let mut bump_down: usize = 9999999999999;
            let score = &game.score;
            for scores in game.prev_scores.iter() {
                if scores > score {
                    new_scores.push(*scores);
                } else if scores <= score && bump_down == 9999999999999 {
                    bump_down = *scores;
                    new_scores.push(game.score);
                } else if *score != 0{
                    new_scores.push(bump_down);
                    bump_down = *scores;
                } else {
                    new_scores.push(0);
                }
            }

            game.prev_scores = new_scores;
            game.score = 0;
            game.playing = false;
            for (snake_entity, _) in snake_query.iter() {
                commands.entity(snake_entity).despawn_recursive();
            }
            for (tail_entity, _) in tail_query.iter() {
                commands.entity(tail_entity).despawn_recursive();
            }
            for (fruit_entity, _) in fruit_query.iter() {
                commands.entity(fruit_entity).despawn_recursive();
            }
            restart.send(EventRestart {});
        }
    } 
    
    pub fn restart (
        mut commands: Commands,
        mut restart_event: EventReader<EventRestart>,
        mut game: ResMut<GameState>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        if !game.playing {
            for _ in restart_event.iter() {
                println!("RESTART");
                let cell_size = game.cell_size as f32;
                let snake_pos = Vec2::new(0.0, -6.0);
                let last_pos = Vec2::new(-1.0, -6.0);
                commands
                    .spawn_bundle(SpriteBundle {
                        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                        transform: Transform::from_translation(Vec3::new(0.0, snake_pos.y * game.cell_size as f32, 0.0)),
                        sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                        ..Default::default()
                    })
                    .insert(Snake { 
                        direction: SnakeDirection::RIGHT,
                        position: snake_pos,
                        last_position: last_pos,
                        movement_locked: false,
                        next_move: SnakeDirection::RIGHT
                    })
                    .insert(Collider::Snake);
                game.playing = true;
            }
        }
    }
}