use bevy::{math::vec3, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::{player::Player, state::GameState};

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(OnEnter(GameState::Loading), setup_camera)
            .add_systems(
                Update,
                update_camera_position.run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

fn update_camera_position(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;

    camera_transform.translation = camera_transform
        .translation
        .lerp(vec3(player_transform.x, player_transform.y, 0.0), 0.1);
}
