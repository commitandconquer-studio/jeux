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
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pose {
    Walk,
    Wave,
}

/// Graphe d'animation d'un personnage (clips des GLB : 1 repos, 2 marche, 9 mort, 11 « oui »).
#[derive(Component)]
pub struct Person {
    graph: Handle<AnimationGraph>,
    walk: AnimationNodeIndex,
    wave: AnimationNodeIndex,
    die: AnimationNodeIndex,
    pose: Pose,
    /// (modèle de base Mini Characters, numéro d'apparence) ; `None` : couleurs d'origine.
    look: Option<(usize, u32)>,
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
    let die = graph.add_clip(clip(9), 1.0, graph.root);
    if let Some(chair) = k.chair {
        let scene = server.load(GltfAssetLabel::Scene(0).from_asset(format!("kenney/mini-characters/{chair}.glb")));
        commands.spawn((WorldAssetRoot(scene), Transform::from_scale(Vec3::splat(k.scale * size)), ChildOf(parent)));
    }
    commands.spawn((
        WorldAssetRoot(scene),
        Transform::from_scale(Vec3::splat(k.scale * size)),
        Person { graph: graphs.add(graph), walk, wave, die, pose, look: base_model(kind % KINDS.len()).map(|m| (m, look)) },
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
