
pub mod fruit_logic {
    use bevy::prelude::*;
    use crate::snake::snake_data::*;
    use crate::game::game_data::*;
    use rand::*;
    pub fn fruit_spawner(
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
}