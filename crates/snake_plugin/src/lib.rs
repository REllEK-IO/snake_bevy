#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub use snake_game:: {
    snake::snake_functions::*,
    snake::snake_data::*,
    game::game_data::*,
    game::game_functions::*,
};

