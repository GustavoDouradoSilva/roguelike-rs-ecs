use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::component::Component;
//use bevy::ecs::entity::Entity;
//use bevy::ecs::query::With;
use bevy::ecs::system::Commands;
//use bevy::ecs::system::Query;
//use bevy::ecs::system::*;
pub use bevy::prelude::*;

//use crate::terminal::*;


pub use bevy::winit::WinitSettings;

use crate::terminal::DrawTerm;

#[derive(Component, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
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
        commands.spawn((Object, Name::new("player"), Position { x: 10, y: 9 },DrawTerm{ch:'@',color:Color::GOLD}, RenderAbove));
        commands.spawn((Object, Name::new("enemy"), Position { x: 5, y: 5 },DrawTerm{ch:'@',color:Color::RED}, RenderAbove));
    }
}
impl Plugin for PopulatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(PopulatePlugin::add_player);
    }
}
