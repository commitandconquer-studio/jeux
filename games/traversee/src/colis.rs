//! Les colis : on ramasse une caisse de couleur, puis on la livre au destinataire de la même couleur,
//! posté sur le rebord de la zone de jeu. Un destinataire d'une autre couleur attend aussi : il se trompe de colis.
use bevy::prelude::*;
use game_common::ui::ThemedText;
use game_common::{Rng, Score, flat_material, tx};

use crate::GameState;
use crate::annonces::Annonce;
use crate::audio::Sfx;
use crate::frog::Frog;
use crate::exploits::Livraison;
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
pub struct Porte(pub Option<usize>);

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

#[derive(Resource)]
struct ColisMats {
    teintes: [Handle<StandardMaterial>; 4],
    lumiere: [Handle<StandardMaterial>; 4],
    ruban: Handle<StandardMaterial>,
    peau: Handle<StandardMaterial>,
}

const CARTE: Color = Color::srgb(0.99, 0.97, 0.92);
const ENCRE: Color = Color::srgb(0.12, 0.14, 0.2);
const DOUX: Color = Color::srgb(0.46, 0.46, 0.5);

/// La carte « Livraison en cours » (en bas à droite) et ses morceaux à mettre à jour.
#[derive(Component)]
struct CarteColis;
#[derive(Component)]
struct CCaisse;
#[derive(Component)]
struct CPoint;
#[derive(Component)]
struct CDest;
#[derive(Component)]
struct CFleche;

fn nom_teinte(i: usize) -> &'static str {
    match i {
        0 => tx!("rouge", "red", "rot", "rosso", "rojo").t(),
        1 => tx!("bleu", "blue", "blau", "blu", "azul").t(),
        2 => tx!("vert", "green", "grün", "verde", "verde").t(),
        _ => tx!("jaune", "yellow", "gelb", "giallo", "amarillo").t(),
    }
}

pub struct ColisPlugin;

impl Plugin for ColisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Porte>()
            .add_systems(Startup, (setup, setup_carte))
            .add_systems(OnEnter(GameState::Dying), |mut porte: ResMut<Porte>| porte.0 = None)
            .add_systems(Update, (habiller, flotter, rebondir, majeur_carte))
            .add_systems(Update, (logique, placer, porteur).chain().run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut commands: Commands, mut mats: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(ColisMats {
        teintes: TEINTES.map(|c| mats.add(flat_material(c))),
        lumiere: TEINTES.map(|c| mats.add(StandardMaterial { unlit: true, ..flat_material(c.lighter(0.2)) })),
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
    if essai() > 0 { 1.0 } else { 0.12 }
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
        // Un fin rayon de sa couleur : on repère les colis de loin (et on ne les prend pas pour des cadeaux).
        commands.spawn((Mesh3d(kit.cube.clone()), MeshMaterial3d(mats.lumiere[c.teinte].clone()), Transform::from_xyz(0.0, 0.9, 0.0).with_scale(Vec3::new(0.07, 1.4, 0.07)), bevy::light::NotShadowCaster, ChildOf(e)));
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
        // Une flèche de sa couleur, pointée vers lui, qui rebondit : « c'est ici qu'on livre ».
        let fleche = commands
            .spawn((Transform::from_xyz(0.0, 2.5, 0.0), Visibility::default(), Rebondit { base: 2.5, phase: d.row as f32 }, ChildOf(e)))
            .id();
        let l = &mats.teintes[d.teinte];
        commands.spawn((Mesh3d(kit.cube.clone()), MeshMaterial3d(l.clone()), Transform::from_xyz(0.0, 0.34, 0.0).with_scale(Vec3::new(0.26, 0.5, 0.26)), bevy::light::NotShadowCaster, ChildOf(fleche)));
        commands.spawn((
            Mesh3d(kit.cube.clone()),
            MeshMaterial3d(l.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)).with_scale(Vec3::new(0.52, 0.52, 0.26)),
            bevy::light::NotShadowCaster,
            ChildOf(fleche),
        ));
    }
}

/// Monte et descend en tournant sur elle-même (la flèche d'un destinataire).
#[derive(Component)]
struct Rebondit {
    base: f32,
    phase: f32,
}

fn rebondir(time: Res<Time>, mut q: Query<(&Rebondit, &mut Transform)>) {
    let t = time.elapsed_secs();
    for (r, mut tf) in &mut q {
        tf.translation.y = r.base + (t * 3.0 + r.phase).sin() * 0.18;
        tf.rotation = Quat::from_rotation_y(t * 2.2 + r.phase);
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
    mut annonces: MessageWriter<Annonce>,
    mut livraisons: MessageWriter<Livraison>,
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
            annonces.write(Annonce(texte.t(), Color::WHITE));
        }
        return;
    }
    if colis.iter().any(|(_, c)| c.row == case.0 && c.col == case.1) {
        annonces.write(Annonce::blanc(
            tx!(
                "Vos mains sont pleines. Livrez d'abord !",
                "Your hands are full. Deliver first!",
                "Ihre Hände sind voll. Erst liefern!",
                "Hai le mani piene. Prima consegna!",
                "Tiene las manos llenas. ¡Primero entregue!"
            )
            .t(),
        ));
        return;
    }
    let Some((_, d)) = dests.iter().find(|(_, d)| d.row == case.0 && d.col == case.1) else { return };
    if Some(d.teinte) == porte.0 {
        livraisons.write(Livraison { reussie: true });
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
        annonces.write(Annonce(texte.t(), Color::srgb(0.6, 1.0, 0.6)));
    } else {
        livraisons.write(Livraison { reussie: false });
        sfx.write(Sfx::Alarm);
        let texte = tx!(
            "Mauvaise couleur. On recrute vraiment n'importe qui.",
            "Wrong colour. We really hire anyone.",
            "Falsche Farbe. Wir stellen wirklich jeden ein.",
            "Colore sbagliato. Assumiamo proprio chiunque.",
            "Color equivocado. Contratamos a cualquiera."
        );
        annonces.write(Annonce(texte.t(), Color::srgb(1.0, 0.65, 0.4)));
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
        tf.translation = frog.visual_pos() + Vec3::Y * 1.5;
        tf.rotation = Quat::IDENTITY;
    }
}

/// Carte « Livraison en cours » : la caisse portée, son destinataire et une flèche dans sa direction.
fn setup_carte(mut commands: Commands, server: Res<AssetServer>) {
    let carte = commands
        .spawn((
            Node { position_type: PositionType::Absolute, bottom: px(14), right: px(14), width: px(450), align_items: AlignItems::Center, column_gap: px(14), padding: UiRect::axes(px(14), px(12)), border_radius: BorderRadius::all(px(22)), ..default() },
            BackgroundColor(CARTE),
            BoxShadow::new(Color::srgba(0.0, 0.0, 0.0, 0.25), px(0), px(4), px(0), px(10)),
            GlobalZIndex(10),
            Pickable::IGNORE,
            Visibility::Hidden,
            CarteColis,
        ))
        .id();
    let caisse = commands
        .spawn((
            Node { width: px(76), height: px(76), flex_shrink: 0.0, justify_content: JustifyContent::Center, align_items: AlignItems::Center, border_radius: BorderRadius::all(px(16)), ..default() },
            BackgroundColor(TEINTES[1]),
            Pickable::IGNORE,
            CCaisse,
            ChildOf(carte),
        ))
        .id();
    for (l, h) in [(14.0, 76.0), (76.0, 14.0)] {
        commands.spawn((Node { position_type: PositionType::Absolute, width: px(l), height: px(h), ..default() }, BackgroundColor(Color::srgb(0.98, 0.96, 0.9)), Pickable::IGNORE, ChildOf(caisse)));
    }
    let col = commands.spawn((Node { flex_direction: FlexDirection::Column, row_gap: px(3), flex_grow: 1.0, ..default() }, Pickable::IGNORE, ChildOf(carte))).id();
    let texte = |commands: &mut Commands, parent: Entity, t: &str, taille: f32, c: Color| -> Entity {
        commands.spawn((Text::new(t), TextFont::from_font_size(taille), TextColor(c), Pickable::IGNORE, ThemedText, ChildOf(parent))).id()
    };
    texte(&mut commands, col, tx!("Livraison en cours", "Delivery in progress", "Lieferung unterwegs", "Consegna in corso", "Entrega en curso").t(), 18.0, ENCRE);
    let ligne = commands.spawn((Node { align_items: AlignItems::Center, column_gap: px(8), ..default() }, Pickable::IGNORE, ChildOf(col))).id();
    commands.spawn((Node { width: px(16), height: px(16), border_radius: BorderRadius::MAX, ..default() }, BackgroundColor(TEINTES[1]), Pickable::IGNORE, CPoint, ChildOf(ligne)));
    let dest = texte(&mut commands, ligne, "", 20.0, ENCRE);
    commands.entity(dest).insert(CDest);
    texte(&mut commands, col, tx!("Récompense : +25 pts", "Reward: +25 pts", "Belohnung: +25 Pkt.", "Ricompensa: +25 pt", "Recompensa: +25 pts").t(), 15.0, DOUX);
    let bouton = commands
        .spawn((
            Node { width: px(54), height: px(54), flex_shrink: 0.0, justify_content: JustifyContent::Center, align_items: AlignItems::Center, border_radius: BorderRadius::all(px(14)), ..default() },
            BackgroundColor(Color::srgb(0.2, 0.5, 0.95)),
            Pickable::IGNORE,
            ChildOf(carte),
        ))
        .id();
    commands.spawn((ImageNode::new(server.load("branding/hud/fleche.png")), Node { width: px(34), height: px(34), ..default() }, UiTransform::default(), Pickable::IGNORE, CFleche, ChildOf(bouton)));
}

#[allow(clippy::too_many_arguments)]
fn majeur_carte(
    state: Res<State<GameState>>,
    porte: Res<Porte>,
    frog: Res<Frog>,
    dests: Query<&Destinataire>,
    mut carte: Query<&mut Visibility, With<CarteColis>>,
    mut caisse: Query<&mut BackgroundColor, (With<CCaisse>, Without<CPoint>)>,
    mut point: Query<&mut BackgroundColor, (With<CPoint>, Without<CCaisse>)>,
    mut dest: Query<&mut Text, With<CDest>>,
    mut fleche: Query<&mut UiTransform, With<CFleche>>,
) {
    let Some(teinte) = porte.0.filter(|_| *state.get() == GameState::Playing) else {
        for mut v in &mut carte {
            *v = Visibility::Hidden;
        }
        return;
    };
    for mut v in &mut carte {
        *v = Visibility::Inherited;
    }
    for mut c in &mut caisse {
        c.0 = TEINTES[teinte];
    }
    for mut c in &mut point {
        c.0 = TEINTES[teinte];
    }
    for mut t in &mut dest {
        t.0 = tx!("Destinataire {}", "Recipient: {}", "Empfänger: {}", "Destinatario: {}", "Destinatario: {}").avec(&[&nom_teinte(teinte)]);
    }
    // La flèche montre le destinataire de la bonne couleur : l'angle est celui qu'on voit à l'écran
    // (la caméra est tournée : « devant » va en haut à droite, « à droite » en bas à droite).
    let angle = dests.iter().find(|d| d.teinte == teinte).map(|d| {
        let dx = d.col as f32 - frog.pos.x;
        let dz = -(d.row as f32) - frog.pos.z;
        let droite = (9.0 * dx - 3.0 * dz) / 9.49;
        let haut = 0.873 * (-3.0 * dx - 9.0 * dz) / 9.49;
        droite.atan2(haut)
    });
    for mut t in &mut fleche {
        t.rotation = Rot2::radians(angle.unwrap_or(0.0));
    }
}
