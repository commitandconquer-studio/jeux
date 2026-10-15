//! Rend l'emblème « CC » du studio (modélisé dans `game_common::brand`) en
//! PNG transparent : `cargo run -p logo-gen` écrit `assets/branding/embleme.png`.
//! Avec `LOGO_ANGLE=…`, on peut essayer d'autres points de vue (en degrés).
use bevy::camera::{RenderTarget, ScalingMode};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};
use game_common::brand;

const SIZE: u32 = 1024;

#[derive(Resource)]
struct Target(Handle<Image>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "logo-gen".into(), resolution: (320, 240).into(), ..default() }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, shoot)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let image = images.add(Image::new_target_texture(SIZE, SIZE, TextureFormat::Rgba8UnormSrgb, None));
    commands.insert_resource(Target(image.clone()));
    brand::spawn_emblem(&mut commands, &mut meshes, &mut materials, Transform::default());
    // Vue de trois quarts, d'en haut à gauche : on voit la face des lettres,
    // leurs dessus éclairés et leurs flancs gauches à l'ombre.
    let dir = Vec3::new(-0.5, 0.5, 1.0).normalize();
    let look = Vec3::new(0.0, 1.1, 0.0);
    commands.spawn((
        Camera3d::default(),
        Camera { clear_color: ClearColorConfig::Custom(Color::NONE), ..default() },
        RenderTarget::Image(image.into()),
        Tonemapping::None,
        DebandDither::Disabled,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 12.0, height: 12.0 },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(look + dir * 30.0).looking_at(look, Vec3::Y),
    ));
}

const OUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/branding/embleme.png");

fn shoot(time: Res<Time>, mut commands: Commands, mut frame: Local<u32>, target: Res<Target>, mut exit: MessageWriter<AppExit>) {
    // On laisse aux shaders le temps de compiler (en arrière-plan) avant la photo.
    if time.elapsed_secs() < 3.0 {
        return;
    }
    *frame += 1;
    if *frame == 1 {
        let _ = std::fs::remove_file(OUT);
        commands.spawn(Screenshot::image(target.0.clone())).observe(save_rgba);
    }
    // La capture est asynchrone : on quitte une fois le fichier écrit.
    if *frame > 1 && std::path::Path::new(OUT).exists() || *frame > 2000 {
        exit.write(AppExit::Success);
    }
}

/// Comme `save_to_disk`, mais en gardant la transparence.
fn save_rgba(captured: On<ScreenshotCaptured>) {
    match captured.image.clone().try_into_dynamic() {
        Ok(img) => {
            let tmp = format!("{OUT}.tmp.png");
            img.to_rgba8().save(&tmp).expect("écriture du PNG");
            std::fs::rename(&tmp, OUT).expect("renommage du PNG");
        }
        Err(e) => error!("capture illisible : {e:?}"),
    }
}
