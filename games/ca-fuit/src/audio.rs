//! Le son : la musique de la prairie et les effets des événements du jeu.
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::jeu::Evenement;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, musique).add_systems(Update, effets);
    }
}

fn musique(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_PRAIRIE)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Musique, Mutable));
}

fn effets(mut commands: Commands, server: Res<AssetServer>, son: Res<SoundOn>, mut evenements: MessageReader<Evenement>) {
    for e in evenements.read() {
        let fichier = match e {
            Evenement::Posee => lib::TOCK,
            Evenement::Tournee => lib::TURN,
            Evenement::Refusee => lib::CLUNK,
            Evenement::Piochee => lib::BLIP,
            Evenement::Deborde => lib::SPLASH,
            Evenement::Bouchee => lib::CHIME,
            Evenement::Finie => lib::WIN,
        };
        // Son coupé : l'effet n'est pas joué ; son volume suit « Volume des effets ».
        if son.0 {
            commands.spawn((AudioPlayer::new(server.load(fichier)), PlaybackSettings::DESPAWN));
        }
    }
}
