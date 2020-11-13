pub mod game_data {
    use bevy::prelude::*;
    pub struct GameTimer(pub Timer);
    pub struct EventGameOver{}
    pub struct EventRestart{}
    pub struct ScoreText;
}

pub mod game_functions {
    use bevy::prelude::*;
    use super::game_data::*;
    use crate::snake::snake_data::*;

    pub fn game_over (
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
    
    pub fn restart (
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
    
    pub fn update_score (
        game: Res<GameState>,
        mut score_query: Query<(&mut Text, &ScoreText)>
    ) {
        for (mut text, _) in score_query.iter_mut() {
            text.value = format!("Score {}", game.score);
        }
    }
}