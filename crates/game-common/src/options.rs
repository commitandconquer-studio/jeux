//! Le panneau « Options », commun à tous les jeux : la langue, le son, la
//! musique et son volume, le volume des effets, et l'accès à l'ambiance.
//! Ces réglages valent pour tous les jeux du studio.
use bevy::prelude::*;

use crate::audio::{Reglages, SoundOn};
use crate::langue::{Langue, LangueChangee};
use crate::ui::{spawn_button, spawn_button_sized, spawn_gap, spawn_panel, spawn_text};

/// Bouton qui ouvre les Options (à mettre dans les menus).
#[derive(Component)]
pub struct OptionsBouton;

/// Bouton « Options » de taille standard.
pub fn spawn_options_button(commands: &mut Commands, parent: Entity) -> Entity {
    let (b, _) = spawn_button(commands, parent, tx!("Options", "Options", "Optionen", "Opzioni", "Opciones").t());
    commands.entity(b).insert(OptionsBouton);
    b
}

/// À insérer par un jeu qui veut la ligne « Style de mort » dans les Options.
#[derive(Resource)]
pub struct StyleMortOption;

#[derive(Component)]
struct Panneau;

#[derive(Component, Clone, Copy)]
enum Choix {
    Langue,
    Son,
    Musique,
    MusiqueMoins,
    MusiquePlus,
    EffetsMoins,
    EffetsPlus,
    StyleMort,
    Ambiance,
    Fermer,
}

/// La langue à l'ouverture du panneau (pour savoir s'il faut rebâtir le menu).
#[derive(Resource)]
struct LangueOuverture(Langue);

pub(crate) struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LangueOuverture(Langue::Fr)).add_observer(au_clic);
        if std::env::var_os("CC_OPEN_OPTIONS").is_some() {
            app.add_systems(Update, ouvrir_pour_test);
        }
    }
}

/// Test : `CC_OPEN_OPTIONS=1` ouvre les Options au bout de 2 secondes (captures).
fn ouvrir_pour_test(time: Res<Time>, mut fait: Local<bool>, mut commands: Commands, l: Res<Langue>, son: Res<SoundOn>, r: Res<Reglages>, style: Option<Res<StyleMortOption>>) {
    if !*fait && time.elapsed_secs() > 2.0 {
        *fait = true;
        ouvrir(&mut commands, *l, son.0, &r, style.is_some());
    }
}

fn etat(oui: bool) -> &'static str {
    if oui { tx!("activée", "on", "an", "attiva", "activada").t() } else { tx!("désactivée", "off", "aus", "disattivata", "desactivada").t() }
}

fn ouvrir(commands: &mut Commands, l: Langue, son: bool, r: &Reglages, style_mort: bool) {
    let panel = spawn_panel(commands, (Panneau, BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.35)), GlobalZIndex(50)));
    commands.entity(panel).insert(crate::ui::PanneauOpaque);
    commands.entity(panel).insert(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(9.0),
        padding: UiRect::axes(Val::Px(32.0), Val::Px(22.0)),
        border: UiRect::all(Val::Px(2.0)),
        border_radius: BorderRadius::all(Val::Px(24.0)),
        ..default()
    });
    spawn_text(commands, panel, tx!("Options", "Options", "Optionen", "Opzioni", "Opciones").t(), 36.0);
    spawn_gap(commands, panel, 2.0);
    let ligne = |commands: &mut Commands, texte: String, choix: Choix| {
        let (b, _) = spawn_button_sized(commands, panel, texte, 400.0, 44.0, 19.0);
        commands.entity(b).insert(choix);
    };
    ligne(commands, format!("{}{}{}", tx!("Langue", "Language", "Sprache", "Lingua", "Idioma").t(), crate::langue::deux_points(), l.nom()), Choix::Langue);
    ligne(commands, SoundOn(son).label(), Choix::Son);
    ligne(commands, format!("{}{}{}", tx!("Musique", "Music", "Musik", "Musica", "Música").t(), crate::langue::deux_points(), etat(r.musique)), Choix::Musique);
    reglette(commands, panel, tx!("Volume de la musique", "Music volume", "Musiklautstärke", "Volume musica", "Volumen de la música").t(), r.volume_musique, Choix::MusiqueMoins, Choix::MusiquePlus);
    reglette(commands, panel, tx!("Volume des effets", "Effects volume", "Effektlautstärke", "Volume effetti", "Volumen de efectos").t(), r.volume_effets, Choix::EffetsMoins, Choix::EffetsPlus);
    if style_mort {
        let nom = match r.style_mort {
            0 => tx!("Choupi", "Cute", "Niedlich", "Tenero", "Tierno"),
            2 => tx!("Gore ridicule", "Silly gore", "Alberner Splatter", "Gore ridicolo", "Gore ridículo"),
            _ => tx!("Normal", "Normal", "Normal", "Normale", "Normal"),
        };
        ligne(commands, format!("{}{}{}", tx!("Mort", "Death", "Tod", "Morte", "Muerte").t(), crate::langue::deux_points(), nom.t()), Choix::StyleMort);
    }
    ligne(commands, format!("{}…", tx!("Ambiance", "Atmosphere", "Stimmung", "Atmosfera", "Ambiente").t()), Choix::Ambiance);
    spawn_gap(commands, panel, 2.0);
    let (b, _) = spawn_button(commands, panel, tx!("Fermer", "Close", "Schließen", "Chiudi", "Cerrar").t());
    commands.entity(b).insert(Choix::Fermer);
}

/// Une ligne « Volume … : − 70 % + ».
fn reglette(commands: &mut Commands, panel: Entity, titre: &str, valeur: u8, moins: Choix, plus: Choix) {
    let rang = commands
        .spawn((Node { width: Val::Px(400.0), align_items: AlignItems::Center, justify_content: JustifyContent::SpaceBetween, column_gap: Val::Px(8.0), ..default() }, ChildOf(panel)))
        .id();
    let (b, _) = spawn_button_sized(commands, rang, "-", 52.0, 44.0, 26.0);
    commands.entity(b).insert(moins);
    spawn_text(commands, rang, format!("{titre}{}{} %", crate::langue::deux_points(), valeur as u32 * 10), 18.0);
    let (b, _) = spawn_button_sized(commands, rang, "+", 52.0, 44.0, 24.0);
    commands.entity(b).insert(plus);
}

#[allow(clippy::too_many_arguments)]
fn au_clic(
    clic: On<PointerClick>,
    mut commands: Commands,
    boutons: Query<(), With<OptionsBouton>>,
    choix: Query<&Choix>,
    panneaux: Query<Entity, With<Panneau>>,
    mut langue: ResMut<Langue>,
    mut son: ResMut<SoundOn>,
    mut r: ResMut<Reglages>,
    mut ouverture: ResMut<LangueOuverture>,
    mut changee: MessageWriter<LangueChangee>,
    style: Option<Res<StyleMortOption>>,
) {
    let fermer = |commands: &mut Commands| panneaux.iter().for_each(|e| commands.entity(e).despawn());
    if boutons.contains(clic.entity) {
        if panneaux.is_empty() {
            ouverture.0 = *langue;
            ouvrir(&mut commands, *langue, son.0, &r, style.is_some());
        }
        return;
    }
    let Ok(c) = choix.get(clic.entity) else { return };
    match c {
        Choix::Fermer | Choix::Ambiance => {
            fermer(&mut commands);
            // Langue changée : le jeu rebâtit son menu dans la nouvelle langue.
            if *langue != ouverture.0 {
                changee.write(LangueChangee);
            }
            if matches!(c, Choix::Ambiance) {
                commands.run_system_cached(crate::ambiance_panneau::ouvrir_depuis_options);
            }
            return;
        }
        Choix::Langue => {
            *langue = langue.suivante();
            langue.enregistrer();
            crate::langue::definir_maintenant(*langue);
        }
        Choix::Son => son.0 = !son.0,
        Choix::StyleMort => r.style_mort = (r.style_mort + 1) % 3,
        Choix::Musique => r.musique = !r.musique,
        Choix::MusiqueMoins => r.volume_musique = r.volume_musique.saturating_sub(1),
        Choix::MusiquePlus => r.volume_musique = (r.volume_musique + 1).min(10),
        Choix::EffetsMoins => r.volume_effets = r.volume_effets.saturating_sub(1),
        Choix::EffetsPlus => r.volume_effets = (r.volume_effets + 1).min(10),
    }
    fermer(&mut commands);
    ouvrir(&mut commands, *langue, son.0, &r, style.is_some());
}
