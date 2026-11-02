//! L'écran « Mode scénario » : du texte, en trois colonnes. À gauche les campagnes, au milieu les niveaux de la campagne choisie
//! (une ligne chacun, avec une icône d'état), à droite le détail du niveau choisi. Tout vient des données (`story::SCENARIOS`,
//! `story::StoryProgress`) : une campagne ajoutée apparaît toute seule, une liste de 200 niveaux défile.
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use game_common::ui::{spawn_gap, spawn_text};

use crate::GameState;
use crate::audio::Sfx;
use crate::levels::{Level, Place};
use crate::story::{SCENARIOS, StoryProgress};
use crate::ui::plank;
use crate::voyage::{Mode, par_of};

const CREAM: Color = Color::srgb(1.0, 0.96, 0.84);
const GOLD: Color = Color::srgb(1.0, 0.84, 0.35);
const BROWN: Color = Color::srgb(0.30, 0.17, 0.05);

// ---------------------------------------------------------------------------------------------------------------------
// Données : qu'est-ce qu'on montre, dans quel état.
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Status {
    /// Gagnée : on peut la rejouer.
    Done,
    /// La prochaine à jouer.
    Next,
    /// Pas encore accessible.
    Locked,
}

/// Une étape jouable (pas une scène) vue par l'écran.
#[derive(Clone, Copy)]
struct Entry {
    scenario: usize,
    step: usize,
    /// Numéro d'étape dans l'aventure, en ne comptant que les niveaux jouables (1, 2, 3…).
    number: usize,
    /// Rang de l'étape dans son lieu (1, 2, 3…).
    in_place: usize,
}

impl Entry {
    fn level(&self) -> &'static Level {
        &SCENARIOS[self.scenario].steps[self.step]
    }
}

/// Première étape jouable à partir de la reprise : c'est « la prochaine ».
fn frontier(scenario: usize, progress: &StoryProgress) -> Option<usize> {
    let sc = SCENARIOS.get(scenario)?;
    let from = progress.next_step(&sc.title);
    (from..sc.steps.len()).find(|k| sc.steps[*k].scene.is_none())
}

fn state_of(e: &Entry, progress: &StoryProgress) -> Status {
    let sc = &SCENARIOS[e.scenario];
    if e.step < progress.next_step(&sc.title) {
        Status::Done
    } else if frontier(e.scenario, progress) == Some(e.step) {
        Status::Next
    } else {
        Status::Locked
    }
}

/// Toutes les étapes jouables d'une aventure, dans l'ordre réel du fichier.
fn entries(scenario: usize) -> Vec<Entry> {
    let mut per_place: Vec<(Place, usize)> = Vec::new();
    let mut out = Vec::new();
    for (k, l) in SCENARIOS[scenario].steps.iter().enumerate().filter(|(_, l)| l.scene.is_none()) {
        let rank = match per_place.iter_mut().find(|(p, _)| *p == l.place) {
            Some((_, n)) => {
                *n += 1;
                *n
            }
            None => {
                per_place.push((l.place, 1));
                1
            }
        };
        out.push(Entry { scenario, step: k, number: out.len() + 1, in_place: rank });
    }
    out
}

fn stars_text(n: u8) -> String {
    format!("{n} / 3")
}

// ---------------------------------------------------------------------------------------------------------------------
// État de l'écran.
// ---------------------------------------------------------------------------------------------------------------------

#[derive(Resource, Default)]
struct Browser {
    scenario: usize,
    /// Indice dans `entries(scenario)`.
    selected: Option<usize>,
    scroll: f32,
    dirty: bool,
}

#[derive(Component)]
struct Root;

#[derive(Component)]
struct LevelList;

#[derive(Component)]
struct PickCampaign(usize);

#[derive(Component)]
struct PickLevel(usize);

#[derive(Component)]
struct PlayBtn(usize);

#[derive(Component)]
struct BackBtn;

fn begin(mut browser: ResMut<Browser>, progress: Res<StoryProgress>, mode: Res<Mode>) {
    // On rouvre sur la campagne et le niveau qu'on vient de quitter, sinon sur la prochaine étape à jouer.
    if let Mode::Scenario { scenario, .. } = *mode {
        browser.scenario = scenario.min(SCENARIOS.len().saturating_sub(1));
    }
    let all = entries(browser.scenario);
    let last = if let Mode::Scenario { scenario, step } = *mode { (scenario == browser.scenario).then_some(step) } else { None };
    browser.selected = last
        .and_then(|k| all.iter().position(|e| e.step == k))
        .or_else(|| frontier(browser.scenario, &progress).and_then(|k| all.iter().position(|e| e.step == k)))
        .or_else(|| all.len().checked_sub(1));
    browser.scroll = 0.0;
    browser.dirty = true;
    // Contrôle des captures : SNAKE_ECRAN=verrouillee choisit un niveau pas encore accessible.
    if std::env::var("SNAKE_ECRAN").as_deref() == Ok("verrouillee") {
        browser.selected = all.iter().position(|e| state_of(e, &progress) == Status::Locked);
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Construction : trois colonnes.
// ---------------------------------------------------------------------------------------------------------------------

fn text_row(commands: &mut Commands, parent: Entity, selected: bool, locked: bool) -> Entity {
    commands
        .spawn((
            Node { align_items: AlignItems::Center, column_gap: Val::Px(10.0), padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)), border_radius: BorderRadius::all(Val::Px(8.0)), flex_shrink: 0.0, ..default() },
            BackgroundColor(if selected { Color::srgb(0.95, 0.74, 0.25) } else if locked { Color::srgba(1.0, 1.0, 1.0, 0.05) } else { Color::srgba(1.0, 0.96, 0.84, 0.12) }),
            ChildOf(parent),
        ))
        .id()
}

fn rebuild(mut commands: Commands, mut browser: ResMut<Browser>, progress: Res<StoryProgress>, server: Res<AssetServer>, old: Query<Entity, With<Root>>) {
    if !browser.dirty {
        return;
    }
    browser.dirty = false;
    for e in &old {
        commands.entity(e).despawn();
    }
    let img = |name: &str| server.load(format!("ui/suivez/{name}.png"));
    let icon = |commands: &mut Commands, parent: Entity, status: Status, size: f32| {
        // Terminé : étoile dorée ; à jouer : boussole ; verrouillé : cadenas.
        let name = match status {
            Status::Done => "icone_etoile_doree",
            Status::Next => "icone_boussole",
            Status::Locked => "icone_cadenas",
        };
        commands.spawn((Node { width: Val::Px(size), height: Val::Px(size), flex_shrink: 0.0, ..default() }, ImageNode::new(img(name)), Pickable::IGNORE, ChildOf(parent)));
    };
    let sc = &SCENARIOS[browser.scenario];
    let all = entries(browser.scenario);
    let states: Vec<Status> = all.iter().map(|e| state_of(e, &progress)).collect();

    let root = commands
        .spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(10.0), padding: UiRect::axes(Val::Px(22.0), Val::Px(14.0)), ..default() },
            DespawnOnExit(GameState::Scenarios),
            Root,
        ))
        .id();
    let head = commands.spawn((Node { align_items: AlignItems::Center, column_gap: Val::Px(16.0), ..default() }, ChildOf(root))).id();
    let (b, _) = plank(&mut commands, &server, head, "panneau_bois_fonce_large", "fleche_precedent", 170.0, "Retour", None);
    commands.entity(b).insert(BackBtn);
    let t = spawn_text(&mut commands, head, "Mode scénario", 36.0);
    commands.entity(t).insert((TextColor(CREAM), TextShadow { offset: Vec2::splat(3.0), color: Color::srgba(0.12, 0.07, 0.04, 0.9) }));

    let body = commands.spawn((Node { flex_grow: 1.0, min_height: Val::Px(0.0), column_gap: Val::Px(12.0), ..default() }, ChildOf(root))).id();
    let panel = |commands: &mut Commands, width: Val, grow: f32, light: bool| -> Entity {
        commands
            .spawn((
                Node { width, flex_grow: grow, flex_basis: if grow > 0.0 { Val::Px(0.0) } else { Val::Auto }, min_width: Val::Px(0.0), flex_shrink: if grow > 0.0 { 1.0 } else { 0.0 }, flex_direction: FlexDirection::Column, row_gap: Val::Px(6.0), padding: UiRect::all(Val::Px(12.0)), border_radius: BorderRadius::all(Val::Px(16.0)), overflow: Overflow::clip(), ..default() },
                BackgroundColor(if light { Color::srgb(0.95, 0.88, 0.70) } else { Color::srgba(0.09, 0.07, 0.05, 0.86) }),
                ChildOf(body),
            ))
            .id()
    };

    // 1. Les campagnes.
    let left = panel(&mut commands, Val::Px(250.0), 0.0, false);
    let t = spawn_text(&mut commands, left, "Campagnes", 20.0);
    commands.entity(t).insert(TextColor(GOLD));
    for (i, c) in SCENARIOS.iter().enumerate() {
        let n = entries(i).iter().filter(|e| state_of(e, &progress) == Status::Done).count();
        let row = text_row(&mut commands, left, i == browser.scenario, false);
        commands.entity(row).insert(PickCampaign(i));
        let col = commands.spawn((Node { flex_direction: FlexDirection::Column, align_items: AlignItems::FlexStart, ..default() }, Pickable::IGNORE, ChildOf(row))).id();
        let t = spawn_text(&mut commands, col, c.title.clone(), 18.0);
        commands.entity(t).insert((TextColor(if i == browser.scenario { BROWN } else { CREAM }), TextLayout::justify(Justify::Left)));
        let t = spawn_text(&mut commands, col, format!("{n} / {} niveaux terminés", entries(i).len()), 13.0);
        commands.entity(t).insert((TextColor(if i == browser.scenario { BROWN } else { Color::srgb(1.0, 0.9, 0.62) }), TextLayout::justify(Justify::Left)));
    }

    // 2. Les niveaux de la campagne choisie : une ligne de texte chacun.
    let mid = panel(&mut commands, Val::Auto, 1.0, false);
    let done = states.iter().filter(|s| **s == Status::Done).count();
    let t = spawn_text(&mut commands, mid, format!("{} — {done} / {} niveaux — {} étoiles", sc.title, all.len(), progress.total_stars(&sc.title)), 20.0);
    commands.entity(t).insert((TextColor(GOLD), TextLayout::justify(Justify::Left)));
    let list = commands
        .spawn((
            Node { flex_grow: 1.0, min_height: Val::Px(0.0), flex_direction: FlexDirection::Column, row_gap: Val::Px(3.0), overflow: Overflow::scroll_y(), padding: UiRect::right(Val::Px(6.0)), ..default() },
            ScrollPosition(Vec2::new(0.0, browser.scroll)),
            LevelList,
            ChildOf(mid),
        ))
        .id();
    for (i, e) in all.iter().enumerate() {
        let selected = browser.selected == Some(i);
        let row = text_row(&mut commands, list, selected, states[i] == Status::Locked);
        commands.entity(row).insert(PickLevel(i));
        let color = if selected { BROWN } else if states[i] == Status::Locked { Color::srgb(0.6, 0.64, 0.72) } else { CREAM };
        icon(&mut commands, row, states[i], 22.0);
        let t = spawn_text(&mut commands, row, format!("{}", e.number), 16.0);
        commands.entity(t).insert(TextColor(color));
        let holder = commands.spawn((Node { flex_grow: 1.0, ..default() }, Pickable::IGNORE, ChildOf(row))).id();
        let t = spawn_text(&mut commands, holder, e.level().name.clone(), 17.0);
        commands.entity(t).insert((TextColor(color), TextLayout::justify(Justify::Left)));
        let t = spawn_text(&mut commands, row, e.level().place.label(), 14.0);
        commands.entity(t).insert(TextColor(if selected { BROWN } else { Color::srgb(0.75, 0.7, 0.55) }));
    }

    // 3. Le détail du niveau choisi.
    let right = panel(&mut commands, Val::Px(330.0), 0.0, true);
    commands.entity(right).insert(Node { width: Val::Px(330.0), flex_shrink: 0.0, flex_direction: FlexDirection::Column, align_items: AlignItems::Center, row_gap: Val::Px(7.0), padding: UiRect::all(Val::Px(14.0)), border_radius: BorderRadius::all(Val::Px(16.0)), overflow: Overflow::clip(), ..default() });
    match browser.selected.and_then(|i| all.get(i).map(|e| (i, e))) {
        Some((i, e)) => detail(&mut commands, &server, right, e, i, states[i], &progress, &states, &all),
        None => {
            let t = spawn_text(&mut commands, right, "Choisis un niveau pour voir ses détails.", 18.0);
            commands.entity(t).insert(TextColor(BROWN));
        }
    }
}

/// Le panneau de détail de l'étape choisie.
#[allow(clippy::too_many_arguments)]
fn detail(commands: &mut Commands, server: &AssetServer, side: Entity, e: &Entry, index: usize, state: Status, progress: &StoryProgress, states: &[Status], all: &[Entry]) {
    let level = e.level();
    let best = progress.best(&SCENARIOS[e.scenario].title, e.step);
    let img = |name: &str| server.load(format!("ui/suivez/{name}.png"));
    let t = spawn_text(commands, side, format!("ÉTAPE {}", e.number), 15.0);
    commands.entity(t).insert(TextColor(Color::srgb(0.75, 0.38, 0.05)));
    let t = spawn_text(commands, side, level.name.clone(), 26.0);
    commands.entity(t).insert(TextColor(BROWN));
    let t = spawn_text(commands, side, format!("{} · étape {} du lieu", level.place.label(), e.in_place), 14.0);
    commands.entity(t).insert(TextColor(Color::srgb(0.45, 0.30, 0.15)));
    let (label, color) = match state {
        Status::Done => ("Terminée", Color::srgb(0.30, 0.55, 0.20)),
        Status::Next => ("À jouer", Color::srgb(0.85, 0.48, 0.08)),
        Status::Locked => ("Verrouillée", Color::srgb(0.40, 0.45, 0.55)),
    };
    let chip = commands.spawn((Node { padding: UiRect::axes(Val::Px(14.0), Val::Px(3.0)), border_radius: BorderRadius::MAX, ..default() }, BackgroundColor(color), ChildOf(side))).id();
    let t = spawn_text(commands, chip, label, 14.0);
    commands.entity(t).insert(TextColor(Color::WHITE));

    // Aperçu du plateau : une case par case, à l'échelle.
    let layout = level.layout(0);
    let size = 128.0;
    let cell = (size / layout.w.max(layout.h) as f32).floor().max(2.0);
    let exit = level.grid.as_ref().and_then(|g| g.exit());
    let frame = commands.spawn((Node { flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(5.0)), border_radius: BorderRadius::all(Val::Px(8.0)), ..default() }, BackgroundColor(Color::srgb(0.30, 0.17, 0.05)), ChildOf(side))).id();
    for y in 0..layout.h {
        let row = commands.spawn((Node { ..default() }, ChildOf(frame))).id();
        for x in 0..layout.w {
            let c = IVec2::new(x, y);
            let color = if exit == Some(c) {
                Color::srgb(0.9, 0.3, 0.25)
            } else if c == layout.start {
                Color::srgb(1.0, 0.84, 0.2)
            } else if layout.walkable(c) {
                if (x + y) % 2 == 0 { Color::srgb(0.56, 0.78, 0.40) } else { Color::srgb(0.50, 0.72, 0.36) }
            } else {
                Color::srgba(0.0, 0.0, 0.0, 0.0)
            };
            commands.spawn((Node { width: Val::Px(cell), height: Val::Px(cell), ..default() }, BackgroundColor(color), ChildOf(row)));
        }
    }

    // Objectif, résultats.
    let goal = if level.exit { format!("Recruter {} touristes, puis les mener à la sortie", level.tourists) } else { format!("Recruter {} touristes", level.tourists) };
    let t = spawn_text(commands, side, goal, 16.0);
    commands.entity(t).insert(TextColor(BROWN));
    let row = commands.spawn((Node { column_gap: Val::Px(3.0), align_items: AlignItems::Center, ..default() }, ChildOf(side))).id();
    let n = best.map_or(0, |b| b.0);
    for k in 0..3u8 {
        commands.spawn((Node { width: Val::Px(30.0), height: Val::Px(30.0), ..default() }, ImageNode::new(img(if k < n { "icone_etoile_doree" } else { "icone_etoile_grise" })), ChildOf(row)));
    }
    if state == Status::Done && best.is_none() {
        commands.entity(row).insert(Visibility::Hidden);
    }
    let info = match (state, best) {
        (Status::Locked, _) => "Pas encore joué".to_owned(),
        (_, Some((s, t))) => format!("{} étoile(s) · meilleur temps {} s", stars_text(s).split(' ').next().unwrap_or("0"), t.round() as u32),
        (Status::Done, None) => "Terminée avant l'enregistrement des étoiles : rejoue-la pour en obtenir.".to_owned(),
        (Status::Next, None) => "Pas encore joué".to_owned(),
    };
    let t = spawn_text(commands, side, info, 14.0);
    commands.entity(t).insert(TextColor(Color::srgb(0.45, 0.30, 0.15)));
    let par = par_of(level);
    let t = spawn_text(commands, side, format!("2 étoiles en {} s, 3 étoiles en {} s", par.round() as u32, (par * 0.75).round() as u32), 13.0);
    commands.entity(t).insert(TextColor(Color::srgb(0.45, 0.30, 0.15)));
    spawn_gap(commands, side, 2.0);

    match state {
        Status::Done => {
            let (b, _) = plank(commands, server, side, "panneau_bois_dore_large", "fleche_suivant", 230.0, "Rejouer", None);
            commands.entity(b).insert(PlayBtn(index));
        }
        Status::Next => {
            let (b, _) = plank(commands, server, side, "panneau_bois_dore_large", "fleche_suivant", 230.0, "Jouer", None);
            commands.entity(b).insert(PlayBtn(index));
        }
        Status::Locked => {
            let before = all.iter().zip(states).filter(|(_, s)| **s != Status::Locked).last().map(|(e, _)| e.level().name.clone());
            let t = spawn_text(
                commands,
                side,
                match before {
                    Some(n) => format!("Verrouillée. Termine d'abord « {n} » pour la débloquer."),
                    None => "Verrouillée.".to_owned(),
                },
                15.0,
            );
            commands.entity(t).insert(TextColor(Color::srgb(0.40, 0.30, 0.25)));
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Interactions.
// ---------------------------------------------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn on_click(
    click: On<PointerClick>,
    campaigns: Query<&PickCampaign>,
    levels: Query<&PickLevel>,
    plays: Query<&PlayBtn>,
    back: Query<(), With<BackBtn>>,
    mut browser: ResMut<Browser>,
    progress: Res<StoryProgress>,
    mut mode: ResMut<Mode>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::Scenarios {
        return;
    }
    let target = click.entity;
    if back.contains(target) {
        sfx.write(Sfx::Click);
        next.set(GameState::Menu);
    } else if let Ok(PickCampaign(i)) = campaigns.get(target) {
        sfx.write(Sfx::Click);
        browser.scenario = *i;
        let all = entries(*i);
        browser.selected = frontier(*i, &progress).and_then(|k| all.iter().position(|e| e.step == k)).or_else(|| all.len().checked_sub(1));
        browser.scroll = 0.0;
        browser.dirty = true;
    } else if let Ok(PickLevel(i)) = levels.get(target) {
        sfx.write(Sfx::Click);
        browser.selected = Some(*i);
        browser.dirty = true;
    } else if let Ok(PlayBtn(i)) = plays.get(target) {
        play(*i, &browser, &progress, &mut mode, &mut next, &mut sfx);
    }
}

/// Lance une étape : la prochaine reprend là où on s'était arrêté (scènes comprises) ; une étape gagnée se rejoue directement.
fn play(index: usize, browser: &Browser, progress: &StoryProgress, mode: &mut Mode, next: &mut NextState<GameState>, sfx: &mut MessageWriter<Sfx>) {
    let all = entries(browser.scenario);
    let Some(e) = all.get(index) else { return };
    let step = match state_of(e, progress) {
        Status::Locked => return,
        Status::Done => e.step,
        Status::Next => progress.next_step(&SCENARIOS[e.scenario].title).min(e.step),
    };
    sfx.write(Sfx::Click);
    *mode = Mode::Scenario { scenario: e.scenario, step };
    next.set(GameState::Step);
}

#[allow(clippy::too_many_arguments)]
fn keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut browser: ResMut<Browser>,
    progress: Res<StoryProgress>,
    mut mode: ResMut<Mode>,
    mut next: ResMut<NextState<GameState>>,
    mut sfx: MessageWriter<Sfx>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next.set(GameState::Menu);
        return;
    }
    let len = entries(browser.scenario).len();
    let down = keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS);
    let up = keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW);
    if (down || up) && len > 0 {
        let cur = browser.selected.unwrap_or(0);
        let to = if down { (cur + 1).min(len - 1) } else { cur.saturating_sub(1) };
        browser.selected = Some(to);
        browser.dirty = true;
        sfx.write(Sfx::Click);
    }
    // Gauche / droite : change de campagne.
    let right = keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD);
    let left = keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA);
    if (right || left) && !SCENARIOS.is_empty() {
        browser.scenario = if right { (browser.scenario + 1).min(SCENARIOS.len() - 1) } else { browser.scenario.saturating_sub(1) };
        let all = entries(browser.scenario);
        browser.selected = frontier(browser.scenario, &progress).and_then(|k| all.iter().position(|e| e.step == k)).or_else(|| all.len().checked_sub(1));
        browser.scroll = 0.0;
        browser.dirty = true;
        sfx.write(Sfx::Click);
    }
    if (keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space))
        && let Some(i) = browser.selected
    {
        play(i, &browser, &progress, &mut mode, &mut next, &mut sfx);
    }
}

/// La molette fait défiler la liste ; la position est retenue pour la retrouver quand l'écran est redessiné.
fn wheel(mut events: MessageReader<MouseWheel>, mut browser: ResMut<Browser>, mut scroll: Query<&mut ScrollPosition, With<LevelList>>) {
    let mut delta = 0.0;
    for ev in events.read() {
        delta += match ev.unit {
            MouseScrollUnit::Line => ev.y * 48.0,
            MouseScrollUnit::Pixel => ev.y,
        };
    }
    if delta == 0.0 {
        return;
    }
    browser.scroll = (browser.scroll - delta).max(0.0);
    for mut p in &mut scroll {
        p.0.y = browser.scroll;
    }
}

pub struct ScenariosPlugin;

impl Plugin for ScenariosPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Browser>()
            .add_systems(OnEnter(GameState::Scenarios), begin)
            .add_systems(Update, (rebuild, keys, wheel).run_if(in_state(GameState::Scenarios)))
            .add_observer(on_click);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_follow_the_real_order() {
        for sc in 0..SCENARIOS.len() {
            let all = entries(sc);
            assert!(all.windows(2).all(|w| w[0].step < w[1].step && w[1].number == w[0].number + 1));
            assert!(all.iter().all(|e| SCENARIOS[sc].steps[e.step].scene.is_none()));
        }
        assert_eq!(entries(0).len(), 67);
    }

    #[test]
    fn a_new_game_has_one_step_to_play_and_the_rest_locked() {
        let progress = StoryProgress::default();
        let all = entries(0);
        let states: Vec<Status> = all.iter().map(|e| state_of(e, &progress)).collect();
        assert_eq!(states[0], Status::Next);
        assert!(states[1..].iter().all(|s| *s == Status::Locked));
    }

    #[test]
    fn par_is_estimated_for_adventure_levels() {
        let all = entries(0);
        let first = par_of(all[0].level());
        let last = par_of(all[all.len() - 1].level());
        assert!(first > 0.0 && last > first);
    }
}
