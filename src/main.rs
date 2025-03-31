use bevy::prelude::*;
use rand::Rng;
use std::time::{Duration, Instant};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zombie plant".to_string(),
                resolution: (800., 800.).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (player, update, add))
        .run();
}

const WIDTH: f32 = 50.0;
const HEIGHT: f32 = 50.0;
const SIZE_WIDTH: std::ops::Range<i32> = -7..7;
const SIZE_HEIGHT: std::ops::Range<i32> = -5..4;

#[derive(Debug, Clone, Resource)]
enum Tile {
    White,
    Gray,
}

impl Tile {
    fn color(&self) -> Color {
        match self {
            Tile::White => Color::WHITE,
            Tile::Gray => Color::srgb(0.5, 0.5, 0.5),
        }
    }
}

#[derive(Component)]
struct TileSprite;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Zon;
#[derive(Component)]
struct Zombie {
    time: f32,
}
#[derive(Component)]
struct Plant;

struct MoneyUI {
    money: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let mut tile = Tile::White;

    for y in SIZE_HEIGHT {
        for x in SIZE_WIDTH {
            commands.spawn((
                TileSprite,
                Sprite {
                    custom_size: Some(Vec2::new(WIDTH - 2.0, HEIGHT - 2.0)),
                    color: tile.color(),
                    ..default()
                },
                Transform::from_xyz(x as f32 * WIDTH, y as f32 * HEIGHT, 0.0),
            ));

            tile = match tile {
                Tile::White => Tile::Gray,
                Tile::Gray => Tile::White,
            };
        }
    }

    // println!("{:?}",SIZE_HEIGHT.end);
    for y in SIZE_HEIGHT {
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::new(WIDTH / 2.0, HEIGHT / 2.0)),
                color: Color::srgba(0.0, 1.0, 0.0, 0.5),
                ..default()
            },
            Transform::from_xyz((SIZE_WIDTH.end - 1) as f32 * WIDTH, y as f32 * HEIGHT, 1.0),
        ));
    }

    commands.spawn((
        Player,
        Sprite {
            custom_size: Some(Vec2::new(WIDTH - 2.0, HEIGHT - 2.0)),
            color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        },
        Transform::from_xyz(
            SIZE_WIDTH.start as f32 * WIDTH,
            SIZE_HEIGHT.start as f32 * HEIGHT,
            2.0,
        ),
        TimeSpawn { time: 3.0 },
    ));
 
    commands.spawn((
        Text2d {
            text: Text::new("Money: 0.0".to_string()),
            font_size: 50.0,
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(
            (SIZE_WIDTH.end - 1) as f32 * WIDTH,
            (SIZE_HEIGHT.end - 1) as f32 * HEIGHT,
            2.0,
        ),
    ));
}

fn update(mut commands: Commands, mut query: Query<(Entity, &Zombie, &mut Transform)>) {
    for mut entity in query.iter_mut() {
        entity.2.translation.x -= entity.1.time;
        if entity.2.translation.x <= SIZE_WIDTH.start as f32 * WIDTH {
            commands.entity(entity.0).despawn();
        }
    }
}

#[derive(Component)]
struct TimeSpawn {
    time: f32,
}

enum Direction {
    Up,
    Down,
}

fn add(mut commands: Commands, mut query: Query<(Entity, &mut TimeSpawn)>, time: Res<Time>) {
    for (entity, mut item) in query.iter_mut() {
        if item.time > 0.0 {
            item.time -= time.delta_secs();
        } else {
            let mut rng = rand::rng();
            let n1: i32 = rng.random_range(SIZE_HEIGHT.start..SIZE_HEIGHT.end);

            item.time = rng.random_range(0.2..3.0);

            commands.spawn((
                Zombie { time: item.time },
                Sprite {
                    custom_size: Some(Vec2::new(WIDTH - 2.0, HEIGHT - 2.0)),
                    color: Color::srgb(0.0, 1.0, 0.0),
                    ..default()
                },
                Transform::from_xyz(
                    (SIZE_WIDTH.end - 1) as f32 * WIDTH,
                    (n1) as f32 * HEIGHT,
                    2.0,
                ),
            ));
        }
    }
}

fn player(
    mut query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            transform.translation.y += WIDTH;
        }
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            transform.translation.x -= WIDTH;
        }
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            transform.translation.y -= WIDTH;
        }
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            transform.translation.x += WIDTH;
        }

        transform.translation.x = transform.translation.x.clamp(
            SIZE_WIDTH.start as f32 * WIDTH,
            (SIZE_WIDTH.end - 1) as f32 * WIDTH,
        );
        transform.translation.y = transform.translation.y.clamp(
            SIZE_HEIGHT.start as f32 * HEIGHT,
            (SIZE_HEIGHT.end - 1) as f32 * HEIGHT,
        );
    }
}
