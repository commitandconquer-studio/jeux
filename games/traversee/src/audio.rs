//! Sons et musique, tirés de la bibliothèque commune (`assets/sounds/`).
use bevy::audio::Volume;
use bevy::prelude::*;
use game_common::sounds as lib;
pub use game_common::audio::{Mutable, SoundOn};
use game_common::audio::Musique;

use crate::GameState;
use crate::frog::Frog;
use crate::lanes::Mover;

/// Pendant le développement (`cargo run`, compilation de test), le jeu démarre sans son :
/// `TRAVERSEE_SON=1` le remet. Ne touche pas au réglage enregistré du joueur.
pub fn silence_dev() -> bool {
    static MUET: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *MUET.get_or_init(|| {
        !cfg!(target_arch = "wasm32") && std::env::var_os("TRAVERSEE_SON").is_none() && (cfg!(debug_assertions) || std::env::var_os("CARGO_MANIFEST_DIR").is_some())
    })
}

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
    /// Chute dans le vide : un long souffle grave.
    Fall,
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
    splash: Handle<AudioSource>,
    honk: Handle<AudioSource>,
    honk2: Handle<AudioSource>,
    pub bell: Handle<AudioSource>,
    train: Handle<AudioSource>,
    alarm: Handle<AudioSource>,
    explosion: Handle<AudioSource>,
    /// Cri de rapace et souffle de chute : sons déjà accélérés / ralentis (pas de changement de vitesse en cours de lecture).
    screech: Handle<AudioSource>,
    music: Handle<AudioSource>,
    /// Une musique par monde : prairie, désert, neige, automne.
    worlds: [Handle<AudioSource>; 4],
    /// Les bruitages de Kenney (CC0, `assets/sounds/kenney`).
    k: KenneySons,
}

/// Les sons de Kenney, par catégorie (plusieurs variantes chacune, tirées au hasard).
struct KenneySons {
    pas_herbe: Vec<Handle<AudioSource>>,
    pas_bois: Vec<Handle<AudioSource>>,
    pas_pierre: Vec<Handle<AudioSource>>,
    pas_moquette: Vec<Handle<AudioSource>>,
    pas_metal: Vec<Handle<AudioSource>>,
    choc: Vec<Handle<AudioSource>>,
    clic: Vec<Handle<AudioSource>>,
    cadeau: Vec<Handle<AudioSource>>,
    cadeau_or: Vec<Handle<AudioSource>>,
    /// Petits jingles : début de partie, déblocage, record, fin de partie ; et la chute dans le vide.
    depart: Handle<AudioSource>,
    debloque: Handle<AudioSource>,
    record: Handle<AudioSource>,
    fin: Handle<AudioSource>,
    chute: Handle<AudioSource>,
}

impl KenneySons {
    fn charger(server: &AssetServer) -> Self {
        let lot = |prefixe: &str, n: usize, debut: usize, largeur: usize| -> Vec<Handle<AudioSource>> {
            (debut..debut + n).map(|i| server.load(format!("sounds/kenney/{prefixe}{i:0largeur$}.ogg"))).collect()
        };
        let un = |nom: &str| -> Handle<AudioSource> { server.load(format!("sounds/kenney/{nom}.ogg")) };
        let mut choc = lot("impactMetal_heavy_", 5, 0, 3);
        choc.extend(lot("impactMetal_light_", 2, 0, 3));
        Self {
            pas_herbe: lot("footstep_grass_", 5, 0, 3),
            pas_bois: lot("footstep_wood_", 5, 0, 3),
            pas_pierre: lot("footstep_concrete_", 5, 0, 3),
            pas_moquette: lot("footstep_carpet_", 5, 0, 3),
            pas_metal: lot("impactMetal_light_", 5, 0, 3),
            choc,
            clic: lot("click_", 5, 1, 3),
            cadeau: lot("confirmation_", 4, 1, 3),
            cadeau_or: lot("powerUp", 4, 1, 0),
            depart: un("jingles_PIZZI00"),
            debloque: un("jingles_PIZZI05"),
            record: un("jingles_NES04"),
            fin: un("jingles_HIT08"),
            chute: un("lowDown"),
        }
    }
}

/// Une variante au hasard.
fn au_hasard<'a>(sons: &'a [Handle<AudioSource>], rng: &mut game_common::Rng) -> &'a Handle<AudioSource> {
    &sons[rng.below(sons.len() as u32) as usize]
}

impl FromWorld for Sounds {
    fn from_world(world: &mut World) -> Self {
        let server = world.resource::<AssetServer>();
        let load = |path: &'static str| server.load(path);
        Sounds {
            splash: load(lib::SPLASH),
            honk: load(lib::HONK),
            honk2: load(lib::HONK2),
            bell: load(lib::BELL),
            train: load(lib::TRAIN),
            alarm: load(lib::ALARM),
            explosion: load(lib::EXPLOSION),
            screech: load("sounds/sfx/screech.wav"),
            music: load(lib::MUSIC_CAMPAGNE),
            worlds: [load(lib::MUSIC_PRAIRIE), load(lib::MUSIC_DESERT), load(lib::MUSIC_BANQUISE), load(lib::MUSIC_NOCTURNE)],
            k: KenneySons::charger(server),
        }
    }
}

fn play_sfx(
    mut commands: Commands,
    mut reader: MessageReader<Sfx>,
    sounds: Res<Sounds>,
    on: Res<SoundOn>,
    mut honk_alt: Local<bool>,
    mut rng: ResMut<game_common::Rng>,
    frog: Res<Frog>,
    progress: Res<crate::progress::Progress>,
    score: Res<game_common::Score>,
) {
    for sfx in reader.read() {
        if !on.0 || silence_dev() {
            continue;
        }
        let (handle, volume) = match *sfx {
            Sfx::Hop => {
                // Un pas par saut, sur le sol du monde : herbe, planches, moquette du cimetière, pierre, métal.
                use crate::extra::Biome;
                let monde = Biome::of(frog.max_row_i(), progress.total + score.current);
                let sons = match monde {
                    Biome::Pirate => &sounds.k.pas_bois,
                    Biome::Halloween => &sounds.k.pas_moquette,
                    Biome::Medieval => &sounds.k.pas_pierre,
                    Biome::Space => &sounds.k.pas_metal,
                    _ => &sounds.k.pas_herbe,
                };
                // Petite variation de hauteur, pour que deux pas ne sonnent jamais pareil.
                let vitesse = 0.92 + rng.below(16) as f32 * 0.01;
                commands.spawn((AudioPlayer::new(au_hasard(sons, &mut rng).clone()), PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.55)).with_speed(vitesse)));
                continue;
            }
            Sfx::Power => (au_hasard(&sounds.k.cadeau_or, &mut rng), 0.7),
            Sfx::Start => (&sounds.k.debloque, 0.55),
            Sfx::GameOver => (&sounds.k.fin, 0.7),
            // Au début de partie : un petit jingle (le cri du crapaud n'a plus de sens avec des humains).
            Sfx::Croak => (&sounds.k.depart, 0.6),
            Sfx::Splash => (&sounds.splash, 0.9),
            Sfx::Crash => (au_hasard(&sounds.k.choc, &mut rng), 0.9),
            Sfx::Honk => {
                *honk_alt = !*honk_alt;
                (if *honk_alt { &sounds.honk } else { &sounds.honk2 }, 0.45)
            }
            Sfx::Chime => (&sounds.k.debloque, 0.6),
            Sfx::Click => (au_hasard(&sounds.k.clic, &mut rng), 0.6),
            Sfx::Eat => (au_hasard(&sounds.k.cadeau, &mut rng), 0.8),
            Sfx::Alarm => (&sounds.alarm, 0.5),
            Sfx::Win => (&sounds.k.record, 0.6),
            Sfx::Explosion => (&sounds.explosion, 0.55),
            Sfx::Fall => (&sounds.k.chute, 0.6),
            // Le hurlement du loup, accéléré, fait un cri de rapace (fichier préparé : un changement de vitesse en cours de lecture figeait la version web).
            Sfx::Screech => (&sounds.screech, 0.6),
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

/// Interrupteur général de la musique : coupée tant qu'on n'a pas trouvé mieux. Les bruitages restent.
const MUSIQUE: bool = false;

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
    // Musique coupée pour de bon, en attendant mieux (décision d'Olivier, 26 octobre 2026).
    if !MUSIQUE || silence_dev() {
        return;
    }
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
        Some(crate::extra::Biome::Space) => sounds.worlds[2].clone(),
        Some(crate::extra::Biome::FarWest) => sounds.worlds[1].clone(),
        Some(crate::extra::Biome::Halloween) => sounds.worlds[3].clone(),
        Some(crate::extra::Biome::Cyber) => sounds.worlds[2].clone(),
        Some(crate::extra::Biome::Medieval) => sounds.worlds[0].clone(),
        Some(crate::extra::Biome::Pirate) => sounds.worlds[1].clone(),
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
    // Dans le navigateur, changer la vitesse de la musique à chaque image fige la page
    // (le lecteur recrée son rééchantillonneur) : la musique garde sa vitesse normale.
    if cfg!(target_arch = "wasm32") {
        return;
    }
    for sink in &mut sinks {
        sink.set_speed(*speed);
    }
}
