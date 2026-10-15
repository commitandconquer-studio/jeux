//! Noms des fichiers de la bibliothèque de sons commune (`assets/sounds/`).
//!
//! Les sons sont générés par `cargo run -p sfx-gen` ; la liste complète avec
//! leurs descriptions est dans `assets/sounds/CATALOGUE.md`.

// Interface
pub const CLICK: &str = "sounds/ui/click.wav";
pub const START: &str = "sounds/ui/start.wav";
pub const CHIME: &str = "sounds/ui/chime.wav";
pub const GAME_OVER: &str = "sounds/ui/gameover.wav";
pub const WIN: &str = "sounds/ui/win.wav";

// Effets
pub const HOP: &str = "sounds/sfx/hop.wav";
pub const CROAK: &str = "sounds/sfx/croak.wav";
pub const SPLASH: &str = "sounds/sfx/splash.wav";
pub const CRASH: &str = "sounds/sfx/crash.wav";
pub const HONK: &str = "sounds/sfx/honk.wav";
pub const HONK2: &str = "sounds/sfx/honk2.wav";
pub const BELL: &str = "sounds/sfx/bell.wav";
pub const TRAIN: &str = "sounds/sfx/train.wav";
pub const EAT: &str = "sounds/sfx/eat.wav";
pub const TURN: &str = "sounds/sfx/turn.wav";
pub const TOCK: &str = "sounds/sfx/tock.wav";
pub const PUSH: &str = "sounds/sfx/push.wav";
pub const LASER: &str = "sounds/sfx/laser.wav";
pub const BLIP: &str = "sounds/sfx/blip.wav";
pub const CLUNK: &str = "sounds/sfx/clunk.wav";
pub const CHARGE: &str = "sounds/sfx/charge.wav";
pub const ALARM: &str = "sounds/sfx/alarm.wav";
pub const EXPLOSION: &str = "sounds/sfx/explosion.wav";

// Musiques (en boucle)
pub const MUSIC_CAMPAGNE: &str = "sounds/music/campagne.wav";
pub const MUSIC_PRAIRIE: &str = "sounds/music/prairie.wav";
pub const MUSIC_DESERT: &str = "sounds/music/desert.wav";
pub const MUSIC_BANQUISE: &str = "sounds/music/banquise.wav";
pub const MUSIC_OCEAN: &str = "sounds/music/ocean.wav";
pub const MUSIC_VOLCAN: &str = "sounds/music/volcan.wav";
pub const MUSIC_ZEN: &str = "sounds/music/zen.wav";
pub const MUSIC_NOVA: &str = "sounds/music/nova.wav";

// Polices (une par univers ; toutes sous licence OFL, voir assets/fonts/)
/// Croâ & Cie : Noto Sans, ronde et lisible.
pub const FONT: &str = "fonts/NotoSans-Bold.ttf";
/// Nova : Space Grotesk, plus technique.
pub const FONT_NOVA: &str = "fonts/SpaceGrotesk-Bold.ttf";
/// Eurêka : Comic Relief, façon écriture à la main.
pub const FONT_EUREKA: &str = "fonts/ComicRelief-Bold.ttf";
