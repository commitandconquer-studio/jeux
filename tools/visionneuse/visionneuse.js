// Commit & Conquer — licence GPL-3.0-or-later.
// Visionneuse / éditeur 3D des modèles de Traversée (voir tools/visionneuse.py).
import * as THREE from 'three';
import { OrbitControls } from './vendor/OrbitControls.js';

const $ = (s) => document.querySelector(s);
const FORMES = ['cube', 'cylindre', 'cone', 'sphere'];
const MATIERES = ['peinture', 'couleur_2', 'couleur_3', 'vitre', 'sombre', 'pneu', 'enjoliveur', 'phare', 'feu_arriere', 'lumiere_rouge', 'lumiere_bleue', 'blanc', 'noir', 'bulle'];
const COULEURS = { vitre: '#42617f', sombre: '#33363d', pneu: '#1d1d20', enjoliveur: '#c8ccd4', phare: '#fff1b0', feu_arriere: '#d63a2a', lumiere_rouge: '#ff3b30', lumiere_bleue: '#3b8cff', blanc: '#f4f4ee', noir: '#16161a', bulle: '#bfe9ff' };
const LUMINEUSES = new Set(['phare', 'lumiere_rouge', 'lumiere_bleue']);
const VERT_HEROS = '#5aaa46', PEINTURE_VEHICULE = '#3b82d6';

let modeles = [], courant = null, travail = null, selection = -1, sale = false, pile = [], mtimes = {}, parUrl = new URLSearchParams(location.search);
let anime = parUrl.get('anim') !== '0', ortho = parUrl.get('ortho') === '1', grille = true, ref = parUrl.get('ref') === '1', fondSombre = false, vueActuelle = 'iso';

// ---------------------------------------------------------------- scène
const toile = $('#toile');
const rendu = new THREE.WebGLRenderer({ canvas: toile, antialias: true, preserveDrawingBuffer: true });
rendu.shadowMap.enabled = true;
const scene = new THREE.Scene();
scene.background = new THREE.Color(0xd6dde6);
scene.add(new THREE.HemisphereLight(0xffffff, 0x8a94a6, 1.15));
const soleil = new THREE.DirectionalLight(0xffffff, 1.6);
soleil.position.set(3, 8, 5);
soleil.castShadow = true;
soleil.shadow.mapSize.set(2048, 2048);
Object.assign(soleil.shadow.camera, { left: -8, right: 8, top: 8, bottom: -8, near: 0.5, far: 30 });
scene.add(soleil);
const sol = new THREE.Mesh(new THREE.PlaneGeometry(40, 40), new THREE.MeshStandardMaterial({ color: 0xc3ccd6, roughness: 1 }));
sol.rotation.x = -Math.PI / 2;
sol.position.y = -0.001;
sol.receiveShadow = true;
scene.add(sol);
const grilleObj = new THREE.GridHelper(40, 40, 0x7b8798, 0xaab4c2);
grilleObj.position.y = 0.002;
scene.add(grilleObj);
const fleche = new THREE.ArrowHelper(new THREE.Vector3(1, 0, 0), new THREE.Vector3(0, 0.02, 0), 2.4, 0xe04040, 0.35, 0.2);
scene.add(fleche);
const groupeModele = new THREE.Group();
scene.add(groupeModele);
const crapaud = new THREE.Mesh(new THREE.BoxGeometry(0.6, 0.45, 0.55), new THREE.MeshStandardMaterial({ color: 0x5aaa46, roughness: 1 }));
crapaud.position.set(-3.2, 0.225, 0);
crapaud.castShadow = true;
scene.add(crapaud);

const camPerspective = new THREE.PerspectiveCamera(32, 1, 0.05, 200);
const camOrtho = new THREE.OrthographicCamera(-4, 4, 3, -3, -50, 100);
let camera = camPerspective;
const controles = new OrbitControls(camera, toile);
controles.enableDamping = true;
controles.target.set(0, 0.6, 0);

function redimensionner() {
  const r = $('#centre').getBoundingClientRect();
  rendu.setSize(r.width, r.height, false);
  camPerspective.aspect = r.width / r.height;
  camPerspective.updateProjectionMatrix();
  ajusterOrtho();
}
function ajusterOrtho() {
  const r = $('#centre').getBoundingClientRect();
  const h = 3.2 / camOrtho.zoom, w = h * (r.width / r.height);
  Object.assign(camOrtho, { left: -w, right: w, top: h, bottom: -h });
  camOrtho.updateProjectionMatrix();
}
addEventListener('resize', redimensionner);

function basculerCamera(o) {
  ortho = o;
  const ancienne = camera;
  camera = o ? camOrtho : camPerspective;
  camera.position.copy(ancienne.position);
  camera.quaternion.copy(ancienne.quaternion);
  controles.object = camera;
  if (o) camOrtho.zoom = 1;
  ajusterOrtho();
  $('#b-ortho').classList.toggle('actif', o);
  cadrer(vueActuelle === 'libre' ? 'iso' : vueActuelle);
}

// ---------------------------------------------------------------- construction des modèles
const materiaux = new Map();
function materiau(m, matiere) {
  const cle = `${m.type}|${m.c2}|${m.c3}|${matiere}`;
  if (materiaux.has(cle)) return materiaux.get(cle);
  let couleur;
  if (matiere === 'couleur_2') couleur = m.c2 || '#888888';
  else if (matiere === 'couleur_3') couleur = m.c3 || '#888888';
  else if (matiere === 'peinture') couleur = m.type === 'héros' ? VERT_HEROS : m.type === 'véhicule' ? PEINTURE_VEHICULE : m.c2 || '#888888';
  else couleur = COULEURS[matiere] || '#ff00ff';
  const params = { color: couleur, roughness: 1, metalness: 0 };
  if (LUMINEUSES.has(matiere)) Object.assign(params, { emissive: couleur, emissiveIntensity: 0.9 });
  if (matiere === 'bulle') Object.assign(params, { transparent: true, opacity: 0.32, depthWrite: false });
  const mat = new THREE.MeshStandardMaterial(params);
  materiaux.set(cle, mat);
  return mat;
}
const geometries = {
  cube: new THREE.BoxGeometry(1, 1, 1),
  cylindre: new THREE.CylinderGeometry(0.5, 0.5, 1, 24),
  cone: new THREE.ConeGeometry(0.5, 1, 24),
  sphere: new THREE.SphereGeometry(0.5, 24, 16),
};

let animes = [];   // objets animés : { obj, base, sortes, pivotZ }
let maillages = []; // maillage de chaque pièce (même ordre que travail.pieces)

function pointPivot(boite, sorte) {
  const { lo, hi } = boite, mid = lo.clone().add(hi).multiplyScalar(0.5);
  switch (sorte) {
    case 'haut': return new THREE.Vector3(mid.x, hi.y, mid.z);
    case 'aile': return new THREE.Vector3(mid.x, mid.y, mid.z > 0 ? lo.z : hi.z);
    case 'bas': return new THREE.Vector3(mid.x, lo.y, mid.z);
    case 'queue': return new THREE.Vector3(hi.x, mid.y, mid.z);
    default: return mid;
  }
}
// Quelle animation fait pivoter un groupe, et autour de quel point (comme dans le jeu).
function sorteDePivot(a) {
  const t = a.animation || a.type || '';
  if (t === 'pendule' && a.axe === 'z') return a.pivot === 'base' ? 'bas' : 'haut';
  if (t === 'pendule' && a.axe === 'x') return 'aile';
  if (t === 'pendule' && a.axe === 'y') return 'bas';
  if (t === 'balance' && a.axe) return 'queue';
  if (t === 'rotation_groupe') return 'centre';
  return null;
}

function construire() {
  groupeModele.clear();
  animes = [];
  maillages = [];
  if (!travail) return;
  const m = travail;
  const mobiles = m.mobiles || [];
  // groupes pivotés : un par nom de pièce concerné
  const pivots = new Map();
  for (const a of mobiles) {
    const sorte = sorteDePivot(a);
    if (!sorte || a.piece.includes('*')) continue;
    const membres = m.pieces.map((p, i) => ({ p, i })).filter(({ p }) => p.nom === a.piece);
    if (!membres.length) continue;
    const lo = new THREE.Vector3(1e9, 1e9, 1e9), hi = new THREE.Vector3(-1e9, -1e9, -1e9);
    for (const { p } of membres) {
      const c = new THREE.Vector3(...p.position), h = new THREE.Vector3(...p.taille).multiplyScalar(0.5);
      lo.min(c.clone().sub(h));
      hi.max(c.clone().add(h));
    }
    if (!pivots.has(a.piece)) {
      const pt = pointPivot({ lo, hi }, sorte);
      const g = new THREE.Group();
      g.position.copy(pt);
      groupeModele.add(g);
      pivots.set(a.piece, { g, pt });
    }
  }
  m.pieces.forEach((p, i) => {
    const mesh = new THREE.Mesh(geometries[p.forme] || geometries.cube, materiau(m, p.matiere));
    mesh.scale.set(...p.taille);
    mesh.position.set(...p.position);
    mesh.rotation.set(...p.rotation_deg.map((d) => (d * Math.PI) / 180), 'ZYX');
    mesh.castShadow = p.matiere !== 'bulle';
    mesh.receiveShadow = true;
    mesh.userData.i = i;
    const piv = p.nom && pivots.get(p.nom);
    if (piv) { mesh.position.sub(piv.pt); piv.g.add(mesh); } else groupeModele.add(mesh);
    maillages[i] = mesh;
  });
  // animations
  const global = [];
  for (const a of mobiles) {
    const cible = a.piece;
    const sorte = { ...a, nom: a.animation || a.type };
    if (cible === '*') { global.push(sorte); continue; }
    const piv = pivots.get(cible);
    if (piv && sorteDePivot(a)) {
      let e = animes.find((x) => x.obj === piv.g);
      if (!e) { e = { obj: piv.g, base: sauvegarder(piv.g), sortes: [], pivotZ: piv.pt.z }; animes.push(e); }
      e.sortes.push(sorte);
      continue;
    }
    // animation simple : sur les pièces portant ce nom (ou un motif préfixe*/*contient)
    maillages.forEach((mesh, i) => {
      const id = m.pieces[i].nom;
      if (!id) return;
      const ok = cible.startsWith('*') ? id.includes(cible.slice(1)) : cible.endsWith('*') ? id.startsWith(cible.slice(0, -1)) : id === cible;
      if (!ok) return;
      const cibleObj = mesh.parent !== groupeModele ? mesh.parent : mesh;
      let e = animes.find((x) => x.obj === cibleObj);
      if (!e) { e = { obj: cibleObj, base: sauvegarder(cibleObj), sortes: [], pivotZ: 0 }; animes.push(e); }
      e.sortes.push(sorte);
    });
  }
  if (global.length) animes.push({ obj: groupeModele, base: sauvegarder(groupeModele), sortes: global, pivotZ: 0 });
  // surbrillance de la pièce choisie
  if (selection >= 0 && maillages[selection]) {
    const aretes = new THREE.LineSegments(new THREE.EdgesGeometry(geometries[travail.pieces[selection].forme]), new THREE.LineBasicMaterial({ color: 0xffb43c, depthTest: false }));
    aretes.renderOrder = 10;
    maillages[selection].add(aretes);
  }
}
function sauvegarder(o) { return { p: o.position.clone(), s: o.scale.clone(), q: o.quaternion.clone() }; }

function mettreAJour(t) {
  for (const e of animes) {
    e.obj.position.copy(e.base.p); e.obj.scale.copy(e.base.s); e.obj.quaternion.copy(e.base.q);
    e.obj.visible = true;
    const tt = t;
    for (const a of e.sortes) {
      const n = a.nom || '';
      const phase = (a.phase || 0) * Math.PI * 2;
      const amp = a.amplitude ?? (parseFloat(n.split('_').pop()) || null);
      if (n === 'pendule') {
        if (a.axe === 'z') e.obj.rotation.z = (a.pivot === 'base' ? Math.sin(tt * 3 + phase) * 0.5 : Math.sin(tt * 8 + phase) * 0.6);
        else if (a.axe === 'x') e.obj.rotation.x = -Math.sign(e.pivotZ || 1) * Math.sin(tt * 7 + phase) * 0.6;
        else if (a.axe === 'y') e.obj.rotation.y = Math.sin(tt * 3 + phase) * 0.6;
      } else if (n === 'balance') {
        if (a.axe === 'y') e.obj.rotation.y = Math.sin(tt * 5) * 0.35;
        else if (a.axe === 'z') e.obj.rotation.z = Math.sin(tt * 5) * 0.2;
        else if (a.axe === 'x') e.obj.rotation.x = Math.sin(tt * 5) * 0.2;
        else e.obj.rotation.z = Math.sin(tt * 3) * 0.14;
      } else if (n === 'rebond' || n.startsWith('oscille_vertical')) e.obj.position.y += Math.sin(tt * 6) * (amp || 0.04);
      else if (n.startsWith('flotte_verticale')) e.obj.position.y += Math.sin(tt * 2) * (amp || 0.03);
      else if (n === 'pulse') e.obj.scale.multiplyScalar(1 + 0.2 * Math.sin(tt * 5));
      else if (n === 'clignote') e.obj.visible = Math.floor(tt * 3) % 2 === 0;
      else if (n === 'clignote_alterne') e.obj.visible = Math.floor(tt * 3) % 2 !== 0;
      else if (n === 'clignote_lent') e.obj.visible = Math.floor(tt * 1.1) % 2 === 0;
      else if (n === 'clignote_clin_doeil') e.obj.visible = tt % 2.6 < 2.45;
      else if (n === 'derive_haut' || n === 'fumee') { const f = (tt * 0.7) % 1; e.obj.position.y += f * 0.3; e.obj.scale.multiplyScalar(1 - 0.5 * f); }
      else if (n === 'derive_arriere') { const f = (tt * 0.6) % 1; e.obj.position.x -= f * 0.5; e.obj.scale.multiplyScalar(1 - 0.6 * f); }
      else if (n === 'rotation_groupe') e.obj.rotation.set(tt * 0.6, tt * 0.9, 0, 'YXZ');
      else if (n === 'vibre' || n === 'tremble') e.obj.position.add(new THREE.Vector3(Math.sin(tt * 61), Math.sin(tt * 53), Math.sin(tt * 47)).multiplyScalar(amp || 0.012));
    }
  }
}

// ---------------------------------------------------------------- vues
function boite() {
  const b = new THREE.Box3().setFromObject(groupeModele);
  if (b.isEmpty()) b.set(new THREE.Vector3(-1, 0, -1), new THREE.Vector3(1, 1, 1));
  return b;
}
function cadrer(vue) {
  vueActuelle = vue;
  document.querySelectorAll('[data-vue]').forEach((b) => b.classList.toggle('actif', b.dataset.vue === vue));
  if (vue === 'libre') return;
  const b = boite(), c = b.getCenter(new THREE.Vector3()), s = b.getSize(new THREE.Vector3());
  const rayon = Math.max(s.x, s.y, s.z, 1.2);
  const dirs = { iso: [0.55, 0.62, 0.56], profil: [0, 0.12, 1], face: [1, 0.12, 0], arriere: [-1, 0.12, 0], dessus: [0.0001, 1, 0.0001] };
  const d = new THREE.Vector3(...(dirs[vue] || dirs.iso)).normalize();
  controles.target.copy(c);
  // Dans une fenêtre étroite, on recule pour que le modèle tienne en largeur.
  const largeur = camPerspective.aspect || 1;
  camera.position.copy(c).addScaledVector(d, (rayon * 2.6) / Math.min(1, largeur * 1.15));
  if (ortho) { camOrtho.zoom = 3.2 / (rayon * 0.75); ajusterOrtho(); }
  camera.lookAt(c);
  controles.update();
}
document.querySelectorAll('[data-vue]').forEach((b) => b.addEventListener('click', () => cadrer(b.dataset.vue)));
controles.addEventListener('start', () => { if (vueActuelle !== 'libre') { vueActuelle = 'libre'; document.querySelectorAll('[data-vue]').forEach((b) => b.classList.toggle('actif', b.dataset.vue === 'libre')); } });

// ---------------------------------------------------------------- interface
function statut(texte, classe = '') { const s = $('#statut'); s.textContent = texte; s.className = classe; }
function copie(o) { return JSON.parse(JSON.stringify(o)); }

function remplirListe() {
  const q = $('#recherche').value.trim().toLowerCase(), type = $('#type').value;
  const liste = $('#liste');
  liste.innerHTML = '';
  let dernier = '';
  for (const m of modeles) {
    if (type && m.type !== type) continue;
    if (q && !(m.nom + ' ' + m.id + ' ' + m.fichier).toLowerCase().includes(q)) continue;
    if (m.fichier !== dernier) {
      dernier = m.fichier;
      const g = document.createElement('div');
      g.className = 'groupe';
      g.textContent = m.fichier.replace('import/', '').replace('.json', '');
      liste.appendChild(g);
    }
    const d = document.createElement('div');
    d.className = 'modele' + (courant && courant.cle === m.cle ? ' courant' : '');
    d.innerHTML = `<span></span><small></small>`;
    d.children[0].textContent = m.nom || m.id;
    d.children[1].textContent = m.type + (m.editable ? '' : ' 🔒');
    d.addEventListener('click', () => charger(m.cle));
    liste.appendChild(d);
  }
}
function remplirTypes() {
  const types = [...new Set(modeles.map((m) => m.type))];
  const sel = $('#type');
  sel.innerHTML = '<option value="">Tous les types</option>' + types.map((t) => `<option>${t}</option>`).join('');
}

function afficherInfos() {
  const m = travail;
  $('#titre').textContent = m ? `${m.nom || m.id} — ${m.type}${sale ? ' •' : ''}` : '—';
  $('#b-sauver').disabled = !m || !m.editable || !sale;
  $('#b-annuler').disabled = !pile.length;
  if (!m) { $('#infos').textContent = ''; return; }
  $('#infos').innerHTML = `<div class="ligne"><label>id</label><b>${m.id}</b></div>
    <div class="ligne"><label>fichier</label><span>${m.fichier}</span></div>
    <div class="ligne"><label>couleur 2</label><input type="color" id="c2" value="${m.c2 && m.c2.startsWith('#') ? m.c2 : '#888888'}"><label style="width:auto">couleur 3</label><input type="color" id="c3" value="${m.c3 && m.c3.startsWith('#') ? m.c3 : '#888888'}"></div>
    ${m.longueur ? `<div class="ligne"><label>taille</label><span>longueur ${m.longueur} · largeur ${m.largeur ?? '?'}</span></div>` : ''}
    ${m.editable ? '' : '<div class="ligne" style="color:var(--mal)">Lecture seule (fichier produit par un script).</div>'}`;
  for (const k of ['c2', 'c3']) $('#' + k).addEventListener('input', (e) => { modifier(() => { travail[k] = e.target.value; }); });
  $('#nb').textContent = `(${m.pieces.length})`;
  const lp = $('#pieces');
  lp.innerHTML = '';
  m.pieces.forEach((p, i) => {
    const d = document.createElement('div');
    d.className = 'piece' + (i === selection ? ' sel' : '');
    d.innerHTML = '<span></span><span></span><span></span>';
    d.children[0].textContent = i;
    d.children[1].textContent = (p.nom ? p.nom + ' · ' : '') + p.forme;
    d.children[2].textContent = p.matiere;
    d.addEventListener('click', () => choisir(i));
    lp.appendChild(d);
  });
  $('#anims').value = JSON.stringify(m.mobiles || [], null, 1);
  afficherFormulaire();
}

function champ(etiquette, valeurs, cle, pas) {
  return `<div class="ligne"><label>${etiquette}</label>` + valeurs.map((v, k) => `<input type="number" step="${pas}" data-champ="${cle}" data-k="${k}" value="${Math.round(v * 10000) / 10000}">`).join('') + '</div>';
}
function afficherFormulaire() {
  const f = $('#form');
  if (!travail || selection < 0 || !travail.pieces[selection]) { f.innerHTML = '<span class="aide">Cliquez une pièce dans la vue ou dans la liste.</span>'; return; }
  const p = travail.pieces[selection];
  f.innerHTML = `<div class="ligne"><label>forme</label><select id="f-forme">${FORMES.map((x) => `<option ${x === p.forme ? 'selected' : ''}>${x}</option>`).join('')}</select></div>
    ${champ('position', p.position, 'position', 0.01)}${champ('taille', p.taille, 'taille', 0.01)}${champ('rotation °', p.rotation_deg, 'rotation_deg', 1)}
    <div class="ligne"><label>matière</label><select id="f-mat">${MATIERES.map((x) => `<option ${x === p.matiere ? 'selected' : ''}>${x}</option>`).join('')}</select></div>
    <div class="ligne"><label>nom</label><input id="f-nom" style="flex:1" value="${p.nom || ''}" placeholder="(pour l'animer)"></div>`;
  $('#f-forme').addEventListener('change', (e) => modifier(() => { p.forme = e.target.value; }));
  $('#f-mat').addEventListener('change', (e) => modifier(() => { p.matiere = e.target.value; }));
  $('#f-nom').addEventListener('change', (e) => modifier(() => { if (e.target.value) p.nom = e.target.value; else delete p.nom; }));
  f.querySelectorAll('[data-champ]').forEach((i) => i.addEventListener('input', (e) => {
    const v = parseFloat(e.target.value);
    if (Number.isNaN(v)) return;
    modifier(() => { p[e.target.dataset.champ][+e.target.dataset.k] = v; }, true);
  }));
}

function choisir(i) {
  selection = i;
  construire();
  afficherInfosLegeres();
}
function afficherInfosLegeres() {
  document.querySelectorAll('.piece').forEach((d, k) => d.classList.toggle('sel', k === selection));
  afficherFormulaire();
}

let dernierPush = 0;
function modifier(fn, regrouper = false) {
  if (!travail || !travail.editable) { statut('Lecture seule : ce modèle ne peut pas être modifié ici.', 'mal'); return; }
  const maintenant = performance.now();
  if (!regrouper || maintenant - dernierPush > 800) { pile.push(copie(travail)); if (pile.length > 60) pile.shift(); }
  dernierPush = maintenant;
  fn();
  sale = true;
  construire();
  const champs = $('#form').querySelector(':focus');
  if (!champs) afficherInfos(); else { $('#titre').textContent = `${travail.nom || travail.id} — ${travail.type} •`; $('#b-sauver').disabled = false; $('#b-annuler').disabled = false; }
}
function annuler() {
  if (!pile.length) return;
  travail = pile.pop();
  sale = true;
  construire();
  afficherInfos();
}

async function sauverModele() {
  if (!travail || !sale) return;
  const rep = await fetch('/api/sauver', { method: 'POST', body: JSON.stringify({ fichier: travail.fichier, chemin: travail.chemin, modele: { pieces: travail.pieces, mobiles: travail.mobiles, c2: travail.c2, c3: travail.c3 } }) });
  const r = await rep.json();
  if (r.ok) {
    sale = false;
    const maj = await (await fetch('/api/mtimes')).json();
    mtimes = maj;
    const i = modeles.findIndex((m) => m.cle === travail.cle);
    if (i >= 0) modeles[i] = copie(travail);
    statut('Enregistré dans ' + travail.fichier, 'ok');
    afficherInfos();
  } else statut('Échec : ' + r.erreur, 'mal');
}

function charger(cle, garderVue = false) {
  const m = modeles.find((x) => x.cle === cle || x.id === cle);
  if (!m) return;
  if (sale && !garderVue && !confirm('Des modifications ne sont pas enregistrées. Les abandonner ?')) return;
  courant = m;
  travail = copie(m);
  selection = -1;
  sale = false;
  pile = [];
  construire();
  if (!garderVue) cadrer(parUrl.get('vue') && !chargerDeja ? parUrl.get('vue') : vueActuelle === 'libre' ? 'iso' : vueActuelle);
  chargerDeja = true;
  afficherInfos();
  remplirListe();
  history.replaceState(null, '', `?modele=${encodeURIComponent(m.cle)}${anime ? '' : '&anim=0'}`);
  statut('');
}
let chargerDeja = false;

// ---------------------------------------------------------------- boutons et clavier
$('#recherche').addEventListener('input', remplirListe);
$('#type').addEventListener('change', remplirListe);
function basculer(id, etat, fn) { $(id).classList.toggle('actif', etat); fn(etat); }
$('#b-anim').addEventListener('click', () => { anime = !anime; $('#b-anim').classList.toggle('actif', anime); if (!anime) { mettreAJour(0); } });
$('#b-ortho').addEventListener('click', () => basculerCamera(!ortho));
$('#b-grille').addEventListener('click', () => { grille = !grille; grilleObj.visible = grille; $('#b-grille').classList.toggle('actif', grille); });
$('#b-ref').addEventListener('click', () => { ref = !ref; crapaud.visible = ref; $('#b-ref').classList.toggle('actif', ref); });
$('#b-fond').addEventListener('click', () => { fondSombre = !fondSombre; scene.background.set(fondSombre ? 0x1b2130 : 0xd6dde6); sol.material.color.set(fondSombre ? 0x2a3347 : 0xc3ccd6); $('#b-fond').classList.toggle('actif', fondSombre); });
$('#b-sauver').addEventListener('click', sauverModele);
$('#b-annuler').addEventListener('click', annuler);
$('#b-capture').addEventListener('click', () => { rendu.render(scene, camera); const a = document.createElement('a'); a.download = `${travail ? travail.id : 'modele'}.png`; a.href = toile.toDataURL('image/png'); a.click(); });
$('#b-anims').addEventListener('click', () => {
  try { const v = JSON.parse($('#anims').value); if (!Array.isArray(v)) throw new Error('une liste est attendue'); modifier(() => { travail.mobiles = v; }); $('#anims-etat').textContent = 'ok'; }
  catch (e) { $('#anims-etat').textContent = 'JSON invalide : ' + e.message; }
});
$('#b-ajouter').addEventListener('click', () => modifier(() => { travail.pieces.push({ forme: 'cube', position: [0, 0.5, 0], taille: [0.5, 0.5, 0.5], rotation_deg: [0, 0, 0], matiere: 'couleur_2' }); selection = travail.pieces.length - 1; }) || afficherInfos());
$('#b-dupliquer').addEventListener('click', () => { if (selection < 0) return; modifier(() => { const p = copie(travail.pieces[selection]); p.position[0] += 0.2; travail.pieces.push(p); selection = travail.pieces.length - 1; }); afficherInfos(); });
$('#b-miroir').addEventListener('click', () => { if (selection < 0) return; modifier(() => { const p = copie(travail.pieces[selection]); p.position[2] = -p.position[2]; p.rotation_deg[0] = -p.rotation_deg[0]; if (p.nom) p.nom = p.nom.replace(/_g\b/, '_d~').replace(/_d\b/, '_g').replace('_d~', '_d'); travail.pieces.push(p); selection = travail.pieces.length - 1; }); afficherInfos(); });
$('#b-supprimer').addEventListener('click', supprimerPiece);
function supprimerPiece() { if (selection < 0) return; modifier(() => { travail.pieces.splice(selection, 1); selection = -1; }); afficherInfos(); }

addEventListener('keydown', (e) => {
  const saisie = ['INPUT', 'TEXTAREA', 'SELECT'].includes(document.activeElement.tagName);
  if ((e.ctrlKey || e.metaKey) && e.key === 's') { e.preventDefault(); sauverModele(); return; }
  if ((e.ctrlKey || e.metaKey) && e.key === 'z') { e.preventDefault(); annuler(); return; }
  if (saisie || selection < 0 || !travail) return;
  const pas = e.shiftKey ? 0.01 : 0.05, p = travail.pieces[selection];
  const dep = { ArrowRight: [0, pas], ArrowLeft: [0, -pas], ArrowDown: [2, pas], ArrowUp: [2, -pas], PageUp: [1, pas], PageDown: [1, -pas] }[e.key];
  if (dep) { e.preventDefault(); modifier(() => { p.position[dep[0]] = Math.round((p.position[dep[0]] + dep[1]) * 1000) / 1000; }, true); afficherInfosLegeres(); }
  else if (e.key === 'Delete') supprimerPiece();
});

// clic sur une pièce de la vue
let appui = null;
toile.addEventListener('pointerdown', (e) => { appui = [e.clientX, e.clientY]; });
toile.addEventListener('pointerup', (e) => {
  if (!appui || Math.hypot(e.clientX - appui[0], e.clientY - appui[1]) > 4) return;
  const r = toile.getBoundingClientRect();
  const rc = new THREE.Raycaster();
  rc.setFromCamera(new THREE.Vector2(((e.clientX - r.left) / r.width) * 2 - 1, -((e.clientY - r.top) / r.height) * 2 + 1), camera);
  const touches = rc.intersectObjects(maillages.filter(Boolean), false).filter((h) => h.object.material.opacity === undefined || h.object.material.opacity > 0.5);
  choisir(touches.length ? touches[0].object.userData.i : -1);
});

// ---------------------------------------------------------------- chargement et rechargement automatique
async function recharger(garder) {
  const [liste, mt] = await Promise.all([fetch('/api/modeles').then((r) => r.json()), fetch('/api/mtimes').then((r) => r.json())]);
  modeles = liste;
  mtimes = mt;
  remplirTypes();
  if (!garder) return;
  const encore = modeles.find((m) => courant && m.cle === courant.cle);
  if (encore && !sale) { const sel = selection; courant = encore; travail = copie(encore); selection = sel; construire(); afficherInfos(); statut('Rechargé : le fichier a changé sur le disque.', 'ok'); }
  remplirListe();
}
setInterval(async () => {
  try {
    const mt = await (await fetch('/api/mtimes')).json();
    const change = Object.keys(mt).filter((f) => mtimes[f] !== mt[f]);
    if (!change.length) return;
    if (sale && courant && change.includes(courant.fichier)) { statut('Le fichier a changé sur le disque : enregistrez ou annulez pour recharger.', 'mal'); mtimes = mt; return; }
    await recharger(true);
  } catch (e) { /* serveur arrêté : on réessaie */ }
}, 1500);

const horloge = new THREE.Clock();
function boucle() {
  requestAnimationFrame(boucle);
  if (anime) mettreAJour(horloge.getElapsedTime());
  controles.update();
  rendu.render(scene, camera);
}

(async function demarrer() {
  redimensionner();
  $('#b-anim').classList.toggle('actif', anime);
  $('#b-grille').classList.add('actif');
  $('#b-ref').classList.toggle('actif', ref);
  crapaud.visible = ref;
  await recharger(false);
  if (ortho) basculerCamera(true);
  remplirListe();
  const voulu = parUrl.get('modele');
  charger(voulu || (modeles[0] && modeles[0].cle));
  boucle();
  // petite interface pour piloter la visionneuse depuis la console ou un script
  window.visionneuse = { charger, cadrer, choisir, modeles: () => modeles, travail: () => travail, mettreAJour, rendu, scene, camera: () => camera };
})();
