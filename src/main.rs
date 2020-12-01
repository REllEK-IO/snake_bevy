use bevy::{prelude::*, render::pass::ClearColor};

use snake_plugin::plugin::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::BLACK))
        .add_plugin(SnakeGame)
        .run();
}
