pub mod snake_functions {
    use super::snake_data::*;
    use crate::game::game_data::*;
    use bevy::prelude::*;

    pub fn snake_movement(
        time: Res<Time>,
        mut timer: ResMut<GameTimer>,
        keyboard_input: Res<Input<KeyCode>>,
        mut move_tail: ResMut<Events<EventMoveTail>>,
        game: Res<GameState>,
        mut query: Query<(&mut Snake, &mut Transform)>,
    ) {
        timer.0.tick(time.delta_seconds);
        if timer.0.finished && game.playing {
            for (mut snake, mut transform) in query.iter_mut() {
                snake.last_position = snake.position;
                snake.position = snake.position + snake.direction.into_vec2();
                transform.translation = snake_pos_to_translation(snake.position, game.cell_size);
                move_tail.send(EventMoveTail {
                    position: snake.last_position,
                });
                snake.movement_locked = false;
            }
        }

        for (mut snake, _) in query.iter_mut() {
            for direction in SnakeDirection::enumerate() {
                if keyboard_input.pressed(direction.into_keycode())
                    && !direction.is_inverse(snake.direction)
                {
                    snake.next_move = direction;
                }
            }

            if !snake.movement_locked {
                snake.direction = snake.next_move;
                snake.movement_locked = true;
            }
        }
    }

    pub fn snake_collision(
        mut commands: Commands,
        timer: Res<GameTimer>,
        mut game: ResMut<GameState>,
        mut grow_tail: ResMut<Events<EventGrowTail>>,
        mut game_over: ResMut<Events<EventGameOver>>,
        snake_query: Query<(Entity, &Snake)>,
        tail_query: Query<(Entity, &Tail)>,
        collider_query: Query<(Entity, &Collider, &Transform)>,
        fruit_query: Query<(Entity, &Fruit)>,
    ) {
        let mut hit = false;
        if timer.0.finished && game.playing {
            for (_, snake) in snake_query.iter() {
                for (_, collider, collider_transform) in collider_query.iter() {
                    match collider {
                        Collider::Snake => {
                            let grid_max = (game.play_area / game.cell_size as f32 / 2.0).round();
                            if snake.position.x().abs() == grid_max
                                || snake.position.y().abs() == grid_max
                            {
                                hit = true;
                            }
                        }

                        Collider::Tail => {
                            for (_, tail_segment) in tail_query.iter() {
                                if snake.position.x() == tail_segment.position.x()
                                    && snake.position.y() == tail_segment.position.y()
                                {
                                    hit = true;
                                }
                            }
                        }

                        Collider::Fruit => {
                            let fruit_x = (collider_transform.translation.x()
                                / game.cell_size as f32)
                                .round();
                            let fruit_y = (collider_transform.translation.y()
                                / game.cell_size as f32)
                                .round();
                            if fruit_x == snake.position.x() && fruit_y == snake.position.y() {
                                game.score += 1;
                                grow_tail.send(EventGrowTail {});
                                for (fruit_entity, _) in fruit_query.iter() {
                                    commands.despawn(fruit_entity);
                                }
                                println!(" S C O R E : {} !", game.score);
                                break;
                            }
                            for (_, segment) in tail_query.iter() {
                                if fruit_x == segment.position.x()
                                    && fruit_y == segment.position.y()
                                {
                                    game.score += 1;
                                    grow_tail.send(EventGrowTail {});
                                    for (fruit_entity, _) in fruit_query.iter() {
                                        commands.despawn(fruit_entity);
                                    }
                                    println!(" S C O R E : {} !", game.score);
                                    break;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        if hit {
            game_over.send(EventGameOver {});
        }
    }

    fn snake_pos_to_translation(snake_pos: Vec2, c_size: f64) -> Vec3 {
        return Vec3::new(
            (snake_pos.x() * c_size as f32).floor(),
            (snake_pos.y() * c_size as f32).floor(),
            0.0,
        );
    }

    pub fn move_tail_listener(
        mut move_reader: Local<EventReader<EventMoveTail>>,
        move_event: Res<Events<EventMoveTail>>,
        mut tail_query: Query<(&mut Tail, &mut Transform)>,
    ) {
        for move_event in move_reader.iter(&move_event) {
            let mut last_pos = move_event.position;
            for (mut segment, mut segment_transform) in tail_query.iter_mut() {
                let next_pos = segment.position;
                segment.position = last_pos;
                last_pos = next_pos;
                segment_transform.translation = Vec3::new(
                    segment.position.x() * 25.0,
                    segment.position.y() * 25.0,
                    0.0,
                );
            }
        }
    }

    pub fn grow_tail_listener(
        mut commands: Commands,
        mut grow_reader: Local<EventReader<EventGrowTail>>,
        game: Res<GameState>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        grow_event: Res<Events<EventGrowTail>>,
        snake_query: Query<&Snake>,
    ) {
        for _ in grow_reader.iter(&grow_event) {
            let cell_size = game.cell_size as f32;
            for snake in snake_query.iter() {
                commands
                    .spawn(SpriteComponents {
                        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                        transform: Transform::from_translation(Vec3::new(
                            snake.last_position.x() * cell_size,
                            snake.last_position.y() * cell_size,
                            0.0,
                        )),
                        sprite: Sprite::new(Vec2::new(cell_size - 2.0, cell_size - 2.0)),
                        ..Default::default()
                    })
                    .with(Tail {
                        position: snake.last_position,
                    })
                    .with(Collider::Tail);
            }
        }
    }
}

pub mod snake_data {
    use bevy::prelude::*;

    pub struct Fruit;

    pub struct Snake {
        pub position: Vec2,
        pub last_position: Vec2,
        pub direction: SnakeDirection,
        pub movement_locked: bool,
        pub next_move: SnakeDirection,
    }

    pub struct Tail {
        pub position: Vec2,
    }

    pub struct EventGrowTail;
    pub struct EventMoveTail {
        pub position: Vec2,
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum SnakeDirection {
        Up,
        Down,
        Left,
        Right,
    }

    pub enum Collider {
        Solid,
        Snake,
        Fruit,
        Tail,
    }

    impl SnakeDirection {
        pub fn into_vec2(self) -> Vec2 {
            use SnakeDirection::*;
            match self {
                Up => Vec2::new(0.0, 1.0),
                Left => Vec2::new(-1.0, 0.0),
                Right => Vec2::new(1.0, 0.0),
                Down => Vec2::new(0.0, -1.0),
            }
        }

        pub fn inverse(self) -> Self {
            use SnakeDirection::*;
            match self {
                Up => Down,
                Left => Right,
                Right => Left,
                Down => Up,
            }
        }

        pub fn is_inverse(self, other: Self) -> bool {
            self.inverse() == other
        }

        pub fn into_keycode(self) -> KeyCode {
            use SnakeDirection::*;
            match self {
                Up => KeyCode::Up,
                Down => KeyCode::Down,
                Right => KeyCode::Right,
                Left => KeyCode::Left,
            }
        }

        pub fn enumerate() -> impl Iterator<Item = Self> {
            use SnakeDirection::*;
            [Up, Down, Left, Right].iter().copied()
        }
    }
}
