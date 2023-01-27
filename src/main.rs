mod gameobjects;
mod map;
mod player;
mod terminal;

use bevy::app::App;
use gameobjects::*;
use map::*;
use player::*;
use terminal::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PopulatePlugin)
        .add_plugin(bevy_ascii_terminal::TerminalPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(TerminalDrawPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
