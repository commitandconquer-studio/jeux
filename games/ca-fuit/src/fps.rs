//! `CA_FUIT_FPS=1` : les images par seconde dans le terminal, pour mesurer.
//! `CA_FUIT_OMBRES=0|1024|2048` : pour essayer sans ombres ou avec une carte
//! d'ombres plus petite.
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::light::DirectionalLightShadowMap;
use bevy::prelude::*;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        if std::env::var_os("CA_FUIT_FPS").is_some() {
            app.add_plugins((FrameTimeDiagnosticsPlugin::default(), LogDiagnosticsPlugin::default()));
        }
        if let Some(taille) = std::env::var("CA_FUIT_OMBRES").ok().and_then(|v| v.parse::<usize>().ok()) {
            if taille == 0 {
                app.add_systems(Update, sans_ombres);
            } else {
                app.insert_resource(DirectionalLightShadowMap { size: taille });
            }
        }
    }
}

fn sans_ombres(mut q: Query<&mut DirectionalLight>) {
    for mut l in &mut q {
        l.shadow_maps_enabled = false;
    }
}
