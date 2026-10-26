// Visionneuse de personnages de Traversée : les humains du Mini Characters de Kenney (CC0), que l'on peut
// recolorier (cases de la palette) et habiller (pièces accrochées aux os du squelette).
// Les personnages enregistrés vont dans games/traversee/data/personnages/<id>.json.
import * as THREE from 'three';
import { OrbitControls } from './vendor/OrbitControls.js';
import { GLTFLoader } from './vendor/GLTFLoader.js';

const BASES = ['a', 'b', 'c', 'd', 'e', 'f'].flatMap((l) => [`character-male-${l}`, `character-female-${l}`]);
const DOSSIER = '/kenney/mini-characters/';
const PARTIES = ['peau', 'cheveux', 'yeux', 'haut', 'bas', 'chaussures', 'costume', 'autre'];
const OS = ['root', 'torso', 'head', 'arm-left', 'arm-right', 'leg-left', 'leg-right'];
const FORMES = ['cube', 'sphere', 'cone', 'cylindre'];
const GRILLE = 32; // la palette (512×512) est découpée en cases de 16 px

const $ = (s) => document.querySelector(s);
const statut = (m, ok = true) => { const e = $('#statut'); e.textContent = m; e.className = ok ? 'ok' : 'mal'; clearTimeout(statut.t); statut.t = setTimeout(() => (e.textContent = ''), 3500); };
const hex = (c) => '#' + c.map((v) => v.toString(16).padStart(2, '0')).join('');
const rgb = (h) => [1, 3, 5].map((i) => parseInt(h.slice(i, i + 2), 16));

// ---- Scène ----
const toile = $('#toile');
const renderer = new THREE.WebGLRenderer({ canvas: toile, antialias: true, preserveDrawingBuffer: true });
renderer.setPixelRatio(Math.min(devicePixelRatio, 2));
const scene = new THREE.Scene();
const fonds = [0x3a4260, 0xdfe6f0, 0x1b2130];
let fondN = 0;
scene.background = new THREE.Color(fonds[0]);
const camera = new THREE.PerspectiveCamera(32, 1, 0.05, 50);
const controles = new OrbitControls(camera, toile);
controles.target.set(0, 0.33, 0);
controles.enableDamping = true;
scene.add(new THREE.HemisphereLight(0xffffff, 0x556070, 1.3));
const soleil = new THREE.DirectionalLight(0xffffff, 2.2);
soleil.position.set(1.5, 3, 2.5);
scene.add(soleil);
const sol = new THREE.Mesh(new THREE.CylinderGeometry(0.5, 0.5, 0.02, 48), new THREE.MeshStandardMaterial({ color: 0x8a93a8, roughness: 1 }));
sol.position.y = -0.011;
scene.add(sol);

function vue(nom) {
  const d = 1.6, h = 0.34;
  const p = { face: [0, h, d], profil: [d, h, 0], dos: [0, h, -d], haut: [0, d, 0.01] }[nom] || [1.0, 0.75, 1.2];
  camera.position.set(...p);
  controles.target.set(0, 0.33, 0);
  $('#b-tourner').classList.remove('actif');
  tourner = false;
}
let tourner = true;
vue('iso'); tourner = true; $('#b-tourner').classList.add('actif');
camera.position.set(0.9, 0.7, 1.35);

function ajuster() {
  const c = $('#centre');
  renderer.setSize(c.clientWidth, c.clientHeight, false);
  camera.aspect = c.clientWidth / c.clientHeight;
  camera.updateProjectionMatrix();
}
addEventListener('resize', ajuster);
ajuster();

// ---- État du personnage édité ----
let spec = nouveauSpec(BASES[0]);
let cartes = {}; // base -> { cellule -> partie }
let sauvegardes = [];
let modele = null; // { racine, mixer, anims, os, cases, canvasOrig, canvas, texture }
let pieceSel = -1;

function nouveauSpec(base) {
  return { id: '', nom: '', base, parties: {}, cellules: {}, pieces: [] };
}

// ---- Chargement du modèle ----
async function chargerPalette() {
  const img = new Image();
  img.src = DOSSIER + 'Textures/colormap.png';
  await img.decode();
  const c = document.createElement('canvas');
  c.width = img.width; c.height = img.height;
  c.getContext('2d').drawImage(img, 0, 0);
  return c;
}

let palette = null;
async function charger(base) {
  if (!palette) palette = await chargerPalette();
  if (modele) { scene.remove(modele.racine); }
  const gltf = await new GLTFLoader().loadAsync(DOSSIER + base + '.glb');
  const racine = gltf.scene;
  scene.add(racine);
  // Les cases de la palette utilisées par ce modèle
  const cases = new Map();
  const maillages = [];
  racine.traverse((o) => {
    if (!o.isMesh) return;
    maillages.push(o);
    const uv = o.geometry.attributes.uv, pos = o.geometry.attributes.position;
    for (let i = 0; i < uv.count; i++) {
      const cx = Math.min(GRILLE - 1, Math.floor(uv.getX(i) * GRILLE)), cy = Math.min(GRILLE - 1, Math.floor(uv.getY(i) * GRILLE));
      const k = `${cx},${cy}`;
      const c = cases.get(k) || { k, cx, cy, n: 0, ys: 0 };
      c.n++; c.ys += pos.getY(i);
      cases.set(k, c);
    }
  });
  const canvas = document.createElement('canvas');
  canvas.width = palette.width; canvas.height = palette.height;
  const original = maillages[0].material.map;
  const texture = original.clone();
  texture.image = canvas;
  texture.needsUpdate = true;
  for (const m of maillages) { m.material.map = texture; m.material.needsUpdate = true; }
  const os = {};
  for (const n of OS) os[n] = racine.getObjectByName(n) || null;
  const mixer = new THREE.AnimationMixer(racine);
  modele = { racine, mixer, anims: gltf.animations, os, cases: [...cases.values()].sort((a, b) => b.ys / b.n - a.ys / a.n), canvas, texture, survol: null, action: null };
  const sel = $('#anim');
  sel.innerHTML = gltf.animations.map((a, i) => `<option value="${i}">${a.name}</option>`).join('');
  const idle = gltf.animations.findIndex((a) => a.name === 'idle');
  sel.value = String(idle >= 0 ? idle : 0);
  lancerAnim();
  repeindre();
  construirePieces();
  panneauCases();
  panneauParties();
}

function lancerAnim() {
  if (!modele) return;
  if (modele.action) modele.action.stop();
  const a = modele.anims[Number($('#anim').value)];
  if (!a) return;
  modele.action = modele.mixer.clipAction(a);
  modele.action.play();
}

// ---- Couleurs ----
function partieDe(k) { return (cartes[spec.base] || {})[k]; }

function luminance(h) { const [r, g, b] = rgb(h); return 0.3 * r + 0.59 * g + 0.11 * b; }

// Une partie a plusieurs cases de nuances différentes (ombres) : la couleur choisie est celle de la case la plus claire,
// les autres cases gardent leur écart de luminosité d'origine.
function nuancer(partie, h, k) {
  const cases = modele.cases.filter((c) => partieDe(c.k) === partie);
  const lums = cases.map((c) => luminance(couleurOrigine(c)));
  const ref = Math.max(...lums, 1);
  // Les nuances d'origine sont adoucies de moitié : sinon les ombres (sous le menton…) font des taches grises.
  const ratio = Math.min(1.15, luminance(couleurOrigine(cases.find((c) => c.k === k))) / ref);
  const f = 1 - (1 - ratio) * 0.45;
  return hex(rgb(h).map((v) => Math.max(0, Math.min(255, Math.round(v * f)))));
}

function couleurCase(k) {
  if (spec.cellules[k]) return spec.cellules[k];
  const p = partieDe(k);
  if (p && spec.parties[p]) return nuancer(p, spec.parties[p], k);
  return null;
}

function couleurOrigine(c) {
  const d = palette.getContext('2d').getImageData(c.cx * 16 + 8, c.cy * 16 + 8, 1, 1).data;
  return hex([d[0], d[1], d[2]]);
}

function repeindre() {
  if (!modele) return;
  const ctx = modele.canvas.getContext('2d');
  ctx.drawImage(palette, 0, 0);
  for (const c of modele.cases) {
    const h = couleurCase(c.k);
    if (h) { ctx.fillStyle = h; ctx.fillRect(c.cx * 16, c.cy * 16, 16, 16); }
  }
  if (modele.survol) {
    const [cx, cy] = modele.survol.split(',').map(Number);
    ctx.fillStyle = '#ff2fd0';
    ctx.fillRect(cx * 16, cy * 16, 16, 16);
  }
  modele.texture.needsUpdate = true;
}

function panneauCases() {
  const e = $('#cases');
  e.innerHTML = '';
  $('#nb-cases').textContent = `(${modele.cases.length})`;
  for (const c of modele.cases) {
    const ligne = document.createElement('div');
    ligne.className = 'case';
    const val = couleurCase(c.k) || couleurOrigine(c);
    ligne.innerHTML = `<input type="color" value="${val}"><small>${c.k} · ${c.n} pts · h ${(c.ys / c.n).toFixed(2)}</small><select>${['—', ...PARTIES].map((p) => `<option ${((partieDe(c.k) || '—') === p) ? 'selected' : ''}>${p}</option>`).join('')}</select>`;
    ligne.addEventListener('mouseenter', () => { modele.survol = c.k; repeindre(); });
    ligne.addEventListener('mouseleave', () => { modele.survol = null; repeindre(); });
    ligne.querySelector('input').addEventListener('input', (ev) => { spec.cellules[c.k] = ev.target.value; repeindre(); });
    ligne.querySelector('select').addEventListener('change', (ev) => {
      cartes[spec.base] = cartes[spec.base] || {};
      if (ev.target.value === '—') delete cartes[spec.base][c.k]; else cartes[spec.base][c.k] = ev.target.value;
      enregistrerCartes();
      panneauParties();
    });
    e.appendChild(ligne);
  }
}

function panneauParties() {
  const e = $('#parties');
  e.innerHTML = '';
  const carte = cartes[spec.base] || {};
  const utilisees = PARTIES.filter((p) => Object.values(carte).includes(p));
  $('#aide-parties').textContent = utilisees.length ? '' : '(aucune case rangée : voir plus bas)';
  for (const p of utilisees) {
    const premiere = modele.cases.find((c) => carte[c.k] === p);
    const val = spec.parties[p] || (premiere ? couleurOrigine(premiere) : '#ffffff');
    const l = document.createElement('div');
    l.className = 'ligne';
    l.innerHTML = `<label>${p}</label><input type="color" value="${val}"><span class="aide">${modele.cases.filter((c) => carte[c.k] === p).length} cases</span>`;
    l.querySelector('input').addEventListener('input', (ev) => {
      spec.parties[p] = ev.target.value;
      for (const c of modele.cases) if (carte[c.k] === p) delete spec.cellules[c.k];
      repeindre();
      panneauCases();
    });
    e.appendChild(l);
  }
}

let enregistrerCartes = () => {
  clearTimeout(enregistrerCartes.t);
  enregistrerCartes.t = setTimeout(() => fetch('/api/personnage-cartes', { method: 'POST', body: JSON.stringify(cartes) }), 300);
};

// ---- Pièces accrochées aux os ----
const geometries = {
  cube: new THREE.BoxGeometry(1, 1, 1),
  sphere: new THREE.SphereGeometry(0.5, 20, 14),
  cone: new THREE.ConeGeometry(0.5, 1, 14),
  cylindre: new THREE.CylinderGeometry(0.5, 0.5, 1, 20),
};
let meshesPieces = [];

function construirePieces() {
  for (const m of meshesPieces) m.removeFromParent();
  meshesPieces = [];
  spec.pieces.forEach((p, i) => {
    const os = modele.os[p.os] || modele.racine;
    const m = new THREE.Mesh(geometries[p.forme] || geometries.cube, new THREE.MeshStandardMaterial({ color: p.couleur || '#ffffff', roughness: 0.85, emissive: i === pieceSel ? 0x553300 : 0x000000 }));
    m.position.set(...p.pos);
    m.scale.set(...p.taille);
    m.rotation.set(...p.rot.map((d) => (d * Math.PI) / 180));
    os.add(m);
    meshesPieces.push(m);
  });
  panneauPieces();
}

function panneauPieces() {
  const e = $('#pieces');
  e.innerHTML = '';
  $('#nb-pieces').textContent = `(${spec.pieces.length})`;
  spec.pieces.forEach((p, i) => {
    const l = document.createElement('div');
    l.className = 'piece' + (i === pieceSel ? ' sel' : '');
    l.innerHTML = `<span>${i}</span><span>${p.nom || p.forme}</span><span>${p.os}</span>`;
    l.addEventListener('click', () => { pieceSel = i; construirePieces(); formulaire(); });
    e.appendChild(l);
  });
}

function formulaire() {
  const e = $('#form');
  const p = spec.pieces[pieceSel];
  if (!p) { e.innerHTML = '<span class="aide">Choisissez une pièce dans la liste.</span>'; return; }
  const nombres = (cle, pas) => p[cle].map((v, j) => `<input type="number" step="${pas}" value="${v}" data-c="${cle}" data-j="${j}">`).join('');
  e.innerHTML = `
    <div class="ligne"><label>Nom</label><input data-t="nom" value="${p.nom || ''}" style="flex:1"></div>
    <div class="ligne"><label>Os</label><select data-t="os">${OS.map((o) => `<option ${o === p.os ? 'selected' : ''}>${o}</option>`).join('')}</select>
      <select data-t="forme">${FORMES.map((f) => `<option ${f === p.forme ? 'selected' : ''}>${f}</option>`).join('')}</select>
      <input type="color" data-t="couleur" value="${p.couleur || '#ffffff'}"></div>
    <div class="ligne"><label>Position</label>${nombres('pos', 0.01)}</div>
    <div class="ligne"><label>Taille</label>${nombres('taille', 0.01)}</div>
    <div class="ligne"><label>Rotation °</label>${nombres('rot', 5)}</div>`;
  e.querySelectorAll('input[data-c]').forEach((i) => i.addEventListener('input', () => { p[i.dataset.c][Number(i.dataset.j)] = Number(i.value); construirePieces(); }));
  e.querySelectorAll('[data-t]').forEach((i) => i.addEventListener('input', () => { p[i.dataset.t] = i.value; construirePieces(); }));
}

$('#b-ajouter').onclick = () => { spec.pieces.push({ nom: 'pièce', os: 'head', forme: 'cube', pos: [0, 0.1, 0], taille: [0.1, 0.1, 0.1], rot: [0, 0, 0], couleur: '#ffcc44' }); pieceSel = spec.pieces.length - 1; construirePieces(); formulaire(); };
$('#b-dupliquer').onclick = () => { const p = spec.pieces[pieceSel]; if (!p) return; spec.pieces.push(JSON.parse(JSON.stringify(p))); pieceSel = spec.pieces.length - 1; construirePieces(); formulaire(); };
$('#b-supprimer').onclick = () => { if (pieceSel < 0) return; spec.pieces.splice(pieceSel, 1); pieceSel = -1; construirePieces(); formulaire(); };
$('#b-miroir').onclick = () => {
  const p = spec.pieces[pieceSel]; if (!p) return;
  const q = JSON.parse(JSON.stringify(p));
  q.pos[0] = -q.pos[0]; q.rot[2] = -q.rot[2]; q.rot[1] = -q.rot[1];
  if (q.os === 'arm-left') q.os = 'arm-right'; else if (q.os === 'arm-right') q.os = 'arm-left';
  if (q.os === 'leg-left') q.os = 'leg-right'; else if (q.os === 'leg-right') q.os = 'leg-left';
  spec.pieces.push(q); pieceSel = spec.pieces.length - 1; construirePieces(); formulaire();
};

// ---- Liste, enregistrement ----
function liste() {
  const e = $('#liste');
  e.innerHTML = '<div class="groupe">Enregistrés</div>';
  for (const s of sauvegardes) {
    const d = document.createElement('div');
    d.className = 'item' + (spec.id === s.id ? ' courant' : '');
    d.textContent = s.nom || s.id;
    d.onclick = async () => { spec = JSON.parse(JSON.stringify(s)); pieceSel = -1; await charger(spec.base); remplirIdentite(); liste(); };
    e.appendChild(d);
  }
  e.insertAdjacentHTML('beforeend', '<div class="groupe">Nouveau, à partir de…</div>');
  for (const b of BASES) {
    const d = document.createElement('div');
    d.className = 'item' + (spec.id === '' && spec.base === b ? ' courant' : '');
    d.textContent = b.replace('character-', '');
    d.onclick = async () => { spec = nouveauSpec(b); pieceSel = -1; await charger(b); remplirIdentite(); liste(); };
    e.appendChild(d);
  }
}

function remplirIdentite() {
  $('#nom').value = spec.nom || '';
  $('#ident').value = spec.id || '';
  $('#base').textContent = spec.base;
  $('#titre').textContent = (spec.nom || spec.base) + ' — ' + spec.base;
  formulaire();
}

$('#nom').oninput = () => { spec.nom = $('#nom').value; };
$('#ident').oninput = () => { spec.id = $('#ident').value; };
$('#b-reinit').onclick = async () => { const b = spec.base; spec = nouveauSpec(b); pieceSel = -1; await charger(b); remplirIdentite(); liste(); };
$('#b-sauver').onclick = async () => {
  if (!spec.id) { statut('Donnez un identifiant (en haut à droite).', false); return; }
  // Couleurs finales de chaque case modifiée (c'est ce que le jeu lira), plus les parties pour pouvoir rééditer.
  const finales = {};
  for (const c of modele.cases) { const h = couleurCase(c.k); if (h) finales[c.k] = h; }
  const r = await fetch('/api/personnage-sauver', { method: 'POST', body: JSON.stringify({ ...spec, couleurs: finales }) });
  if (r.ok) { statut('Enregistré : games/traversee/data/personnages/' + spec.id + '.json'); sauvegardes = await (await fetch('/api/personnages')).json(); liste(); }
  else statut('Échec de l\'enregistrement', false);
};

// ---- Barre ----
$('#anim').onchange = lancerAnim;
$('#b-tourner').onclick = () => { tourner = !tourner; $('#b-tourner').classList.toggle('actif', tourner); };
document.querySelectorAll('[data-vue]').forEach((b) => (b.onclick = () => vue(b.dataset.vue)));
$('#b-fond').onclick = () => { fondN = (fondN + 1) % fonds.length; scene.background = new THREE.Color(fonds[fondN]); };
$('#b-capture').onclick = () => { const a = document.createElement('a'); a.download = (spec.id || spec.base) + '.png'; a.href = toile.toDataURL('image/png'); a.click(); };

// ---- Boucle ----
const horloge = new THREE.Clock();
let lastW = 0, lastH = 0;
function boucle() {
  requestAnimationFrame(boucle);
  const c = $('#centre');
  if (toile.clientWidth !== lastW || toile.clientHeight !== lastH) { lastW = toile.clientWidth; lastH = toile.clientHeight; ajuster(); }
  const dt = horloge.getDelta();
  if (modele) { modele.mixer.update(dt); if (tourner) modele.racine.rotation.y += dt * 0.8; }
  controles.update();
  renderer.render(scene, camera);
}

// ---- Démarrage (adresse : ?perso=<id> ou ?base=<modele>, &anim=<nom>, &rot=<degres>, &vue=face) ----
(async function () {
 try {
  try { cartes = await (await fetch('personnages-cartes.json', { cache: 'no-store' })).json(); } catch { cartes = {}; }
  sauvegardes = await (await fetch('/api/personnages')).json();
  const q = new URLSearchParams(location.search);
  const voulu = sauvegardes.find((s) => s.id === q.get('perso'));
  spec = voulu ? JSON.parse(JSON.stringify(voulu)) : nouveauSpec(BASES.includes(q.get('base')) ? q.get('base') : BASES[0]);
  await charger(spec.base);
  if (q.get('anim')) { const i = modele.anims.findIndex((a) => a.name === q.get('anim')); if (i >= 0) { $('#anim').value = String(i); lancerAnim(); } }
  if (q.get('vue')) vue(q.get('vue'));
  if (q.has('rot')) { tourner = false; $('#b-tourner').classList.remove('actif'); modele.racine.rotation.y = (Number(q.get('rot')) * Math.PI) / 180; }
  if (q.get('fond')) { fondN = Number(q.get('fond')) % fonds.length; scene.background = new THREE.Color(fonds[fondN]); }
  remplirIdentite();
  liste();
  boucle();
 } catch (e) { console.error(e); statut('Erreur : ' + e.message, false); document.title = 'ERREUR ' + e.message; }
})();

// Pour les essais en console et les captures : window.perso
window.perso = { get spec() { return spec; }, get modele() { return modele; }, get cartes() { return cartes; }, repeindre, panneauCases, panneauParties, construirePieces };
