//! Le son : la musique de la traversée et les effets (tirs, rochers, bonus,
//! bouclier, explosions, et le « blabla » de Boulon quand il parle).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::duo::{Parle, Qui};
use crate::partie::Son;

pub struct AudioPlugin;

/// Un clic sur un bouton du jeu.
#[derive(Message, Clone, Copy)]
pub struct UiClick;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UiClick>().add_systems(Startup, musique).add_systems(Update, effets);
    }
}

fn musique(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((AudioPlayer::new(server.load(lib::MUSIC_PARSECS)), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)), Musique, Mutable));
}

fn effets(
    mut commands: Commands,
    server: Res<AssetServer>,
    son: Res<SoundOn>,
    mut sons: MessageReader<Son>,
    mut parle: MessageReader<Parle>,
    mut clics: MessageReader<UiClick>,
    time: Res<Time>,
    mut dernier_tir: Local<f32>,
) {
    let mut jouer = |chemin: &'static str, volume: f32| {
        if son.0 {
            commands.spawn((AudioPlayer::new(server.load(chemin)), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume))));
        }
    };
    let t = time.elapsed_secs();
    for s in sons.read() {
        match s {
            // Les tirs rapides ne doivent pas saturer : au plus un son tous les 0,09 s.
            Son::Tir => {
                if t - *dernier_tir > 0.09 {
                    *dernier_tir = t;
                    jouer(lib::LASER, 0.18);
                }
            }
            Son::Rocher => jouer(lib::ROCK, 0.5),
            Son::Explosion => jouer(lib::EXPLOSION, 0.6),
            Son::Bonus => jouer(lib::POWER_UP, 0.6),
            Son::Bouclier => jouer(lib::SHIELD, 0.55),
            Son::Alarme => jouer(lib::ALARM, 0.4),
        }
    }
    for p in parle.read() {
        match p.0 {
            Qui::Boulon => jouer(lib::CHATTER, 0.45),
            Qui::Pilote => jouer(lib::BLIP, 0.3),
        }
    }
    for _ in clics.read() {
        jouer(lib::CLICK, 0.6);
    }
}
