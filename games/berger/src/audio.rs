//! Le son : la musique du pré et les effets (aboiement, bêlements, loups).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::jeu::Evt;
use berger_core::{Evenement, Fin};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, musique).add_systems(Update, effets);
    }
}

fn musique(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_CAMPAGNE)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Musique, Mutable));
}

fn effets(mut commands: Commands, server: Res<AssetServer>, son: Res<SoundOn>, mut evts: MessageReader<Evt>, time: Res<Time>, mut dernier_belement: Local<f32>) {
    for Evt(e) in evts.read() {
        // Son coupé : l'effet n'est pas joué ; son volume suit « Volume des effets ».
        if !son.0 {
            continue;
        }
        let jouer = |commands: &mut Commands, nom: &'static str| {
            commands.spawn((AudioPlayer::new(server.load(nom)), PlaybackSettings::DESPAWN));
        };
        match e {
            Evenement::Aboiement => jouer(&mut commands, lib::DOG_BARK),
            Evenement::Bele(_) => {
                // Pas plus de deux bêlements par seconde, sinon c'est un vacarme.
                if time.elapsed_secs() - *dernier_belement > 0.5 {
                    *dernier_belement = time.elapsed_secs();
                    jouer(&mut commands, lib::SHEEP_BLEAT);
                }
            }
            Evenement::Entre(_) => jouer(&mut commands, lib::EAT),
            Evenement::Mange(_) => jouer(&mut commands, lib::CRASH),
            Evenement::LoupArrive(_) => jouer(&mut commands, lib::WOLF_HOWL),
            Evenement::LoupFuit(_) => jouer(&mut commands, lib::HOP),
            Evenement::Fin(Fin::Victoire) => jouer(&mut commands, lib::WIN),
            Evenement::Fin(_) => jouer(&mut commands, lib::GAME_OVER),
            Evenement::Sorti(_) | Evenement::Sursaut(_) => {}
        }
    }
}
