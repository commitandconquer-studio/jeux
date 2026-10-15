//! Sons et musique, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{AudioKitPlugin, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::GameState;
use crate::game::NovaEvent;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioKitPlugin)
            .add_message::<UiClick>()
            .add_systems(Startup, start_music)
            .add_systems(Update, play_sounds);
    }
}

/// Clic sur un bouton de l'interface.
#[derive(Message, Clone, Copy)]
pub struct UiClick;

fn start_music(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(server.load(lib::MUSIC_NOVA)),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)),
        Mutable,
    ));
}

fn play_sounds(
    mut commands: Commands,
    server: Res<AssetServer>,
    sound: Res<SoundOn>,
    state: Res<State<GameState>>,
    mut events: MessageReader<NovaEvent>,
    mut clicks: MessageReader<UiClick>,
) {
    let mut play = |path: &'static str, volume: f32| {
        if sound.0 {
            commands.spawn((AudioPlayer::new(server.load(path)), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume))));
        }
    };
    // La démo du menu joue en silence.
    let in_game = *state.get() == GameState::Playing;
    for event in events.read() {
        if !in_game {
            continue;
        }
        match event {
            NovaEvent::Rotate => play(lib::BLIP, 0.35),
            NovaEvent::Lock => play(lib::CLUNK, 0.45),
            NovaEvent::HardDrop => play(lib::TURN, 0.4),
            NovaEvent::Hold => play(lib::BLIP, 0.3),
            NovaEvent::Lines(rows) => {
                play(lib::CHARGE, 0.6);
                play(lib::LASER, 0.5 + rows.len() as f32 * 0.1);
                if rows.len() == 4 {
                    play(lib::CHIME, 0.5);
                }
            }
            NovaEvent::Warning => play(lib::ALARM, 0.55),
            NovaEvent::EnemyShot => play(lib::CRASH, 0.6),
            NovaEvent::CruiserDestroyed => {
                play(lib::EXPLOSION, 1.0);
                play(lib::WIN, 0.6);
            }
            NovaEvent::Defeat => play(lib::GAME_OVER, 0.8),
        }
    }
    for _ in clicks.read() {
        play(lib::CLICK, 0.6);
    }
}
