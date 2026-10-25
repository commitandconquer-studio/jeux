//! Le garage : tous les véhicules en vitrine. Ceux qu'on a débloqués tournent
//! lentement avec leur nom ; les autres restent des ombres, avec les points
//! qu'il faut pour les débloquer.
use bevy::camera::visibility::Visibility;
use bevy::prelude::*;
use game_common::ui::{spawn_button, spawn_text};
use game_common::{flat_material, tx};

use crate::GameState;
use crate::extra::{Extra, Vehicle, spawn_vehicle};
use crate::models::{Kit, part, spawn_car, spawn_train_car, spawn_truck};
use crate::progress::{ALL, Item, Progress, threshold};

/// Le garage est bâti loin de la route, hors de vue du jeu.
pub const ORIGIN: Vec3 = Vec3::new(400.0, 0.0, 0.0);
const COLS: usize = 5;
const SPACING: Vec2 = Vec2::new(5.0, 4.6);

pub struct GaragePlugin;

impl Plugin for GaragePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_back)
            .add_systems(OnEnter(GameState::Garage), spawn_garage)
            .add_systems(
                Update,
                (spin, shade_locked, place_labels, back_key).run_if(in_state(GameState::Garage)),
            );
    }
}

#[derive(Component)]
struct Spin(f32);

/// Un véhicule pas encore débloqué : on le peint en ombre.
#[derive(Component)]
struct Locked;

#[derive(Component)]
struct Shaded;

/// Texte accroché sous un véhicule (position dans le monde).
#[derive(Component)]
struct Label(Vec3);

#[derive(Component)]
struct BackButton;

fn slot_pos(i: usize) -> Vec3 {
    let (col, row) = ((i % COLS) as f32, (i / COLS) as f32);
    // Les lignes du bas ont le dernier mot : on centre la vitrine.
    ORIGIN + Vec3::new((col - 2.0) * SPACING.x, 0.0, (row - 1.15) * SPACING.y)
}

fn spawn_garage(
    mut commands: Commands,
    kit: Res<Kit>,
    extra: Res<Extra>,
    progress: Res<Progress>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let total = progress.total;
    let floor = mats.add(flat_material(Color::srgb(0.86, 0.88, 0.80)));
    let plate = mats.add(flat_material(Color::srgb(0.70, 0.74, 0.66)));
    let scope = DespawnOnExit(GameState::Garage);
    let root = commands.spawn((Transform::from_translation(ORIGIN), Visibility::default(), scope.clone())).id();
    part(&mut commands, root, &kit.cube, &floor, Vec3::new(0.0, -0.3, 0.0), Vec3::new(46.0, 0.2, 34.0));
    for (i, item) in ALL.iter().enumerate() {
        let at = slot_pos(i) - ORIGIN;
        part(&mut commands, root, &kit.cyl, &plate, at + Vec3::Y * -0.15, Vec3::new(3.6, 0.2, 3.0));
        // Les longs véhicules sont réduits pour tenir sur leur plateau.
        let len = match *item {
            Item::Trains => 2.95,
            Item::Vehicle(v) => v.len(),
        };
        let fit = (2.7 / len).min(1.15);
        let slot = commands
            .spawn((
                Transform::from_translation(at + Vec3::Y * 0.05).with_rotation(Quat::from_rotation_y(0.5)).with_scale(Vec3::splat(fit)),
                Visibility::default(),
                Spin(0.5 + i as f32 * 0.3),
                ChildOf(root),
            ))
            .id();
        let color = i * 5 + 1;
        match *item {
            Item::Trains => spawn_train_car(&mut commands, &kit, slot, true),
            Item::Vehicle(Vehicle::Car) => spawn_car(&mut commands, &kit, slot, color),
            Item::Vehicle(Vehicle::Truck) => spawn_truck(&mut commands, &kit, slot, color),
            Item::Vehicle(v) => spawn_vehicle(&mut commands, &kit, &extra, slot, v, color),
        }
        let open = total >= threshold(*item);
        if !open {
            commands.entity(slot).insert(Locked);
        }
        // Nom, ou mystère et points à atteindre
        let (title, sub) = if open {
            (item.name().to_string(), String::new())
        } else {
            (
                "???".to_string(),
                tx!("{} pts", "{} pts", "{} Pkt.", "{} pt", "{} pts").avec(&[&threshold(*item)]),
            )
        };
        label(&mut commands, slot_pos(i) + Vec3::new(0.0, 0.0, 1.9), title, sub, open, scope.clone());
    }

    // Titre en haut, bouton de retour dans le coin.
    let heading = tx!(
        "Garage : {} / {} véhicules",
        "Garage: {} / {} vehicles",
        "Garage: {} / {} Fahrzeuge",
        "Garage: {} / {} veicoli",
        "Garaje: {} / {} vehículos"
    )
    .avec(&[&progress.count(), &ALL.len()]);
    let top = commands
        .spawn((
            Node { position_type: PositionType::Absolute, width: percent(100), top: px(16), justify_content: JustifyContent::Center, ..default() },
            Pickable::IGNORE,
            DespawnOnExit(GameState::Garage),
        ))
        .id();
    spawn_text(&mut commands, top, heading, 34.0);
    let corner = commands
        .spawn((
            Node { position_type: PositionType::Absolute, left: px(20), bottom: px(20), ..default() },
            DespawnOnExit(GameState::Garage),
        ))
        .id();
    let (back, _) = spawn_button(&mut commands, corner, tx!("Retour", "Back", "Zurück", "Indietro", "Atrás").t());
    commands.entity(back).insert(BackButton);
}

fn label(commands: &mut Commands, at: Vec3, title: String, sub: String, open: bool, scope: impl Bundle) {
    let color = if open { Color::srgb(0.12, 0.14, 0.2) } else { Color::srgb(0.38, 0.40, 0.46) };
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: px(190),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            Label(at),
            Pickable::IGNORE,
            scope,
        ))
        .with_children(|p| {
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

/// Peint les véhicules verrouillés en ombre (une seule fois).
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

/// Les textes suivent les véhicules à l'écran.
fn place_labels(
    camera: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut labels: Query<(&Label, &mut Node)>,
) {
    let Ok((cam, gt)) = camera.single() else {
        return;
    };
    for (l, mut node) in &mut labels {
        if let Ok(p) = cam.world_to_viewport(gt, l.0) {
            node.left = px(p.x - 95.0);
            node.top = px(p.y);
        }
    }
}

fn back_key(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(GameState::Menu);
    }
}

/// Clic sur « Retour ».
fn on_back(
    click: On<PointerClick>,
    buttons: Query<(), With<BackButton>>,
    mut next: ResMut<NextState<GameState>>,
) {
    if buttons.get(click.entity).is_ok() {
        next.set(GameState::Menu);
    }
}
