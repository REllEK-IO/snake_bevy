pub mod game_data {
    use bevy::prelude::*;
    #[derive(Default)]
    pub struct GameState {
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
    use super::game_data::*;
    use crate::snake::snake_data::*;
    use bevy::prelude::*;

    pub fn game_over(
        mut commands: Commands,
        mut game_over_reader: Local<EventReader<EventGameOver>>,
        game_over_event: Res<Events<EventGameOver>>,
        mut game: ResMut<GameState>,
        mut restart: ResMut<Events<EventRestart>>,
        snake_query: Query<(Entity, &Snake)>,
        tail_query: Query<(Entity, &Tail)>,
        fruit_query: Query<(Entity, &Fruit)>,
    ) {
        for _ in game_over_reader.iter(&game_over_event) {
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
                } else if *score != 0 {
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

    pub fn restart(
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
                        transform: Transform::from_translation(Vec3::new(
                            0.0,
                            snake_pos.y() * game.cell_size as f32,
                            0.0,
                        )),
                        sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                        ..Default::default()
                    })
                    .with(Snake {
                        direction: SnakeDirection::Right,
                        position: snake_pos,
                        last_position: last_pos,
                        movement_locked: false,
                        next_move: SnakeDirection::Right,
                    })
                    .with(Collider::Snake);
                game.playing = true;
            }
        }
    }
}
