//! Les personnages de la chenille : modèles animés de Kenney (CC0, `assets/kenney`).
//! Tous ont le même squelette et les mêmes clips, regardent vers +Z et mesurent
//! environ 0,4 unité avant mise à l'échelle.
use bevy::prelude::*;

/// Fichier (sans dossier ni extension), pack, échelle.
pub struct Kind {
    pub pack: &'static str,
    pub file: &'static str,
    pub scale: f32,
    /// Fauteuil roulant (modèle du même pack) dans lequel le personnage est assis.
    pub chair: Option<&'static str>,
}

const fn k(pack: &'static str, file: &'static str, scale: f32) -> Kind {
    Kind { pack, file, scale, chair: None }
}

const fn seated(file: &'static str, chair: &'static str) -> Kind {
    Kind { pack: "mini-characters", file, scale: 1.9, chair: Some(chair) }
}

/// Les 12 humains de Mini Characters, les 4 monstres du Cimetière, puis 3 personnes en fauteuil roulant.
pub const KINDS: [Kind; 19] = [
    k("mini-characters", "character-male-a", 1.9),
    k("mini-characters", "character-female-a", 1.9),
    k("mini-characters", "character-male-b", 1.9),
    k("mini-characters", "character-female-b", 1.9),
    k("mini-characters", "character-male-c", 1.9),
    k("mini-characters", "character-female-c", 1.9),
    k("mini-characters", "character-male-d", 1.9),
    k("mini-characters", "character-female-d", 1.9),
    k("mini-characters", "character-male-e", 1.9),
    k("mini-characters", "character-female-e", 1.9),
    k("mini-characters", "character-male-f", 1.9),
    k("mini-characters", "character-female-f", 1.9),
    k("graveyard-kit", "character-zombie", 1.9),
    k("graveyard-kit", "character-skeleton", 1.9),
    k("graveyard-kit", "character-vampire", 1.9),
    k("graveyard-kit", "character-ghost", 1.9),
    seated("character-male-e", "wheelchair"),
    seated("character-female-b", "wheelchair-deluxe"),
    seated("character-male-c", "wheelchair-power"),
];

/// Ce que fait un personnage en boucle.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pose {
    Walk,
    Wave,
    /// Debout, au repos (scènes).
    Idle,
    /// N'importe quelle animation du modèle (indice du clip : voir `CLIPS`).
    Clip(u8),
}

/// Les animations des personnages Mini Characters, avec leur nom dans les fichiers de scène (`animation = …`) et leur indice de clip.
pub const CLIPS: [(&str, u8); 32] = [
    ("fixe", 0), ("repos", 1), ("marche", 2), ("court", 3), ("saute", 4), ("tombe", 5), ("accroupi", 6), ("assis", 7),
    ("conduit", 8), ("mort", 9), ("ramasse", 10), ("salut", 11), ("non", 12), ("tient-droite", 13), ("tient-gauche", 14),
    ("tient-deux", 15), ("vise-droite", 16), ("vise-gauche", 17), ("vise-deux", 18), ("frappe-droite", 19), ("frappe-gauche", 20),
    ("coup-pied-droite", 21), ("coup-pied-gauche", 22), ("interagit-droite", 23), ("interagit-gauche", 24), ("fauteuil-assis", 25),
    ("fauteuil-regarde-gauche", 26), ("fauteuil-regarde-droite", 27), ("fauteuil-avance", 28), ("fauteuil-recule", 29),
    ("fauteuil-gauche", 30), ("fauteuil-droite", 31),
];

impl Pose {
    /// La pose d'un nom d'animation de scène.
    pub fn from_name(name: &str) -> Option<Pose> {
        Some(match name {
            "repos" => Pose::Idle,
            "salut" => Pose::Wave,
            "marche" => Pose::Walk,
            other => Pose::Clip(CLIPS.iter().find(|(n, _)| *n == other)?.1),
        })
    }
}

/// Un accessoire (modèles « aid » de Mini Characters) accroché à un os du personnage (les cannes et béquilles se posent au sol à côté de lui, sur l'os « root »).
pub struct Acc {
    /// Nom dans les fichiers de scène (`accessoires = lunettes+canne`).
    pub key: &'static str,
    pub file: &'static str,
    /// Os où il s'accroche : « head » ou « root » (les pieds).
    pub bone: &'static str,
    /// Décalage (dans l'espace du modèle) et rotation (degrés) par rapport à l'os.
    pub pos: [f32; 3],
    pub rot: [f32; 3],
}

pub const ACCESSORIES: [Acc; 10] = [
    Acc { key: "lunettes", file: "aid-glasses", bone: "head", pos: [0.0, 0.13, 0.08], rot: [0.0, 0.0, 0.0] },
    Acc { key: "lunettes-soleil", file: "aid-sunglasses", bone: "head", pos: [0.0, 0.13, 0.08], rot: [0.0, 0.0, 0.0] },
    Acc { key: "masque", file: "aid-mask", bone: "head", pos: [0.0, 0.08, -0.02], rot: [0.0, 0.0, 0.0] },
    Acc { key: "appareil-auditif", file: "aid_hearing", bone: "head", pos: [0.24, 0.12, 0.0], rot: [0.0, 0.0, 0.0] },
    Acc { key: "canne", file: "aid-cane", bone: "root", pos: [-0.36, 0.0, 0.06], rot: [0.0, 0.0, 0.0] },
    Acc { key: "canne-blanche", file: "aid-cane-blind", bone: "root", pos: [-0.36, 0.0, 0.06], rot: [0.0, 0.0, 0.0] },
    Acc { key: "canne-malvoyant", file: "aid-cane-low-vision", bone: "root", pos: [-0.36, 0.0, 0.06], rot: [0.0, 0.0, 0.0] },
    Acc { key: "bequille", file: "aid-crutch", bone: "root", pos: [0.36, 0.0, 0.06], rot: [0.0, 0.0, 0.0] },
    Acc { key: "defibrillateur-vert", file: "aid-defibrillator-green", bone: "root", pos: [0.36, 0.0, 0.06], rot: [0.0, 0.0, 0.0] },
    Acc { key: "defibrillateur-rouge", file: "aid-defibrillator-red", bone: "root", pos: [0.36, 0.0, 0.06], rot: [0.0, 0.0, 0.0] },
];

/// Graphe d'animation d'un personnage (clips des GLB : 1 repos, 2 marche, 9 mort, 11 « oui »).
#[derive(Component)]
pub struct Person {
    graph: Handle<AnimationGraph>,
    walk: AnimationNodeIndex,
    wave: AnimationNodeIndex,
    idle: AnimationNodeIndex,
    /// Le clip demandé pour une pose `Clip` (scènes).
    extra: Option<AnimationNodeIndex>,
    die: AnimationNodeIndex,
    pose: Pose,
    /// (modèle de base Mini Characters, numéro d'apparence) ; `None` : couleurs d'origine.
    look: Option<(usize, u32)>,
    /// Le guide porte une casquette et un drapeau.
    guide: bool,
    /// Accessoires portés (indices dans `ACCESSORIES`).
    acc: Vec<u8>,
}

impl Person {
    pub fn look(&self) -> Option<(usize, u32)> {
        self.look
    }
}

/// Indice du modèle Mini Characters (0–11) sur lequel repose ce personnage, s'il peut changer de couleurs.
fn base_model(kind: usize) -> Option<usize> {
    let file = KINDS[kind].file;
    KINDS[..12].iter().position(|k| k.file == file)
}

/// Pose le modèle `kind` sous `parent`.
pub fn spawn_model(
    commands: &mut Commands,
    server: &AssetServer,
    graphs: &mut Assets<AnimationGraph>,
    parent: Entity,
    kind: usize,
    pose: Pose,
    size: f32,
    look: u32,
    guide: bool,
) {
    spawn_model_with(commands, server, graphs, parent, kind, pose, size, look, guide, &[]);
}

/// Comme `spawn_model`, avec des accessoires (indices dans `ACCESSORIES`).
#[allow(clippy::too_many_arguments)]
pub fn spawn_model_with(
    commands: &mut Commands,
    server: &AssetServer,
    graphs: &mut Assets<AnimationGraph>,
    parent: Entity,
    kind: usize,
    pose: Pose,
    size: f32,
    look: u32,
    guide: bool,
    acc: &[u8],
) {
    let k = &KINDS[kind % KINDS.len()];
    let file = format!("kenney/{}/{}.glb", k.pack, k.file);
    let scene = server.load(GltfAssetLabel::Scene(0).from_asset(file.clone()));
    let clip = |i: usize| server.load(GltfAssetLabel::Animation(i).from_asset(file.clone()));
    let mut graph = AnimationGraph::new();
    // En fauteuil : « wheelchair-move-forward » (28) pour avancer, « wheelchair-sit » (25) à l'arrêt.
    let (walk_clip, wave_clip) = if k.chair.is_some() { (28, 25) } else { (2, 11) };
    let walk = graph.add_clip(clip(walk_clip), 1.0, graph.root);
    let wave = graph.add_clip(clip(wave_clip), 1.0, graph.root);
    let idle = graph.add_clip(clip(if k.chair.is_some() { 25 } else { 1 }), 1.0, graph.root);
    let die = graph.add_clip(clip(9), 1.0, graph.root);
    let extra = if let Pose::Clip(i) = pose { Some(graph.add_clip(clip(i as usize), 1.0, graph.root)) } else { None };
    if let Some(chair) = k.chair {
        let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/mini-characters/{chair}.glb")));
        commands.spawn((WorldAssetRoot(scene), Transform::from_scale(Vec3::splat(k.scale * size)), ChildOf(parent)));
    }
    commands.spawn((
        WorldAssetRoot(scene),
        Transform::from_scale(Vec3::splat(k.scale * size)),
        Person { graph: graphs.add(graph), walk, wave, idle, extra, die, pose, look: base_model(kind % KINDS.len()).map(|m| (m, look)), guide, acc: acc.to_vec() },
        ChildOf(parent),
    ));
}

fn person_of<'a>(mut e: Entity, parents: &Query<&ChildOf>, persons: &'a Query<&Person>) -> Option<&'a Person> {
    loop {
        if let Ok(p) = persons.get(e) {
            return Some(p);
        }
        e = parents.get(e).ok()?.parent();
    }
}

/// Lance l'animation dès que le squelette du modèle est chargé.
pub fn start_animations(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    persons: Query<&Person>,
) {
    for (e, mut player) in &mut players {
        let Some(p) = person_of(e, &parents, &persons) else { continue };
        commands.entity(e).insert(AnimationGraphHandle(p.graph.clone()));
        match p.pose {
            Pose::Walk => player.play(p.walk).repeat().set_speed(1.6),
            Pose::Wave => player.play(p.wave).repeat().set_speed(1.0),
            Pose::Idle => player.play(p.idle).repeat().set_speed(1.0),
            Pose::Clip(i) => {
                // La mort ne se répète pas : le personnage reste au sol.
                let a = player.play(p.extra.unwrap_or(p.idle));
                if i != 9 {
                    a.repeat();
                }
                a
            }
        };
    }
}

/// Tous les marcheurs s'effondrent (fin de partie).
pub fn all_die(
    mut players: Query<(Entity, &mut AnimationPlayer)>,
    parents: Query<&ChildOf>,
    persons: Query<&Person>,
) {
    for (e, mut player) in &mut players {
        if let Some(p) = person_of(e, &parents, &persons)
            && p.pose == Pose::Walk
        {
            player.stop_all();
            player.play(p.die);
        }
    }
}

/// Prénom ou nom du personnage.
pub fn name(kind: usize) -> &'static str {
    use game_common::tx;
    match kind {
        0 => tx!("Léo", "Leo", "Leo", "Leo", "Leo"),
        1 => tx!("Lola", "Lola", "Lola", "Lola", "Lola"),
        2 => tx!("Max", "Max", "Max", "Max", "Max"),
        3 => tx!("Inès", "Ines", "Ines", "Ines", "Inés"),
        4 => tx!("Noé", "Noah", "Noah", "Noè", "Noé"),
        5 => tx!("Zoé", "Zoe", "Zoe", "Zoe", "Zoe"),
        6 => tx!("Hugo", "Hugo", "Hugo", "Ugo", "Hugo"),
        7 => tx!("Jade", "Jade", "Jade", "Jade", "Jade"),
        8 => tx!("Sacha", "Sasha", "Sascha", "Sasha", "Sasha"),
        9 => tx!("Maya", "Maya", "Maya", "Maya", "Maya"),
        10 => tx!("Théo", "Theo", "Theo", "Teo", "Teo"),
        11 => tx!("Lou", "Lou", "Lou", "Lou", "Lou"),
        12 => tx!("Zombie", "Zombie", "Zombie", "Zombie", "Zombi"),
        13 => tx!("Squelette", "Skeleton", "Skelett", "Scheletro", "Esqueleto"),
        14 => tx!("Vampire", "Vampire", "Vampir", "Vampiro", "Vampiro"),
        15 => tx!("Fantôme", "Ghost", "Geist", "Fantasma", "Fantasma"),
        16 => tx!("Rémi", "Remy", "Remo", "Remo", "Remi"),
        17 => tx!("Camille", "Camille", "Camille", "Camilla", "Camila"),
        _ => tx!("Sam", "Sam", "Sam", "Sam", "Sam"),
    }
    .t()
}

/// Accroche les accessoires aux os de la tête et des bras dès que le squelette est chargé.
pub fn attach_accessories(
    mut commands: Commands,
    server: Res<AssetServer>,
    bones: Query<(Entity, &Name), Added<Name>>,
    parents: Query<&ChildOf>,
    persons: Query<&Person>,
) {
    for (bone, name) in &bones {
        if !matches!(name.as_str(), "head" | "root") {
            continue;
        }
        let Some(p) = person_of(bone, &parents, &persons) else { continue };
        for &i in &p.acc {
            let a = &ACCESSORIES[i as usize];
            if a.bone != name.as_str() {
                continue;
            }
            let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/mini-characters/{}.glb", a.file)));
            let rot = Quat::from_euler(EulerRot::XYZ, a.rot[0].to_radians(), a.rot[1].to_radians(), a.rot[2].to_radians());
            commands.spawn((WorldAssetRoot(scene), Transform::from_translation(Vec3::from(a.pos)).with_rotation(rot), ChildOf(bone)));
        }
    }
}

/// Casquette rouge et petit drapeau jaune : ce qui fait le guide, sur n'importe quel personnage.
pub fn attach_guide(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    heads: Query<(Entity, &Name), Added<Name>>,
    parents: Query<&ChildOf>,
    persons: Query<&Person>,
    rivals: Query<(), With<crate::board::RivalCar>>,
) {
    for (head, name) in &heads {
        if name.as_str() != "head" {
            continue;
        }
        let mut up = head;
        let mut rival = false;
        let guide = loop {
            rival |= rivals.contains(up);
            if let Ok(p) = persons.get(up) {
                break p.guide;
            }
            match parents.get(up) {
                Ok(p) => up = p.parent(),
                Err(_) => break false,
            }
        };
        if !guide {
            continue;
        }
        let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let mut mat = |c: Color| materials.add(game_common::flat_material(c));
        let (red, yellow, dark) = (mat(if rival { Color::srgb(0.16, 0.36, 0.9) } else { Color::srgb(0.88, 0.16, 0.14) }), mat(Color::srgb(1.0, 0.82, 0.1)), mat(Color::srgb(0.25, 0.2, 0.18)));
        let mut part = |m: &Handle<StandardMaterial>, pos: Vec3, size: Vec3| {
            commands.spawn((Mesh3d(cube.clone()), MeshMaterial3d(m.clone()), Transform::from_translation(pos).with_scale(size), ChildOf(head)));
        };
        // Casquette : calotte et visière (le modèle regarde vers +Z).
        part(&red, Vec3::new(0.0, 0.30, 0.0), Vec3::new(0.30, 0.11, 0.30));
        part(&red, Vec3::new(0.0, 0.26, 0.19), Vec3::new(0.24, 0.025, 0.13));
        // Drapeau dans le dos : une hampe et un fanion.
        part(&dark, Vec3::new(0.0, 0.55, -0.22), Vec3::new(0.03, 0.7, 0.03));
        part(&yellow, Vec3::new(0.0, 0.80, -0.35), Vec3::new(0.025, 0.16, 0.26));
    }
}
