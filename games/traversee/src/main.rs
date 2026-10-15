// Traversée — Commit & Conquer. Jeu libre sous licence GPL-3.0-or-later.
//
// Aidez le crapaud à traverser routes, rivières et voies ferrées.
mod audio;
mod capture;
mod frog;
mod lanes;
mod models;
mod ui;

use bevy::prelude::*;
use game_common::{LowPolyStylePlugin, Rng, Score, iso_camera};

use frog::{Cause, Death, Frog, FrogModel};
use lanes::LaneGen;
use models::Kit;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Dying,
    GameOver,
}

/// Position de la caméra le long de l'axe Z (elle n'avance que vers l'avant).
#[derive(Resource, Default)]
pub struct CamZ(pub f32);

/// La partie précédente est finie : il faut repartir de zéro.
#[derive(Resource)]
struct NeedsReset(bool);

/// La caméra vise un peu devant elle, pour montrer ce qui arrive.
const LOOK_AHEAD: f32 = 2.5;
/// Caméra plus haute et plus de face que le style commun : les voies restent
/// presque horizontales et les véhicules ne masquent pas la voie suivante.
const CAM_OFFSET: Vec3 = Vec3::new(3.0, 17.0, 9.0);

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Traversée — Commit & Conquer".into(),
                    ..default()
                }),
                ..default()
            })
            .set(game_common::shared_assets()),
    )
    .add_plugins((LowPolyStylePlugin, ui::UiPlugin, audio::AudioPlugin, capture::CapturePlugin))
    .insert_resource(ClearColor(Color::srgb(0.80, 0.90, 0.95)))
    .init_state::<GameState>()
    .init_resource::<Kit>()
    .init_resource::<Score>()
    .init_resource::<Frog>()
    .init_resource::<Death>()
    .init_resource::<CamZ>()
    .insert_resource(NeedsReset(true));
    let seed = Rng::from_time().next_u64();
    app.insert_resource(Rng::new(seed))
        .insert_resource(LaneGen::new(seed))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Menu), reset_if_needed)
        .add_systems(OnEnter(GameState::Playing), reset_if_needed)
        .add_systems(OnEnter(GameState::GameOver), |mut r: ResMut<NeedsReset>| r.0 = true)
        .add_systems(
            Update,
            (
                lanes::ensure_lanes,
                lanes::move_movers,
                lanes::run_rails,
                (frog::read_input, frog::update_frog, frog::collide)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
                frog::dying.run_if(in_state(GameState::Dying)),
                follow_camera,
                frog::sync_model,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, kit: Res<Kit>) {
    commands.spawn(iso_camera(Vec3::new(0.0, 0.0, -LOOK_AHEAD), 15.0, 13.0));
    let frog = models::spawn_frog(&mut commands, &kit);
    commands.entity(frog).insert(FrogModel);
}

#[allow(clippy::too_many_arguments)]
fn reset_if_needed(
    mut needs: ResMut<NeedsReset>,
    mut generator: ResMut<LaneGen>,
    mut rng: ResMut<Rng>,
    mut frog: ResMut<Frog>,
    mut death: ResMut<Death>,
    mut cam: ResMut<CamZ>,
    mut score: ResMut<Score>,
) {
    if !needs.0 {
        return;
    }
    needs.0 = false;
    generator.request_reset(rng.next_u64());
    *frog = Frog::default();
    *death = Death::default();
    cam.0 = 0.0;
    score.reset();
}

/// La caméra suit le crapaud et avance doucement toute seule : si le crapaud
/// traîne et sort de l'écran par le bas, c'est perdu.
fn follow_camera(
    time: Res<Time>,
    state: Res<State<GameState>>,
    frog: Res<Frog>,
    mut cam: ResMut<CamZ>,
    mut death: ResMut<Death>,
    mut next: ResMut<NextState<GameState>>,
    mut q: Query<&mut Transform, With<Camera3d>>,
) {
    let dt = time.delta_secs();
    let playing = *state.get() == GameState::Playing;
    if playing && frog.started {
        cam.0 -= (0.3 + frog.row as f32 * 0.004).min(1.0) * dt;
    }
    if frog.pos.z < cam.0 {
        cam.0 += (frog.pos.z - cam.0) * (dt * 4.0).min(1.0);
    }
    if playing && frog.pos.z > cam.0 + 6.0 {
        frog::die(Cause::TooSlow, &mut death, &mut next);
    }
    let target = Vec3::new(0.0, 0.0, cam.0 - LOOK_AHEAD);
    for mut tf in &mut q {
        *tf = Transform::from_translation(target + CAM_OFFSET).looking_at(target, Vec3::Y);
    }
}
