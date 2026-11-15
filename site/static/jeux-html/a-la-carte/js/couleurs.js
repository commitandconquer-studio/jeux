// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Modes « Les bonnes couleurs » (choisir le drapeau aux vraies couleurs) et
// « Recolorier » (redonner leurs couleurs au drapeau). Les drapeaux sont recolorés
// en remplaçant les couleurs écrites dans le SVG ; le blanc, le noir et les gris ne bougent pas.
"use strict";

const Teinte = (() => {
  const NOMMEES = { red: "#ff0000", gold: "#ffd700", gray: "#808080", olive: "#808000", purple: "#800080" };
  const RE_NOMMEES = new RegExp(`(?<=[="\\s:])(${Object.keys(NOMMEES).join("|")})(?=["\\s;])`, "g");
  const RE_HEX = /#[0-9a-fA-F]{6}\b|#[0-9a-fA-F]{3}\b/g;
  const textes = {};

  function norm(h) {
    h = h.toLowerCase();
    return h.length === 4 ? "#" + [...h.slice(1)].map(c => c + c).join("") : h;
  }
  function rvb(h) { return [1, 3, 5].map(i => parseInt(h.slice(i, i + 2), 16)); }
  function hexDe(r, g, b) {
    return "#" + [r, g, b].map(v => Math.max(0, Math.min(255, Math.round(v))).toString(16).padStart(2, "0")).join("");
  }
  function versTsl(h) {
    const [r, g, b] = rvb(h).map(v => v / 255);
    const max = Math.max(r, g, b), min = Math.min(r, g, b), l = (max + min) / 2, d = max - min;
    let t = 0, s = 0;
    if (d) {
      s = d / (1 - Math.abs(2 * l - 1));
      t = max === r ? ((g - b) / d + 6) % 6 : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
      t *= 60;
    }
    return [t, s, l];
  }
  function deTsl(t, s, l) {
    const c = (1 - Math.abs(2 * l - 1)) * s, x = c * (1 - Math.abs((t / 60) % 2 - 1)), m = l - c / 2;
    const [r, g, b] = t < 60 ? [c, x, 0] : t < 120 ? [x, c, 0] : t < 180 ? [0, c, x] : t < 240 ? [0, x, c] : t < 300 ? [x, 0, c] : [c, 0, x];
    return hexDe((r + m) * 255, (g + m) * 255, (b + m) * 255);
  }
  /** Décale la teinte (degrés) et la luminosité (−1..1) d'une couleur. */
  function decaler(h, dt, dl = 0) {
    const [t, s, l] = versTsl(h);
    if (s <= 0.18 || l >= 0.93 || l <= 0.08) return h;   // blanc, noir, gris : inchangés
    return deTsl((t + dt + 360) % 360, s, Math.max(0.05, Math.min(0.95, l + dl)));
  }

  /* Distance perceptuelle approchée (CIE Lab) pour trouver la couleur nommée la plus proche. */
  function lab(h) {
    const [r, g, b] = rvb(h).map(v => { v /= 255; return v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; });
    const f = t => (t > 0.008856 ? Math.cbrt(t) : 7.787 * t + 16 / 116);
    const x = f((0.4124 * r + 0.3576 * g + 0.1805 * b) / 0.95047);
    const y = f(0.2126 * r + 0.7152 * g + 0.0722 * b);
    const z = f((0.0193 * r + 0.1192 * g + 0.9505 * b) / 1.08883);
    return [116 * y - 16, 500 * (x - y), 200 * (y - z)];
  }
  function ecart(a, b) {
    const A = lab(a), B = lab(b);
    return Math.hypot(A[0] - B[0], A[1] - B[1], A[2] - B[2]);
  }
  function plusProche(h, palette) {
    let m = 0, dmin = Infinity;
    palette.forEach((p, i) => { const d = ecart(h, p.hex); if (d < dmin) { dmin = d; m = i; } });
    return m;
  }

  async function texte(code) {
    if (!textes[code]) {
      const t = await (await fetch(`data/drapeaux/${code}.svg`)).text();
      textes[code] = t.replace(RE_NOMMEES, m => NOMMEES[m]);
    }
    return textes[code];
  }
  /** URL (data:) du drapeau dont chaque couleur c devient f(c) (f renvoie undefined pour ne pas toucher). */
  async function uri(code, f) {
    const t = (await texte(code)).replace(RE_HEX, m => f(norm(m)) || m);
    return "data:image/svg+xml;charset=utf-8," + encodeURIComponent(t);
  }

  return { norm, decaler, versTsl, plusProche, ecart, uri };
})();

const BonnesCouleurs = (() => {
  // Écart de teinte (degrés) des faux drapeaux par rapport au vrai.
  const NIVEAUX = [
    { id: "facile", nom: "Facile", texte: "Les faux drapeaux ont des couleurs très différentes", min: 100, max: 170, gap: 45, lum: 0, pool: "celebres" },
    { id: "moyen", nom: "Moyen", texte: "Les couleurs sont nettement décalées", min: 55, max: 85, gap: 25, lum: 0, pool: "tous" },
    { id: "difficile", nom: "Difficile", texte: "Les couleurs sont assez proches", min: 25, max: 40, gap: 12, lum: 0.06, pool: "tous" },
    { id: "expert", nom: "Expert", texte: "Les couleurs sont presque les mêmes", min: 10, max: 16, gap: 5, lum: 0.04, pool: "tous" },
  ];
  const QUESTIONS = 10;
  let pays = [], couleurs = {}, niveau = null, partie = null, q = null;

  let alea = Math.random;   // remplacé par un générateur à graine pour les défis du jour
  const hasard = n => Math.floor(alea() * n);
  function melange(a) {
    a = a.slice();
    for (let i = a.length - 1; i > 0; i--) { const j = hasard(i + 1); [a[i], a[j]] = [a[j], a[i]]; }
    return a;
  }

  async function charger() {
    if (pays.length) return;
    const [p, c] = await Promise.all([fetch("data/pays.json"), fetch("data/couleurs.json")]);
    couleurs = await c.json();
    pays = (await p.json()).filter(x => couleurs[x.code] && couleurs[x.code].length >= 1);
  }

  /** Trois écarts de teinte (± degrés) assez éloignés les uns des autres et du vrai (0). */
  function ecarts(n) {
    for (let essai = 0; essai < 500; essai++) {
      const v = [];
      for (let i = 0; i < 3; i++) {
        const m = n.min + alea() * (n.max - n.min);
        v.push(alea() < 0.5 ? -m : m);
      }
      if (v.every((a, i) => v.every((b, j) => i === j || Math.abs(a - b) >= n.gap))) return v;
    }
    return [-n.max, n.min, n.max];
  }

  async function demarrer(id, graine = null) {
    await charger();
    niveau = NIVEAUX.find(n => n.id === id);
    const base = niveau.pool === "celebres" ? pays.slice().sort((a, b) => b.sl - a.sl).slice(0, 70) : pays;
    alea = graine ? Commun.prng(graine + ":pile") : Math.random;
    partie = { pile: melange(base).slice(0, QUESTIONS), numero: 0, bonnes: 0, graine, resultats: [] };
    return suivante();
  }

  function suivante() {
    alea = partie.graine ? Commun.prng(partie.graine + ":" + partie.numero) : Math.random;
    const cible = partie.pile[partie.numero++];
    const options = [{ dh: 0, dl: 0, vrai: true }, ...ecarts(niveau).map(dh => ({
      dh, dl: niveau.lum ? (alea() < 0.5 ? -1 : 1) * niveau.lum : 0, vrai: false,
    }))];
    q = { cible, options: melange(options), fini: false, bonne: false };
    return q;
  }

  /** Les quatre images du drapeau (une seule a les vraies couleurs). */
  function images() {
    return Promise.all(q.options.map(o => Teinte.uri(q.cible.code, c => (o.vrai ? c : Teinte.decaler(c, o.dh, o.dl)))
      .then(u => (o.url = u))));
  }

  function repondre(i) {
    const ok = q.options[i].vrai;
    q.fini = true; q.bonne = ok; q.reponse = i;
    if (ok) partie.bonnes++;
    partie.resultats.push(ok);
    return ok;
  }

  function etoilesPour(b) { return b >= 9 ? 3 : b >= 7 ? 2 : b >= 5 ? 1 : 0; }
  function resume() {
    const e = etoilesPour(partie.bonnes);
    if (!partie.graine) Commun.noter("couleurs", niveau.id, e);
    return { etoiles: e, bonnes: partie.bonnes, sur: QUESTIONS, resultats: partie.resultats.slice() };
  }

  return {
    NIVEAUX, QUESTIONS, demarrer, suivante, repondre, resume, images,
    get q() { return q; }, get partie() { return partie; }, get niveau() { return niveau; },
  };
})();

const Recolorier = (() => {
  const BASE = [
    { nom: "Rouge", hex: "#CE1126" }, { nom: "Orange", hex: "#FF8200" }, { nom: "Jaune", hex: "#FCD116" },
    { nom: "Vert", hex: "#009A44" }, { nom: "Bleu", hex: "#1E50B4" }, { nom: "Bleu ciel", hex: "#4FB0E8" },
    { nom: "Violet", hex: "#6D2E8C" }, { nom: "Marron", hex: "#7A4A24" },
  ];
  const LARGE = BASE.concat([
    { nom: "Rouge foncé", hex: "#8B1A2B" }, { nom: "Vert foncé", hex: "#006233" }, { nom: "Vert clair", hex: "#7AC943" },
    { nom: "Bleu marine", hex: "#0B2A63" }, { nom: "Rose", hex: "#F28AB2" },
  ]);
  const NIVEAUX = [
    { id: "facile", nom: "Facile", texte: "Drapeaux à deux couleurs, huit couleurs au choix", maxG: 2, palette: BASE },
    { id: "moyen", nom: "Moyen", texte: "Jusqu'à trois couleurs, huit couleurs au choix", maxG: 3, palette: BASE },
    { id: "difficile", nom: "Difficile", texte: "Jusqu'à quatre couleurs, treize nuances au choix", maxG: 4, palette: LARGE },
    { id: "expert", nom: "Expert", texte: "Jusqu'à quatre couleurs, treize nuances : les bons tons", maxG: 4, palette: LARGE, minG: 3 },
  ];
  const QUESTIONS = 10;
  let pays = [], couleurs = {}, niveau = null, partie = null, q = null;

  let alea = Math.random;   // remplacé par un générateur à graine pour les défis du jour
  const hasard = n => Math.floor(alea() * n);
  function melange(a) {
    a = a.slice();
    for (let i = a.length - 1; i > 0; i--) { const j = hasard(i + 1); [a[i], a[j]] = [a[j], a[i]]; }
    return a;
  }

  async function charger() {
    if (pays.length) return;
    const [p, c] = await Promise.all([fetch("data/pays.json"), fetch("data/couleurs.json")]);
    couleurs = await c.json();
    pays = (await p.json()).filter(x => couleurs[x.code] && couleurs[x.code].length >= 1);
  }

  async function demarrer(id, graine = null) {
    await charger();
    niveau = NIVEAUX.find(n => n.id === id);
    const base = pays.filter(p => {
      const n = couleurs[p.code].length;
      return n <= niveau.maxG && n >= (niveau.minG || 1);
    });
    alea = graine ? Commun.prng(graine + ":pile") : Math.random;
    partie = { pile: melange(base).slice(0, QUESTIONS), numero: 0, bonnes: 0, graine, resultats: [] };
    return suivante();
  }

  function suivante() {
    alea = partie.graine ? Commun.prng(partie.graine + ":" + partie.numero) : Math.random;
    const cible = partie.pile[partie.numero++];
    const groupes = couleurs[cible.code].map(liste => {
      const rep = liste[0];
      const bon = Teinte.plusProche(rep, niveau.palette);
      // Couleur de départ : volontairement fausse (au moins 90° plus loin, et pas proche de la bonne).
      let dh = (alea() < 0.5 ? -1 : 1) * (90 + hasard(80)), depart = Teinte.decaler(rep, dh);
      if (Teinte.plusProche(depart, niveau.palette) === bon) { dh = -dh; depart = Teinte.decaler(rep, dh); }
      return { liste, rep, bon, dh, choix: null };
    });
    q = { cible, groupes, fini: false, bonne: false };
    return q;
  }

  /** Image actuelle : couleurs choisies, sinon couleur de départ fausse. */
  function image() {
    const t = {};
    q.groupes.forEach(g => g.liste.forEach(c => {
      t[c] = g.choix !== null ? niveau.palette[g.choix].hex.toLowerCase() : Teinte.decaler(c, g.dh);
    }));
    return Teinte.uri(q.cible.code, c => t[c]);
  }
  function imageVraie() { return Teinte.uri(q.cible.code, () => undefined); }

  function choisir(i, p) { q.groupes[i].choix = p; }

  function valider() {
    const detail = q.groupes.map(g => g.choix === g.bon);
    q.fini = true; q.bonne = detail.every(Boolean);
    if (q.bonne) partie.bonnes++;
    partie.resultats.push(q.bonne);
    return detail;
  }

  function etoilesPour(b) { return b >= 9 ? 3 : b >= 7 ? 2 : b >= 5 ? 1 : 0; }
  function resume() {
    const e = etoilesPour(partie.bonnes);
    if (!partie.graine) Commun.noter("recolorier", niveau.id, e);
    return { etoiles: e, bonnes: partie.bonnes, sur: QUESTIONS, resultats: partie.resultats.slice() };
  }

  return {
    NIVEAUX, QUESTIONS, demarrer, suivante, image, imageVraie, choisir, valider, resume,
    get q() { return q; }, get partie() { return partie; }, get niveau() { return niveau; },
  };
})();

if (typeof module !== "undefined") module.exports = { Teinte, BonnesCouleurs, Recolorier };
