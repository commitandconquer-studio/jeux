//! Les écrans : accueil, réglages, bandeau de jeu, pause et fin de partie.
use bevy::prelude::*;
use game_common::calendar::Date;
use game_common::langue::Texte;
use game_common::tx;
use game_common::ui::{GameHelp, pied_de_menu, spawn_button_sized, spawn_hud, spawn_panel, spawn_panel_left, spawn_text};

use crate::GameState;
use crate::audio::UiClick;
use crate::hangar::points;
use crate::partie::{DEFI_PARSEC, Evenement, Mode, PARSECS, Partie};
use crate::reglages::{Pilotage, Reglages};
use crate::vaisseaux::{Gain, Progression, Vaisseau};

/// Aide du jeu (bouton « Aide » du pied de menu).
const AIDE: Texte = tx!(
    "Traversez les 12 parsecs de la ceinture d'astéroïdes !\n\
Les rochers se brisent en morceaux plus petits.\n\
Glacés : ils ralentissent · Métalliques : plus durs\n\
Explosifs : ils emportent leurs voisins.\n\
Un boss tous les trois parsecs.\n\
\n\
Flèches ou ZQSD : piloter · Espace : tirer\n\
Échap : pause · M : son\n\
Tactile : glisser pour piloter, le tir est automatique.\n\
\n\
Chaque point compte : le total cumulé débloque\n\
de nouveaux vaisseaux au hangar.",
    "Cross the 12 parsecs of the asteroid belt!\n\
Rocks break into smaller pieces.\n\
Icy: they slow you down · Metal: tougher\n\
Explosive: they take their neighbours with them.\n\
A boss every three parsecs.\n\
\n\
Arrow keys or WASD: fly · Space: shoot\n\
Esc: pause · M: sound\n\
Touch: slide to fly, shooting is automatic.\n\
\n\
Every point counts: your running total unlocks\n\
new ships in the hangar.",
    "Durchquere die 12 Parsec des Asteroidengürtels!\n\
Felsen zerbrechen in kleinere Stücke.\n\
Eisig: bremsen dich · Metall: härter\n\
Explosiv: reißen ihre Nachbarn mit.\n\
Alle drei Parsec ein Boss.\n\
\n\
Pfeiltasten oder WASD: fliegen · Leertaste: schießen\n\
Esc: Pause · M: Ton\n\
Touch: wischen zum Fliegen, Schießen geht automatisch.\n\
\n\
Jeder Punkt zählt: Die Gesamtpunkte schalten\n\
neue Schiffe im Hangar frei.",
    "Attraversa i 12 parsec della cintura di asteroidi!\n\
Le rocce si spezzano in pezzi più piccoli.\n\
Ghiacciate: ti rallentano · Metalliche: più dure\n\
Esplosive: si portano via le vicine.\n\
Un boss ogni tre parsec.\n\
\n\
Frecce o WASD: pilotare · Spazio: sparare\n\
Esc: pausa · M: audio\n\
Touch: trascina per pilotare, il tiro è automatico.\n\
\n\
Ogni punto conta: il totale accumulato sblocca\n\
nuove navi nell'hangar.",
    "¡Cruza los 12 pársecs del cinturón de asteroides!\n\
Las rocas se rompen en trozos más pequeños.\n\
Heladas: te frenan · Metálicas: más duras\n\
Explosivas: se llevan a sus vecinas.\n\
Un jefe cada tres pársecs.\n\
\n\
Flechas o WASD: pilotar · Espacio: disparar\n\
Esc: pausa · M: sonido\n\
Táctil: desliza para pilotar, el disparo es automático.\n\
\n\
Cada punto cuenta: el total acumulado desbloquea\n\
naves nuevas en el hangar."
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameHelp(AIDE))
            .init_resource::<Bilan>()
            .add_systems(OnEnter(GameState::Menu), menu)
            .add_systems(OnEnter(GameState::Reglages), reglages)
            .add_systems(OnEnter(GameState::Playing), bandeau)
            .add_systems(OnEnter(GameState::Over), (bilan, fin).chain())
            .add_systems(Update, (maj_bandeau, pause_au_clavier).run_if(in_state(GameState::Playing)))
            .add_systems(Update, (maj_pause, gains_en_direct).run_if(in_state(GameState::Playing)))
            .add_systems(Update, jouer_au_clavier.run_if(in_state(GameState::Menu)))
            .add_observer(au_clic);
    }
}

#[derive(Component, Clone, Copy)]
enum Action {
    Jouer,
    Defi,
    Hangar,
    Reglages,
    Pilotage,
    TirAuto,
    Retour,
    Reprendre,
    Menu,
    Rejouer,
}

fn bouton(c: &mut Commands, parent: Entity, texte: impl Into<String>, action: Action) -> Entity {
    let (b, t) = spawn_button_sized(c, parent, texte, 320.0, 46.0, 21.0);
    c.entity(b).insert(action);
    t
}

const TITRE: &str = "12 PARSECS";

fn menu(mut c: Commands, p: Res<Progression>) {
    let panel = spawn_panel_left(&mut c, DespawnOnExit(GameState::Menu));
    // Panneau resserré : il y a beaucoup de choix.
    c.entity(panel).insert(Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: Val::Px(7.0),
        padding: UiRect::axes(Val::Px(28.0), Val::Px(18.0)),
        border: UiRect::all(Val::Px(2.0)),
        border_radius: BorderRadius::all(Val::Px(24.0)),
        ..default()
    });
    spawn_text(&mut c, panel, TITRE, 46.0);
    spawn_text(&mut c, panel, tx!("La traversée de la ceinture", "Crossing the belt", "Durch den Gürtel", "Attraverso la cintura", "La travesía del cinturón").t(), 20.0);
    if p.record > 0 {
        let r = tx!("Record : {} · Parsec {}", "Best: {} · Parsec {}", "Rekord: {} · Parsec {}", "Record: {} · Parsec {}", "Récord: {} · Pársec {}").avec(&[&points(p.record), &p.meilleur_parsec]);
        spawn_text(&mut c, panel, r, 16.0);
    }
    bouton(&mut c, panel, tx!("Jouer", "Play", "Spielen", "Gioca", "Jugar").t(), Action::Jouer);
    let defi = if p.defi_fait == Some(Date::today()) {
        tx!("Défi du jour : réussi !", "Daily challenge: done!", "Tagesaufgabe: geschafft!", "Sfida del giorno: fatta!", "Reto del día: ¡logrado!").t()
    } else {
        tx!("Défi du jour", "Daily challenge", "Tagesaufgabe", "Sfida del giorno", "Reto del día").t()
    };
    bouton(&mut c, panel, defi, Action::Defi);
    let gagnes = Vaisseau::TOUS.iter().filter(|v| p.gagne(**v)).count();
    bouton(&mut c, panel, tx!("Hangar ({}/4)", "Hangar ({}/4)", "Hangar ({}/4)", "Hangar ({}/4)", "Hangar ({}/4)").avec(&[&gagnes]), Action::Hangar);
    bouton(&mut c, panel, tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes").t(), Action::Reglages);
    // On voit tout de suite ce qu'il reste à gagner.
    let suite = match p.prochain() {
        Some(v) => tx!("Prochain vaisseau : {} à {} points\n(encore {})", "Next ship: {} at {} points\n({} to go)", "Nächstes Schiff: {} bei {} Punkten\n(noch {})", "Prossima nave: {} a {} punti\n(ancora {})", "Próxima nave: {} con {} puntos\n(faltan {})")
            .avec(&[&v.fiche().nom, &points(v.fiche().palier), &points(v.fiche().palier - p.total)]),
        None => tx!("Tous les vaisseaux sont à vous !", "All the ships are yours!", "Alle Schiffe gehören dir!", "Tutte le navi sono tue!", "¡Todas las naves son tuyas!").t().to_string(),
    };
    spawn_text(&mut c, panel, suite, 15.0);
    pied_de_menu(&mut c, panel);
}

fn jouer_au_clavier(keys: Res<ButtonInput<KeyCode>>, mut partie: ResMut<Partie>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Enter) {
        partie.mode = Mode::Arcade;
        next.set(GameState::Playing);
    }
}

fn libelle_pilotage(r: &Reglages) -> String {
    let p = if r.pilotage == Pilotage::Classique {
        tx!("classique (tourner et accélérer)", "classic (rotate and thrust)", "klassisch (drehen und Schub)", "classico (ruota e accelera)", "clásico (girar y acelerar)")
    } else {
        tx!("direct (le vaisseau va où l'on pousse)", "direct (go where you push)", "direkt (fliegt, wohin du drückst)", "diretto (va dove spingi)", "directo (va hacia donde empujas)")
    };
    tx!("Pilotage : {}", "Controls: {}", "Steuerung: {}", "Comandi: {}", "Control: {}").avec(&[&p.t()])
}

fn libelle_tir(r: &Reglages) -> String {
    let e = if r.tir_auto { tx!("oui", "on", "an", "sì", "sí") } else { tx!("non", "off", "aus", "no", "no") };
    tx!("Tir automatique : {}", "Auto fire: {}", "Dauerfeuer: {}", "Fuoco automatico: {}", "Disparo automático: {}").avec(&[&e.t()])
}

#[derive(Component)]
struct TexteReglage(Action);

fn reglages(mut c: Commands, r: Res<Reglages>) {
    let panel = spawn_panel(&mut c, DespawnOnExit(GameState::Reglages));
    spawn_text(&mut c, panel, tx!("Réglages", "Settings", "Einstellungen", "Impostazioni", "Ajustes").t(), 36.0);
    let (b, t) = spawn_button_sized(&mut c, panel, libelle_pilotage(&r), 480.0, 50.0, 19.0);
    c.entity(b).insert(Action::Pilotage);
    c.entity(t).insert(TexteReglage(Action::Pilotage));
    let (b, t) = spawn_button_sized(&mut c, panel, libelle_tir(&r), 480.0, 50.0, 19.0);
    c.entity(b).insert(Action::TirAuto);
    c.entity(t).insert(TexteReglage(Action::TirAuto));
    bouton(&mut c, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), Action::Retour);
}

// ---- Le bandeau de jeu --------------------------------------------------------

#[derive(Component)]
struct TexteScore;

#[derive(Component)]
struct TexteEtat;

#[derive(Component)]
struct Jauge;

#[derive(Component)]
struct JaugeBouclier;

#[derive(Component)]
struct PanneauPause;

fn bandeau(mut c: Commands) {
    let (hud, texte) = spawn_hud(&mut c);
    c.entity(hud).insert(DespawnOnExit(GameState::Playing));
    c.entity(texte).insert(TexteScore);
    // En haut à droite : la traversée (12 segments), le bouclier, les bonus.
    let droite = c
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(14.0),
                right: Val::Px(14.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(6.0),
                ..default()
            },
            Pickable::IGNORE,
            DespawnOnExit(GameState::Playing),
        ))
        .id();
    let jauge = c.spawn((Node { column_gap: Val::Px(3.0), ..default() }, Pickable::IGNORE, ChildOf(droite))).id();
    for i in 0..PARSECS {
        c.spawn((
            Node { width: Val::Px(22.0), height: Val::Px(10.0), border_radius: BorderRadius::all(Val::Px(3.0)), ..default() },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
            Jauge,
            JaugeIndex(i),
            Pickable::IGNORE,
            ChildOf(jauge),
        ));
    }
    let fond = c
        .spawn((
            Node { width: Val::Px(287.0), height: Val::Px(8.0), border_radius: BorderRadius::MAX, ..default() },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)),
            Pickable::IGNORE,
            ChildOf(droite),
        ))
        .id();
    c.spawn((
        Node { width: Val::Percent(100.0), height: Val::Percent(100.0), border_radius: BorderRadius::MAX, ..default() },
        BackgroundColor(Color::srgb(0.35, 0.8, 1.0)),
        JaugeBouclier,
        Pickable::IGNORE,
        ChildOf(fond),
    ));
    c.spawn((Text::new(""), TextFont::from_font_size(15.0), TextColor(Color::srgb(0.85, 0.92, 1.0)), TexteEtat, Pickable::IGNORE, ChildOf(droite)));
    // Le panneau de pause, caché.
    let pause = spawn_panel(&mut c, (DespawnOnExit(GameState::Playing), PanneauPause, Visibility::Hidden));
    spawn_text(&mut c, pause, tx!("Pause", "Paused", "Pause", "Pausa", "Pausa").t(), 40.0);
    bouton(&mut c, pause, tx!("Reprendre", "Resume", "Weiter", "Riprendi", "Continuar").t(), Action::Reprendre);
    bouton(&mut c, pause, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), Action::Menu);
}

#[derive(Component)]
struct JaugeIndex(u32);

fn maj_bandeau(
    partie: Res<Partie>,
    p: Res<Progression>,
    mut scores: Query<&mut Text, (With<TexteScore>, Without<TexteEtat>)>,
    mut etats: Query<&mut Text, (With<TexteEtat>, Without<TexteScore>)>,
    mut segments: Query<(&JaugeIndex, &mut BackgroundColor), Without<JaugeBouclier>>,
    mut bouclier: Query<(&mut Node, &mut BackgroundColor), With<JaugeBouclier>>,
) {
    let record = p.record.max(partie.score);
    for mut t in &mut scores {
        t.0 = tx!("Score : {}    Record : {}    Vies : {}", "Score: {}    Best: {}    Lives: {}", "Punkte: {}    Rekord: {}    Leben: {}", "Punti: {}    Record: {}    Vite: {}", "Puntos: {}    Récord: {}    Vidas: {}")
            .avec(&[&points(partie.score), &points(record), &partie.vies]);
    }
    let avant = partie.avancee();
    for (i, mut bg) in &mut segments {
        let n = i.0 + 1;
        bg.0 = if n < partie.parsec {
            Color::srgb(0.45, 0.95, 0.6)
        } else if n == partie.parsec {
            Color::srgb(0.45, 0.95, 0.6).with_alpha(0.25 + 0.6 * avant)
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.15)
        };
    }
    for (mut n, mut bg) in &mut bouclier {
        let part = (partie.bouclier / partie.bouclier_max).clamp(0.0, 1.0);
        n.width = Val::Percent(part * 100.0);
        bg.0 = if part < 0.3 { Color::srgb(1.0, 0.4, 0.3) } else { Color::srgb(0.35, 0.8, 1.0) };
    }
    let mut lignes = vec![if partie.parsec > PARSECS {
        tx!("Au-delà des 12 parsecs : {}", "Beyond the 12 parsecs: {}", "Jenseits der 12 Parsec: {}", "Oltre i 12 parsec: {}", "Más allá de los 12 pársecs: {}").avec(&[&partie.parsec])
    } else {
        tx!("Parsec {} / 12", "Parsec {} / 12", "Parsec {} / 12", "Parsec {} / 12", "Pársec {} / 12").avec(&[&partie.parsec])
    }];
    let bonus: [(f32, Texte); 5] = [
        (partie.triple, tx!("Tir triple", "Triple shot", "Dreifachschuss", "Colpo triplo", "Disparo triple")),
        (partie.laser, tx!("Laser", "Laser", "Laser", "Laser", "Láser")),
        (partie.missiles, tx!("Missiles", "Missiles", "Raketen", "Missili", "Misiles")),
        (partie.ralenti, tx!("Ralenti", "Slow-mo", "Zeitlupe", "Rallentatore", "Cámara lenta")),
        (partie.aimant, tx!("Aimant", "Magnet", "Magnet", "Calamita", "Imán")),
    ];
    for (t, nom) in bonus {
        if t > 0.0 {
            lignes.push(format!("{} {:.0} s", nom.t(), t.ceil()));
        }
    }
    if partie.mode == Mode::Defi {
        lignes.push(tx!("Défi du jour : atteindre le parsec {}", "Daily challenge: reach parsec {}", "Tagesaufgabe: Parsec {} erreichen", "Sfida del giorno: raggiungi il parsec {}", "Reto del día: llega al pársec {}").avec(&[&DEFI_PARSEC]));
    }
    for mut t in &mut etats {
        t.0 = lignes.join("\n");
    }
}

fn pause_au_clavier(keys: Res<ButtonInput<KeyCode>>, mut partie: ResMut<Partie>) {
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyP) {
        partie.pause = !partie.pause;
    }
}

fn maj_pause(partie: Res<Partie>, mut q: Query<&mut Visibility, With<PanneauPause>>, mut time: ResMut<Time<Virtual>>) {
    if !partie.is_changed() {
        return;
    }
    for mut v in &mut q {
        *v = if partie.pause { Visibility::Inherited } else { Visibility::Hidden };
    }
    if partie.pause { time.pause() } else { time.unpause() }
}

// ---- La fin de partie ---------------------------------------------------------

/// Ce que la partie a rapporté.
#[derive(Resource, Default)]
struct Bilan {
    points: u64,
    gains: Vec<Gain>,
    record: bool,
    defi_reussi: bool,
}

/// Pendant la partie, on annonce un vaisseau débloqué dès que le palier est franchi.
fn gains_en_direct(partie: Res<Partie>, p: Res<Progression>, mut annonces: Local<Vec<Vaisseau>>, mut evenements: MessageWriter<Evenement>) {
    if !partie.is_changed() {
        return;
    }
    if partie.temps < 0.5 {
        annonces.clear();
    }
    for v in Vaisseau::TOUS {
        if !p.gagne(v) && p.total + partie.score >= v.fiche().palier && !annonces.contains(&v) {
            annonces.push(v);
            evenements.write(Evenement::Debloque);
        }
    }
    let _ = &p;
}

fn bilan(mut partie: ResMut<Partie>, mut p: ResMut<Progression>, mut bilan: ResMut<Bilan>, mut evenements: MessageWriter<Evenement>) {
    let points = partie.score;
    let record = points > p.record;
    if record {
        p.record = points;
        evenements.write(Evenement::Record);
    }
    p.meilleur_parsec = p.meilleur_parsec.max(partie.parsec.min(PARSECS + 1) - 1);
    let mut defi_reussi = false;
    if partie.mode == Mode::Defi && partie.parsec >= DEFI_PARSEC && p.defi_fait != Some(Date::today()) {
        defi_reussi = true;
        p.serie = if p.defi_fait == Some(Date::today().plus(-1)) { p.serie + 1 } else { 1 };
        p.defi_fait = Some(Date::today());
    }
    let gains = p.ajouter(points);
    p.enregistrer();
    partie.pause = false;
    *bilan = Bilan { points, gains, record, defi_reussi };
}

fn fin(mut c: Commands, bilan: Res<Bilan>, p: Res<Progression>, partie: Res<Partie>) {
    let panel = spawn_panel(&mut c, DespawnOnExit(GameState::Over));
    let titre = if partie.parsec > PARSECS {
        tx!("Traversée réussie !", "Crossing complete!", "Durchquerung geschafft!", "Traversata riuscita!", "¡Travesía lograda!")
    } else {
        tx!("Le vaisseau est tombé…", "Your ship went down…", "Dein Schiff ist gefallen…", "La nave è caduta…", "La nave ha caído…")
    };
    spawn_text(&mut c, panel, titre.t(), 40.0);
    spawn_text(&mut c, panel, tx!("Score : {} · Parsec {}", "Score: {} · Parsec {}", "Punkte: {} · Parsec {}", "Punti: {} · Parsec {}", "Puntos: {} · Pársec {}").avec(&[&points(bilan.points), &partie.parsec]), 22.0);
    if bilan.record {
        spawn_text(&mut c, panel, tx!("Nouveau record !", "New record!", "Neuer Rekord!", "Nuovo record!", "¡Nuevo récord!").t(), 22.0);
    }
    if bilan.defi_reussi {
        spawn_text(&mut c, panel, tx!("Défi du jour réussi ! Série : {} jour(s)", "Daily challenge done! Streak: {} day(s)", "Tagesaufgabe geschafft! Serie: {} Tag(e)", "Sfida del giorno fatta! Serie: {} giorno/i", "¡Reto del día logrado! Racha: {} día(s)").avec(&[&p.serie]), 18.0);
    }
    spawn_text(&mut c, panel, tx!("Total cumulé : {} points", "Running total: {} points", "Gesamt: {} Punkte", "Totale accumulato: {} punti", "Total acumulado: {} puntos").avec(&[&points(p.total)]), 18.0);
    for g in &bilan.gains {
        let texte = match g {
            Gain::Vaisseau(v) => tx!("Nouveau vaisseau : {} !", "New ship: {}!", "Neues Schiff: {}!", "Nuova nave: {}!", "¡Nave nueva: {}!").avec(&[&v.fiche().nom]),
            Gain::Livree(v, i) => tx!("Nouvelle livrée pour {} : {}", "New livery for {}: {}", "Neue Lackierung für {}: {}", "Nuova livrea per {}: {}", "Nueva librea para {}: {}").avec(&[&v.fiche().nom, &v.livrees()[*i].0.t()]),
        };
        let t = spawn_text(&mut c, panel, texte, 20.0);
        c.entity(t).insert(TextColor(Color::srgb(1.0, 0.85, 0.35)));
    }
    if let Some(v) = p.prochain() {
        let reste = v.fiche().palier - p.total;
        spawn_text(&mut c, panel, tx!("Prochain : {} (encore {} points)", "Next: {} ({} points to go)", "Nächstes: {} (noch {} Punkte)", "Prossima: {} (ancora {} punti)", "Siguiente: {} (faltan {} puntos)").avec(&[&v.fiche().nom, &points(reste)]), 16.0);
    }
    bouton(&mut c, panel, tx!("Rejouer", "Play again", "Nochmal", "Rigioca", "Jugar otra vez").t(), Action::Rejouer);
    bouton(&mut c, panel, tx!("Hangar", "Hangar", "Hangar", "Hangar", "Hangar").t(), Action::Hangar);
    bouton(&mut c, panel, tx!("Menu", "Menu", "Menü", "Menu", "Menú").t(), Action::Menu);
}

#[allow(clippy::too_many_arguments)]
fn au_clic(
    clic: On<PointerClick>,
    actions: Query<&Action>,
    mut partie: ResMut<Partie>,
    mut r: ResMut<Reglages>,
    mut textes: Query<(&TexteReglage, &mut Text)>,
    mut next: ResMut<NextState<GameState>>,
    mut clics: MessageWriter<UiClick>,
) {
    let Ok(a) = actions.get(clic.entity) else { return };
    clics.write(UiClick);
    match *a {
        Action::Jouer | Action::Rejouer => {
            if matches!(*a, Action::Jouer) {
                partie.mode = Mode::Arcade;
            }
            next.set(GameState::Playing);
        }
        Action::Defi => {
            partie.mode = Mode::Defi;
            next.set(GameState::Playing);
        }
        Action::Hangar => next.set(GameState::Hangar),
        Action::Reglages => next.set(GameState::Reglages),
        Action::Pilotage | Action::TirAuto => {
            if matches!(*a, Action::Pilotage) {
                r.pilotage = if r.pilotage == Pilotage::Classique { Pilotage::Direct } else { Pilotage::Classique };
            } else {
                r.tir_auto = !r.tir_auto;
            }
            r.enregistrer();
            for (quoi, mut t) in &mut textes {
                t.0 = if matches!(quoi.0, Action::Pilotage) { libelle_pilotage(&r) } else { libelle_tir(&r) };
            }
        }
        Action::Retour | Action::Menu => {
            partie.pause = false;
            next.set(GameState::Menu);
        }
        Action::Reprendre => partie.pause = false,
    }
}
