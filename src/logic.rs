use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::scene::GUTTER_HEIGHT;

const BALL_SIZE: f32 = 5.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;
const PADDLE_SPEED: f32 = 5.0;
const BALL_SPEED: f32 = 5.0;

#[derive(Component, Default)]
pub struct Shape(pub Vec2);

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

pub enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
pub struct Scored(pub Scorer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
#[require(
    Position,
    Shape(|| Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
    Velocity
)]
struct Paddle;

#[derive(Component)]
#[require(
    Position,
    Velocity(|| Velocity(Vec2::new(-1.0, 1.0))),
    Shape(|| Shape(Vec2::new(BALL_SIZE, BALL_SIZE))),
)]
struct Ball;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.0;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.0;
        } else {
            velocity.0.y = 0.0;
        }
    }
}

fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2.0 - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.0;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}

fn spawn_pandle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    println!("Spawning paddles ...");

    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.0;
        let right_padding_x = window_width / 2.0 - padding;
        let left_padding_x = -window_width / 2.0 + padding;

        let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);

        let mesh = meshes.add(shape);
        let player_color = materials.add(Color::srgb(0.0, 1.0, 0.0));
        let ai_color = materials.add(Color::srgb(0.0, 1.0, 1.0));

        commands.spawn((
            Player,
            Paddle,
            Position(Vec2::new(right_padding_x, 0.0)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(player_color.clone()),
        ));

        commands.spawn((
            Ai,
            Paddle,
            Position(Vec2::new(left_padding_x, 0.0)),
            Mesh2d(mesh.clone()),
            MeshMaterial2d(ai_color.clone()),
        ));
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning ball ...");

    let shape: Circle = Circle::new(BALL_SIZE);
    let color: Color = Color::srgb(1.0, 0.0, 0.0);

    let mesh: Handle<Mesh> = meshes.add(shape);
    let material: Handle<ColorMaterial> = materials.add(color);

    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}

fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.0);
    }
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0.0 {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0 / 2.0),
            ) {
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.0;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.0;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.0;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.0;
                    }
                }
            }
        }
    }
}

fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.get_single_mut() {
            if ball.0.x > window_width / 2.0 {
                events.send(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2.0 {
                events.send(Scored(Scorer::Player));
            }
        }
    }
}

fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((mut position, mut velocity)) = ball.get_single_mut() {
            match event.0 {
                Scorer::Ai => {
                    position.0 = Vec2::new(0.0, 0.0);
                    velocity.0 = Vec2::new(-1.0, 1.0);
                }
                Scorer::Player => {
                    position.0 = Vec2::new(0.0, 0.0);
                    velocity.0 = Vec2::new(1.0, 1.0);
                }
            }
        }
    }
}

fn update_score(mut score: ResMut<Score>, mut events: EventReader<Scored>) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => score.ai += 1,
            Scorer::Player => score.player += 1,
        }
    }
}

fn move_ai(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = ai.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.0 - position.0;
            velocity.0.y = a_to_b.y.signum();
        }
    }
}

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
        app.add_event::<Scored>();
        app.add_systems(Startup, (spawn_ball, spawn_pandle));
        app.add_systems(
            Update,
            (
                (move_ball, project_positions, handle_collisions).chain(),
                (handle_player_input, move_paddles).chain(),
                (detect_scoring, reset_ball, update_score).chain(),
                move_ai,
            ),
        );
    }
}
