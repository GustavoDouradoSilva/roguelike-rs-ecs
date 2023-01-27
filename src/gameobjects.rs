use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
pub use bevy::prelude::*;

pub use bevy::winit::WinitSettings;

use crate::map::StartPos;
use crate::terminal::DrawTerm;
use crate::MapPlugin;
use crate::Player;

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Seen;
#[derive(Component)]
pub struct Visible;

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn move_to(
        &mut self,
        new_pos: &Position,
        query_collider: &Query<&Position, (With<Collider>, Without<Player>)>,
    ) {
        for pos in query_collider {
            if pos == new_pos {
                return;
            }
        }
        self.x = new_pos.x;
        self.y = new_pos.y;
    }
}

#[derive(Component)]
pub struct Collider;

//appears in console above everything else
#[derive(Component)]
pub struct RenderAbove;

#[derive(Component)]
pub struct Character;
impl Character {}

pub struct PopulatePlugin;
impl PopulatePlugin {
    pub fn add_player(mut commands: Commands, start_pos: Res<StartPos>) {
        commands.spawn((
            Player,
            Character,
            Name::new("player"),
            start_pos.pos.clone(),
            DrawTerm {
                ch: '@',
                color: Color::GOLD,
            },
            RenderAbove,
        ));
        /* commands.spawn((
            Character,
            Name::new("enemy"),
            Position { x: 5, y: 5 },
            DrawTerm {
                ch: '@',
                color: Color::RED,
            },
            RenderAbove,
        )); */
    }
}
impl Plugin for PopulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(PopulatePlugin::add_player.after(MapPlugin::setup));
    }
}
