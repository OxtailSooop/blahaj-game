use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::color::Color;
use bevy::gltf::GltfAssetLabel;
use bevy::pbr::PointLight;
use bevy::prelude::{
    AudioSinkPlayback, Camera3d, ImageNode, IntoSystemConfigs, ResMut, Resource, Text,
};
use bevy::scene::SceneRoot;
use bevy::text::{TextFont, TextLayout};
use bevy::time::Time;
use bevy::ui::{BackgroundColor, Node, PositionType};
use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    audio::AudioSink,
    input::ButtonInput,
    math::Vec3,
    prelude::{Commands, Component, KeyCode, Query, Res, With},
    render::camera::ClearColor,
    text::JustifyText,
    transform::components::Transform,
    ui::{UiRect, Val},
    utils::default,
    DefaultPlugins,
};

use camera_controller::{CameraController, CameraControllerPlugin};

mod camera_controller;

#[derive(Component)]
struct Music;

#[derive(Resource)]
struct BlahajGame {
    spin_speed: f32,
    paused: bool,
}

#[derive(Component)]
struct Blahaj {
    origin: Vec3,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraControllerPlugin))
        .add_systems(Startup, (setup_world, start_background_music, spawn_player))
        .add_systems(
            Update,
            (
                control_blahaj,
                transform_blahaj.run_if(transform_blahaj_should_run),
                reproduce,
            ),
        )
        .insert_resource(ClearColor(Color::srgb(0.9, 0.3, 0.6)))
        .insert_resource(BlahajGame {
            spin_speed: 1.0,
            paused: false,
        })
        .run();
}

fn setup_world(asset: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((Transform::from_xyz(0.0, 0.0, 2.0), PointLight::default()));

    commands.spawn((
        Transform::from_xyz(0.0, -0.5, 0.0).with_scale(Vec3::new(0.8, 0.8, 0.8)),
        SceneRoot(asset.load("low_poly_blahaj/mod.gltf#Scene0")),
        Blahaj {
            origin: Vec3::new(0.0, -0.5, 0.0),
        },
    ));
}

fn start_background_music(asset: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset.load("music.ogg")),
        PlaybackSettings::LOOP,
        Music,
    ));
}

fn spawn_player(asset: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Px(350.0),
            height: Val::Px(200.0),
            margin: UiRect::bottom(Val::VMin(0.)),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        ImageNode {
            image: asset.load("ikea.bmp"),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("ESC: FPS Controller\nP: Pause\nPlus: Increase Speed\nMinus: Decrease Speed\nQ: Down\nE: Up\nJ: Spawn Blahaj\nM: I Muted it... My way."),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Left),
        Node { // idk how to make it render over the ikea logo
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            ..default()
        },
    ));

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 3.0),
        Camera3d::default(),
        CameraController { ..default() },
    ));
}

fn control_blahaj(
    key_input: Res<ButtonInput<KeyCode>>,
    music: Query<&mut AudioSink, With<Music>>,
    mut blahaj: ResMut<BlahajGame>,
) {
    if key_input.pressed(KeyCode::Equal) {
        blahaj.spin_speed += 0.25;
        music
            .single()
            .set_speed(1.0 + (blahaj.spin_speed / 0.25) * 0.0025);
    } else if key_input.pressed(KeyCode::Minus) {
        blahaj.spin_speed -= 0.25;
        music
            .single()
            .set_speed(1.0 + (blahaj.spin_speed / 0.25) * 0.0025);
    } else if key_input.just_pressed(KeyCode::KeyP) {
        blahaj.paused = !blahaj.paused;
        music.single().toggle();
    } else if key_input.just_pressed(KeyCode::KeyM) {
        if music.single().volume() == 0.0 {
            music.single().set_volume(1.0);
        } else {
            music.single().set_volume(0.0);
        }
    }
}

fn reproduce(
    asset: Res<AssetServer>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    player: Query<&Transform, With<CameraController>>,
) {
    if key_input.just_pressed(KeyCode::KeyJ) {
        commands.spawn((
            Transform::from_scale(Vec3::new(0.8, 0.8, 0.8))
                .with_translation(player.single().translation),
            SceneRoot(asset.load(GltfAssetLabel::Scene(0).from_asset("low_poly_blahaj/mod.gltf"))),
            Blahaj {
                origin: Vec3::from_array(player.single().translation.to_array()),
            },
        ));
    }
}

fn transform_blahaj(
    mut transforms: Query<(&Blahaj, &mut Transform), With<Blahaj>>,
    blahaj_game: Res<BlahajGame>,
    time: Res<Time>,
) {
    for (blahaj_entity, mut transform) in &mut transforms {
        transform.rotate_y(blahaj_game.spin_speed * time.delta().as_secs_f32());
        transform.translation.y = blahaj_entity.origin.y + f32::sin(transform.rotation.y) / 2.0;
    }
}

fn transform_blahaj_should_run(blahaj_game: Res<BlahajGame>) -> bool {
    !blahaj_game.paused
}
