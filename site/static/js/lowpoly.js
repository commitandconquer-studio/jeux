// Commit & Conquer — kit 3D low poly commun à toutes les scènes du site.
// Facettes colorées une à une, couleurs de la charte, et un petit moteur qui
// gère la taille, la pause hors écran, la souris et la parallaxe au défilement.
import * as THREE from "three";
export { THREE };

export const C = {
  blue: 0x168fe8, green: 0x80c92b, lemon: 0xd8f43f, coral: 0xff493e,
  night: 0x142c40, sky: 0x8bd9ff, ivory: 0xfff9ed,
};
export const mix = (x, y, t) => new THREE.Color(x).lerp(new THREE.Color(y), t).getHex();

/** Ambiance du moment (voir ambiance.js) : moment, saison, fête, météo. */
export const AMB = window.CC_AMBIANCE || { moment: "jour", saison: "ete", fete: "", meteo: "" };

/** Teintes de l'herbe et des sapins selon la saison. */
const SAISONS = {
  printemps: { herbe: 0x8fd14f, clair: 0xf7b6d2, bord: 0x5a9a1c, feuillage: [C.green, 0x5a9a1c, 0x9be36a] },
  ete:       { herbe: C.green, clair: mix(C.green, C.lemon, 0.45), bord: 0x5a9a1c, feuillage: [C.green, 0x5a9a1c, mix(C.green, C.lemon, 0.5)] },
  automne:   { herbe: 0xb5a642, clair: 0xd9a55a, bord: 0x8a6d2b, feuillage: [0x5a9a1c, 0xc9842f, 0xd9a55a] },
  hiver:     { herbe: 0xf2f7fb, clair: 0xffffff, bord: 0xc9d8e4, feuillage: [0x3f7a3a, 0x356b32, 0xffffff] },
};
export const SAISON = SAISONS[AMB.saison] || SAISONS.ete;

// Aléatoire déterministe : la même scène à chaque visite.
let graine = 7;
export const semer = (g) => { graine = g; };
export const alea = () => ((graine = (graine * 16807) % 2147483647) / 2147483647);
export const entre = (a, b) => a + (b - a) * alea();

/** Peint chaque facette (couleur choisie selon sa normale et son centre). */
export function peindre(geo, couleur, nuance = 0.06) {
  const g = geo.index ? geo.toNonIndexed() : geo;
  const pos = g.attributes.position;
  const cols = new Float32Array(pos.count * 3);
  const a = new THREE.Vector3(), b = new THREE.Vector3(), c = new THREE.Vector3();
  const ab = new THREE.Vector3(), n = new THREE.Vector3(), centre = new THREE.Vector3(), col = new THREE.Color();
  for (let i = 0; i < pos.count; i += 3) {
    a.fromBufferAttribute(pos, i); b.fromBufferAttribute(pos, i + 1); c.fromBufferAttribute(pos, i + 2);
    n.subVectors(c, b).cross(ab.subVectors(a, b)).normalize();
    centre.copy(a).add(b).add(c).divideScalar(3);
    col.set(couleur(n, centre));
    const k = 1 + (alea() * 2 - 1) * nuance;
    for (let j = 0; j < 3; j++) {
      cols[(i + j) * 3] = Math.min(1, col.r * k);
      cols[(i + j) * 3 + 1] = Math.min(1, col.g * k);
      cols[(i + j) * 3 + 2] = Math.min(1, col.b * k);
    }
  }
  g.setAttribute("color", new THREE.BufferAttribute(cols, 3));
  g.computeVertexNormals();
  return g;
}

export const mat = new THREE.MeshLambertMaterial({ vertexColors: true, flatShading: true });
export const matPlat = new THREE.MeshBasicMaterial({ vertexColors: true, side: THREE.DoubleSide });
export const matLumineux = new THREE.MeshLambertMaterial({ vertexColors: true, flatShading: true, emissive: 0x9fb4c4 });
/** Maillage à facettes d'une seule teinte (nuancée). */
export const bloc = (geo, couleur, m = mat) => new THREE.Mesh(peindre(geo, typeof couleur === "function" ? couleur : () => couleur), m);

/** Île flottante : herbe (ou autre) dessus, terre puis roche en pointe dessous. */
export function ile(rayon, n = 14, teintes = {}) {
  const T = { herbe: SAISON.herbe, clair: SAISON.clair, bord: SAISON.bord, terre: 0x9b7448, roche: [0x7d8794, 0x939caa], ...teintes };
  const anneaux = [{ y: 0.35, r: 1.0 }, { y: -0.25, r: 1.03 }, { y: -0.8, r: 0.9 }, { y: -1.5, r: 0.68 }, { y: -2.2, r: 0.4 }];
  const bosse = Array.from({ length: n }, () => entre(0.85, 1.12));
  const pts = anneaux.map((an, k) => Array.from({ length: n }, (_, i) => {
    const ang = (i / n) * Math.PI * 2 + (k % 2) * 0.12;
    const r = rayon * an.r * bosse[i] * entre(0.94, 1.06);
    return new THREE.Vector3(Math.cos(ang) * r, an.y * rayon * 0.42 + (k === 0 ? entre(-0.05, 0.08) : 0), Math.sin(ang) * r);
  }));
  const pointe = new THREE.Vector3(entre(-0.3, 0.3), -3.0 * rayon * 0.42, entre(-0.3, 0.3));
  const centre = new THREE.Vector3(0, 0.38 * rayon * 0.42, 0);
  const v = [];
  const tri = (p, q, r) => v.push(p.x, p.y, p.z, q.x, q.y, q.z, r.x, r.y, r.z);
  for (let i = 0; i < n; i++) tri(centre, pts[0][(i + 1) % n], pts[0][i]);
  for (let k = 0; k < pts.length - 1; k++)
    for (let i = 0; i < n; i++) {
      const a = pts[k][i], b = pts[k][(i + 1) % n], c = pts[k + 1][i], d = pts[k + 1][(i + 1) % n];
      tri(a, b, d); tri(a, d, c);
    }
  const last = pts[pts.length - 1];
  for (let i = 0; i < n; i++) tri(last[i], last[(i + 1) % n], pointe);
  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.Float32BufferAttribute(v, 3));
  const u = rayon * 0.42;
  const m = new THREE.Mesh(peindre(geo, (nrm, ctr) => {
    if (nrm.y > 0.55) return alea() < 0.25 ? T.clair : T.herbe;
    if (ctr.y > 0) return T.bord;
    if (ctr.y > -1.2 * u) return T.terre;
    return T.roche[alea() < 0.5 ? 0 : 1];
  }), mat);
  m.userData.dessus = 0.35 * u;
  return m;
}

/** Lettre C extrudée, colorée comme l'emblème (face, dessus éclairé, flancs). */
export function lettreC(face, dessus, flanc) {
  const W = 4.2, H = 4.8, T = 1.65, K = 1.3;
  const s = new THREE.Shape();
  [[0, 0], [W, 0], [W, T], [T, T], [T, H - T], [W, H - T], [W, H], [K, H], [0, H - K]]
    .forEach(([x, y], i) => (i ? s.lineTo(x - W / 2, y - H / 2) : s.moveTo(x - W / 2, y - H / 2)));
  const geo = new THREE.ExtrudeGeometry(s, { depth: 2.1, bevelEnabled: false });
  geo.translate(0, 0, -1.05);
  return new THREE.Mesh(peindre(geo, (nrm) => (nrm.z > 0.5 ? face : nrm.y > 0.5 ? dessus : flanc)), matPlat);
}

/** Drapeau en bande de facettes ; `onduler(t)` le fait flotter. */
export function drapeau(couleur = C.coral, L = 2.3, Hd = 1.4) {
  const seg = 5, base = [];
  for (let i = 0; i <= seg; i++) {
    const x = (i / seg) * L, h = Hd * (1 - i / seg) / 2;
    base.push([x, h], [x, -h]);
  }
  const v = [];
  for (let i = 0; i < seg; i++) {
    const [a, b, c, d] = [base[i * 2], base[i * 2 + 1], base[i * 2 + 2], base[i * 2 + 3]];
    v.push(a[0], a[1], 0, b[0], b[1], 0, d[0], d[1], 0, a[0], a[1], 0, d[0], d[1], 0, c[0], c[1], 0);
  }
  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.Float32BufferAttribute(v, 3));
  const cols = [];
  for (let i = 0; i < v.length / 9; i++) {
    const col = new THREE.Color(i % 2 ? mix(couleur, 0x000000, 0.12) : couleur);
    for (let j = 0; j < 3; j++) cols.push(col.r, col.g, col.b);
  }
  geo.setAttribute("color", new THREE.Float32BufferAttribute(cols, 3));
  const m = new THREE.Mesh(geo, matPlat);
  const x0 = v.filter((_, i) => i % 3 === 0);
  const pos = geo.attributes.position;
  m.userData.onduler = (t) => {
    for (let i = 0; i < pos.count; i++) pos.setZ(i, Math.sin(t * 4 - x0[i] * 2.2) * 0.22 * x0[i]);
    pos.needsUpdate = true;
  };
  return m;
}

/** L'emblème « CC » avec son mât et son drapeau. */
export function emblème() {
  const g = new THREE.Group();
  const bleu = lettreC(C.blue, mix(C.blue, C.sky, 0.55), mix(C.blue, C.night, 0.35));
  const vert = lettreC(C.green, mix(C.green, C.lemon, 0.7), mix(C.green, C.night, 0.3));
  bleu.position.set(-1.15, 1.0, -1.1);
  vert.position.set(1.15, -1.0, 1.1);
  const mât = new THREE.Mesh(new THREE.BoxGeometry(0.34, 2.9, 0.34), new THREE.MeshBasicMaterial({ color: C.night }));
  mât.position.set(0.25, 4.85, -1.5);
  const fanion = drapeau();
  fanion.position.set(0.42, 6.25, -1.5);
  g.add(bleu, vert, mât, fanion);
  g.userData.onduler = fanion.userData.onduler;
  return g;
}

export function sapin(h = 1) {
  const g = new THREE.Group();
  const tronc = bloc(new THREE.CylinderGeometry(0.13, 0.18, 0.6, 5), 0x7a5232);
  tronc.position.y = 0.3;
  g.add(tronc);
  [[0.95, 1.5, 0.95], [0.72, 1.25, 1.75], [0.48, 1.0, 2.45]].forEach(([r, hh, y]) => {
    const f = SAISON.feuillage;
    const cone = bloc(new THREE.ConeGeometry(r, hh, 6), (n) => (n.y > 0.3 && (AMB.saison === "hiver" || alea() < 0.4) ? f[2] : alea() < 0.5 ? f[1] : f[0]));
    cone.position.y = y;
    cone.rotation.y = alea() * 3;
    g.add(cone);
  });
  g.scale.setScalar(h);
  g.userData.sapin = true;
  return g;
}

export function rocher(t = 1, teintes = [0x8f98a5, 0xa9b1bc]) {
  const m = bloc(new THREE.DodecahedronGeometry(0.6, 0), () => teintes[alea() < 0.5 ? 0 : 1]);
  m.scale.set(t * entre(1, 1.5), t * entre(0.6, 0.9), t);
  m.rotation.set(alea(), alea() * 3, alea());
  return m;
}

export function nuage() {
  const g = new THREE.Group();
  const n = 3 + Math.floor(alea() * 3);
  for (let i = 0; i < n; i++) {
    const b = bloc(new THREE.IcosahedronGeometry(entre(0.6, 1.1), 0), (nr) => (nr.y > 0 ? 0xffffff : 0xf0f6fb), matLumineux);
    b.position.set(i * 1.1 - n * 0.5, entre(-0.2, 0.4), entre(-0.4, 0.4));
    b.scale.y = 0.75;
    g.add(b);
  }
  return g;
}

/**
 * Monte une scène 3D dans `conteneur` (qui contient un <canvas>).
 * - `cadrer({ camera, large, aspect })` : place la caméra selon l'écran ;
 * - `animer(t, defil)` : appelé à chaque image ; `defil` va de 0 (haut de la
 *   scène) à 1 (scène sortie de l'écran par le haut) ;
 * - `couche(k)` : groupe de parallaxe ; il monte de `k` unités quand on fait
 *   défiler la scène hors de l'écran (k > 0 : plan proche, k < 0 : lointain).
 * Si WebGL manque, le conteneur reçoit la classe `no-3d` et rien ne casse.
 */
export function monter(conteneur, { cadrer, animer, lumieres = true, ambiance = true, fov = 32 } = {}) {
  const canvas = conteneur.querySelector("canvas");
  let renderer;
  try {
    renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
  } catch (e) {
    conteneur.classList.add("no-3d");
    return null;
  }
  conteneur.classList.add("a-3d");
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(fov, 1, 0.1, 300);
  const ciel = new THREE.HemisphereLight(0xdff4ff, 0x6f9a3a, 1.9);
  const soleil = new THREE.DirectionalLight(0xfff2dc, 2.4);
  if (lumieres) {
    scene.add(ciel, soleil);
    if (ambiance) eclairer(ciel, soleil, scene);
  }
  const couches = [];
  const couche = (k) => { const g = new THREE.Group(); g.userData.k = k; scene.add(g); couches.push(g); return g; };

  const calme = matchMedia("(prefers-reduced-motion: reduce)").matches;
  const souris = { x: 0, y: 0 };
  addEventListener("pointermove", (e) => {
    souris.x = (e.clientX / innerWidth) * 2 - 1;
    souris.y = (e.clientY / innerHeight) * 2 - 1;
  });
  let visible = true;
  new IntersectionObserver(([e]) => { visible = e.isIntersecting; }).observe(conteneur);

  const etat = { scene, camera, couche, souris, renderer, large: true, base: new THREE.Vector3(), cible: new THREE.Vector3() };
  // Ciel de nuit (étoiles et lune) et météo, sauf pour les scènes qui ont leur propre ciel.
  if (lumieres && ambiance) {
    if (AMB.moment === "nuit") cielDeNuit(couche(-6));
    etat.meteo = new Meteo(scene, couche(-2), ciel, soleil);
    addEventListener("cc-ambiance", () => { eclairer(ciel, soleil, scene); etat.meteo.changer(); });
  }
  function redimensionner() {
    const w = conteneur.clientWidth, h = conteneur.clientHeight;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    etat.large = camera.aspect > 1.15;
    cadrer?.({ camera, large: etat.large, aspect: camera.aspect, etat });
    etat.base.copy(camera.position);
    camera.lookAt(etat.cible);
    camera.updateProjectionMatrix();
  }
  const horloge = new THREE.Clock();
  function image() {
    const t = horloge.getElapsedTime();
    const r = conteneur.getBoundingClientRect();
    const defil = calme ? 0 : Math.min(1, Math.max(0, -r.top / Math.max(1, r.height)));
    couches.forEach((g) => { g.position.y = defil * g.userData.k; });
    animer?.(t, defil, etat);
    etat.meteo?.animer(t, camera);
    if (!calme) {
      camera.position.x += (etat.base.x + souris.x * 1.6 - camera.position.x) * 0.04;
      camera.position.y += (etat.base.y - souris.y * 1.0 - camera.position.y) * 0.04;
    }
    camera.lookAt(etat.cible);
    renderer.render(scene, camera);
  }
  // `etat` est prêt : la scène peut maintenant être remplie, puis on cadre.
  etat.demarrer = () => {
    redimensionner();
    addEventListener("resize", () => { redimensionner(); if (calme) image(); });
    if (calme) image();
    else renderer.setAnimationLoop(() => { if (visible && !document.hidden) image(); });
  };
  return etat;
}

// ---- Ambiance : lumière, ciel de nuit, météo, décors de fête --------------

/** Règle la lumière selon le moment et la météo. */
function eclairer(ciel, soleil, scene) {
  const L = {
    matin: [0xffe6d2, 0x6f9a3a, 1.6, 0xffc49a, 2.0, [-14, 5, 8]],
    jour:  [0xdff4ff, 0x6f9a3a, 1.9, 0xfff2dc, 2.4, [-6, 12, 9]],
    soir:  [0xffc9ad, 0x4a5a3a, 1.35, 0xff9152, 2.0, [14, 4, 7]],
    nuit:  [0x7088bd, 0x1b2a3a, 0.95, 0xbcd2ff, 0.9, [-8, 10, 6]],
  }[AMB.moment] || [];
  if (!L.length) return;
  ciel.color.set(L[0]); ciel.groundColor.set(L[1]); ciel.intensity = L[2];
  soleil.color.set(L[3]); soleil.intensity = L[4]; soleil.position.set(...L[5]);
  const gris = ["nuageux", "pluie", "orage", "brouillard", "neige"].includes(AMB.meteo);
  if (gris) { soleil.intensity *= 0.45; ciel.color.lerp(new THREE.Color(0xb7c3cc), 0.5); }
  // Nuages : gris quand il pleut, bleutés la nuit.
  matLumineux.color.set(AMB.meteo === "pluie" || AMB.meteo === "orage" ? 0xa6b1ba : AMB.moment === "nuit" ? 0x8193b4 : 0xffffff);
  matLumineux.emissive.set(AMB.moment === "nuit" ? 0x1d2b44 : gris ? 0x5d6b77 : 0x9fb4c4);
  scene.fog = AMB.meteo === "brouillard" ? new THREE.Fog(AMB.moment === "nuit" ? 0x2a3a50 : 0xdfe6ea, 30, 75) : null;
}

/** Étoiles et lune, pour les scènes la nuit. */
function cielDeNuit(couche) {
  const n = 500, pts = new Float32Array(n * 3);
  for (let i = 0; i < n; i++) pts.set([entre(-120, 120), entre(0, 70), entre(-120, -40)], i * 3);
  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.BufferAttribute(pts, 3));
  couche.add(new THREE.Points(geo, new THREE.PointsMaterial({ color: 0xfff9ed, size: 0.5, sizeAttenuation: true })));
  const lune = new THREE.Mesh(new THREE.IcosahedronGeometry(3, 1), new THREE.MeshBasicMaterial({ color: 0xfff4d6 }));
  lune.position.set(-34, 26, -70);
  couche.add(lune);
}

/** Pluie, neige, orage et bancs de nuages selon la météo (peut changer en cours de visite). */
class Meteo {
  constructor(scene, couche, ciel, soleil) {
    Object.assign(this, { scene, couche, ciel, soleil });
    this.groupe = new THREE.Group();
    couche.add(this.groupe);
    this.changer();
  }
  changer() {
    this.groupe.clear();
    this.gouttes = null;
    const m = AMB.meteo;
    // Banc de nuages supplémentaire quand le ciel se couvre.
    if (["nuageux", "pluie", "orage", "neige"].includes(m)) {
      for (let i = 0; i < 10; i++) {
        const n = nuage();
        n.position.set(entre(-34, 34), entre(9, 15), entre(-30, -6));
        n.scale.setScalar(entre(1.0, 1.8));
        this.groupe.add(n);
      }
    }
    if (m === "pluie" || m === "orage" || m === "neige") {
      const neige = m === "neige", n = neige ? 900 : 1400;
      const pts = new Float32Array(n * 3);
      for (let i = 0; i < n; i++) pts.set([entre(-30, 30), entre(-12, 22), entre(-20, 14)], i * 3);
      const geo = new THREE.BufferGeometry();
      geo.setAttribute("position", new THREE.BufferAttribute(pts, 3));
      const matiere = new THREE.PointsMaterial({
        color: neige ? 0xffffff : 0xa9cbe6, size: neige ? 0.28 : 0.16, transparent: true, opacity: neige ? 0.95 : 0.7, sizeAttenuation: true,
      });
      this.gouttes = new THREE.Points(geo, matiere);
      this.gouttes.userData = { neige, vitesse: neige ? 0.05 : 0.55 };
      this.groupe.add(this.gouttes);
    }
    this.orage = m === "orage";
    this.base = this.ciel.intensity;
  }
  animer(t) {
    if (this.gouttes) {
      const pos = this.gouttes.geometry.attributes.position, { neige, vitesse } = this.gouttes.userData;
      for (let i = 0; i < pos.count; i++) {
        let y = pos.getY(i) - vitesse;
        if (y < -12) y = 22;
        pos.setY(i, y);
        if (neige) pos.setX(i, pos.getX(i) + Math.sin(t + i) * 0.01);
      }
      pos.needsUpdate = true;
    }
    if (this.orage) {
      // Un éclair de temps en temps.
      const flash = Math.sin(t * 0.7) > 0.995 || (Math.sin(t * 0.7) > 0.99 && Math.sin(t * 40) > 0);
      this.ciel.intensity = flash ? this.base * 3.5 : this.base;
    }
  }
}

/**
 * Décors de fête sur une île : `ile` (groupe), `y` (dessus de l'herbe),
 * `rayon`, et `devant` (couche proche, pour ce qui vole).
 * Renvoie une fonction d'animation (ou rien).
 */
export function decorer(ile, y, rayon, devant) {
  const f = AMB.fete;
  const sapins = [];
  ile.traverse((o) => { if (o.userData.sapin) sapins.push(o); });
  const surHerbe = (obj, a, d) => { obj.position.set(Math.cos(a) * rayon * d, y, Math.sin(a) * rayon * d); ile.add(obj); return obj; };
  if (f === "halloween") {
    [0.4, 1.9, 3.3, 4.9].forEach((a, i) => {
      const c = new THREE.Group();
      const corps = bloc(new THREE.IcosahedronGeometry(0.5, 1), (n) => (n.y > 0.3 ? 0xffa53a : 0xf07f12));
      corps.scale.y = 0.75;
      corps.position.y = 0.36;
      const tige = bloc(new THREE.CylinderGeometry(0.06, 0.08, 0.25, 5), 0x3d6b10);
      tige.position.y = 0.8;
      c.add(corps, tige);
      c.scale.setScalar(0.9 + i * 0.12);
      surHerbe(c, a, 0.55);
    });
    const vols = [0, 1, 2].map((i) => {
      const g = new THREE.Group();
      const aile = (sx) => { const m = new THREE.Mesh(new THREE.ConeGeometry(0.35, 0.9, 3), new THREE.MeshBasicMaterial({ color: 0x1b1026 })); m.rotation.z = sx * Math.PI / 2; m.position.x = sx * 0.45; m.scale.z = 0.2; return m; };
      g.add(new THREE.Mesh(new THREE.IcosahedronGeometry(0.22, 0), new THREE.MeshBasicMaterial({ color: 0x1b1026 })), aile(1), aile(-1));
      devant.add(g);
      return g;
    });
    return (t) => vols.forEach((g, i) => {
      const a = t * (0.7 + i * 0.2) + i * 2;
      g.position.set(Math.cos(a) * (6 + i), 6 + i + Math.sin(t * 2 + i), Math.sin(a) * 4);
      g.children[1].rotation.x = g.children[2].rotation.x = Math.sin(t * 18 + i) * 0.7;
    });
  }
  if (f === "avent" || f === "noel" || f === "saint-nicolas") {
    sapins.forEach((sp, k) => {
      for (let i = 0; i < 5; i++) {
        const boule = new THREE.Mesh(new THREE.IcosahedronGeometry(0.13, 0), new THREE.MeshBasicMaterial({ color: [C.coral, 0xffd166, C.sky][(i + k) % 3] }));
        const h = 1.0 + i * 0.32, a = i * 2.4 + k;
        boule.position.set(Math.cos(a) * (0.9 - i * 0.12), h, Math.sin(a) * (0.9 - i * 0.12));
        sp.add(boule);
      }
      if (k === 0 || f === "noel") {
        const étoile = new THREE.Mesh(new THREE.OctahedronGeometry(0.28, 0), new THREE.MeshBasicMaterial({ color: 0xffd166 }));
        étoile.position.y = 3.15;
        sp.add(étoile);
      }
    });
    if (f === "saint-nicolas" || f === "noel") {
      [1.2, 3.8].forEach((a, i) => {
        const p = bloc(new THREE.BoxGeometry(0.7, 0.55, 0.7), i ? C.blue : C.coral);
        p.position.y = 0.28;
        const ruban = new THREE.Mesh(new THREE.BoxGeometry(0.74, 0.58, 0.14), new THREE.MeshBasicMaterial({ color: 0xffd166 }));
        ruban.position.y = 0.28;
        const g = new THREE.Group(); g.add(p, ruban);
        surHerbe(g, a, 0.45);
      });
    }
    return null;
  }
  if (f === "paques") {
    const pastels = [0xf7b6d2, 0xbfe3ff, 0xfff1a8, 0xc8f0b0, 0xe2c8ff];
    for (let i = 0; i < 7; i++) {
      const œuf = bloc(new THREE.IcosahedronGeometry(0.22, 1), pastels[i % 5]);
      œuf.scale.y = 1.35;
      œuf.position.y = 0.26;
      const g = new THREE.Group(); g.add(œuf);
      surHerbe(g, i * 0.9 + 0.3, 0.35 + (i % 3) * 0.17);
    }
    return null;
  }
  if (f === "rameaux") {
    [0.8, 3.6].forEach((a) => {
      const g = new THREE.Group();
      const tronc = bloc(new THREE.CylinderGeometry(0.1, 0.14, 1.6, 5), 0x9b7448);
      tronc.position.y = 0.8;
      g.add(tronc);
      for (let i = 0; i < 6; i++) {
        const palme = bloc(new THREE.ConeGeometry(0.14, 1.3, 3), C.green);
        palme.position.set(0, 1.6, 0);
        palme.rotation.set(1.1, i * 1.05, 0);
        palme.translateY(0.6);
        g.add(palme);
      }
      surHerbe(g, a, 0.5);
    });
    return null;
  }
  if (f === "epiphanie") {
    const couronne = new THREE.Group();
    couronne.add(new THREE.Mesh(new THREE.CylinderGeometry(0.7, 0.7, 0.35, 8, 1, true), new THREE.MeshBasicMaterial({ color: 0xffd166, side: THREE.DoubleSide })));
    for (let i = 0; i < 8; i++) {
      const pointe = new THREE.Mesh(new THREE.ConeGeometry(0.16, 0.4, 4), new THREE.MeshBasicMaterial({ color: 0xffc23a }));
      pointe.position.set(Math.cos(i * 0.785) * 0.66, 0.36, Math.sin(i * 0.785) * 0.66);
      couronne.add(pointe);
    }
    couronne.position.y = 0.2;
    surHerbe(couronne, 2.2, 0.4);
    return null;
  }
  return null;
}
