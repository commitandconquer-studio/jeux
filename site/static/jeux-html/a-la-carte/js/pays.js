// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Mode « Devine le pays » : silhouette normale, tournée, retournée.
// Silhouettes : Natural Earth (domaine public), simplifiées par tools/a-la-carte-formes.py.
"use strict";

const Pays = (() => {
  const NIVEAUX = [
    { id: "facile", nom: "Facile", texte: "Silhouettes de pays célèbres, quatre propositions", pool: "celebres", mode: "qcm" },
    { id: "moyen", nom: "Moyen", texte: "Tous les pays, quatre propositions", pool: "tous", mode: "qcm" },
    { id: "difficile", nom: "Difficile", texte: "Silhouette tournée, quatre propositions", pool: "tous", mode: "qcm", rotation: true },
    { id: "expert", nom: "Expert", texte: "Silhouette tournée ou retournée : tape le nom du pays", pool: "tous", mode: "saisie", rotation: true, miroir: true },
  ];
  const QUESTIONS = 10;
  const SURFACE_MIN = 0.45; // degrés carrés : en dessous, la silhouette est trop petite pour être reconnue

  let pays = [], index = [], formes = {};
  let niveau = null, partie = null, q = null;

  async function charger() {
    if (pays.length) return;
    const [p, f] = await Promise.all([fetch("data/pays.json"), fetch("data/formes.json")]);
    const tous = await p.json();
    formes = await f.json();
    pays = tous.filter(x => formes[x.code] && formes[x.code].s >= SURFACE_MIN);
    index = pays.map(x => Commun.normaliser(x.nom));
  }

  const hasard = n => Math.floor(Math.random() * n);
  function melange(a) {
    a = a.slice();
    for (let i = a.length - 1; i > 0; i--) { const j = hasard(i + 1); [a[i], a[j]] = [a[j], a[i]]; }
    return a;
  }
  const celebres = () => pays.slice().sort((a, b) => b.sl - a.sl).slice(0, 60);

  /** SVG de la silhouette, dans un carré de 100 x 100. */
  function svg(p) {
    return `<svg viewBox="-4 -4 108 108" role="img" aria-label="Silhouette d'un pays"><path d="${formes[p.code].d}" fill-rule="evenodd"/></svg>`;
  }

  async function demarrer(id) {
    await charger();
    niveau = NIVEAUX.find(n => n.id === id);
    const base = niveau.pool === "celebres" ? celebres() : pays;
    partie = { pile: melange(base).slice(0, QUESTIONS), numero: 0, bonnes: 0 };
    return suivante();
  }

  function propositions(cible, pool) {
    const meme = melange(pool.filter(p => p !== cible && p.continent === cible.continent));
    const autres = melange(pool.filter(p => p !== cible && p.continent !== cible.continent));
    const faux = meme.slice(0, 3);
    for (const p of autres) { if (faux.length >= 3) break; faux.push(p); }
    return melange([cible, ...faux]);
  }

  function suivante() {
    const cible = partie.pile[partie.numero++];
    const pool = niveau.pool === "celebres" ? celebres() : pays;
    // Rotation d'au moins 40° pour qu'elle se remarque ; miroir une fois sur deux.
    const angle = niveau.rotation ? 40 + hasard(281) : 0;
    q = {
      cible, fini: false, bonne: false, equiv: [],
      choix: niveau.mode === "qcm" ? propositions(cible, pool) : null,
      angle, miroir: niveau.miroir && hasard(2) === 1,
    };
    return q;
  }

  function repondre(p) {
    const ok = p === q.cible;
    q.fini = true; q.bonne = ok; q.reponse = p;
    if (ok) partie.bonnes++;
    return ok;
  }

  function suggestions(saisie, max = 6) {
    const s = Commun.normaliser(saisie);
    if (!s) return [];
    const debut = [], contient = [];
    index.forEach((n, i) => {
      if (n.startsWith(s)) debut.push(pays[i]);
      else if (n.includes(s)) contient.push(pays[i]);
    });
    return debut.concat(contient).slice(0, max);
  }

  function etoilesPour(bonnes) { return bonnes >= 9 ? 3 : bonnes >= 7 ? 2 : bonnes >= 5 ? 1 : 0; }

  function resume() {
    const e = etoilesPour(partie.bonnes);
    Commun.noter("pays", niveau.id, e);
    return { etoiles: e, bonnes: partie.bonnes, sur: QUESTIONS };
  }

  return {
    NIVEAUX, QUESTIONS, charger, demarrer, suivante, repondre, suggestions, resume, svg,
    get q() { return q; }, get partie() { return partie; }, get niveau() { return niveau; },
    get pays() { return pays; },
  };
})();

if (typeof module !== "undefined") module.exports = Pays; // pour les tests
