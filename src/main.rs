use bevy::prelude::*;
use logic::LogicPlugin;
use scene::ScenePlugin;
use scoreboard::ScoreBoardPlugin;

mod logic;
mod scene;
mod scoreboard;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenePlugin)
        .add_plugins(ScoreBoardPlugin)
        .add_plugins(LogicPlugin)
        .run();
}
