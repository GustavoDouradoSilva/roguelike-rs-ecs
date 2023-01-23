//use crate::gameobjects::Name;
use crate::gameobjects::*;
//use crate::map::*;

use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::component::Component;

use crate::map::*;
use bevy::ecs::system::*;

#[derive(Component, Clone)]
pub struct DrawTerm {
    pub ch: char,
    pub color: Color,
}
impl DrawTerm {
    /*
    pub fn draw(mut commands: Commands, mut query: Query<(Entity, &Name)>) {
        let player: DrawTerm = DrawTerm {
            ch: '@',
            color: TermColor::Player as u32,
        };
        let wall: DrawTerm = DrawTerm {
            ch: '#',
            color: TermColor::Wall as u32,
        };
        let floor: DrawTerm = DrawTerm {
            ch: '.',
            color: TermColor::Floor as u32,
        };
        let character: DrawTerm = DrawTerm {
            ch: '@',
            color: TermColor::Character as u32,
        };
        for (entity, name) in query.iter_mut() {
            match name.as_str() {
                "player" => {
                    commands.entity(entity).insert(player.clone());
                }
                "wall" => {
                    commands.entity(entity).insert(wall.clone());
                }
                "floor" => {
                    commands.entity(entity).insert(floor.clone());
                }
                "character" => {
                    commands.entity(entity).insert(character.clone());
                }

                _ => todo!(),
            }
        }
    }
    */
}

//identification for the type of terminal, it does nothing
#[derive(Component)]
pub struct GameTerminal;
#[derive(Component)]
pub struct LogTerminal;

use bevy_ascii_terminal::prelude::*;
pub struct TerminalDrawPlugin;
impl Plugin for TerminalDrawPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(TerminalDrawPlugin::terminal_setup);
        app.add_system(TerminalDrawPlugin::draw);
    }
}
impl TerminalDrawPlugin {
    pub fn terminal_setup(mut commands: Commands) {
        // Create the terminal
        let terminal = Terminal::new([MAP_WIDTH, MAP_HEIGHT]).with_border(Border::single_line());

        let log_terminal = Terminal::new([30, MAP_HEIGHT]).with_border(Border::single_line());

        commands.spawn((
            // Spawn the terminal bundle from our terminal
            TerminalBundle::from(terminal),
            // Automatically set up the camera to render the terminal
            AutoCamera,
            GameTerminal,
        ));
        commands.spawn((
            // Spawn the terminal bundle from our terminal
            TerminalBundle::from(log_terminal).with_position([MAP_WIDTH, 0]),
            // Automatically set up the camera to render the terminal
            AutoCamera,
            LogTerminal,
        ));
    }
    pub fn draw(
        mut term_query: Query<&mut Terminal, With<GameTerminal>>,
        draw_query: Query<(&DrawTerm, &Position),  Without<RenderAbove>>,
        draw_query_above: Query<(&DrawTerm, &Position),  With<RenderAbove>>
    ) {
        for mut terminal in &mut term_query {
            terminal.clear();
            for (draw, pos) in &draw_query {
                terminal.put_char([pos.x, pos.y], draw.ch.fg(draw.color));
            }
            for (draw, pos) in &draw_query_above {
                terminal.put_char([pos.x, pos.y], draw.ch.fg(draw.color));
            }
        }
    }
}