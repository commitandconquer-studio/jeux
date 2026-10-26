//! La Collection : la vitrine des ennemis (ce qui traverse la route), des décors (les mondes) et des
//! personnages. Ce qui est débloqué tourne avec son nom ; le reste reste une ombre, avec ce qu'il
//! faut pour le débloquer (des points). Tout est montré avec les modèles Kenney.
use bevy::prelude::*;
use game_common::ui::{spawn_button, spawn_button_sized, spawn_text};
use game_common::{flat_material, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::characters::{CHARACTERS, Character, Selection, unlocked_character};
use crate::extra::{Biome, Scenery, Vehicle};
use crate::models::{Kit, Prop, part};
use crate::progress::{Item, Progress, threshold};

/// Le garage est bâti loin de la route, hors de vue du jeu.
pub const ORIGIN: Vec3 = Vec3::new(400.0, 0.0, 0.0);
const PER_PAGE: usize = 35;

/// Inclinaison de la caméra de la Collection (degrés au-dessus de l'horizontale) : presque de profil, avec un peu de plongée.
pub const PITCH_DEG: f32 = 24.0;
/// Hauteur du point regardé, pour centrer les modèles à l'écran.
pub const LOOK_HEIGHT: f32 = 1.0;

/// Un ennemi de la collection : un véhicule de la prairie, ou ce qui traverse la route d'un monde.
#[derive(Clone, Copy)]
enum Ennemi {
    Veh(Vehicle),
    Monde(Biome, usize),
}

/// Tous les ennemis, dans l'ordre d'affichage (les mondes : l'index est celui de `kenney::trafic_modeles`).
fn ennemis() -> Vec<Ennemi> {
    let mut v: Vec<Ennemi> = [Vehicle::Car, Vehicle::Truck, Vehicle::Taxi, Vehicle::Moto, Vehicle::Bus, Vehicle::Pickup, Vehicle::Sport, Vehicle::Police, Vehicle::Tractor, Vehicle::Sheriff].into_iter().map(Ennemi::Veh).collect();
    v.extend([0, 1].map(|i| Ennemi::Monde(Biome::Pirate, i)));
    v.extend([0, 1, 2, 3].map(|i| Ennemi::Monde(Biome::Halloween, i)));
    v.extend([0, 1, 2, 3, 4].map(|i| Ennemi::Monde(Biome::Medieval, i)));
    v.extend([0, 2, 3, 5, 6, 7].map(|i| Ennemi::Monde(Biome::Space, i)));
    v.extend([0, 1, 2].map(|i| Ennemi::Monde(Biome::FarWest, i)));
    v.extend([0, 1, 2, 3].map(|i| Ennemi::Monde(Biome::Cyber, i)));
    v
}

impl Ennemi {
    fn item(self) -> Item {
        match self {
            Ennemi::Veh(v) => Item::Vehicle(v),
            Ennemi::Monde(b, _) => Item::World(b),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Ennemi::Veh(v) => return Item::Vehicle(v).name(),
            Ennemi::Monde(Biome::Pirate, 0) => tx!("Canon Roulant", "Rolling Cannon", "Rollende Kanone", "Cannone Rotolante", "Cañón Rodante"),
            Ennemi::Monde(Biome::Pirate, _) => tx!("Tonneau Fou", "Mad Barrel", "Irres Fass", "Botte Pazza", "Barril Loco"),
            Ennemi::Monde(Biome::Halloween, 0) => tx!("Zombie Pas Frais", "Stale Zombie", "Müder Zombie", "Zombie Poco Fresco", "Zombi Rancio"),
            Ennemi::Monde(Biome::Halloween, 1) => tx!("Squelette Pressé", "Hasty Skeleton", "Eiliges Skelett", "Scheletro di Fretta", "Esqueleto con Prisa"),
            Ennemi::Monde(Biome::Halloween, 2) => tx!("Fantôme Farceur", "Prankster Ghost", "Spuk-Scherzbold", "Fantasma Burlone", "Fantasma Bromista"),
            Ennemi::Monde(Biome::Halloween, _) => tx!("Comte Croc", "Count Fang", "Graf Zahn", "Conte Zanna", "Conde Colmillo"),
            Ennemi::Monde(Biome::Medieval, 0) => tx!("Bélier Têtu", "Stubborn Ram", "Sturer Rammbock", "Ariete Testardo", "Ariete Terco"),
            Ennemi::Monde(Biome::Medieval, 1) => tx!("Catapulte Folle", "Mad Catapult", "Wilde Katapult", "Catapulta Pazza", "Catapulta Loca"),
            Ennemi::Monde(Biome::Medieval, 2) => tx!("Baliste Vive", "Quick Ballista", "Flinke Balliste", "Balista Vivace", "Balista Veloz"),
            Ennemi::Monde(Biome::Medieval, 3) => tx!("Trébuchet Géant", "Giant Trebuchet", "Riesen-Tribok", "Trabucco Gigante", "Trabuquete Gigante"),
            Ennemi::Monde(Biome::Medieval, _) => tx!("Tour de Siège", "Siege Tower", "Belagerungsturm", "Torre d'Assedio", "Torre de Asedio"),
            Ennemi::Monde(Biome::Cyber, 0) => tx!("Speeder Néon", "Neon Speeder", "Neon-Speeder", "Speeder al Neon", "Speeder de Neón"),
            Ennemi::Monde(Biome::Cyber, 1) => tx!("Taxi Volant", "Flying Cab", "Flugtaxi", "Taxi Volante", "Taxi Volador"),
            Ennemi::Monde(Biome::Cyber, 2) => tx!("Racer des Toits", "Rooftop Racer", "Dach-Racer", "Racer dei Tetti", "Racer de los Tejados"),
            Ennemi::Monde(Biome::Cyber, _) => tx!("Bolide du Futur", "Future Bolide", "Zukunfts-Bolide", "Bolide del Futuro", "Bólido del Futuro"),
            Ennemi::Monde(Biome::FarWest, 0) => tx!("Cheval Fou", "Mad Horse", "Verrücktes Pferd", "Cavallo Pazzo", "Caballo Loco"),
            Ennemi::Monde(Biome::FarWest, 1) => tx!("Bison Têtu", "Stubborn Bison", "Sturer Bison", "Bisonte Testardo", "Bisonte Terco"),
            Ennemi::Monde(Biome::FarWest, 2) => tx!("Chien Pressé", "Hasty Dog", "Eiliger Hund", "Cane di Fretta", "Perro con Prisa"),
            Ennemi::Monde(Biome::Space, 0) => tx!("Speeder Éclair", "Lightning Speeder", "Blitz-Speeder", "Speeder Lampo", "Speeder Rayo"),
            Ennemi::Monde(Biome::Space, 2) => tx!("Racer Cosmique", "Cosmic Racer", "Kosmo-Racer", "Racer Cosmico", "Racer Cósmico"),
            Ennemi::Monde(Biome::Space, 3) => tx!("Cargo Lourd", "Heavy Cargo", "Schwerfrachter", "Cargo Pesante", "Carguero Pesado"),
            Ennemi::Monde(Biome::Space, 5) => tx!("Mineur d'Astéroïdes", "Asteroid Miner", "Asteroidenbergmann", "Minatore di Asteroidi", "Minero de Asteroides"),
            Ennemi::Monde(Biome::Space, 6) => tx!("Alien Curieux", "Curious Alien", "Neugieriger Alien", "Alieno Curioso", "Alien Curioso"),
            Ennemi::Monde(_, _) => tx!("Petit Rover", "Little Rover", "Kleiner Rover", "Piccolo Rover", "Pequeño Rover"),
        }
        .t()
    }

    /// Longueur approximative (pour que les longs modèles tiennent sur leur plateau).
    fn len(self) -> f32 {
        match self {
            Ennemi::Veh(v) => v.len(),
            Ennemi::Monde(b, i) => crate::kenney::trafic_modeles(b)[i].2,
        }
    }
}

pub struct GaragePlugin;

impl Plugin for GaragePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<View>()
            .add_observer(on_click)
            .add_systems(OnEnter(GameState::Garage), |mut v: ResMut<View>| v.dirty = true)
            .add_systems(
                Update,
                (build.run_if(|v: Res<View>| v.dirty), spin, shade_locked, place_labels, back_key).chain().run_if(in_state(GameState::Garage)),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Tab {
    #[default]
    Vehicles,
    Characters,
    Costumes,
    Worlds,
}

#[derive(Resource, Default)]
pub struct View {
    pub tab: Tab,
    pub dirty: bool,
    /// Page des véhicules (28 par page).
    pub page: usize,
}

/// Tout ce que le garage fabrique : effacé et refait quand l'affichage change.
#[derive(Component)]
struct GarageItem;

#[derive(Component)]
struct Spin(f32);

/// Pas encore débloqué : on le peint en ombre.
#[derive(Component)]
struct Locked;

/// Texte accroché sous un modèle (position dans le monde).
#[derive(Component)]
struct Label(Vec3);

/// Ce que fait un clic sur un bouton du garage.
#[derive(Component, Clone, Copy)]
enum Act {
    Back,
    Tab(Tab),
    Char(Character),
    Page(i32),
    World(Biome),
}

fn scope() -> impl Bundle {
    (GarageItem, DespawnOnExit(GameState::Garage))
}

fn vehicle_slot(i: usize) -> Vec3 {
    const COLS: usize = 7;
    let (col, row) = ((i % COLS) as f32, (i / COLS) as f32);
    ORIGIN + Vec3::new((col - 3.0) * 5.1, 0.0, (row - 2.0) * 8.8 - 0.45)
}

/// Les mondes : une rangée (de profil, les rangées se recouvriraient).
fn world_slot(i: usize) -> Vec3 {
    ORIGIN + Vec3::new((i as f32 - 2.0) * 5.4, 0.0, -1.0)
}

/// Les personnages : deux rangées de six.
fn character_slot(i: usize) -> Vec3 {
    let (col, row) = ((i % 6) as f32, (i / 6) as f32);
    ORIGIN + Vec3::new((col - 2.5) * 5.5, 0.0, (row - 0.5) * 11.0 - 1.0)
}

#[allow(clippy::too_many_arguments)]
fn build(
    mut commands: Commands,
    old: Query<Entity, With<GarageItem>>,
    mut view: ResMut<View>,
    kit: Res<Kit>,
    progress: Res<Progress>,
    selection: Res<Selection>,
    server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    view.dirty = false;
    for e in &old {
        commands.entity(e).despawn();
    }
    let floor = mats.add(flat_material(Color::srgb(0.86, 0.88, 0.80)));
    let plate = mats.add(flat_material(Color::srgb(0.70, 0.74, 0.66)));
    let root = commands.spawn((Transform::from_translation(ORIGIN), Visibility::default(), scope())).id();
    part(&mut commands, root, &kit.cube, &floor, Vec3::new(0.0, -0.3, 0.0), Vec3::new(120.0, 0.2, 90.0));

    let heading;
    match view.tab {
        Tab::Vehicles => {
            let liste = ennemis();
            let pages = liste.len().div_ceil(PER_PAGE);
            view.page = view.page.min(pages - 1);
            let first = view.page * PER_PAGE;
            let title = tx!("Ennemis : {} / {}", "Enemies: {} / {}", "Gegner: {} / {}", "Nemici: {} / {}", "Enemigos: {} / {}")
                .avec(&[&liste.iter().filter(|e| progress.has(e.item())).count(), &liste.len()]);
            heading = if pages > 1 { format!("{title}  ({}/{})", view.page + 1, pages) } else { title };
            for (i, en) in liste.iter().enumerate().skip(first).take(PER_PAGE) {
                let i = i - first;
                let at = vehicle_slot(i) - ORIGIN;
                part(&mut commands, root, &kit.cyl, &plate, at + Vec3::Y * -0.15, Vec3::new(3.1, 0.2, 2.5));
                // Les longs modèles sont réduits pour tenir sur leur plateau.
                let slot = slot_entity(&mut commands, root, at, (1.9 / en.len()).clamp(0.3, 1.5), i);
                match *en {
                    Ennemi::Veh(v) => {
                        crate::kenney::car(&mut commands, &server, None, slot, v, i * 5 + 1);
                    }
                    Ennemi::Monde(b, k) => {
                        crate::kenney::trafic(&mut commands, &server, &mut graphs, None, slot, b, k);
                    }
                }
                let open = progress.has(en.item());
                if !open {
                    commands.entity(slot).insert(Locked);
                }
                let sub = if open { String::new() } else { pts(threshold(en.item())) };
                label(&mut commands, vehicle_slot(i) + Vec3::new(0.0, 0.0, 1.25), if open { en.name() } else { "???" }, sub, open, false, None, true);
            }
        }
        Tab::Characters | Tab::Costumes => {
            let have = CHARACTERS.iter().filter(|c| unlocked_character(&progress, **c)).count();
            heading = tx!("Choisis ton personnage : {} / {}", "Choose your character: {} / {}", "Wähle deine Figur: {} / {}", "Scegli il personaggio: {} / {}", "Elige tu personaje: {} / {}")
                .avec(&[&have, &CHARACTERS.len()]);
            for (i, ch) in CHARACTERS.iter().enumerate() {
                let at = character_slot(i) - ORIGIN;
                part(&mut commands, root, &kit.cyl, &plate, at + Vec3::Y * -0.15, Vec3::new(4.6, 0.2, 3.6));
                let slot = slot_entity(&mut commands, root, at, 4.4, i);
                crate::kenney::humain(&mut commands, &server, &mut graphs, slot, ch.index());
                let open = unlocked_character(&progress, *ch);
                if !open {
                    commands.entity(slot).insert(Locked);
                }
                let chosen = open && selection.character == *ch;
                let sub = if !open {
                    pts(ch.threshold())
                } else if chosen {
                    tx!("— choisi —", "— chosen —", "— gewählt —", "— scelto —", "— elegido —").t().to_string()
                } else {
                    String::new()
                };
                label(&mut commands, character_slot(i) + Vec3::new(0.0, 0.0, 2.1), if open { ch.name() } else { "???" }, sub, open, chosen, open.then_some(Act::Char(*ch)), false);
            }
        }
        Tab::Worlds => {
            let worlds = Biome::ALL;
            let have = worlds.iter().filter(|b| progress.has(Item::World(**b))).count();
            heading = tx!("Décors : {} / {}", "Scenery: {} / {}", "Kulissen: {} / {}", "Scenari: {} / {}", "Escenarios: {} / {}").avec(&[&have, &worlds.len()]);
            for (i, b) in worlds.iter().enumerate() {
                let center = world_slot(i);
                let at = center - ORIGIN;
                part(&mut commands, root, &kit.cyl, &plate, at + Vec3::Y * -0.15, Vec3::new(3.6, 0.2, 3.0));
                let slot = slot_entity(&mut commands, root, at, 1.8, i);
                world_sample(&mut commands, &server, slot, *b);
                let open = progress.has(Item::World(*b));
                if !open {
                    commands.entity(slot).insert(Locked);
                }
                let chosen = open && crate::extra::START_WORLD.load(std::sync::atomic::Ordering::Relaxed) == b.index();
                let sub = if !open {
                    pts(threshold(Item::World(*b)))
                } else if chosen {
                    tx!("— départ ici —", "— start here —", "— Start hier —", "— si parte qui —", "— se empieza aquí —").t().to_string()
                } else {
                    String::new()
                };
                label(&mut commands, center + Vec3::new(0.0, 0.0, 2.1), if open { b.name() } else { "???" }, sub, open, chosen, open.then_some(Act::World(*b)), false);
            }
        }
    }

    // Titre, onglets et retour
    let top = commands
        .spawn((
            Node { position_type: PositionType::Absolute, width: percent(100), top: px(14), flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: px(8), ..default() },
            Pickable::IGNORE,
            scope(),
        ))
        .id();
    spawn_text(&mut commands, top, heading, 32.0);
    let tabs = commands.spawn((Node { column_gap: px(10), ..default() }, ChildOf(top))).id();
    for (tab, name) in [
        (Tab::Vehicles, tx!("Ennemis", "Enemies", "Gegner", "Nemici", "Enemigos").t()),
        (Tab::Worlds, tx!("Décors", "Scenery", "Kulissen", "Scenari", "Escenarios").t()),
        (Tab::Characters, tx!("Personnages", "Characters", "Figuren", "Personaggi", "Personajes").t()),
    ] {
        let (b, _) = spawn_button_sized(&mut commands, tabs, name, 190.0, 44.0, 20.0);
        commands.entity(b).insert(Act::Tab(tab));
        if view.tab == tab {
            commands.entity(b).insert((BackgroundColor(Color::srgb(1.0, 0.88, 0.45)), game_common::ui::ButtonColor(Color::srgb(1.0, 0.88, 0.45))));
        }
    }
    if view.tab == Tab::Vehicles && ennemis().len() > PER_PAGE {
        // Pages : précédent / suivant, en bas à droite.
        let nav = commands.spawn((Node { position_type: PositionType::Absolute, right: px(20), bottom: px(20), column_gap: px(10), ..default() }, scope())).id();
        for (label, d) in [(tx!("< Précédent", "< Previous", "< Zurück", "< Precedente", "< Anterior").t(), -1), (tx!("Suivant >", "Next >", "Weiter >", "Successivo >", "Siguiente >").t(), 1)] {
            let (b, _) = spawn_button_sized(&mut commands, nav, label, 190.0, 56.0, 22.0);
            commands.entity(b).insert(Act::Page(d));
        }
    }
    let corner = commands.spawn((Node { position_type: PositionType::Absolute, left: px(20), bottom: px(20), ..default() }, scope())).id();
    let (back, _) = spawn_button(&mut commands, corner, tx!("Retour", "Back", "Zurück", "Indietro", "Atrás").t());
    commands.entity(back).insert(Act::Back);
}

/// Un petit bout de monde sur son plateau : un arbre, un cactus, une tombe… (modèles Kenney).
fn world_sample(c: &mut Commands, server: &AssetServer, slot: Entity, biome: Biome) {
    match biome {
        Biome::Prairie => {
            crate::kenney::prop(c, server, slot, Prop::Tree, Vec3::new(-0.3, 0.0, 0.0), 1);
            crate::kenney::prop(c, server, slot, Prop::Pine, Vec3::new(0.7, 0.0, 0.4), 2);
        }
        _ => {
            crate::kenney::decor(c, server, slot, biome, Scenery::Block, Vec3::new(-0.3, 0.0, 0.0), 0, 0);
            crate::kenney::decor(c, server, slot, biome, Scenery::Block, Vec3::new(0.7, 0.0, 0.5), 3, 1);
        }
    }
}

fn pts(n: u32) -> String {
    tx!("{} pts", "{} pts", "{} Pkt.", "{} pt", "{} pts").avec(&[&n])
}

fn slot_entity(commands: &mut Commands, root: Entity, at: Vec3, scale: f32, i: usize) -> Entity {
    commands
        .spawn((
            Transform::from_translation(at + Vec3::Y * 0.05).with_rotation(Quat::from_rotation_y(0.5)).with_scale(Vec3::splat(scale)),
            Visibility::default(),
            Spin(0.5 + i as f32 * 0.3),
            ChildOf(root),
        ))
        .id()
}

fn label(commands: &mut Commands, at: Vec3, title: &str, sub: String, open: bool, chosen: bool, act: Option<Act>, compact: bool) {
    let (width, size, small) = if compact { (190.0, 14.0, 12.0) } else { (260.0, 18.0, 15.0) };
    let color = if chosen {
        Color::srgb(0.75, 0.45, 0.0)
    } else if open {
        Color::srgb(0.12, 0.14, 0.2)
    } else {
        Color::srgb(0.38, 0.40, 0.46)
    };
    let mut e = commands.spawn((
        // La zone cliquable monte jusqu'au personnage : on peut cliquer dessus, pas seulement sur le nom.
        Node { position_type: PositionType::Absolute, width: px(width), height: px(230), flex_direction: FlexDirection::Column, justify_content: JustifyContent::FlexEnd, align_items: AlignItems::Center, ..default() },
        Label(at),
        scope(),
    ));
    match act {
        Some(a) => e.insert(a),
        None => e.insert(Pickable::IGNORE),
    };
    e.with_children(|p| {
        p.spawn((Text::new(title), TextFont::from_font_size(size), TextColor(color), TextLayout::justify(Justify::Center), Pickable::IGNORE, game_common::ui::ThemedText));
        // La ligne d'état est toujours là (même vide) : le nom reste à la même place, sans toucher le plateau.
        let sub = if sub.is_empty() { " ".to_string() } else { sub };
        p.spawn((Text::new(sub), TextFont::from_font_size(small), TextColor(color), TextLayout::justify(Justify::Center), Pickable::IGNORE, game_common::ui::ThemedText));
    });
}

fn spin(time: Res<Time>, mut q: Query<(&Spin, &mut Transform), Without<Locked>>) {
    for (s, mut tf) in &mut q {
        tf.rotation = Quat::from_rotation_y(time.elapsed_secs() * 0.6 + s.0);
    }
}

/// Peint ce qui est verrouillé en ombre. Les modèles GLB se chargent après coup : on repasse à chaque image sur ce qui n'est pas encore une ombre.
fn shade_locked(
    mut mats: ResMut<Assets<StandardMaterial>>,
    locked: Query<Entity, With<Locked>>,
    children: Query<&Children>,
    mut parts: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut shadow: Local<Option<Handle<StandardMaterial>>>,
) {
    let shadow = shadow.get_or_insert_with(|| mats.add(StandardMaterial { unlit: true, ..flat_material(Color::srgb(0.30, 0.33, 0.38)) })).clone();
    for root in &locked {
        for e in children.iter_descendants(root) {
            if let Ok(mut m) = parts.get_mut(e) {
                if m.0 != shadow {
                    m.0 = shadow.clone();
                }
            }
        }
    }
}

/// Les textes suivent les modèles à l'écran.
fn place_labels(camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>, mut labels: Query<(&Label, &mut Node)>) {
    let Ok((cam, gt)) = camera.single() else {
        return;
    };
    for (l, mut node) in &mut labels {
        if let Ok(p) = cam.world_to_viewport(gt, l.0) {
            let half = if let Val::Px(w) = node.width { w / 2.0 } else { 130.0 };
            node.left = px(p.x - half);
            node.top = px(p.y - 188.0);
        }
    }
}

fn back_key(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(GameState::Menu);
    }
}

fn on_click(
    click: On<PointerClick>,
    acts: Query<&Act>,
    mut view: ResMut<View>,
    mut selection: ResMut<Selection>,
    progress: Res<Progress>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    let Ok(act) = acts.get(click.entity) else {
        return;
    };
    sfx.write(Sfx::Click);
    match *act {
        Act::Back => next.set(GameState::Menu),
        Act::Page(d) => {
            let pages = ennemis().len().div_ceil(PER_PAGE) as i32;
            view.page = (view.page as i32 + d).rem_euclid(pages) as usize;
            view.dirty = true;
        }
        Act::Tab(t) => {
            view.tab = t;
            view.dirty = true;
        }
        Act::Char(c) => {
            if unlocked_character(&progress, c) {
                selection.character = c;
                selection.save();
                view.dirty = true;
            }
        }
        Act::World(b) => {
            if progress.has(Item::World(b)) {
                crate::extra::START_WORLD.store(b.index(), std::sync::atomic::Ordering::Relaxed);
                info!("monde de départ choisi : {:?}", b);
                view.dirty = true;
            }
        }
    }
}
