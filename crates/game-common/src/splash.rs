//! Écran de démarrage du studio, commun à tous les jeux : l'illustration de
//! la charte, une barre de chargement, puis un fondu vers le jeu.
//!
//! C'est une couche d'interface posée au-dessus de tout : le jeu démarre
//! normalement en dessous (sa caméra, son menu, sa musique). Un clic ou une
//! touche passe l'écran.
//!
//! - `CC_NO_SPLASH=1` ou un mode capture d'un jeu (`…_CAPTURE`) : pas d'écran.
//! - `SPLASH_CAPTURE=fichier.png` : capture l'écran de démarrage et quitte.
use bevy::prelude::*;
use bevy::text::LetterSpacing;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use bevy::window::PrimaryWindow;

use crate::brand::{self, colors};
use crate::ui::OwnFont;

/// Durée de remplissage de la barre, puis du fondu (secondes).
const FILL: f32 = 1.8;
const HOLD: f32 = 0.35;
const FADE: f32 = 0.5;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // (Dans le navigateur, la liste des variables d'environnement n'existe pas.)
        let capture = !cfg!(target_arch = "wasm32") && std::env::vars().any(|(k, _)| k.ends_with("_CAPTURE") && k != "SPLASH_CAPTURE");
        if capture || std::env::var_os("CC_NO_SPLASH").is_some() {
            return;
        }
        app.add_systems(Startup, spawn).add_systems(Update, (fit, animate, skip));
    }
}

#[derive(Resource)]
struct Splash {
    image: Handle<Image>,
    /// Temps écoulé depuis que l'illustration est chargée.
    t: f32,
    /// Temps d'attente du chargement (on n'attend pas plus d'une seconde).
    waited: f32,
    shot: bool,
}

#[derive(Component)]
struct SplashRoot;

/// Partie de l'écran, et son opacité de base (pour le fondu).
#[derive(Component, Clone, Copy)]
enum Part {
    Picture,
    Track,
    Fill,
    Label,
}

fn spawn(mut commands: Commands, server: Res<AssetServer>) {
    let image = server.load(brand::SPLASH_IMAGE);
    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(colors::SKY),
            GlobalZIndex(i32::MAX - 1),
            SplashRoot,
        ))
        .id();
    commands.spawn((
        Node { flex_shrink: 0.0, ..default() },
        ImageNode { image: image.clone(), color: Color::WHITE.with_alpha(0.0), ..default() },
        Part::Picture,
        Pickable::IGNORE,
        ChildOf(root),
    ));
    // Barre de chargement et légende, en bas.
    let bottom = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Vh(5.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Vh(1.6),
                ..default()
            },
            Pickable::IGNORE,
            ChildOf(root),
        ))
        .id();
    let track = commands
        .spawn((
            Node {
                width: Val::VMin(46.0),
                height: Val::VMin(2.2),
                padding: UiRect::all(Val::VMin(0.3)),
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(colors::NIGHT.with_alpha(0.0)),
            Part::Track,
            Pickable::IGNORE,
            ChildOf(bottom),
        ))
        .id();
    commands.spawn((
        Node { width: Val::Percent(0.0), height: Val::Percent(100.0), border_radius: BorderRadius::MAX, ..default() },
        bar_gradient(0.0),
        Part::Fill,
        Pickable::IGNORE,
        ChildOf(track),
    ));
    commands.spawn((
        Text::new(tx!("CHARGEMENT DES MONDES…", "LOADING WORLDS…", "WELTEN WERDEN GELADEN…", "CARICAMENTO DEI MONDI…", "CARGANDO MUNDOS…").t()),
        TextFont::from_font_size(FontSize::VMin(2.0)).with_font(server.load(brand::FONT_LOGO)),
        LetterSpacing::Px(3.0),
        TextColor(colors::IVORY.with_alpha(0.0)),
        TextShadow { offset: Vec2::new(0.0, 2.0), color: colors::NIGHT.with_alpha(0.0) },
        OwnFont,
        Part::Label,
        Pickable::IGNORE,
        ChildOf(bottom),
    ));
    commands.insert_resource(Splash { image, t: 0.0, waited: 0.0, shot: false });
}

/// Dégradé vert → citron de la barre, à l'opacité `alpha`.
fn bar_gradient(alpha: f32) -> BackgroundGradient {
    BackgroundGradient::from(LinearGradient::to_right(vec![
        ColorStop::auto(colors::GREEN.with_alpha(alpha)),
        ColorStop::auto(colors::LEMON.with_alpha(alpha)),
    ]))
}

/// Recadre l'illustration pour couvrir toute la fenêtre, sans la déformer.
fn fit(window: Query<&Window, With<PrimaryWindow>>, mut parts: Query<(&Part, &mut Node)>) {
    let Ok(w) = window.single() else { return };
    let wide = w.width() / w.height().max(1.0) > brand::SPLASH_RATIO;
    for (part, mut node) in &mut parts {
        if matches!(part, Part::Picture) {
            if wide {
                node.width = Val::Vw(100.0);
                node.height = Val::Vw(100.0 / brand::SPLASH_RATIO);
            } else {
                node.height = Val::Vh(100.0);
                node.width = Val::Vh(100.0 * brand::SPLASH_RATIO);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn animate(
    mut commands: Commands,
    time: Res<Time>,
    server: Res<AssetServer>,
    splash: Option<ResMut<Splash>>,
    root: Query<Entity, With<SplashRoot>>,
    mut root_bg: Query<&mut BackgroundColor, (With<SplashRoot>, Without<Part>)>,
    mut parts: Query<(&Part, &mut Node, Option<&mut ImageNode>, Option<&mut BackgroundColor>, Option<&mut BackgroundGradient>, Option<&mut TextColor>, Option<&mut TextShadow>), Without<SplashRoot>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(mut splash) = splash else { return };
    let dt = time.delta_secs().min(0.1);
    if server.is_loaded_with_dependencies(&splash.image) || splash.waited > 1.0 {
        splash.t += dt;
    } else {
        splash.waited += dt;
        return;
    }
    let t = splash.t;
    // Apparition rapide, barre qui se remplit (en ralentissant), fondu final.
    let appear = (t / 0.25).min(1.0);
    let fade = 1.0 - ((t - FILL - HOLD) / FADE).clamp(0.0, 1.0);
    let alpha = appear * fade;
    let p = (t / FILL).clamp(0.0, 1.0);
    let progress = 1.0 - (1.0 - p) * (1.0 - p);
    for mut bg in &mut root_bg {
        bg.0 = colors::SKY.with_alpha(fade);
    }
    for (part, mut node, image, bg, gradient, text, shadow) in &mut parts {
        match part {
            Part::Picture => {
                if let Some(mut image) = image {
                    image.color = Color::WHITE.with_alpha(alpha);
                }
            }
            Part::Track => {
                if let Some(mut bg) = bg {
                    bg.0 = colors::NIGHT.with_alpha(0.6 * alpha);
                }
            }
            Part::Fill => {
                node.width = Val::Percent(100.0 * progress);
                if let Some(mut gradient) = gradient {
                    *gradient = bar_gradient(alpha);
                }
            }
            Part::Label => {
                if let Some(mut text) = text {
                    text.0 = colors::IVORY.with_alpha(alpha);
                }
                if let Some(mut shadow) = shadow {
                    shadow.color = colors::NIGHT.with_alpha(0.6 * alpha);
                }
            }
        }
    }
    if let Ok(path) = std::env::var("SPLASH_CAPTURE") {
        if !splash.shot && t > FILL * 0.6 {
            splash.shot = true;
            commands.spawn(Screenshot::primary_window()).observe(save_to_disk(path));
        } else if splash.shot && t > FILL * 0.6 + 1.0 {
            exit.write(AppExit::Success);
        }
        // En mode capture, on reste sur la barre en cours de remplissage.
        splash.t = splash.t.min(FILL * 0.6 + 1.1);
        return;
    }
    if t >= FILL + HOLD + FADE {
        for e in &root {
            commands.entity(e).despawn();
        }
        commands.remove_resource::<Splash>();
    }
}

/// Un clic, une touche ou un toucher passe directement au fondu.
fn skip(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    splash: Option<ResMut<Splash>>,
) {
    let Some(mut splash) = splash else { return };
    let pressed = keys.get_just_pressed().next().is_some() || mouse.get_just_pressed().next().is_some() || touches.any_just_pressed();
    if pressed && std::env::var_os("SPLASH_CAPTURE").is_none() {
        splash.t = splash.t.max(FILL + HOLD);
    }
}
