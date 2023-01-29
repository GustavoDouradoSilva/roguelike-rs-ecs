use crate::gameobjects::*;

use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::component::Component;

use crate::map::*;
use bevy::ecs::system::*;

#[derive(Resource)]
pub struct LogText {
    pub log_text: Vec<(String, Color)>,
}

#[derive(Component, Clone)]
pub struct DrawTerm {
    pub ch: char,
    pub color: Color,
}

//identification for the type of terminal, it does nothing
#[derive(Component)]
pub struct GameTerm;
#[derive(Component)]
pub struct LogTerm;

use bevy_ascii_terminal::prelude::*;
pub struct TerminalDrawPlugin;
impl Plugin for TerminalDrawPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LogText {
            log_text: Vec::new(),
        });
        app.add_startup_system(TerminalDrawPlugin::terminal_setup);
        app.add_system(TerminalDrawPlugin::draw);
        app.add_system(TerminalDrawPlugin::draw_log);
    }
}
impl TerminalDrawPlugin {
    pub fn terminal_setup(mut commands: Commands) {
        // Create the terminal
        let terminal = Terminal::new([MAP_WIDTH, MAP_HEIGHT]).with_border(Border::single_line());

        let log_terminal = Terminal::new([50, MAP_HEIGHT]).with_border(Border::single_line());

        commands.spawn((
            // Spawn the terminal bundle from our terminal
            TerminalBundle::from(terminal),
            // Automatically set up the camera to render the terminal
            AutoCamera,
            GameTerm,
        ));
        commands.spawn((
            // Spawn the terminal bundle from our terminal
            TerminalBundle::from(log_terminal).with_position([MAP_WIDTH - 11, 0]),
            // Automatically set up the camera to render the terminal
            AutoCamera,
            LogTerm,
        ));
    }
    pub fn draw(
        mut term_query: Query<&mut Terminal, With<GameTerm>>,
        draw_query: Query<(&DrawTerm, &Position, Option<&Visible>), Without<RenderAbove>>,
        draw_query_above: Query<(&DrawTerm, &Position, Option<&Visible>), With<RenderAbove>>,
    ) {
        //renders the game terminal
        for mut terminal in &mut term_query {
            terminal.clear();
            for (draw, pos, visibility) in &draw_query {
                if visibility.is_some()
                    && visibility.unwrap().seen == true
                    && visibility.unwrap().visible == false
                {
                    terminal.put_char([pos.x, pos.y], draw.ch.fg(Color::BLUE));
                } else if visibility.is_some() && visibility.unwrap().seen == false {
                    terminal.put_char([pos.x, pos.y], '%'.fg(Color::GRAY));
                } else {
                    terminal.put_char([pos.x, pos.y], draw.ch.fg(draw.color));
                }
            }
            //drawings with renderabove are rendered after all the rest
            for (draw, pos, visibility) in &draw_query_above {
                if visibility.is_some()
                    && visibility.unwrap().seen == true
                    && visibility.unwrap().visible == false
                {
                    terminal.put_char([pos.x, pos.y], draw.ch.fg(Color::BLUE));
                } else if visibility.is_some() && visibility.unwrap().seen == false {
                    terminal.put_char([pos.x, pos.y], '%'.fg(Color::GRAY));
                } else {
                    terminal.put_char([pos.x, pos.y], draw.ch.fg(draw.color));
                }
                //terminal.put_char([pos.x, pos.y], draw.ch.fg(draw.color));
            }
        }
    }
    pub fn draw_log(mut log_term_query: Query<&mut Terminal, With<LogTerm>>, log: Res<LogText>) {
        //renders the log terminal
        for mut terminal in &mut log_term_query {
            terminal.clear();
            let mut log_text = log.log_text.clone();
            log_text.reverse();
            for (n, (text, color)) in log_text.iter().enumerate() {
                terminal.put_string([1, n], text.bg(*color))
            }
        }
    }
}
