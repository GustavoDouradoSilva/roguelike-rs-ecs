mod gameobjects;
mod map;
mod terminal;
mod map2;

use bevy::app::App;
use gameobjects::*;
//use map2::Map2Plugin;
use terminal::*;
//use map::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PopulatePlugin)
        .add_plugin(bevy_ascii_terminal::TerminalPlugin)
        //.add_plugin(crate::map::MapPlugin)
        .add_plugin(crate::map2::MapPlugin)
        .add_plugin(TerminalDrawPlugin)
        .run();
}
