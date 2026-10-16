//! Le son, commun à tous les jeux : son général (touche M ou Options),
//! musique activée ou non, volume de la musique et volume des effets.
//! Réglages communs à tous les jeux du studio (« studio-son »).
//!
//! Dans un jeu : marquer la musique avec `Musique` (et `Mutable`). Tout le
//! reste est un effet. Les effets courts ne sont pas joués quand le son est
//! coupé (`SoundOn`) ; les sons longs marqués `Mutable` sont mis en sourdine.
use bevy::audio::Volume;
use bevy::prelude::*;

pub struct AudioKitPlugin;

impl Plugin for AudioKitPlugin {
    fn build(&self, app: &mut App) {
        let r = Reglages::charger();
        app.insert_resource(SoundOn(r.son))
            .insert_resource(r)
            .add_systems(Update, (toggle_with_key, enregistrer, apply_mute, appliquer_volumes).chain());
    }
}

/// Son général activé ou coupé.
#[derive(Resource)]
pub struct SoundOn(pub bool);

impl SoundOn {
    /// Libellé d'un bouton de menu.
    pub fn label(&self) -> String {
        let etat = if self.0 { tx!("activé", "on", "an", "attivo", "activado") } else { tx!("coupé", "off", "aus", "spento", "desactivado") };
        format!("{}{}{}", tx!("Son", "Sound", "Ton", "Audio", "Sonido").t(), crate::langue::deux_points(), etat.t())
    }
}

/// Réglages du son (le son général est dans `SoundOn`).
#[derive(Resource, Clone, Copy, PartialEq, Debug)]
pub struct Reglages {
    pub son: bool,
    pub musique: bool,
    /// Volumes de 0 à 10 (dixièmes).
    pub volume_musique: u8,
    pub volume_effets: u8,
}

impl Reglages {
    fn charger() -> Reglages {
        let d = Reglages { son: true, musique: true, volume_musique: 7, volume_effets: 10 };
        let Some(t) = crate::save::load("studio", "son") else { return d };
        let m: Vec<&str> = t.split_whitespace().collect();
        let n = |i: usize, v: u8| m.get(i).and_then(|x| x.parse::<u8>().ok()).map_or(v, |x| x.min(10));
        Reglages { son: m.first() != Some(&"0"), musique: m.get(1) != Some(&"0"), volume_musique: n(2, d.volume_musique), volume_effets: n(3, d.volume_effets) }
    }

    fn enregistrer(&self) {
        let b = |v: bool| if v { 1 } else { 0 };
        crate::save::store("studio", "son", &format!("{} {} {} {}", b(self.son), b(self.musique), self.volume_musique, self.volume_effets));
    }
}

/// Sons longs (musique, sonneries) : mis en sourdine quand on coupe le son.
/// Les effets courts, eux, ne sont simplement pas joués.
#[derive(Component)]
pub struct Mutable;

/// Marque une musique (son volume est celui de la musique, pas des effets).
#[derive(Component)]
pub struct Musique;

/// Volume d'origine d'un son, avant les réglages du joueur.
#[derive(Component)]
struct Base(f32);

fn toggle_with_key(keys: Res<ButtonInput<KeyCode>>, mut on: ResMut<SoundOn>) {
    // Les touches sont physiques : la touche « M » d'un clavier AZERTY est à
    // l'emplacement de « ; » en QWERTY. On accepte les deux.
    if keys.just_pressed(KeyCode::KeyM) || keys.just_pressed(KeyCode::Semicolon) {
        on.0 = !on.0;
    }
}

/// Le son général et les réglages restent d'accord, et sont enregistrés.
fn enregistrer(on: Res<SoundOn>, mut r: ResMut<Reglages>) {
    if on.is_changed() && r.son != on.0 {
        r.son = on.0;
    }
    if r.is_changed() && !r.is_added() {
        r.enregistrer();
    }
}

fn apply_mute(on: Res<SoundOn>, r: Res<Reglages>, mut sinks: Query<(&mut AudioSink, Has<Musique>), With<Mutable>>) {
    for (mut sink, musique) in &mut sinks {
        let muet = !on.0 || (musique && !r.musique);
        if !muet && sink.is_muted() {
            sink.unmute();
        } else if muet && !sink.is_muted() {
            sink.mute();
        }
    }
}

/// Volumes : la musique suit son réglage, les effets le leur.
fn appliquer_volumes(
    mut commands: Commands,
    r: Res<Reglages>,
    mut nouveaux: Query<(Entity, &mut AudioSink, Has<Musique>), Without<Base>>,
    mut connus: Query<(&mut AudioSink, &Base, Has<Musique>)>,
) {
    let facteur = |musique: bool| if musique { r.volume_musique } else { r.volume_effets } as f32 / 10.0;
    for (e, mut sink, musique) in &mut nouveaux {
        let base = sink.volume().to_linear();
        sink.set_volume(Volume::Linear(base * facteur(musique)));
        commands.entity(e).insert(Base(base));
    }
    if r.is_changed() {
        for (mut sink, base, musique) in &mut connus {
            sink.set_volume(Volume::Linear(base.0 * facteur(musique)));
        }
    }
}
