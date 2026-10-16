//! Le hangar : on y choisit son vaisseau et sa livrée. Les vaisseaux pas
//! encore gagnés sont visibles en silhouette, avec les points à atteindre.
//! Sur l'accueil aussi, les quatre vaisseaux sont alignés : on voit tout de
//! suite qu'il y en a d'autres à gagner.
use bevy::prelude::*;
use game_common::tx;
use game_common::ui::{spawn_button_sized, spawn_panel_left, spawn_text};

use crate::GameState;
use crate::audio::UiClick;
use crate::vaisseaux::{PALIERS_LIVREE, Progression, Vaisseau, construire};

pub struct HangarPlugin;

impl Plugin for HangarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Vue(0))
            .add_systems(OnEnter(GameState::Menu), vitrine)
            .add_systems(OnEnter(GameState::Hangar), |mut vue: ResMut<Vue>, p: Res<Progression>| vue.0 = p.choisi.index())
            .add_systems(Update, (rebatir, tourner).run_if(in_state(GameState::Hangar)))
            .add_systems(Update, tourner_vitrine.run_if(in_state(GameState::Menu)))
            .add_observer(au_clic);
    }
}

/// Le vaisseau regardé dans le hangar.
#[derive(Resource)]
struct Vue(usize);

#[derive(Component)]
struct Expose;

#[derive(Component)]
struct Ecran;

#[derive(Component)]
struct EnVitrine(usize);

#[derive(Component, Clone, Copy)]
enum Action {
    Precedent,
    Suivant,
    Choisir,
    Livree(usize),
    Retour,
}

/// Points formatés avec des espaces fines (« 12 500 »).
pub fn points(n: u64) -> String {
    let s = n.to_string();
    let mut sortie = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            sortie.push('\u{202F}');
        }
        sortie.push(c);
    }
    sortie
}

/// Les quatre vaisseaux alignés sur l'accueil (silhouettes pour ceux à gagner).
fn vitrine(mut commands: Commands, p: Res<Progression>, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    for (i, v) in Vaisseau::TOUS.into_iter().enumerate() {
        let racine = commands
            .spawn((
                Transform::from_xyz(5.5 + i as f32 * 3.6, -7.5, 4.0).with_scale(Vec3::splat(1.15)),
                Visibility::default(),
                EnVitrine(i),
                DespawnOnExit(GameState::Menu),
            ))
            .id();
        construire(&mut commands, racine, v, p.livree[i], !p.gagne(v), &mut meshes, &mut mats);
    }
}

fn tourner_vitrine(time: Res<Time>, mut q: Query<(&EnVitrine, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (v, mut tr) in &mut q {
        tr.rotation = Quat::from_rotation_x(-0.5) * Quat::from_rotation_z((t * 0.6 + v.0 as f32).sin() * 0.25);
        tr.translation.y = -7.5 + (t * 1.3 + v.0 as f32).sin() * 0.15;
    }
}

/// Rebâtit le hangar quand on change de vaisseau ou de livrée.
#[allow(clippy::too_many_arguments)]
fn rebatir(
    mut commands: Commands,
    vue: Res<Vue>,
    p: Res<Progression>,
    exposes: Query<Entity, With<Expose>>,
    ecrans: Query<Entity, With<Ecran>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut fait: Local<Option<(usize, u64, [usize; 4], usize)>>,
) {
    let cle = (vue.0, p.total, p.livree, p.choisi.index());
    if *fait == Some(cle) && !exposes.is_empty() {
        return;
    }
    *fait = Some(cle);
    exposes.iter().for_each(|e| commands.entity(e).despawn());
    ecrans.iter().for_each(|e| commands.entity(e).despawn());
    let v = Vaisseau::TOUS[vue.0];
    let fiche = v.fiche();
    let gagne = p.gagne(v);

    // Le vaisseau en grand, qui tourne sur lui-même.
    let racine = commands
        .spawn((Transform::from_xyz(7.0, 0.5, 6.0).with_scale(Vec3::splat(3.2)), Visibility::default(), Expose, DespawnOnExit(GameState::Hangar)))
        .id();
    construire(&mut commands, racine, v, p.livree[vue.0], !gagne, &mut meshes, &mut mats);

    // Le panneau d'informations à gauche.
    let panel = spawn_panel_left(&mut commands, (Ecran, DespawnOnExit(GameState::Hangar)));
    spawn_text(&mut commands, panel, tx!("Hangar", "Hangar", "Hangar", "Hangar", "Hangar").t(), 34.0);
    let navig = commands.spawn((Node { column_gap: Val::Px(10.0), align_items: AlignItems::Center, ..default() }, ChildOf(panel))).id();
    let (b, _) = spawn_button_sized(&mut commands, navig, "<", 50.0, 44.0, 26.0);
    commands.entity(b).insert(Action::Precedent);
    let titre = format!("{}  ({}/4)", fiche.nom, vue.0 + 1);
    let t = spawn_text(&mut commands, navig, titre, 26.0);
    commands.entity(t).insert(Node { width: Val::Px(250.0), ..default() });
    let (b, _) = spawn_button_sized(&mut commands, navig, ">", 50.0, 44.0, 26.0);
    commands.entity(b).insert(Action::Suivant);
    spawn_text(&mut commands, panel, fiche.description.t(), 17.0);

    // Les caractéristiques, en barres.
    let barres = [
        (tx!("Vitesse", "Speed", "Tempo", "Velocità", "Velocidad").t(), fiche.vitesse / 19.0),
        (tx!("Maniabilité", "Handling", "Wendigkeit", "Manovrabilità", "Manejo").t(), fiche.rotation / 5.4),
        (tx!("Bouclier", "Shield", "Schild", "Scudo", "Escudo").t(), fiche.bouclier / 180.0),
        (tx!("Cadence de tir", "Fire rate", "Feuerrate", "Cadenza di tiro", "Cadencia de tiro").t(), 0.11 / fiche.cadence),
    ];
    for (nom, part) in barres {
        let rang = commands.spawn((Node { column_gap: Val::Px(10.0), align_items: AlignItems::Center, ..default() }, ChildOf(panel))).id();
        let t = spawn_text(&mut commands, rang, nom, 16.0);
        commands.entity(t).insert(Node { width: Val::Px(150.0), ..default() });
        let fond = commands
            .spawn((Node { width: Val::Px(190.0), height: Val::Px(10.0), border_radius: BorderRadius::MAX, ..default() }, BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.15)), ChildOf(rang)))
            .id();
        commands.spawn((
            Node { width: Val::Percent(part.clamp(0.05, 1.0) * 100.0), height: Val::Percent(100.0), border_radius: BorderRadius::MAX, ..default() },
            BackgroundColor(Color::srgb(0.45, 0.85, 1.0)),
            ChildOf(fond),
        ));
    }

    if gagne {
        // Les trois livrées : celles gagnées se choisissent, les autres montrent leur palier.
        spawn_text(&mut commands, panel, tx!("Livrées", "Liveries", "Lackierungen", "Livree", "Libreas").t(), 20.0);
        let rang = commands.spawn((Node { column_gap: Val::Px(8.0), ..default() }, ChildOf(panel))).id();
        for (i, (nom, coque, ..)) in v.livrees().into_iter().enumerate() {
            let ok = p.livree_gagnee(v, i);
            let texte = if ok {
                nom.t().to_string()
            } else {
                tx!("à {} pts", "at {} pts", "ab {} P.", "a {} pt", "a {} pts").avec(&[&points(PALIERS_LIVREE[i])])
            };
            let (b, t) = spawn_button_sized(&mut commands, rang, texte, 128.0, 40.0, 15.0);
            if ok {
                commands.entity(b).insert(Action::Livree(i));
                // Un liseré de la couleur de la coque, et la livrée en cours marquée.
                commands.entity(b).insert(BorderColor::all(if p.livree[vue.0] == i { coque } else { Color::srgba(1.0, 1.0, 1.0, 0.3) }));
            } else {
                commands.entity(t).insert(TextColor(Color::srgba(1.0, 1.0, 1.0, 0.45)));
            }
        }
        if p.par_vaisseau[vue.0] < PALIERS_LIVREE[2] {
            let faits = p.par_vaisseau[vue.0];
            let texte = tx!("Points avec ce vaisseau : {}", "Points with this ship: {}", "Punkte mit diesem Schiff: {}", "Punti con questa nave: {}", "Puntos con esta nave: {}").avec(&[&points(faits)]);
            spawn_text(&mut commands, panel, texte, 14.0);
        }
        let choisi = p.choisi == v;
        let libelle = if choisi { tx!("Vaisseau choisi", "Selected ship", "Gewähltes Schiff", "Nave scelta", "Nave elegida") } else { tx!("Choisir ce vaisseau", "Choose this ship", "Dieses Schiff wählen", "Scegli questa nave", "Elegir esta nave") };
        let (b, t) = spawn_button_sized(&mut commands, panel, libelle.t(), 320.0, 48.0, 20.0);
        if choisi {
            commands.entity(t).insert(TextColor(Color::srgb(0.45, 0.95, 0.6)));
        }
        if !choisi {
            commands.entity(b).insert(Action::Choisir);
        }
    } else {
        let manque = fiche.palier - p.total;
        spawn_text(
            &mut commands,
            panel,
            tx!("Verrouillé : débloqué à {} points cumulés", "Locked: unlocked at {} total points", "Gesperrt: frei ab {} Gesamtpunkten", "Bloccata: si sblocca a {} punti totali", "Bloqueada: se desbloquea con {} puntos totales").avec(&[&points(fiche.palier)]),
            19.0,
        );
        spawn_text(&mut commands, panel, tx!("Encore {} points à gagner", "{} more points to go", "Noch {} Punkte", "Ancora {} punti", "Faltan {} puntos").avec(&[&points(manque)]), 17.0);
    }
    let (b, _) = spawn_button_sized(&mut commands, panel, tx!("Retour", "Back", "Zurück", "Indietro", "Volver").t(), 320.0, 48.0, 20.0);
    commands.entity(b).insert(Action::Retour);
}

fn tourner(time: Res<Time>, mut q: Query<&mut Transform, With<Expose>>) {
    for mut tr in &mut q {
        tr.rotation = Quat::from_rotation_x(-0.6) * Quat::from_rotation_z(time.elapsed_secs() * 0.5);
    }
}

fn au_clic(
    clic: On<PointerClick>,
    actions: Query<&Action>,
    mut vue: ResMut<Vue>,
    mut p: ResMut<Progression>,
    mut next: ResMut<NextState<GameState>>,
    mut clics: MessageWriter<UiClick>,
) {
    let Ok(a) = actions.get(clic.entity) else { return };
    clics.write(UiClick);
    match *a {
        Action::Precedent => vue.0 = (vue.0 + 3) % 4,
        Action::Suivant => vue.0 = (vue.0 + 1) % 4,
        Action::Choisir => {
            p.choisi = Vaisseau::TOUS[vue.0];
            p.enregistrer();
        }
        Action::Livree(i) => {
            p.livree[vue.0] = i;
            p.enregistrer();
        }
        Action::Retour => next.set(GameState::Menu),
    }
}
