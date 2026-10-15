//! Sons et musique, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{AudioKitPlugin, Mutable, SoundOn};
use game_common::calendar::{Date, feast_on};
use game_common::sounds as lib;

use crate::play::PlayEvent;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioKitPlugin)
            .add_message::<UiClick>()
            .add_systems(Startup, start_music)
            .add_systems(Update, play_sounds);
    }
}

#[derive(Message, Clone, Copy)]
pub struct UiClick;

fn start_music(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_ORGUE)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Mutable));
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
            commands.spawn((AudioPlayer::new(server.load(path)), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume))));
        }
    };
    for e in events.read() {
        match e {
            PlayEvent::Cross => play(lib::QUILL, 0.5),
            PlayEvent::Bishop => play(lib::STAMP, 0.8),
            PlayEvent::Erase => play(lib::TURN, 0.3),
            PlayEvent::Conflict => play(lib::BLIP, 0.4),
            PlayEvent::Hint => play(lib::CHIME, 0.4),
            PlayEvent::Won => {
                // Du jeudi au samedi saints, les cloches sont parties à Rome !
                let silent = feast_on(Date::today()).is_some_and(|f| f.bells_silent());
                play(if silent { lib::WIN } else { lib::CHURCH_BELLS }, 0.8);
            }
        }
    }
    for _ in clicks.read() {
        play(lib::CLICK, 0.6);
    }
}
