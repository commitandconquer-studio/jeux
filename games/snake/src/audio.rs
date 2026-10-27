//! Sons et musiques, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::audio::{Musique, Mutable, SoundOn};
use game_common::sounds as lib;

use crate::settings::Settings;
use crate::world::Universe;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Sfx>()
            .add_systems(Update, (switch_music, play_sfx));
    }
}

/// Un effet sonore à jouer.
#[derive(Message, Clone, Copy)]
pub enum Sfx {
    /// Bouchée : le son monte un peu avec la longueur du serpent.
    Eat(u32),
    Turn,
    Teleport,
    Boost,
    Spawn,
    Start,
    GameOver,
    Click,
}

#[derive(Component)]
struct Music;

fn music_for(universe: Universe) -> &'static str {
    match universe {
        Universe::Prairie => lib::MUSIC_PRAIRIE,
        Universe::Desert => lib::MUSIC_DESERT,
        Universe::Banquise => lib::MUSIC_BANQUISE,
        Universe::Ocean => lib::MUSIC_OCEAN,
        Universe::Volcan => lib::MUSIC_VOLCAN,
    }
}

/// Une musique par univers : on change de morceau quand l'univers change.
fn switch_music(
    mut commands: Commands,
    settings: Res<Settings>,
    server: Res<AssetServer>,
    sound: Res<SoundOn>,
    mut playing: Local<Option<Universe>>,
    current: Query<Entity, With<Music>>,
) {
    if *playing == Some(settings.universe) {
        return;
    }
    *playing = Some(settings.universe);
    for e in &current {
        commands.entity(e).despawn();
    }
    let settings = PlaybackSettings::LOOP.with_volume(Volume::Linear(0.35));
    commands.spawn((
        AudioPlayer::new(server.load(music_for(playing.unwrap()))),
        // Démarre en sourdine si le son est coupé.
        PlaybackSettings { muted: !sound.0, ..settings },
        Music,
        Musique,
        Mutable,
    ));
}

fn play_sfx(mut commands: Commands, mut reader: MessageReader<Sfx>, server: Res<AssetServer>, sound: Res<SoundOn>) {
    for sfx in reader.read() {
        if !sound.0 {
            continue;
        }
        let (path, volume, speed) = match *sfx {
            Sfx::Eat(score) => (lib::EAT, 0.7, 1.0 + score.min(30) as f32 * 0.02),
            Sfx::Turn => (lib::TURN, 0.25, 1.0),
            Sfx::Teleport => (lib::BLIP, 0.6, 0.8),
            Sfx::Boost => (lib::CHARGE, 0.5, 1.2),
            Sfx::Spawn => (lib::CLUNK, 0.5, 1.0),
            Sfx::Start => (lib::START, 0.6, 1.0),
            Sfx::GameOver => (lib::GAME_OVER, 0.8, 1.0),
            Sfx::Click => (lib::CLICK, 0.6, 1.0),
        };
        commands.spawn((
            AudioPlayer::new(server.load(path)),
            PlaybackSettings { speed, ..PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)) },
        ));
    }
}
