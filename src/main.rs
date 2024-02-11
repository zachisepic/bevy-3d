use bevy::prelude::*;
use bevy_third_person_camera::*;

mod player;
mod camera;
mod world;
mod enemy; 

use world::WorldPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use enemy::EnemyPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins,
        PlayerPlugin,
        CameraPlugin,
        WorldPlugin,
        EnemyPlugin,
        ThirdPersonCameraPlugin,
    )).run()

}




