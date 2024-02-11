use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_enemy);
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let enemy =( 
    PbrBundle{
        mesh: meshes.add(Mesh::from(shape::Cube::new(0.8))),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(2.0, 0.4, 3.0),
        ..default()
 
    },
    );
    commands.spawn(enemy);
}
