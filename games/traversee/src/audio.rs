//! Sons et musique, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::sounds as lib;
pub use game_common::audio::{Mutable, SoundOn};
use game_common::audio::Musique;

use crate::GameState;
use crate::frog::Frog;
use crate::lanes::Mover;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sounds>()
            .add_message::<Sfx>()
            .add_systems(Startup, start_music)
            .add_systems(OnEnter(GameState::Playing), |mut sfx: MessageWriter<Sfx>| {
                sfx.write(Sfx::Croak);
            })
            .add_systems(
                Update,
                (honk_near_misses.run_if(in_state(GameState::Playing)), play_sfx).chain(),
            );
    }
}

/// Un effet sonore à jouer.
#[derive(Message, Clone, Copy)]
pub enum Sfx {
    Hop,
    Croak,
    Splash,
    Crash,
    Honk,
    Chime,
    Click,
    Eat,
    /// Passage du train, avec un volume selon la distance.
    Train(f32),
}

#[derive(Resource)]
pub struct Sounds {
    hop: Handle<AudioSource>,
    croak: Handle<AudioSource>,
    splash: Handle<AudioSource>,
    crash: Handle<AudioSource>,
    honk: Handle<AudioSource>,
    honk2: Handle<AudioSource>,
    pub bell: Handle<AudioSource>,
    train: Handle<AudioSource>,
    chime: Handle<AudioSource>,
    eat: Handle<AudioSource>,
    click: Handle<AudioSource>,
    music: Handle<AudioSource>,
}

impl FromWorld for Sounds {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource::<AssetServer>();
        let load = |path: &'static str| server.load(path);
        Sounds {
            hop: load(lib::HOP),
            croak: load(lib::CROAK),
            splash: load(lib::SPLASH),
            crash: load(lib::CRASH),
            honk: load(lib::HONK),
            honk2: load(lib::HONK2),
            bell: load(lib::BELL),
            train: load(lib::TRAIN),
            chime: load(lib::CHIME),
            eat: load(lib::EAT),
            click: load(lib::CLICK),
            music: load(lib::MUSIC_CAMPAGNE),
        }
    }
}

fn start_music(mut commands: Commands, sounds: Res<Sounds>) {
    commands.spawn((
        AudioPlayer::new(sounds.music.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.35)),
        Musique,
        Mutable,
    ));
}

fn play_sfx(
    mut commands: Commands,
    mut reader: MessageReader<Sfx>,
    sounds: Res<Sounds>,
    on: Res<SoundOn>,
    mut honk_alt: Local<bool>,
) {
    for sfx in reader.read() {
        if !on.0 {
            continue;
        }
        let (handle, volume) = match *sfx {
            Sfx::Hop => (&sounds.hop, 0.5),
            Sfx::Croak => (&sounds.croak, 0.8),
            Sfx::Splash => (&sounds.splash, 0.9),
            Sfx::Crash => (&sounds.crash, 1.0),
            Sfx::Honk => {
                *honk_alt = !*honk_alt;
                (if *honk_alt { &sounds.honk } else { &sounds.honk2 }, 0.45)
            }
            Sfx::Chime => (&sounds.chime, 0.7),
            Sfx::Click => (&sounds.click, 0.6),
            Sfx::Eat => (&sounds.eat, 0.9),
            Sfx::Train(v) => (&sounds.train, v),
        };
        commands.spawn((AudioPlayer::new(handle.clone()), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume))));
    }
}

/// Une voiture qui arrive juste à côté du crapaud klaxonne (pas trop souvent).
fn honk_near_misses(
    time: Res<Time>,
    frog: Res<Frog>,
    movers: Query<(&Mover, &Transform)>,
    mut cooldown: Local<f32>,
    mut sfx: MessageWriter<Sfx>,
) {
    *cooldown -= time.delta_secs();
    if *cooldown > 0.0 {
        return;
    }
    let close = movers.iter().any(|(m, tf)| {
        let dx = tf.translation.x - frog.pos.x;
        // Sur la voie juste devant ou derrière, et qui fonce vers le crapaud.
        m.is_deadly()
            && (m.row - frog.row).abs() == 1
            && dx.abs() < m.half_len + 1.5
            && dx * m.speed < 0.0
    });
    if close {
        sfx.write(Sfx::Honk);
        *cooldown = 2.5;
    }
}
