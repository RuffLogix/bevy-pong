use crate::logic::{Position, Shape};
use bevy::prelude::*;

pub const GUTTER_HEIGHT: f32 = 96.0;

#[derive(Component)]
#[require(Position, Shape)]
struct Gutter;

fn spawn_gutter(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning getter ...");

    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();

        let top_gutter_y = window_height / 2.0 - GUTTER_HEIGHT / 2.0;
        let bottom_gutter_y = -window_height / 2.0 + GUTTER_HEIGHT / 2.0;

        let shape = Rectangle::from_size(Vec2::new(window_width, GUTTER_HEIGHT));
        let color = Color::srgb(0.0, 0.0, 0.0);

        let mesh_handle = meshes.add(shape);
        let material_handle = materials.add(color);

        commands.spawn((
            Gutter,
            Shape(shape.size()),
            Position(Vec2::new(0.0, top_gutter_y)),
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone()),
            Transform::from_translation(Vec3::new(0.0, top_gutter_y, 0.0)),
        ));

        commands.spawn((
            Gutter,
            Shape(shape.size()),
            Position(Vec2::new(0.0, bottom_gutter_y)),
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(material_handle.clone()),
            Transform::from_translation(Vec3::new(0.0, bottom_gutter_y, 0.0)),
        ));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_gutter));
    }
}
