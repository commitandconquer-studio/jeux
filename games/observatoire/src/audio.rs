//! Le son : un nocturne en boîte à musique, et de petits effets de jeu.
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::jeu::JeuEvenement;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, musique).add_systems(Update, effets);
    }
}

fn musique(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_NOCTURNE)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Musique, Mutable));
}

fn effets(mut commands: Commands, server: Res<AssetServer>, son: Res<SoundOn>, mut evenements: MessageReader<JeuEvenement>) {
    for e in evenements.read() {
        if !son.0 {
            continue;
        }
        let (fichier, volume) = match e {
            JeuEvenement::Pose => (lib::TOCK, 0.6),
            JeuEvenement::Efface => (lib::TURN, 0.5),
            JeuEvenement::Erreur => (lib::CLUNK, 0.35),
            JeuEvenement::Indice => (lib::CHIME, 0.5),
            JeuEvenement::Victoire => (lib::WIN, 0.8),
        };
        commands.spawn((AudioPlayer::new(server.load(fichier)), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume))));
    }
}
