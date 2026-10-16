// Commit & Conquer — scène 3D en tête de chaque page d'univers.
// Chaque univers a son décor ; tous partagent le kit low poly et la parallaxe.
import { THREE, C, mix, alea, entre, semer, bloc, mat, ile, sapin, rocher, nuage, drapeau, decorer, monter } from "./lowpoly.js";

const hero = document.querySelector(".univers-hero");
const cle = hero?.dataset.cle;
const animations = [];
const anim = (f) => animations.push(f);

const s = hero && monter(hero, {
  ambiance: cle !== "nova", // dans l'espace : ni pluie, ni lune terrestre
  cadrer({ camera, large, aspect, etat }) {
    camera.filmOffset = large ? -Math.min(9, 4 + aspect * 2.2) : 0;
    etat.cible.set(0, large ? 1.5 : 6, 0);
    camera.position.set(0, 7, large ? 34 : 54);
  },
  animer(t, defil, etat) { animations.forEach((f) => f(t, defil, etat)); },
});

if (s) {
  semer(11);
  ({ croa, nova, eureka, legendes }[cle] || croa)(s);
  s.demarrer();
}

function nuagesLointains(couche, n = 6) {
  for (let i = 0; i < n; i++) {
    const g = nuage();
    g.position.set(entre(-30, 30), entre(5, 12), entre(-34, -12));
    g.scale.setScalar(entre(0.7, 1.2));
    const v = entre(0.2, 0.5);
    couche.add(g);
    anim(() => { g.position.x += v * 0.016; if (g.position.x > 32) g.position.x = -32; });
  }
}

/** Fait flotter un objet doucement. */
function flotter(o, amp = 0.3, vit = 0.8, phase = 0) {
  const y0 = o.position.y;
  anim((t) => { o.position.y = y0 + Math.sin(t * vit + phase) * amp; });
}

// ---- Croâ & Cie : la mare du crapaud ------------------------------------
function croa(s) {
  const loin = s.couche(-4), milieu = s.couche(3), devant = s.couche(7);
  nuagesLointains(loin);
  const île = new THREE.Group();
  milieu.add(île);
  const sol = ile(5.4);
  île.add(sol);
  const y = sol.userData.dessus;
  const mare = bloc(new THREE.CylinderGeometry(2.4, 2.4, 0.12, 9), (n) => (n.y > 0.5 ? (alea() < 0.3 ? 0xa8e3ff : C.sky) : 0x5fb8e6));
  mare.position.set(-0.6, y + 0.02, 0.6);
  mare.scale.z = 0.8;
  île.add(mare);
  const nénuphar = (x, z, r) => {
    const n = bloc(new THREE.CylinderGeometry(r, r, 0.06, 7, 1, false, 0.5, Math.PI * 1.75), (nn) => (nn.y > 0.5 ? C.green : 0x5a9a1c));
    n.position.set(x, y + 0.12, z);
    n.rotation.y = alea() * 6;
    île.add(n);
    return n;
  };
  const feuille = nénuphar(-0.4, 0.8, 0.9);
  nénuphar(-1.9, 0.1, 0.55);
  nénuphar(0.7, 1.6, 0.45);

  // Le crapaud, posé sur le plus grand nénuphar.
  const crapaud = new THREE.Group();
  const corps = bloc(new THREE.DodecahedronGeometry(0.75, 0), (n) => (n.y > 0.3 ? C.green : n.y < -0.3 ? 0xd8f43f : 0x5a9a1c));
  corps.scale.set(1.15, 0.7, 1);
  crapaud.add(corps);
  [-0.35, 0.35].forEach((x) => {
    const œil = bloc(new THREE.IcosahedronGeometry(0.24, 0), C.ivory);
    œil.position.set(x, 0.5, 0.35);
    const pupille = new THREE.Mesh(new THREE.BoxGeometry(0.12, 0.14, 0.06), new THREE.MeshBasicMaterial({ color: C.night }));
    pupille.position.set(x, 0.52, 0.58);
    crapaud.add(œil, pupille);
  });
  const bouche = new THREE.Mesh(new THREE.BoxGeometry(0.5, 0.05, 0.05), new THREE.MeshBasicMaterial({ color: C.coral }));
  bouche.position.set(0, 0.12, 0.72);
  crapaud.add(bouche);
  crapaud.position.set(feuille.position.x, y + 0.6, feuille.position.z);
  crapaud.rotation.y = 0.35;
  crapaud.scale.setScalar(1.1);
  île.add(crapaud);
  // Un saut toutes les trois secondes.
  anim((t) => {
    const p = (t % 3) / 3;
    crapaud.position.y = y + 0.6 + (p < 0.18 ? Math.sin((p / 0.18) * Math.PI) * 1.1 : 0);
    crapaud.scale.y = 1.1 * (p > 0.9 ? 0.85 : 1);
  });

  // Cubes en bois empilés, comme des jouets.
  [[2.4, 0, -1.2, 0xffd166], [3.3, 0, -0.6, C.coral], [2.8, 1, -0.9, C.blue], [3.4, 0, -2.0, C.green]].forEach(([x, étage, z, col]) => {
    const cube = bloc(new THREE.BoxGeometry(0.9, 0.9, 0.9), (n) => (n.y > 0.5 ? mix(col, 0xffffff, 0.25) : col));
    cube.position.set(x, y + 0.45 + étage * 0.9, z);
    cube.rotation.y = alea();
    île.add(cube);
  });
  // Fleurs et sapins sur le pourtour.
  for (let i = 0; i < 9; i++) {
    const a = (i / 9) * Math.PI * 2;
    const f = bloc(new THREE.ConeGeometry(0.18, 0.4, 5), [C.coral, C.lemon, 0xffffff][i % 3]);
    f.position.set(Math.cos(a) * 3.9, y + 0.2, Math.sin(a) * 3.4);
    f.rotation.x = Math.PI;
    île.add(f);
  }
  [[-3.8, -1.6, 0.8], [-3.2, -2.9, 0.6], [4.0, 1.6, 0.7], [1.0, -3.6, 0.55]]
    .forEach(([x, z, h]) => { const a = sapin(h); a.position.set(x, y, z); île.add(a); });
  // Une libellule (deux ailes) qui fait des ronds au premier plan.
  const libellule = new THREE.Group();
  const aile = (sx) => { const m = bloc(new THREE.ConeGeometry(0.2, 0.9, 3), 0xbfeeff, mat); m.rotation.z = sx * Math.PI / 2; m.position.x = sx * 0.4; return m; };
  libellule.add(bloc(new THREE.CylinderGeometry(0.06, 0.06, 0.8, 4), C.blue), aile(1), aile(-1));
  devant.add(libellule);
  anim((t) => {
    libellule.position.set(Math.cos(t * 0.8) * 5, 4 + Math.sin(t * 1.7) * 0.6, Math.sin(t * 0.8) * 3 + 2);
    libellule.rotation.y = -t * 0.8;
    libellule.children[1].rotation.x = libellule.children[2].rotation.x = Math.sin(t * 30) * 0.5;
  });
  const fête = decorer(île, y, 5.4, devant);
  if (fête) anim(fête);
  anim((t) => { île.rotation.y = -0.4 + Math.sin(t * 0.2) * 0.15; île.position.y = Math.sin(t * 0.7) * 0.2; });
}

// ---- Nova : planète, anneau, étoiles et vaisseaux -----------------------
function nova(s) {
  const plans = [[-8, 900, 0.35, 120], [-3, 400, 0.6, 80], [1, 120, 1.0, 50]];
  plans.forEach(([k, n, taille, loin]) => {
    const pts = new Float32Array(n * 3), cols = new Float32Array(n * 3);
    const palette = [C.ivory, C.sky, C.lemon, 0xffffff].map((c) => new THREE.Color(c));
    for (let i = 0; i < n; i++) {
      pts.set([entre(-loin, loin), entre(-loin * 0.6, loin * 0.6), entre(-loin, -10)], i * 3);
      const c = palette[Math.floor(alea() * palette.length)];
      cols.set([c.r, c.g, c.b], i * 3);
    }
    const geo = new THREE.BufferGeometry();
    geo.setAttribute("position", new THREE.BufferAttribute(pts, 3));
    geo.setAttribute("color", new THREE.BufferAttribute(cols, 3));
    s.couche(k).add(new THREE.Points(geo, new THREE.PointsMaterial({ size: taille, vertexColors: true, sizeAttenuation: true })));
  });
  const milieu = s.couche(4), devant = s.couche(9);
  const planète = new THREE.Group();
  milieu.add(planète);
  const boule = bloc(new THREE.IcosahedronGeometry(4.2, 1), (n) => {
    const lat = n.y;
    if (lat > 0.75) return C.ivory;
    return alea() < 0.3 ? mix(C.blue, C.sky, 0.5) : alea() < 0.5 ? C.blue : mix(C.blue, C.night, 0.35);
  });
  planète.add(boule);
  const anneau = bloc(new THREE.TorusGeometry(6.6, 0.32, 3, 28), (n) => (n.y > 0 ? C.lemon : mix(C.lemon, C.green, 0.4)));
  anneau.rotation.set(Math.PI / 2 - 0.35, 0.2, 0);
  planète.add(anneau);
  planète.rotation.z = 0.25;
  const lune = bloc(new THREE.IcosahedronGeometry(0.9, 0), (n) => (n.y > 0 ? 0xc9d8e4 : 0x8aa0b4));
  milieu.add(lune);
  anim((t) => {
    boule.rotation.y = t * 0.08;
    lune.position.set(Math.cos(t * 0.3) * 9, 1.5 + Math.sin(t * 0.3) * 1.2, Math.sin(t * 0.3) * 6);
  });
  // Vaisseaux : un nez en pointe, deux ailerons, une flamme.
  const vaisseau = (couleur) => {
    const g = new THREE.Group();
    const coque = bloc(new THREE.ConeGeometry(0.45, 1.8, 5), (n) => (n.y > 0 ? 0xffffff : couleur));
    coque.rotation.z = -Math.PI / 2;
    const ailes = bloc(new THREE.BoxGeometry(0.7, 0.08, 1.6), couleur);
    ailes.position.x = -0.3;
    const flamme = new THREE.Mesh(new THREE.ConeGeometry(0.25, 0.8, 5), new THREE.MeshBasicMaterial({ color: C.coral }));
    flamme.rotation.z = Math.PI / 2;
    flamme.position.x = -1.25;
    g.add(coque, ailes, flamme);
    g.userData.flamme = flamme;
    return g;
  };
  [[C.sky, 0, 1.0, 5], [C.coral, 7, 0.7, -3], [C.lemon, 13, 0.85, 2]].forEach(([col, décalage, v, yy]) => {
    const n = vaisseau(col);
    devant.add(n);
    anim((t) => {
      const p = ((t * v + décalage) % 24) / 24;
      n.position.set(-26 + p * 52, yy + Math.sin(t + décalage) * 0.6, 4 - décalage * 0.4);
      n.userData.flamme.scale.x = 0.8 + Math.sin(t * 25) * 0.25;
    });
  });
}

// ---- Eurêka : tuiles, atome, sceau et fiole ------------------------------
function tuile(nombre, fond, encre) {
  const toile = document.createElement("canvas");
  toile.width = toile.height = 256;
  const g = toile.getContext("2d");
  g.fillStyle = "#" + new THREE.Color(fond).getHexString();
  g.fillRect(0, 0, 256, 256);
  g.fillStyle = encre;
  g.font = `bold ${nombre >= 1000 ? 84 : nombre >= 100 ? 108 : 140}px "Russo One", system-ui, sans-serif`;
  g.textAlign = "center";
  g.textBaseline = "middle";
  g.fillText(String(nombre), 128, 138);
  const tex = new THREE.CanvasTexture(toile);
  tex.colorSpace = THREE.SRGBColorSpace;
  const côté = new THREE.MeshLambertMaterial({ color: mix(fond, 0x000000, 0.18), flatShading: true });
  const dessus = new THREE.MeshLambertMaterial({ color: mix(fond, 0xffffff, 0.2), flatShading: true });
  const face = new THREE.MeshBasicMaterial({ map: tex });
  return new THREE.Mesh(new THREE.BoxGeometry(2, 2, 0.7), [côté, côté, dessus, côté, face, côté]);
}

function eureka(s) {
  const loin = s.couche(-3), milieu = s.couche(3), devant = s.couche(8);
  [[2, 0xfff1d6, "#142C40", -5, 3, 0], [8, 0xffb38a, "#FFF9ED", -2, -1.5, 1], [64, C.coral, "#FFF9ED", 3.5, 4.5, -1],
   [2048, C.blue, "#FFF9ED", 4.5, -2, 1.5], [16, C.green, "#FFF9ED", 0.5, 7, -3]].forEach(([n, f, e, x, y, z], i) => {
    const m = tuile(n, f, e);
    m.position.set(x, y, z);
    m.rotation.set(entre(-0.3, 0.3), entre(-0.5, 0.2), entre(-0.25, 0.25));
    (i % 2 ? devant : milieu).add(m);
    flotter(m, 0.4, 0.7 + i * 0.13, i);
    const r0 = m.rotation.y;
    anim((t) => { m.rotation.y = r0 + Math.sin(t * 0.5 + i) * 0.15; });
  });
  // Atome : noyau à facettes, trois orbites et leurs électrons.
  const atome = new THREE.Group();
  atome.position.set(-1, 2, -1);
  milieu.add(atome);
  atome.add(bloc(new THREE.IcosahedronGeometry(0.8, 0), () => (alea() < 0.5 ? C.coral : C.blue)));
  const électrons = [0, 1, 2].map((i) => {
    const orbite = new THREE.Group();
    orbite.rotation.set(Math.PI / 2 + i * 1.05, i * 0.6, 0);
    orbite.add(new THREE.Mesh(new THREE.TorusGeometry(2.6, 0.04, 3, 40), new THREE.MeshBasicMaterial({ color: i === 1 ? C.lemon : C.sky })));
    const e = bloc(new THREE.IcosahedronGeometry(0.22, 0), C.lemon);
    orbite.add(e);
    atome.add(orbite);
    return e;
  });
  anim((t) => {
    électrons.forEach((e, i) => { const a = t * (1.2 + i * 0.3) + i * 2; e.position.set(Math.cos(a) * 2.6, Math.sin(a) * 2.6, 0); });
    atome.rotation.y = t * 0.3;
  });
  // Sceau de cire du Concile.
  const sceau = new THREE.Group();
  sceau.add(bloc(new THREE.CylinderGeometry(1.1, 1.2, 0.35, 10), (n) => (n.y > 0.5 ? C.coral : 0xc9302a)));
  const mitre = bloc(new THREE.ConeGeometry(0.45, 0.8, 4), C.ivory);
  mitre.position.y = 0.55;
  sceau.add(mitre);
  sceau.position.set(-6, -2.5, 1);
  sceau.rotation.x = 0.9;
  devant.add(sceau);
  flotter(sceau, 0.35, 0.9, 2);
  anim((t) => { sceau.rotation.z = Math.sin(t * 0.6) * 0.3; });
  // Fiole d'Erlenmeyer (verre à facettes, liquide vert).
  const fiole = new THREE.Group();
  const profil = [[0, -1.4], [1.3, -1.4], [1.35, -1.2], [0.42, 0.5], [0.42, 1.3], [0.55, 1.4]].map(([x, y]) => new THREE.Vector2(x, y));
  const verre = new THREE.Mesh(new THREE.LatheGeometry(profil, 7), new THREE.MeshLambertMaterial({ color: 0xdff4ff, transparent: true, opacity: 0.45, flatShading: true, side: THREE.DoubleSide }));
  const liquide = bloc(new THREE.CylinderGeometry(0.75, 1.25, 0.9, 7), (n) => (n.y > 0.5 ? C.lemon : C.green));
  liquide.position.y = -0.9;
  fiole.add(liquide, verre);
  fiole.position.set(7, 3, -4);
  loin.add(fiole);
  flotter(fiole, 0.3, 0.6, 4);
  anim((t) => { fiole.rotation.z = Math.sin(t * 0.5) * 0.15; });
}

// ---- Légendes : le château sur son rocher --------------------------------
function legendes(s) {
  const loin = s.couche(-4), milieu = s.couche(3), devant = s.couche(9);
  nuagesLointains(loin, 5);
  // Montagnes enneigées au loin.
  [[-14, -20, 7], [-6, -26, 10], [6, -24, 8], [15, -22, 6]].forEach(([x, z, h]) => {
    const m = bloc(new THREE.ConeGeometry(h * 0.8, h, 5), (n, c) => (c.y > h * 0.18 ? 0xffffff : alea() < 0.5 ? 0x8f98a5 : 0x7d8794));
    m.position.set(x, h / 2 - 4, z);
    m.rotation.y = alea() * 3;
    loin.add(m);
  });
  const île = new THREE.Group();
  milieu.add(île);
  const sol = ile(5.4);
  île.add(sol);
  const y = sol.userData.dessus;
  const pierre = (n) => (n.y > 0.5 ? 0xd8d2c9 : alea() < 0.5 ? 0xb6aea4 : 0xa9a197);
  const muraille = bloc(new THREE.BoxGeometry(4.6, 1.8, 3.2), pierre);
  muraille.position.set(0, y + 0.9, -0.4);
  île.add(muraille);
  const porte = new THREE.Mesh(new THREE.BoxGeometry(0.9, 1.1, 0.1), new THREE.MeshBasicMaterial({ color: C.night }));
  porte.position.set(0, y + 0.55, 1.21);
  île.add(porte);
  const drapeaux = [];
  [[-2.3, 1.2, 3.4], [2.3, 1.2, 3.0], [-2.3, -2.0, 2.8], [2.3, -2.0, 3.2], [0, -0.6, 4.6]].forEach(([x, z, h]) => {
    const tour = bloc(new THREE.CylinderGeometry(0.75, 0.85, h, 6), pierre);
    tour.position.set(x, y + h / 2, z);
    const toit = bloc(new THREE.ConeGeometry(1.05, 1.5, 6), (n) => (n.x > 0 ? C.coral : 0xd63a30));
    toit.position.set(x, y + h + 0.75, z);
    const mât = new THREE.Mesh(new THREE.BoxGeometry(0.08, 0.9, 0.08), new THREE.MeshBasicMaterial({ color: C.night }));
    mât.position.set(x, y + h + 1.9, z);
    const d = drapeau(C.coral, 0.9, 0.55);
    d.position.set(x + 0.04, y + h + 2.25, z);
    drapeaux.push(d.userData.onduler);
    île.add(tour, toit, mât, d);
  });
  [[-4.2, 0.8, 0.7], [4.1, -0.5, 0.8], [3.4, 2.6, 0.6], [-3.6, -2.6, 0.6]]
    .forEach(([x, z, h]) => { const a = sapin(h); a.position.set(x, y, z); île.add(a); });
  [[-1.8, 3.4, 0.6], [2.6, 3.6, 0.5]].forEach(([x, z, t]) => { const r = rocher(t); r.position.set(x, y + 0.1, z); île.add(r); });
  // Le dragon : un corps en pointe et deux ailes qui battent.
  const dragon = new THREE.Group();
  const corps = bloc(new THREE.ConeGeometry(0.45, 2.6, 5), (n) => (n.y > 0 ? C.green : 0x5a9a1c));
  corps.rotation.x = Math.PI / 2;
  const tête = bloc(new THREE.ConeGeometry(0.35, 0.8, 4), C.green);
  tête.rotation.x = Math.PI / 2;
  tête.position.z = 1.6;
  const aile = (sx) => {
    const g = new THREE.Group();
    const m = bloc(new THREE.ConeGeometry(0.9, 2.2, 3), C.coral, mat);
    m.rotation.z = sx * Math.PI / 2;
    m.position.x = sx * 1.1;
    m.scale.z = 0.15;
    g.add(m);
    return g;
  };
  const ailes = [aile(1), aile(-1)];
  dragon.add(corps, tête, ...ailes);
  devant.add(dragon);
  anim((t) => {
    const a = t * 0.45;
    dragon.position.set(Math.cos(a) * 9, 6 + Math.sin(t * 0.9) * 0.8, Math.sin(a) * 6);
    dragon.rotation.y = -a + Math.PI;
    ailes[0].rotation.z = Math.sin(t * 6) * 0.5;
    ailes[1].rotation.z = -Math.sin(t * 6) * 0.5;
    drapeaux.forEach((f) => f(t));
  });
  const fête = decorer(île, y, 5.4, devant);
  if (fête) anim(fête);
  anim((t) => { île.rotation.y = -0.5 + Math.sin(t * 0.2) * 0.12; île.position.y = Math.sin(t * 0.7) * 0.2; });
}
