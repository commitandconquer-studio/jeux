//! Son activé / coupé, commun à tous les jeux (touche M ou bouton du menu).
use bevy::prelude::*;

pub struct AudioKitPlugin;

impl Plugin for AudioKitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SoundOn(true)).add_systems(Update, (toggle_with_key, apply_mute).chain());
    }
}

/// Son activé ou coupé.
#[derive(Resource)]
pub struct SoundOn(pub bool);

impl SoundOn {
    /// Libellé du bouton du menu.
    pub fn label(&self) -> &'static str {
        if self.0 { "Son : activé" } else { "Son : coupé" }
    }
}

/// Sons longs (musique, sonneries) : mis en sourdine quand on coupe le son.
/// Les effets courts, eux, ne sont simplement pas joués.
#[derive(Component)]
pub struct Mutable;

fn toggle_with_key(keys: Res<ButtonInput<KeyCode>>, mut on: ResMut<SoundOn>) {
    // Les touches sont physiques : la touche « M » d'un clavier AZERTY est à
    // l'emplacement de « ; » en QWERTY. On accepte les deux.
    if keys.just_pressed(KeyCode::KeyM) || keys.just_pressed(KeyCode::Semicolon) {
        on.0 = !on.0;
    }
}

fn apply_mute(on: Res<SoundOn>, mut sinks: Query<&mut AudioSink, With<Mutable>>) {
    for mut sink in &mut sinks {
        if on.0 && sink.is_muted() {
            sink.unmute();
        } else if !on.0 && !sink.is_muted() {
            sink.mute();
        }
    }
}
