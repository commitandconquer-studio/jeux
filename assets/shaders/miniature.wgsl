// Effet « maquette » (tilt-shift) de Marche ou Crève : flou des bords de l'écran, surtout à gauche et à droite
// (les zones où le personnage ne peut pas aller), un peu en haut et en bas. Le centre reste net.
struct Reglages {
    // Rayon maximal du flou sur les côtés, en fraction de la hauteur de l'écran.
    lateral: f32,
    // Rayon maximal du flou en haut et en bas, en fraction de la hauteur de l'écran.
    vertical: f32,
    // Demi-largeur de la zone nette au centre, en fraction de la largeur (0,5 = tout l'écran).
    net_x: f32,
    // Demi-hauteur de la zone nette au centre, en fraction de la hauteur.
    net_y: f32,
}

@group(0) @binding(0) var ecran: texture_2d<f32>;
@group(0) @binding(1) var echantillonneur: sampler;
@group(0) @binding(2) var<uniform> reglages: Reglages;

const PRISES: i32 = 28;
const ANGLE_D_OR: f32 = 2.39996323;

@fragment
fn fragment(@builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let dims = vec2<f32>(textureDimensions(ecran));
    // 0 au centre, 1 au bord.
    let dx = smoothstep(reglages.net_x, 0.5, abs(uv.x - 0.5));
    let dy = smoothstep(reglages.net_y, 0.5, abs(uv.y - 0.5));
    let rayon = max(reglages.lateral * dx, reglages.vertical * dy) * dims.y;
    if rayon < 0.5 {
        return textureSampleLevel(ecran, echantillonneur, uv, 0.0);
    }
    // Disque de prises en spirale (Vogel) : un flou rond et régulier.
    var somme = textureSampleLevel(ecran, echantillonneur, uv, 0.0);
    for (var i = 0; i < PRISES; i = i + 1) {
        let r = sqrt((f32(i) + 0.5) / f32(PRISES));
        let a = f32(i) * ANGLE_D_OR;
        let decalage = vec2<f32>(cos(a), sin(a)) * r * rayon / dims;
        somme = somme + textureSampleLevel(ecran, echantillonneur, uv + decalage, 0.0);
    }
    return somme / f32(PRISES + 1);
}
