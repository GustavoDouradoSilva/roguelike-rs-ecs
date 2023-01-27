pub const MAP_HEIGHT: usize = 50;
pub const MAP_WIDTH: usize = 50;

use bevy::app::*;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use rand::Rng;

use crate::gameobjects::*;
use crate::terminal::*;

#[derive(Resource)]
pub struct CurrentMap {
    pub map: Vec<Vec<TileType>>,
}

//using until the ECS version
#[derive(Component, Clone)]
pub struct TileType {
    pub ch: char,
    pub walkable: bool,
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StartPos {
            pos: Position { x: 0, y: 0 },
        });
        app.insert_resource(CurrentMap { map: Vec::new() });
        app.add_startup_system(MapPlugin::setup_map);
        app.add_system(MapPlugin::draw_map);
    }
}
impl MapPlugin {
    //generates a map and then updates the values for the present map and the starting position
    pub fn setup_map(mut start_pos: ResMut<StartPos>, mut current_map: ResMut<CurrentMap>) {
        let tile = TileType {
            ch: '#',
            walkable: false,
        };
        let mut map = vec![vec![tile; MAP_WIDTH]; MAP_HEIGHT];
        let n_rooms: i32 = rand::thread_rng().gen_range(5..14);

        let mut rooms: Vec<Room> = Vec::new();

        for i in 0..(n_rooms) {
            rooms.push(Room::new_random());
            add_room_to_map(&rooms[i as usize], &mut map);

            if i > 0 {
                connect_room_centers(
                    &rooms[i as usize - 1].center(),
                    &rooms[i as usize].center(),
                    &mut map,
                )
            }
        }

        start_pos.pos = Position {
            x: rooms[0].center().x,
            y: rooms[0].center().y,
        };

        current_map.map = map;
    }

    pub fn draw_map(mut commands: Commands, map: Res<CurrentMap>) {
        for y in 0..map.map.len() {
            for x in 0..map.map[y].len() {
                match map.map[y][x].ch {
                    '.' => {
                        commands.spawn((
                            Object,
                            Name::new("floor"),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            DrawTerm {
                                ch: '.',
                                color: Color::RED,
                            },
                        ));
                    }
                    ' ' => {
                        commands.spawn((
                            Object,
                            Name::new("floor"),
                            Position {
                                x: x as i32,
                                y: y as i32,
                            },
                            DrawTerm {
                                ch: '.',
                                color: Color::RED,
                            },
                        ));
                        println!("added floor");
                    }
                    '#' => {
                        commands.spawn((
                            Object,
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
                    _ => println!("unexpected tile map"),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Room {
    pub height: i32,
    pub width: i32,
    pub pos: Position,
}

impl Room {
    pub fn center(&self) -> Position {
        Position {
            x: self.pos.x + self.width / 2,
            y: self.pos.y + self.height / 2,
        }
    }

    pub fn new_random() -> Self {
        Room {
            pos: Position {
                x: rand::thread_rng().gen_range(1..(MAP_WIDTH - 20) as i32),
                y: rand::thread_rng().gen_range(1..(MAP_HEIGHT - 10)) as i32,
            },
            height: rand::thread_rng().gen_range(3..9),
            width: rand::thread_rng().gen_range(5..19),
        }
    }
}

pub fn add_room_to_map(room: &Room, map: &mut Vec<Vec<TileType>>) {
    for y in room.pos.y..room.pos.y + room.height {
        for x in room.pos.x..room.pos.x + room.width {
            map[y as usize][x as usize] = TileType {
                ch: ' ',
                walkable: true,
            };
        }
    }
}

pub fn connect_room_centers(center1: &Position, center2: &Position, map: &mut Vec<Vec<TileType>>) {
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
        map[temp.y as usize][temp.x as usize].ch = ' ';
        map[temp.y as usize][temp.x as usize].walkable = true;
    }
}
