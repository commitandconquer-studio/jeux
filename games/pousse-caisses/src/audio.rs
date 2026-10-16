//! Sons et musiques, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::board::Universe;
use crate::play::{Play, PlayEvent};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UiClick>()
            .add_systems(Update, (switch_music, play_sounds));
    }
}

/// Clic sur un bouton de l'interface.
#[derive(Message, Clone, Copy)]
pub struct UiClick;

#[derive(Component)]
struct Music;

/// Chaque univers a sa musique (la même que dans le Snake).
fn switch_music(
    mut commands: Commands,
    play: Res<Play>,
    server: Res<AssetServer>,
    sound: Res<SoundOn>,
    mut playing: Local<Option<Universe>>,
    current: Query<Entity, With<Music>>,
) {
    let universe = play.universe();
    if *playing == Some(universe) {
        return;
    }
    *playing = Some(universe);
    for e in &current {
        commands.entity(e).despawn();
    }
    commands.spawn((
        AudioPlayer::new(server.load(universe.music())),
        PlaybackSettings { muted: !sound.0, ..PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)) },
        Music,
        Musique,
        Mutable,
    ));
}

fn play_sounds(
    mut commands: Commands,
    server: Res<AssetServer>,
    sound: Res<SoundOn>,
    mut events: MessageReader<PlayEvent>,
    mut clicks: MessageReader<UiClick>,
) {
    let mut play = |path: &'static str, volume: f32| {
        if sound.0 {
            commands.spawn((
                AudioPlayer::new(server.load(path)),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)),
            ));
        }
    };
    for event in events.read() {
        match event {
            PlayEvent::Step => play(lib::HOP, 0.35),
            PlayEvent::Push => play(lib::PUSH, 0.7),
            PlayEvent::Stored => {
                play(lib::TOCK, 0.7);
                play(lib::CHIME, 0.4);
            }
            PlayEvent::Undo => play(lib::TURN, 0.5),
            PlayEvent::Won => play(lib::WIN, 0.8),
        }
    }
    for _ in clicks.read() {
        play(lib::CLICK, 0.6);
    }
}
