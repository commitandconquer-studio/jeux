//! Le son : la musique (marquée `Musique`) et les effets de pose, de rotation et de fin.
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
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_PRAIRIE)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Musique, Mutable));
}

fn effets(mut commands: Commands, server: Res<AssetServer>, son: Res<SoundOn>, mut evenements: MessageReader<JeuEvenement>) {
    for e in evenements.read() {
        // Son coupé : l'effet n'est pas joué ; son volume suit « Volume des effets ».
        if !son.0 {
            continue;
        }
        let fichier = match e {
            JeuEvenement::Pose => lib::TOCK,
            JeuEvenement::Tourne => lib::TURN,
            JeuEvenement::Fin => lib::WIN,
        };
        commands.spawn((AudioPlayer::new(server.load(fichier)), PlaybackSettings::DESPAWN));
    }
}
