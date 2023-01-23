//pub const MAP_HEIGHT: usize = 75;
//pub const MAP_WIDTH: usize = 75;

use bevy::app::*;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
//use rand::Rng;

use crate::gameobjects::*;
use crate::map::*;
use crate::terminal::*;

#[derive(Component)]
pub struct MapObject;

#[derive(Clone)]
pub enum MapObjectType {
    Floor,
    Wall,
}

#[derive(Resource)]
pub struct CurrentMap(Vec<Vec<MapObjectType>>);

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StartPos {
            pos: Position { x: 0, y: 0 },
        });
        app.insert_resource(CurrentMap(vec![
            vec![MapObjectType::Floor; MAP_WIDTH];
            MAP_HEIGHT
        ]));
        //app.add_startup_system(MapPlugin::setup);
        app.add_startup_system(MapPlugin::update_rendering_map);
    }
}

impl MapPlugin {
    /*
    fn setup(mut commands: Commands, map: Res<CurrentMap>) {
        for y in 0..map.0.len() {
            for x in 0..map.0[y].len() {
                match &map.0[y][x] {
                    MapObjectType::Floor => {
                        commands.spawn((
                            MapObject,
                            Name::new("floor"),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            DrawTerm {
                                ch: '.',
                                color: Color::GRAY,
                            },
                        ));
                    }
                    MapObjectType::Wall => {
                        commands.spawn((
                            MapObject,
                            Name::new("wall"),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            DrawTerm {
                                ch: '#',
                                color: Color::WHITE,
                            },
                        ));
                    } //_ => println!("a"),
                }
            }
        }
    }
    */
    fn update_rendering_map(
        mut commands: Commands,
        map: Res<CurrentMap>,
        query: Query<Entity, With<MapObject>>,
    ) {
        for entity in &query {
            commands.entity(entity).despawn();
        }

        for y in 0..map.0.len() {
            for x in 0..map.0[y].len() {
                match &map.0[y][x] {
                    MapObjectType::Floor => {
                        commands.spawn((
                            MapObject,
                            Name::new("floor"),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            DrawTerm {
                                ch: '.',
                                color: Color::GRAY,
                            },
                        ));
                    }
                    MapObjectType::Wall => {
                        commands.spawn((
                            MapObject,
                            Name::new("wall"),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            DrawTerm {
                                ch: '#',
                                color: Color::WHITE,
                            },
                        ));
                    }
                }
            }
        }
    }
}
