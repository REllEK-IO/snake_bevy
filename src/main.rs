use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

use snake_plugin::plugin::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(SnakeGame)
        .run();
}