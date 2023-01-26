pub const MAP_HEIGHT: usize = 75;
pub const MAP_WIDTH: usize = 75;
pub const MIN_ROOM_SIZE: usize = 5;
pub const MAX_ROOM_SIZE: usize = 20;
pub const MIN_ROOMS: usize = 7;
pub const MAX_ROOMS: usize = 14;

use rand::Rng;
use std::collections::HashMap;

use bevy::app::*;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;

use crate::gameobjects::*;
use crate::terminal::*;

#[derive(Clone, Copy, Component)]
pub struct MapObject;

#[derive(Clone, Eq, PartialEq, Hash, Copy)]
pub enum MapObjectType {
    Floor,
    Wall,
    Water,
    Lava,
}

#[derive(Resource)]
pub struct StartPos {
    pub pos: Position,
}

#[derive(Resource)]
pub struct CurrentMap {
    pub map: Vec<Vec<MapObjectType>>,
}

#[derive(Clone)]
pub struct Room {
    pub map: Vec<Vec<MapObjectType>>,
    //position in the map
    pub pos: Position,
}

impl Room {
    pub fn new() -> Room {
        Room {
            map: Vec::new(),
            pos: Position::new(),
        }
    }
    pub fn random() -> Room {
        let mut room = Room::new();
        let height = rand::thread_rng().gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);
        let width = rand::thread_rng().gen_range(MIN_ROOM_SIZE..=MAX_ROOM_SIZE);

        for x in 0..height {
            room.map.push([].to_vec());
            for _ in 0..width {
                room.map[x].push(MapObjectType::Floor);
            }
        }
        room
    }
    fn center(&self) -> Position {
        Position {
            x: self.map[0].len() as i32 / 2,
            y: self.map.len() as i32 / 2,
        }
    }

    //the center of the room in the map (room.pos + room.center())
    fn map_center(&self) -> Position {
        Position {
            //I dont know why it is inverted, I just know it works
            x: self.center().x + self.pos.y,
            y: self.center().y + self.pos.x,
        }
    }
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StartPos {
            pos: Position::new(),
        });
        app.insert_resource(CurrentMap {
            map: vec![vec![MapObjectType::Wall; MAP_WIDTH]; MAP_HEIGHT],
        });
        app.add_startup_system(MapPlugin::setup);
        app.add_system(MapPlugin::update_rendering_map);
    }
}

impl MapPlugin {
    pub fn add_room_to_map(current_map: &mut ResMut<CurrentMap>, room: &Room, pos: &Position) {
        for x in 0..room.map.len() {
            for y in 0..room.map[x].len() {
                current_map.map[x + pos.x as usize][y + pos.y as usize] = room.map[x][y];
            }
        }
    }

    pub fn connect_room_centers(
        center1: &Position,
        center2: &Position,
        current_map: &mut ResMut<CurrentMap>,
    ) {
        let mut temp = Position {
            x: center1.x,
            y: center1.y,
        };
        loop {
            if ((temp.x - 1) - center2.x).abs() < (temp.x - center2.x).abs() {
                temp.x -= 1
            } else if ((temp.x + 1) - center2.x).abs() < (temp.x - center2.x).abs() {
                temp.x += 1
            } else if ((temp.y + 1) - center2.y).abs() < (temp.y - center2.y).abs() {
                temp.y += 1
            } else if ((temp.y - 1) - center2.y).abs() < (temp.y - center2.y).abs() {
                temp.y -= 1
            } else {
                break;
            }
            current_map.map[temp.y as usize][temp.x as usize] = MapObjectType::Floor;
        }
    }

    fn setup(mut current_map: ResMut<CurrentMap>) {
        let n_rooms: usize = rand::thread_rng().gen_range(MIN_ROOMS..MAX_ROOMS);
        let mut rooms: Vec<Room> = Vec::new();
        for _ in 0..n_rooms {
            let random_pos = Position {
                x: rand::thread_rng().gen_range(1..(MAP_WIDTH - MAX_ROOM_SIZE) as i32),
                y: rand::thread_rng().gen_range(1..(MAP_HEIGHT - MAX_ROOM_SIZE) as i32),
            };
            let mut random_room = Room::random();
            random_room.pos = random_pos.clone();

            rooms.push(random_room);
        }
        for i in 0..rooms.len() {
            Self::add_room_to_map(&mut current_map, &rooms[i as usize], &rooms[i as usize].pos);
            if i > 0 {
                let center1 = rooms[i - 1].map_center();
                let center2 = rooms[i].map_center();
                Self::connect_room_centers(&center1, &center2, &mut current_map);
            }

            //current_map.map[rooms[i].pos.x as usize][rooms[i].pos.y as usize] = MapObjectType::Water;
        }
    }

    fn update_rendering_map(
        mut commands: Commands,
        current_map: Res<CurrentMap>,
        query: Query<Entity, With<MapObject>>,
    ) {
        let map = &current_map.map;

        //hashmap shall not include position
        let mut hashmap = HashMap::new();
        hashmap.insert(
            MapObjectType::Floor,
            (
                MapObject,
                Name::new("floor"),
                DrawTerm {
                    ch: ' ',
                    color: Color::GRAY,
                },
            ),
        );
        hashmap.insert(
            MapObjectType::Wall,
            (
                MapObject,
                Name::new("wall"),
                DrawTerm {
                    ch: '#',
                    color: Color::WHITE,
                },
            ),
        );
        hashmap.insert(
            MapObjectType::Water,
            (
                MapObject,
                Name::new("water"),
                DrawTerm {
                    ch: '#',
                    color: Color::BLUE,
                },
            ),
        );
        hashmap.insert(
            MapObjectType::Lava,
            (
                MapObject,
                Name::new("lava"),
                DrawTerm {
                    ch: '#',
                    color: Color::RED,
                },
            ),
        );

        for entity in &query {
            commands.entity(entity).despawn();
        }

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                match &map[y][x] {
                    MapObjectType::Floor => {
                        commands.spawn((
                            hashmap.get(&MapObjectType::Floor).unwrap().clone(),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                        ));
                    }
                    MapObjectType::Wall => {
                        commands.spawn((
                            hashmap.get(&MapObjectType::Wall).unwrap().clone(),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                        ));
                    }
                    MapObjectType::Water => {
                        commands.spawn((
                            hashmap.get(&MapObjectType::Water).unwrap().clone(),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                        ));
                    }
                    MapObjectType::Lava => {
                        commands.spawn((
                            hashmap.get(&MapObjectType::Lava).unwrap().clone(),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                        ));
                    } //_ => todo!()
                }
            }
        }
    }
}
