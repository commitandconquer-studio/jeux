//! Effet « maquette » (tilt-shift) : un flou qui grandit vers les bords de l'écran, surtout à gauche et à droite
//! (le personnage n'y va pas), un peu en haut et en bas. Passe plein écran après le rendu, sur le modèle de
//! l'exemple `custom_post_processing` de Bevy. Actif sur ordinateur par défaut.
use bevy::{
    core_pipeline::{schedule::Core3d, Core3dSystems, FullscreenShader},
    prelude::*,
    render::{
        extract_component::{ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice, ViewQuery},
        view::ViewTarget,
        Render, RenderApp, RenderStartup, RenderSystems,
    },
};

const SHADER: &str = "shaders/miniature.wgsl";

/// Le flou « maquette » est-il en route ? Oui sur ordinateur (`TRAVERSEE_FLOU=0` le coupe) ;
/// sur le web il reste à essayer : `TRAVERSEE_FLOU=1` ne s'y lit pas, donc coupé.
pub fn actif() -> bool {
    static A: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *A.get_or_init(|| !cfg!(target_arch = "wasm32") && std::env::var("TRAVERSEE_FLOU").map_or(true, |v| v != "0"))
}

/// Réglages du flou, posés sur la caméra. Tous en fraction de l'écran.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
#[extract_app(RenderApp)]
pub struct Miniature {
    /// Flou maximal sur les côtés (rayon, en fraction de la hauteur de l'écran).
    pub lateral: f32,
    /// Flou maximal en haut et en bas.
    pub vertical: f32,
    /// Demi-largeur de la zone nette.
    pub net_x: f32,
    /// Demi-hauteur de la zone nette.
    pub net_y: f32,
}

impl Default for Miniature {
    fn default() -> Self {
        let lire = |cle: &str, defaut: f32| std::env::var(cle).ok().and_then(|s| s.parse().ok()).unwrap_or(defaut);
        Self {
            lateral: lire("TRAVERSEE_FLOU_COTES", 0.014),
            vertical: lire("TRAVERSEE_FLOU_HAUT_BAS", 0.005),
            // La zone jouable (9 colonnes sur ~15 visibles) tient dans ±0,30 de la largeur.
            net_x: lire("TRAVERSEE_FLOU_NET", 0.28),
            net_y: 0.34,
        }
    }
}

pub struct MiniaturePlugin;

impl Plugin for MiniaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ExtractComponentPlugin::<Miniature>::default(), UniformComponentPlugin::<Miniature>::default()));
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(RenderStartup, init_pipeline)
            .add_systems(Render, preparer_groupes.in_set(RenderSystems::PrepareBindGroups))
            .add_systems(Core3d, passe_flou.in_set(Core3dSystems::PostProcess));
    }
}

/// Les groupes de liaison des deux textures principales (on ne sait pas d'avance laquelle est la source).
#[derive(Component)]
struct Groupes {
    a: (TextureViewId, BindGroup),
    b: (TextureViewId, BindGroup),
}

fn preparer_groupes(
    mut commands: Commands,
    mut vues: Query<(Entity, &ViewTarget, Option<&mut Groupes>)>,
    pipeline: Option<Res<PipelineFlou>>,
    cache: Res<PipelineCache>,
    reglages: Res<ComponentUniforms<Miniature>>,
    device: Res<RenderDevice>,
) {
    let Some(pipeline) = pipeline else {
        return;
    };
    let Some(binding) = reglages.uniforms().binding() else {
        return;
    };
    let creer = |texture: &TextureView| {
        (
            texture.id(),
            device.create_bind_group("miniature_groupe", &cache.get_bind_group_layout(&pipeline.layout), &BindGroupEntries::sequential((texture, &pipeline.sampler, binding.clone()))),
        )
    };
    for (entite, cible, groupes) in &mut vues {
        let principale = cible.main_texture_view();
        let autre = cible.main_texture_other_view();
        match groupes {
            Some(mut g) => {
                if g.a.0 != principale.id() {
                    g.a = creer(principale);
                }
                if g.b.0 != autre.id() {
                    g.b = creer(autre);
                }
            }
            None => {
                commands.entity(entite).insert(Groupes { a: creer(principale), b: creer(autre) });
            }
        }
    }
}

fn passe_flou(vue: ViewQuery<(&ViewTarget, &DynamicUniformIndex<Miniature>, &Groupes)>, pipeline: Option<Res<PipelineFlou>>, cache: Res<PipelineCache>, mut ctx: RenderContext) {
    let Some(pipeline) = pipeline else {
        return;
    };
    let (cible, index, groupes) = vue.into_inner();
    let Some(rendu) = cache.get_render_pipeline(pipeline.pipeline_id) else {
        return;
    };
    let ecriture = cible.post_process_write();
    let (_, groupe) = if groupes.a.0 == ecriture.source.id() { &groupes.a } else { &groupes.b };
    let mut passe = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("miniature_passe"),
        color_attachments: &[Some(RenderPassColorAttachment { view: ecriture.destination, depth_slice: None, resolve_target: None, ops: Operations::default() })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    passe.set_pipeline(rendu);
    passe.set_bind_group(0, groupe, &[index.index()]);
    passe.draw(0..3, 0..1);
}

#[derive(Resource)]
struct PipelineFlou {
    layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

fn init_pipeline(mut commands: Commands, device: Res<RenderDevice>, serveur: Res<AssetServer>, plein_ecran: Res<FullscreenShader>, cache: Res<PipelineCache>) {
    let layout = BindGroupLayoutDescriptor::new(
        "miniature_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (texture_2d(TextureSampleType::Float { filterable: true }), sampler(SamplerBindingType::Filtering), uniform_buffer::<Miniature>(true)),
        ),
    );
    let sampler = device.create_sampler(&SamplerDescriptor { address_mode_u: AddressMode::ClampToEdge, address_mode_v: AddressMode::ClampToEdge, mag_filter: FilterMode::Linear, min_filter: FilterMode::Linear, ..default() });
    let shader = serveur.load(SHADER);
    let pipeline_id = cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("miniature_pipeline".into()),
        layout: vec![layout.clone()],
        vertex: plein_ecran.to_vertex_state(),
        fragment: Some(FragmentState {
            shader,
            targets: vec![Some(ColorTargetState { format: TextureFormat::Rgba8UnormSrgb, blend: None, write_mask: ColorWrites::ALL })],
            ..default()
        }),
        ..default()
    });
    commands.insert_resource(PipelineFlou { layout, sampler, pipeline_id });
}
