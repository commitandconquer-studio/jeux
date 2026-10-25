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
            .add_systems(Update, (music_world, music_speed).chain())
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
    /// Sonnerie d'alerte (l'aigle approche).
    Alarm,
    /// Petite fanfare (déblocage, record).
    Win,
    /// Cri aigu de l'aigle.
    Screech,
    /// Grosse explosion (le Speed-Bus).
    Explosion,
    /// Bonus rare (mouche dorée).
    Power,
    /// Début d'un nouveau monde.
    Start,
    /// Fin de partie sans record ni nouveauté.
    GameOver,
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
    alarm: Handle<AudioSource>,
    win: Handle<AudioSource>,
    explosion: Handle<AudioSource>,
    howl: Handle<AudioSource>,
    click: Handle<AudioSource>,
    music: Handle<AudioSource>,
    power: Handle<AudioSource>,
    start: Handle<AudioSource>,
    gameover: Handle<AudioSource>,
    /// Une musique par monde : prairie, désert, neige, automne.
    worlds: [Handle<AudioSource>; 4],
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
            alarm: load(lib::ALARM),
            win: load(lib::WIN),
            explosion: load(lib::EXPLOSION),
            howl: load(lib::WOLF_HOWL),
            click: load(lib::CLICK),
            music: load(lib::MUSIC_CAMPAGNE),
            power: load(lib::POWER_UP),
            start: load(lib::START),
            gameover: load(lib::GAME_OVER),
            worlds: [load(lib::MUSIC_PRAIRIE), load(lib::MUSIC_DESERT), load(lib::MUSIC_BANQUISE), load(lib::MUSIC_NOCTURNE)],
        }
    }
}

fn play_sfx(
    mut commands: Commands,
    mut reader: MessageReader<Sfx>,
    sounds: Res<Sounds>,
    on: Res<SoundOn>,
    mut honk_alt: Local<bool>,
    time: Res<Time>,
    mut combo: Local<(f32, u32)>,
) {
    for sfx in reader.read() {
        if !on.0 {
            continue;
        }
        let (handle, volume) = match *sfx {
            Sfx::Hop => {
                // Les sauts enchaînés montent d'un cran à chaque fois, comme une gamme.
                let now = time.elapsed_secs();
                combo.1 = if now - combo.0 < 0.7 { (combo.1 + 1).min(12) } else { 0 };
                combo.0 = now;
                let speed = 1.0 + combo.1 as f32 * 0.045;
                commands.spawn((AudioPlayer::new(sounds.hop.clone()), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.5)).with_speed(speed)));
                continue;
            }
            Sfx::Power => (&sounds.power, 0.8),
            Sfx::Start => (&sounds.start, 0.6),
            Sfx::GameOver => (&sounds.gameover, 0.7),
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
            Sfx::Alarm => (&sounds.alarm, 0.5),
            Sfx::Win => (&sounds.win, 0.7),
            Sfx::Explosion => (&sounds.explosion, 0.55),
            Sfx::Screech => {
                // Le hurlement du loup, accéléré, fait un cri de rapace.
                commands.spawn((AudioPlayer::new(sounds.howl.clone()), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.6)).with_speed(2.6)));
                continue;
            }
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

/// La musique change avec le monde (et revient à la prairie au menu).
fn music_world(
    mut commands: Commands,
    sounds: Res<Sounds>,
    state: Res<State<GameState>>,
    frog: Res<Frog>,
    progress: Res<crate::progress::Progress>,
    score: Res<game_common::Score>,
    old: Query<Entity, With<Musique>>,
    mut current: Local<Option<Option<crate::extra::Biome>>>,
) {
    // Au menu et au garage : la musique d'accueil. En jeu : celle du monde.
    let want = match state.get() {
        GameState::Menu | GameState::Garage => None,
        _ => Some(crate::extra::Biome::of(frog.max_row_i(), progress.total + score.current)),
    };
    if *current == Some(want) {
        return;
    }
    *current = Some(want);
    for e in &old {
        commands.entity(e).despawn();
    }
    let track = match want {
        None => sounds.music.clone(),
        Some(crate::extra::Biome::Prairie) => sounds.worlds[0].clone(),
        Some(crate::extra::Biome::Desert) => sounds.worlds[1].clone(),
        Some(crate::extra::Biome::Snow) => sounds.worlds[2].clone(),
        Some(crate::extra::Biome::Autumn) => sounds.worlds[3].clone(),
        Some(crate::extra::Biome::Jurassic) => sounds.worlds[0].clone(),
        Some(crate::extra::Biome::Nippon) => sounds.worlds[3].clone(),
    };
    commands.spawn((AudioPlayer::new(track), PlaybackSettings::LOOP.with_volume(Volume::Linear(0.35)), Musique, Mutable));
}

/// Le tempo monte avec la difficulté ; à la mort, la musique ralentit comme un disque qui s'arrête.
fn music_speed(
    time: Res<Time<Real>>,
    state: Res<State<GameState>>,
    frog: Res<Frog>,
    mut sinks: Query<&mut AudioSink, With<Musique>>,
    mut speed: Local<f32>,
) {
    let d = (frog.max_row_i() as f32 / 150.0).clamp(0.0, 1.0);
    let goal = match state.get() {
        GameState::Playing => 1.0 + 0.22 * d,
        GameState::Dying => 0.55,
        _ => 1.0,
    };
    if *speed == 0.0 {
        *speed = 1.0;
    }
    let rate = if goal < *speed { 1.6 } else { 0.8 };
    *speed += (goal - *speed).clamp(-rate * time.delta_secs(), rate * time.delta_secs());
    for sink in &mut sinks {
        sink.set_speed(*speed);
    }
}
