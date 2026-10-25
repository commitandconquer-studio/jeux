//! Le garage : la vitrine des véhicules et celle des personnages. Ce qui est
//! débloqué tourne avec son nom ; le reste reste une ombre, avec ce qu'il faut
//! pour le débloquer (points pour les véhicules et les personnages, mouches
//! gobées pour les costumes).
use bevy::prelude::*;
use game_common::models::FrogKit;
use game_common::ui::{spawn_button, spawn_button_sized, spawn_text};
use game_common::{flat_material, tx};

use crate::GameState;
use crate::audio::Sfx;
use crate::characters::{CHARACTERS, COSTUMES, Cast, Character, Costume, Selection, spawn_character, unlocked_character, unlocked_costume};
use crate::extra::{Biome, Extra, Scenery, Vehicle, spawn_deco, spawn_scenery, spawn_vehicle};
use crate::models::{Kit, Prop, part, spawn_car, spawn_prop, spawn_train_car, spawn_truck};
use crate::progress::{ALL, Item, Progress, threshold};

/// Le garage est bâti loin de la route, hors de vue du jeu.
pub const ORIGIN: Vec3 = Vec3::new(400.0, 0.0, 0.0);

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
}

/// Tout ce que le garage fabrique : effacé et refait quand l'affichage change.
#[derive(Component)]
struct GarageItem;

#[derive(Component)]
struct Spin(f32);

/// Pas encore débloqué : on le peint en ombre.
#[derive(Component)]
struct Locked;

#[derive(Component)]
struct Shaded;

/// Texte accroché sous un modèle (position dans le monde).
#[derive(Component)]
struct Label(Vec3);

/// Ce que fait un clic sur un bouton du garage.
#[derive(Component, Clone, Copy)]
enum Act {
    Back,
    Tab(Tab),
    Char(Character),
    Costume(Costume),
}

fn scope() -> impl Bundle {
    (GarageItem, DespawnOnExit(GameState::Garage))
}

fn vehicle_slot(i: usize) -> Vec3 {
    const COLS: usize = 5;
    let (col, row) = ((i % COLS) as f32, (i / COLS) as f32);
    ORIGIN + Vec3::new((col - 2.0) * 5.0, 0.0, (row - 1.15) * 4.6)
}

fn character_slot(i: usize) -> Vec3 {
    let (col, row) = ((i % 4) as f32, (i / 4) as f32);
    ORIGIN + Vec3::new((col - 1.5) * 5.6, 0.0, (row - 0.5) * 5.1 - 1.9)
}

#[allow(clippy::too_many_arguments)]
fn build(
    mut commands: Commands,
    old: Query<Entity, With<GarageItem>>,
    mut view: ResMut<View>,
    kit: Res<Kit>,
    extra: Res<Extra>,
    frog_kit: Res<FrogKit>,
    cast: Res<Cast>,
    progress: Res<Progress>,
    selection: Res<Selection>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    view.dirty = false;
    for e in &old {
        commands.entity(e).despawn();
    }
    let floor = mats.add(flat_material(Color::srgb(0.86, 0.88, 0.80)));
    let plate = mats.add(flat_material(Color::srgb(0.70, 0.74, 0.66)));
    let root = commands.spawn((Transform::from_translation(ORIGIN), Visibility::default(), scope())).id();
    part(&mut commands, root, &kit.cube, &floor, Vec3::new(0.0, -0.3, 0.0), Vec3::new(46.0, 0.2, 34.0));

    let heading;
    match view.tab {
        Tab::Vehicles => {
            heading = tx!("Garage : {} / {} véhicules", "Garage: {} / {} vehicles", "Garage: {} / {} Fahrzeuge", "Garage: {} / {} veicoli", "Garaje: {} / {} vehículos")
                .avec(&[&progress.count(), &ALL.len()]);
            for (i, item) in ALL.iter().enumerate() {
                let at = vehicle_slot(i) - ORIGIN;
                part(&mut commands, root, &kit.cyl, &plate, at + Vec3::Y * -0.15, Vec3::new(3.6, 0.2, 3.0));
                // Les longs véhicules sont réduits pour tenir sur leur plateau.
                let len = match *item {
                    Item::Trains => 2.95,
                    Item::Vehicle(v) => v.len(),
                    _ => 2.0,
                };
                let slot = slot_entity(&mut commands, root, at, (2.7 / len).min(1.15), i);
                let color = i * 5 + 1;
                match *item {
                    Item::Trains => spawn_train_car(&mut commands, &kit, slot, true),
                    Item::Vehicle(Vehicle::Car) => spawn_car(&mut commands, &kit, slot, color),
                    Item::Vehicle(Vehicle::Truck) => spawn_truck(&mut commands, &kit, slot, color),
                    Item::Vehicle(v) => spawn_vehicle(&mut commands, &kit, &extra, slot, v, color),
                    _ => {}
                }
                let open = progress.has(*item);
                if !open {
                    commands.entity(slot).insert(Locked);
                }
                let sub = if open { String::new() } else { pts(threshold(*item)) };
                label(&mut commands, vehicle_slot(i) + Vec3::new(0.0, 0.0, 1.9), if open { item.name() } else { "???" }, sub, open, false, None);
            }
        }
        Tab::Characters => {
            let have = CHARACTERS.iter().filter(|c| unlocked_character(&progress, **c)).count();
            heading = tx!("Choisis ton personnage : {} / {}", "Choose your character: {} / {}", "Wähle deine Figur: {} / {}", "Scegli il personaggio: {} / {}", "Elige tu personaje: {} / {}")
                .avec(&[&have, &CHARACTERS.len()]);
            for (i, ch) in CHARACTERS.iter().enumerate() {
                let at = character_slot(i) - ORIGIN;
                part(&mut commands, root, &kit.cyl, &plate, at + Vec3::Y * -0.15, Vec3::new(4.0, 0.2, 3.4));
                let slot = slot_entity(&mut commands, root, at, 2.3, i);
                spawn_character(&mut commands, *ch, Costume::None, &kit, &frog_kit, &cast, &mut mats, slot);
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
                label(&mut commands, character_slot(i) + Vec3::new(0.0, 0.0, 2.1), if open { ch.name() } else { "???" }, sub, open, chosen, open.then_some(Act::Char(*ch)));
            }
        }
        Tab::Costumes => {
            let have = COSTUMES.iter().filter(|c| unlocked_costume(&progress, **c)).count();
            heading = tx!("Accessoires de {} : {} / {}", "{}'s accessories: {} / {}", "Zubehör von {}: {} / {}", "Accessori di {}: {} / {}", "Accesorios de {}: {} / {}")
                .avec(&[&selection.character.name(), &have, &COSTUMES.len()]);
            for (i, co) in COSTUMES.iter().enumerate() {
                let at = character_slot(i) - ORIGIN;
                part(&mut commands, root, &kit.cyl, &plate, at + Vec3::Y * -0.15, Vec3::new(4.0, 0.2, 3.4));
                let slot = slot_entity(&mut commands, root, at, 2.3, i);
                spawn_character(&mut commands, selection.character, *co, &kit, &frog_kit, &cast, &mut mats, slot);
                let open = unlocked_costume(&progress, *co);
                if !open {
                    commands.entity(slot).insert(Locked);
                }
                let chosen = open && selection.costume == *co;
                let sub = if !open {
                    pts(co.threshold())
                } else if chosen {
                    tx!("— choisi —", "— chosen —", "— gewählt —", "— scelto —", "— elegido —").t().to_string()
                } else {
                    String::new()
                };
                label(&mut commands, character_slot(i) + Vec3::new(0.0, 0.0, 2.1), if open { co.name() } else { "???" }, sub, open, chosen, open.then_some(Act::Costume(*co)));
            }
        }
        Tab::Worlds => {
            let worlds = [Biome::Prairie, Biome::Desert, Biome::Snow, Biome::Autumn];
            let have = worlds.iter().filter(|b| progress.has(Item::World(**b))).count();
            heading = tx!("Les mondes : {} / {}", "The worlds: {} / {}", "Die Welten: {} / {}", "I mondi: {} / {}", "Los mundos: {} / {}").avec(&[&have, &worlds.len()]);
            for (i, b) in worlds.iter().enumerate() {
                let center = ORIGIN + Vec3::new((i as f32 - 1.5) * 6.2, 0.0, -1.0);
                let at = center - ORIGIN;
                let (top, _) = extra.ground(&kit, *b, 0);
                part(&mut commands, root, &kit.cyl, top, at + Vec3::Y * -0.15, Vec3::new(5.0, 0.2, 4.2));
                let slot = slot_entity(&mut commands, root, at, 2.0, i);
                world_sample(&mut commands, &kit, &extra, slot, *b);
                let open = progress.has(Item::World(*b));
                if !open {
                    commands.entity(slot).insert(Locked);
                }
                let sub = if open { String::new() } else { pts(threshold(Item::World(*b))) };
                label(&mut commands, center + Vec3::new(0.0, 0.0, 2.6), if open { b.name() } else { "???" }, sub, open, false, None);
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
        (Tab::Vehicles, tx!("Véhicules", "Vehicles", "Fahrzeuge", "Veicoli", "Vehículos").t()),
        (Tab::Characters, tx!("Personnages", "Characters", "Figuren", "Personaggi", "Personajes").t()),
        (Tab::Costumes, tx!("Accessoires", "Accessories", "Zubehör", "Accessori", "Accesorios").t()),
        (Tab::Worlds, tx!("Mondes", "Worlds", "Welten", "Mondi", "Mundos").t()),
    ] {
        let (b, _) = spawn_button_sized(&mut commands, tabs, name, 190.0, 44.0, 20.0);
        commands.entity(b).insert(Act::Tab(tab));
        if view.tab == tab {
            commands.entity(b).insert(BackgroundColor(Color::srgb(1.0, 0.88, 0.45)));
        }
    }
    let corner = commands.spawn((Node { position_type: PositionType::Absolute, left: px(20), bottom: px(20), ..default() }, scope())).id();
    let (back, _) = spawn_button(&mut commands, corner, tx!("Retour", "Back", "Zurück", "Indietro", "Atrás").t());
    commands.entity(back).insert(Act::Back);
}

/// Un petit bout de monde sur son plateau : un arbre, un cactus, un bonhomme de neige…
fn world_sample(c: &mut Commands, kit: &Kit, x: &Extra, slot: Entity, biome: Biome) {
    match biome {
        Biome::Prairie => {
            spawn_prop(c, kit, slot, Prop::Tree, Vec3::new(-0.3, 0.0, 0.0), 1);
            spawn_deco(c, kit, x, slot, crate::extra::Deco::Flowers, Vec3::new(0.7, 0.0, 0.4), 2);
        }
        Biome::Desert => spawn_scenery(c, kit, x, slot, biome, Scenery::Block, Vec3::new(-0.2, 0.0, 0.0), 0, 0),
        Biome::Snow => spawn_scenery(c, kit, x, slot, biome, Scenery::Block, Vec3::new(-0.2, 0.0, 0.0), 4, 0),
        Biome::Autumn => {
            spawn_scenery(c, kit, x, slot, biome, Scenery::Block, Vec3::new(-0.3, 0.0, 0.0), 0, 0);
            spawn_scenery(c, kit, x, slot, biome, Scenery::Block, Vec3::new(0.6, 0.0, 0.4), 6, 0);
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

fn label(commands: &mut Commands, at: Vec3, title: &str, sub: String, open: bool, chosen: bool, act: Option<Act>) {
    let color = if chosen {
        Color::srgb(0.75, 0.45, 0.0)
    } else if open {
        Color::srgb(0.12, 0.14, 0.2)
    } else {
        Color::srgb(0.38, 0.40, 0.46)
    };
    let mut e = commands.spawn((
        // La zone cliquable monte jusqu'au personnage : on peut cliquer dessus, pas seulement sur le nom.
        Node { position_type: PositionType::Absolute, width: px(260), height: px(230), flex_direction: FlexDirection::Column, justify_content: JustifyContent::FlexEnd, align_items: AlignItems::Center, ..default() },
        Label(at),
        scope(),
    ));
    match act {
        Some(a) => e.insert(a),
        None => e.insert(Pickable::IGNORE),
    };
    e.with_children(|p| {
        p.spawn((Text::new(title), TextFont::from_font_size(18.0), TextColor(color), TextLayout::justify(Justify::Center), Pickable::IGNORE, game_common::ui::ThemedText));
        if !sub.is_empty() {
            p.spawn((Text::new(sub), TextFont::from_font_size(15.0), TextColor(color), TextLayout::justify(Justify::Center), Pickable::IGNORE, game_common::ui::ThemedText));
        }
    });
}

fn spin(time: Res<Time>, mut q: Query<(&Spin, &mut Transform), Without<Locked>>) {
    for (s, mut tf) in &mut q {
        tf.rotation = Quat::from_rotation_y(time.elapsed_secs() * 0.6 + s.0);
    }
}

/// Peint ce qui est verrouillé en ombre (une seule fois).
fn shade_locked(
    mut commands: Commands,
    mut mats: ResMut<Assets<StandardMaterial>>,
    locked: Query<Entity, (With<Locked>, Without<Shaded>)>,
    children: Query<&Children>,
    mut parts: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    for root in &locked {
        let shadow = mats.add(StandardMaterial { unlit: true, ..flat_material(Color::srgb(0.30, 0.33, 0.38)) });
        for e in children.iter_descendants(root) {
            if let Ok(mut m) = parts.get_mut(e) {
                m.0 = shadow.clone();
            }
        }
        commands.entity(root).insert(Shaded);
    }
}

/// Les textes suivent les modèles à l'écran.
fn place_labels(camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>, mut labels: Query<(&Label, &mut Node)>) {
    let Ok((cam, gt)) = camera.single() else {
        return;
    };
    for (l, mut node) in &mut labels {
        if let Ok(p) = cam.world_to_viewport(gt, l.0) {
            node.left = px(p.x - 130.0);
            node.top = px(p.y - 175.0);
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
        Act::Tab(t) => {
            view.tab = t;
            view.dirty = true;
        }
        Act::Char(c) => {
            if unlocked_character(&progress, c) {
                selection.character = c;
                selection.save();
                // Personnage choisi : place à l'accessoire.
                view.tab = Tab::Costumes;
                view.dirty = true;
            }
        }
        Act::Costume(c) => {
            if unlocked_costume(&progress, c) {
                selection.costume = c;
                selection.save();
                view.dirty = true;
            }
        }
    }
}
