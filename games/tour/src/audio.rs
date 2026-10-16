//! Sons et musique, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::tower::TowerEvent;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UiClick>()
            .add_systems(Startup, start_music)
            .add_systems(Update, play_sounds);
    }
}

/// Clic sur un bouton de l'interface.
#[derive(Message, Clone, Copy)]
pub struct UiClick;

fn start_music(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(server.load(lib::MUSIC_ZEN)),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.35)),
        Musique,
        Mutable,
    ));
}

fn play_sounds(
    mut commands: Commands,
    server: Res<AssetServer>,
    sound: Res<SoundOn>,
    mut tower: MessageReader<TowerEvent>,
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
    for event in tower.read() {
        match *event {
            TowerEvent::Placed { perfect, combo } => {
                // Le « toc » monte d'un cran à chaque « Parfait » d'affilée.
                play(lib::TOCK, 0.8, 1.0 + combo.min(12) as f32 * 0.06);
                if perfect {
                    play(lib::CHIME, 0.45, 1.0 + combo.min(10) as f32 * 0.05);
                }
            }
            TowerEvent::Missed => {
                play(lib::CRASH, 0.6, 1.0);
                play(lib::GAME_OVER, 0.8, 1.0);
            }
        }
    }
    for _ in clicks.read() {
        play(lib::CLICK, 0.6, 1.0);
    }
}
