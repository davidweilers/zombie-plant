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
        .add_systems(Update, (player, update, add, update_plant, move_bullets, bullet_collision, bullet_despawn))
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
            Tile::White => Color::srgb(0.0, 0.7, 0.0),
            Tile::Gray => Color::srgb(0.0, 0.5, 0.0),
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
struct Plant {
    time: f32,
}

#[derive(Component)]
struct Bullet {
    speed: f32,
    direction: Vec2,
    value: i32,
}

#[derive(Component)]
struct Health {
    pub value: i32,
}

#[derive(Component)]
struct Target;

#[derive(Component)]
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
        MoneyUI { money: 0.0 },
        TextSpan::new("Money: &str"),
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(
            0.0, // (SIZE_WIDTH.start) as f32 * WIDTH,
            (SIZE_HEIGHT.start - 1) as f32 * HEIGHT,
            2.0,
        ),
    ));
}

fn update(mut commands: Commands, mut query: Query<(Entity, &Zombie, &mut Transform)>) {
    for (mut entity, zombie, mut transform) in query.iter_mut() {
        transform.translation.x -= zombie.time;
        if transform.translation.x <= SIZE_WIDTH.start as f32 * WIDTH {
            commands.entity(entity).despawn();
        }
    }
}

fn update_plant(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Plant, &Transform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut item, transform) in query.iter_mut() {
        if item.time > 0.0 {
            item.time -= time.delta_secs();
        } else {
            item.time = 5.0;
            commands.spawn((
                Bullet {
                    speed: 4.0,
                    direction: Vec2::new(1.0, 0.0),
                    value: 1,
                },
                Lifetime {
                    timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                },
                Sprite {
                    custom_size: Some(Vec2::new(WIDTH / 2.0, HEIGHT / 2.0)),
                    image: asset_server.load("bullet.png"),
                    ..default()
                },
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    5.0,
                ),
            ));
        }
    }
}

#[derive(Component)]
struct TimeSpawn {
    time: f32,
}

fn add(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TimeSpawn)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut item) in query.iter_mut() {
        if item.time > 0.0 {
            item.time -= time.delta_secs();
        } else {
            let mut rng = rand::rng();
            let n1: i32 = rng.random_range(SIZE_HEIGHT.start..SIZE_HEIGHT.end);

            item.time = rng.random_range(0.2..0.8);

            commands.spawn((
                Zombie { time: item.time },
                Sprite {
                    custom_size: Some(Vec2::new(WIDTH - 2.0, HEIGHT - 2.0)),
                    // color: Color::srgb(0.0, 1.0, 0.0),
                    image: asset_server.load("Zombie1plant.png"),
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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

        if keyboard_input.just_pressed(KeyCode::Space) {
            commands.spawn((
                Plant {time: 5.0},
                Sprite {
                    image: asset_server.load("pea-shooter-plant-vs-zombie.png"),
                    custom_size: Some(Vec2::new(WIDTH - 2.0, HEIGHT - 2.0)),
                    // color: Color::srgb(1.0, 1.0, 0.0),
                    ..default()
                },
                Transform::from_xyz(transform.translation.x, transform.translation.y, 3.0),
            ));
        }
        if keyboard_input.just_pressed(KeyCode::KeyZ) {
            commands.spawn((
                Zon,
                Sprite {
                    image: asset_server.load("zon.png"),
                    custom_size: Some(Vec2::new(WIDTH - 2.0, HEIGHT - 2.0)),
                    // color: Color::srgb(1.0, 1.0, 0.0),
                    ..default()
                },
                Transform::from_xyz(transform.translation.x, transform.translation.y, 3.0),
            ));
        }
        // if keyboard_input.just_pressed(KeyCode::KeyA) {
        //     commands.spawn((
        //         Plant,
        //         Sprite {
        //             image: asset_server.load("aardappelen.png"),
        //             custom_size: Some(Vec2::new(WIDTH - 2.0, HEIGHT - 2.0)),
        //             // color: Color::srgb(1.0, 1.0, 0.0),
        //             ..default()
        //         },
        //         Transform::from_xyz(transform.translation.x, transform.translation.y, 3.0),
        //     ));
        // }
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation.x += bullet.speed;
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    targets: Query<(Entity, &Transform), With<Zombie>>,
    mut money_query: Query<(Entity, &mut MoneyUI, &mut TextSpan)>,
) {
    for (bullet_entity, bullet_transform) in &bullets {
        for (target_entity, target_transform) in &targets {
            let distance = bullet_transform.translation.distance(target_transform.translation);
            let collision_threshold = 10.0; // Adjust based on your game scale

            if distance < collision_threshold {
                commands.entity(bullet_entity).despawn();
                commands.entity(target_entity).despawn();

                let (e, mut money, mut text) = money_query.single_mut();
                money.money += 10.0;
                **text = format!("{:.2}", money.money);   
            }
            //     // Apply damage or handle zombie logic
            //     // zombie.health -= 10; // Assuming Zombie has a `health` field

            //     // Remove bullet

            //     // Optionally remove the zombie if health reaches 0
            //     // if zombie.health <= 0 {
            //     // }
            // }
        }
    }
}

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
