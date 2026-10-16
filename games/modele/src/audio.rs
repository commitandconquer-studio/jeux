//! Le son : la musique (marquée `Musique`) et un effet quand une pièce est ramassée.
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::play::Ramassee;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, musique).add_systems(Update, effets);
    }
}

fn musique(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_PRAIRIE)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Musique, Mutable));
}

fn effets(mut commands: Commands, server: Res<AssetServer>, son: Res<SoundOn>, mut ramassees: MessageReader<Ramassee>) {
    for _ in ramassees.read() {
        // Son coupé : l'effet n'est pas joué ; son volume suit « Volume des effets ».
        if son.0 {
            commands.spawn((AudioPlayer::new(server.load(lib::EAT)), PlaybackSettings::DESPAWN));
        }
    }
}
