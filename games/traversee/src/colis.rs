//! Les colis : on ramasse une caisse de couleur, puis on la livre au destinataire de la même couleur,
//! posté sur le rebord de la zone de jeu. Un destinataire d'une autre couleur attend aussi : il se trompe de colis.
use bevy::prelude::*;
use game_common::ui::ThemedText;
use game_common::{Rng, Score, flat_material, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::frog::Frog;
use crate::fx::Fx;
use crate::lanes::{COLS, Lane, LaneKind};
use crate::models::Kit;

/// Points d'une livraison réussie.
const POINTS_LIVRAISON: u32 = 25;

const TEINTES: [Color; 4] = [
    Color::srgb(0.92, 0.22, 0.2),
    Color::srgb(0.22, 0.46, 0.95),
    Color::srgb(0.2, 0.74, 0.3),
    Color::srgb(1.0, 0.8, 0.15),
];

/// Une caisse à ramasser, posée sur une case d'herbe.
#[derive(Component)]
pub struct Colis {
    pub row: i32,
    pub col: i32,
    pub teinte: usize,
}

/// Un destinataire sur le rebord (colonne ±COLS) : il attend un colis de sa couleur.
#[derive(Component)]
struct Destinataire {
    row: i32,
    col: i32,
    teinte: usize,
}

/// La caisse que le joueur porte (couleur), s'il en porte une.
#[derive(Resource, Default)]
struct Porte(Option<usize>);

/// La caisse qui flotte au-dessus du personnage.
#[derive(Component)]
struct CaissePortee;

/// Fait flotter et tourner (caisses, flèches).
#[derive(Component)]
struct Flotte {
    base: f32,
    phase: f32,
    tourne: bool,
}

/// Le bandeau de message en haut de l'écran.
#[derive(Component)]
struct Avis {
    age: f32,
}

#[derive(Resource)]
struct ColisMats {
    teintes: [Handle<StandardMaterial>; 4],
    ruban: Handle<StandardMaterial>,
    peau: Handle<StandardMaterial>,
}

pub struct ColisPlugin;

impl Plugin for ColisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Porte>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Dying), |mut porte: ResMut<Porte>| porte.0 = None)
            .add_systems(Update, (habiller, flotter, vieillir_avis))
            .add_systems(Update, (logique, placer, porteur).chain().run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut commands: Commands, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(ColisMats {
        teintes: TEINTES.map(|c| mats.add(flat_material(c))),
        ruban: mats.add(flat_material(Color::srgb(0.98, 0.96, 0.9))),
        peau: mats.add(flat_material(Color::srgb(0.95, 0.78, 0.62))),
    });
}

/// Pose une caisse sur la voie (à appeler quand on bâtit la rangée).
pub fn spawn_colis(c: &mut Commands, lane: Entity, row: i32, col: i32, teinte: usize) {
    c.spawn((
        Transform::from_xyz(col as f32, 0.45, 0.0),
        Visibility::default(),
        Colis { row, col, teinte },
        Flotte { base: 0.45, phase: col as f32 * 1.3, tourne: true },
        ChildOf(lane),
    ));
}

/// Une probabilité de caisse par rangée d'herbe ; `TRAVERSEE_COLIS=1` en met partout (essai).
pub fn chance() -> f32 {
    if essai() > 0 { 1.0 } else { 0.1 }
}

/// `TRAVERSEE_COLIS=2` : caisses et destinataires sur la colonne centrale (pour tester en pilote automatique).
pub fn essai() -> u8 {
    std::env::var("TRAVERSEE_COLIS").ok().and_then(|v| v.parse().ok()).unwrap_or(0)
}

fn caisse(c: &mut Commands, parent: Entity, kit: &Kit, m: &ColisMats, teinte: usize, s: f32) {
    let part = |c: &mut Commands, mat: &Handle<StandardMaterial>, scale: Vec3| {
        c.spawn((Mesh3d(kit.cube.clone()), MeshMaterial3d(mat.clone()), Transform::from_scale(scale), ChildOf(parent)));
    };
    part(c, &m.teintes[teinte], Vec3::splat(s));
    // Ruban en croix.
    part(c, &m.ruban, Vec3::new(s * 1.04, s * 1.04, s * 0.22));
    part(c, &m.ruban, Vec3::new(s * 0.22, s * 1.04, s * 1.04));
}

/// Donne leur forme aux caisses et aux destinataires qui viennent d'apparaître.
fn habiller(
    mut commands: Commands,
    kit: Res<Kit>,
    mats: Res<ColisMats>,
    colis: Query<(Entity, &Colis), Added<Colis>>,
    dests: Query<(Entity, &Destinataire), Added<Destinataire>>,
) {
    for (e, c) in &colis {
        caisse(&mut commands, e, &kit, &mats, c.teinte, 0.5);
    }
    for (e, d) in &dests {
        let t = &mats.teintes[d.teinte];
        let part = |c: &mut Commands, mesh: &Handle<Mesh>, mat: &Handle<StandardMaterial>, at: Vec3, scale: Vec3| {
            c.spawn((Mesh3d(mesh.clone()), MeshMaterial3d(mat.clone()), Transform::from_translation(at).with_scale(scale), ChildOf(e)));
        };
        part(&mut commands, &kit.cube, t, Vec3::new(0.0, 0.4, 0.0), Vec3::new(0.46, 0.55, 0.32));
        part(&mut commands, &kit.cube, t, Vec3::new(-0.3, 0.45, 0.0), Vec3::new(0.14, 0.45, 0.14));
        part(&mut commands, &kit.cube, t, Vec3::new(0.3, 0.45, 0.0), Vec3::new(0.14, 0.45, 0.14));
        part(&mut commands, &kit.ball, &mats.peau, Vec3::new(0.0, 0.85, 0.0), Vec3::splat(0.38));
        // Une caisse de sa couleur flotte au-dessus : on voit de loin qui attend quoi.
        let icone = commands
            .spawn((
                Transform::from_xyz(0.0, 1.45, 0.0),
                Visibility::default(),
                Flotte { base: 1.45, phase: d.row as f32, tourne: true },
                ChildOf(e),
            ))
            .id();
        caisse(&mut commands, icone, &kit, &mats, d.teinte, 0.32);
    }
}

fn flotter(time: Res<Time>, mut q: Query<(&Flotte, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (f, mut tf) in &mut q {
        tf.translation.y = f.base + (t * 2.6 + f.phase).sin() * 0.07;
        if f.tourne {
            tf.rotation = Quat::from_rotation_y(t * 1.6 + f.phase);
        }
    }
}

fn avis(commands: &mut Commands, anciens: &Query<Entity, With<Avis>>, texte: &str, couleur: Color) {
    for e in anciens {
        commands.entity(e).despawn();
    }
    commands.spawn((
        Text::new(texte),
        TextFont::from_font_size(30.0),
        TextColor(couleur),
        TextShadow { offset: Vec2::splat(3.0), color: Color::srgba(0.0, 0.0, 0.0, 0.75) },
        TextLayout::justify(Justify::Center),
        Node { position_type: PositionType::Absolute, width: percent(100), top: percent(22), padding: UiRect::horizontal(px(40)), ..default() },
        GlobalZIndex(36),
        Pickable::IGNORE,
        ThemedText,
        Avis { age: 0.0 },
    ));
}

fn vieillir_avis(mut commands: Commands, time: Res<Time<Real>>, mut q: Query<(Entity, &mut Avis, &mut TextColor)>) {
    for (e, mut a, mut c) in &mut q {
        a.age += time.delta_secs();
        if a.age > 3.2 {
            commands.entity(e).despawn();
        } else {
            c.0 = c.0.with_alpha((3.2 - a.age).clamp(0.0, 1.0).min(1.0));
        }
    }
}

/// Ramasse la caisse de la case, ou livre à un destinataire du rebord.
#[allow(clippy::too_many_arguments)]
fn logique(
    mut commands: Commands,
    mut frog: ResMut<Frog>,
    mut porte: ResMut<Porte>,
    mut score: ResMut<Score>,
    mut dernier: Local<(i32, i32)>,
    colis: Query<(Entity, &Colis)>,
    dests: Query<(Entity, &Destinataire)>,
    anciens: Query<Entity, With<Avis>>,
    mut sfx: MessageWriter<Sfx>,
    mut fx: MessageWriter<Fx>,
) {
    if !frog.started {
        porte.0 = None;
        *dernier = (0, 0);
        return;
    }
    if frog.hop.is_some() {
        return;
    }
    let case = (frog.row, frog.pos.x.round() as i32);
    if case == *dernier {
        return;
    }
    *dernier = case;
    if porte.0.is_none() {
        if let Some((e, c)) = colis.iter().find(|(_, c)| c.row == case.0 && c.col == case.1) {
            porte.0 = Some(c.teinte);
            commands.entity(e).despawn();
            sfx.write(Sfx::Power);
            let texte = tx!(
                "Colis récupéré ! Maintenant, essayez de rester vivant.",
                "Parcel picked up! Now try to stay alive.",
                "Paket abgeholt! Jetzt versuchen Sie, am Leben zu bleiben.",
                "Pacco ritirato! Ora cercate di restare vivi.",
                "¡Paquete recogido! Ahora intente seguir con vida."
            );
            avis(&mut commands, &anciens, texte.t(), Color::WHITE);
        }
        return;
    }
    let Some((_, d)) = dests.iter().find(|(_, d)| d.row == case.0 && d.col == case.1) else { return };
    if Some(d.teinte) == porte.0 {
        frog.add_bonus(POINTS_LIVRAISON);
        score.current = frog.score();
        score.best = score.best.max(score.current);
        porte.0 = None;
        for (e, _) in &dests {
            commands.entity(e).despawn();
        }
        sfx.write(Sfx::Win);
        fx.write(Fx::Gulp(frog.pos + Vec3::Y * 0.8, POINTS_LIVRAISON));
        let texte = tx!(
            "Colis livré ! C'est presque professionnel.",
            "Parcel delivered! Almost professional.",
            "Paket geliefert! Fast professionell.",
            "Pacco consegnato! Quasi professionale.",
            "¡Paquete entregado! Casi profesional."
        );
        avis(&mut commands, &anciens, texte.t(), Color::srgb(0.6, 1.0, 0.6));
    } else {
        sfx.write(Sfx::Alarm);
        let texte = tx!(
            "Mauvaise couleur. On recrute vraiment n'importe qui.",
            "Wrong colour. We really hire anyone.",
            "Falsche Farbe. Wir stellen wirklich jeden ein.",
            "Colore sbagliato. Assumiamo proprio chiunque.",
            "Color equivocado. Contratamos a cualquiera."
        );
        avis(&mut commands, &anciens, texte.t(), Color::srgb(1.0, 0.65, 0.4));
    }
}

/// Quand on porte une caisse et que personne n'attend : place le bon destinataire et un faux, sur les rebords.
fn placer(mut commands: Commands, frog: Res<Frog>, porte: Res<Porte>, dests: Query<&Destinataire>, lanes: Query<(Entity, &Lane)>, mut rng: ResMut<Rng>) {
    let Some(teinte) = porte.0 else { return };
    if !dests.is_empty() {
        return;
    }
    let mut libres: Vec<(Entity, i32, i32)> = Vec::new();
    for (e, l) in &lanes {
        if l.kind != LaneKind::Grass || l.row < frog.row + if essai() > 0 { 2 } else { 5 } || l.row > frog.row + 14 {
            continue;
        }
        for side in if essai() == 2 { [0, 0] } else { [-1, 1] } {
            if !l.blocked.contains(&(side * COLS)) {
                libres.push((e, l.row, side * COLS));
            }
        }
    }
    if libres.len() < 2 {
        return;
    }
    let poser = |commands: &mut Commands, libres: &mut Vec<(Entity, i32, i32)>, rng: &mut Rng, teinte: usize| {
        let (lane, row, col) = libres.swap_remove(rng.below(libres.len() as u32) as usize);
        let side = col.signum() as f32;
        // Le destinataire se tient juste à côté de la dernière case jouable, tourné vers le centre.
        commands.spawn((
            Transform::from_xyz(col as f32 + side * 0.9, 0.0, 0.0).with_rotation(Quat::from_rotation_y(-side * std::f32::consts::FRAC_PI_2)),
            Visibility::default(),
            Destinataire { row, col, teinte },
            ChildOf(lane),
        ));
        row
    };
    let bonne = poser(&mut commands, &mut libres, &mut rng, teinte);
    libres.retain(|(_, r, _)| *r != bonne);
    if !libres.is_empty() {
        let autre = (teinte + 1 + rng.below(3) as usize) % 4;
        poser(&mut commands, &mut libres, &mut rng, autre);
    }
}

/// La caisse portée flotte au-dessus du personnage.
fn porteur(mut commands: Commands, porte: Res<Porte>, frog: Res<Frog>, kit: Res<Kit>, mats: Res<ColisMats>, mut q: Query<(Entity, &mut Transform), With<CaissePortee>>, mut vu: Local<Option<usize>>) {
    if *vu != porte.0 {
        *vu = porte.0;
        for (e, _) in &q {
            commands.entity(e).despawn();
        }
        if let Some(t) = porte.0 {
            let e = commands.spawn((Transform::from_translation(frog.visual_pos()), Visibility::default(), CaissePortee)).id();
            caisse(&mut commands, e, &kit, &mats, t, 0.34);
        }
        return;
    }
    for (_, mut tf) in &mut q {
        tf.translation = frog.visual_pos() + Vec3::Y * 1.15;
        tf.rotation = Quat::IDENTITY;
    }
}
