//! Sons et musique, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{AudioKitPlugin, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::GameState;
use crate::play::PlayEvent;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioKitPlugin)
            .add_message::<UiClick>()
            .add_systems(Startup, start_music)
            .add_systems(OnEnter(GameState::Won), |mut c: Commands, s: Res<AssetServer>, on: Res<SoundOn>| {
                if on.0 {
                    c.spawn((AudioPlayer::new(s.load(lib::WIN)), PlaybackSettings::DESPAWN));
                }
            })
            .add_systems(OnEnter(GameState::Over), |mut c: Commands, s: Res<AssetServer>, on: Res<SoundOn>| {
                if on.0 {
                    c.spawn((AudioPlayer::new(s.load(lib::GAME_OVER)), PlaybackSettings::DESPAWN));
                }
            })
            .add_systems(Update, play_sounds);
    }
}

#[derive(Message, Clone, Copy)]
pub struct UiClick;

fn start_music(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_ZEN)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Mutable));
}

fn play_sounds(
    mut commands: Commands,
    server: Res<AssetServer>,
    sound: Res<SoundOn>,
    mut events: MessageReader<PlayEvent>,
    mut clicks: MessageReader<UiClick>,
) {
    let mut play = |path: &'static str, volume: f32, speed: f32| {
        if sound.0 {
            commands.spawn((
                AudioPlayer::new(server.load(path)),
                PlaybackSettings { speed, ..PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)) },
            ));
        }
    };
    let mut merged = false;
    for e in events.read() {
        match *e {
            PlayEvent::Slide => play(lib::TURN, 0.25, 1.0),
            // Le « toc » monte avec la valeur : 4 grave, 2048 aigu.
            PlayEvent::Merge(v) if !merged => {
                merged = true;
                play(lib::TOCK, 0.7, 0.8 + v.ilog2() as f32 * 0.06);
            }
            PlayEvent::Merge(_) => {}
            PlayEvent::Fact(_) => play(lib::CHIME, 0.5, 1.0),
            PlayEvent::Undo => play(lib::BLIP, 0.4, 0.8),
        }
    }
    for _ in clicks.read() {
        play(lib::CLICK, 0.6, 1.0);
    }
}
