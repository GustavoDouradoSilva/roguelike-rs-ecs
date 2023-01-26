use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
pub use bevy::prelude::*;

pub use bevy::winit::WinitSettings;

use crate::terminal::DrawTerm;

#[derive(Component, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    /* 
    pub fn add(pos1: &Position, pos2: &Position) -> Position {
        Position {
            x: pos1.x + pos2.x,
            y: pos1.y + pos2.y,
        }
    }*/
    pub fn new() -> Position
    {
        Position { x: 0, y: 0 }
    }
}

#[derive(Component)]
pub struct Collider;

//appears in console above everything else
#[derive(Component)]
pub struct RenderAbove;

#[derive(Component)]
pub struct Object;

pub struct PopulatePlugin;
impl PopulatePlugin {
    pub fn add_player(mut commands: Commands) {
        commands.spawn((
            Object,
            Name::new("player"),
            Position { x: 10, y: 9 },
            DrawTerm {
                ch: '@',
                color: Color::GOLD,
            },
            RenderAbove,
        ));
        commands.spawn((
            Object,
            Name::new("enemy"),
            Position { x: 5, y: 5 },
            DrawTerm {
                ch: '@',
                color: Color::RED,
            },
            RenderAbove,
        ));
    }
}
impl Plugin for PopulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(PopulatePlugin::add_player);
    }
}
