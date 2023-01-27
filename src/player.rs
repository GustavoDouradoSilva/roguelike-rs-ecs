use bevy::app::App;
use bevy::app::Plugin;
use bevy::ecs::component::Component;
use bevy::ecs::system::Res;
use bevy::input::Input;
use bevy::prelude::KeyCode;

use crate::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PlayerPlugin::move_to);
    }
}
impl PlayerPlugin {
    fn move_to(
        keys: Res<Input<KeyCode>>,
        query_collider: Query<&Position, (With<Collider>, Without<Player>)>,
        mut query_player: Query<&mut Position, With<Player>>,
    ) {
        for mut player_pos in &mut query_player {
            let old_pos = player_pos.clone();
            if keys.just_pressed(KeyCode::W) {
                *player_pos = Position::new();
                player_pos.move_to(
                    &Position {
                        x: old_pos.x,
                        y: old_pos.y + 1,
                    },
                    &query_collider,
                );
            }
            if keys.just_pressed(KeyCode::A) {
                player_pos.move_to(
                    &Position {
                        x: old_pos.x - 1,
                        y: old_pos.y,
                    },
                    &query_collider,
                );
            }
            if keys.just_pressed(KeyCode::S) {
                player_pos.move_to(
                    &Position {
                        x: old_pos.x,
                        y: old_pos.y - 1,
                    },
                    &query_collider,
                );
            }
            if keys.just_pressed(KeyCode::D) {
                player_pos.move_to(
                    &Position {
                        x: old_pos.x + 1,
                        y: old_pos.y,
                    },
                    &query_collider,
                );
            }
        }
    }
}
