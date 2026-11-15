// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Mode « Devine le drapeau » : classique, couleurs modifiées, proportions de couleurs.
"use strict";

const Drapeau = (() => {
  const NIVEAUX = [
    { id: "facile", nom: "Facile", texte: "Drapeaux de pays célèbres, quatre propositions", pool: "celebres", mode: "qcm" },
    { id: "moyen", nom: "Moyen", texte: "Tous les pays, quatre propositions", pool: "tous", mode: "qcm" },
    { id: "difficile", nom: "Difficile", texte: "Couleurs modifiées, quatre propositions", pool: "tous", mode: "qcm", teinte: true },
    { id: "expert", nom: "Expert", texte: "Seulement les proportions de couleurs : tape le nom du pays", pool: "tous", mode: "saisie", prop: true },
  ];
  const QUESTIONS = 10;

  // Couleurs de base pour classer les pixels d'un drapeau.
  const PALETTE = [
    { nom: "blanc", rgb: [255, 255, 255] }, { nom: "noir", rgb: [0, 0, 0] },
    { nom: "rouge", rgb: [206, 17, 38] }, { nom: "bleu", rgb: [0, 40, 160] },
    { nom: "bleu ciel", rgb: [70, 170, 230] }, { nom: "vert", rgb: [0, 130, 60] },
    { nom: "jaune", rgb: [252, 209, 22] }, { nom: "orange", rgb: [255, 125, 0] },
  ];

  let pays = [], index = [], parCode = {};
  let niveau = null, partie = null, q = null;
  const sigs = {};

  async function charger() {
    if (pays.length) return;
    pays = await (await fetch("data/pays.json")).json();
    index = pays.map(p => Commun.normaliser(p.nom));
    pays.forEach(p => { parCode[p.code] = p; });
  }

  const hasard = n => Math.floor(Math.random() * n);
  function melange(a) {
    a = a.slice();
    for (let i = a.length - 1; i > 0; i--) { const j = hasard(i + 1); [a[i], a[j]] = [a[j], a[i]]; }
    return a;
  }

  const celebres = () => pays.slice().sort((a, b) => b.sl - a.sl).slice(0, 70);

  function src(p) { return `data/drapeaux/${p.code}.svg`; }

  /* Proportions de couleurs, calculées en dessinant le drapeau sur un petit canevas. */
  async function signature(p) {
    if (sigs[p.code]) return sigs[p.code];
    const img = new Image();
    img.src = src(p);
    await img.decode();
    const L = 120, H = 90;
    const cv = document.createElement("canvas");
    cv.width = L; cv.height = H;
    const ctx = cv.getContext("2d", { willReadFrequently: true });
    ctx.drawImage(img, 0, 0, L, H);
    const px = ctx.getImageData(0, 0, L, H).data;
    const n = new Array(PALETTE.length).fill(0);
    for (let i = 0; i < px.length; i += 4) {
      if (px[i + 3] < 128) continue;
      let meilleur = 0, dmin = Infinity;
      for (let k = 0; k < PALETTE.length; k++) {
        const c = PALETTE[k].rgb;
        const d = (px[i] - c[0]) ** 2 + (px[i + 1] - c[1]) ** 2 + (px[i + 2] - c[2]) ** 2;
        if (d < dmin) { dmin = d; meilleur = k; }
      }
      n[meilleur]++;
    }
    let total = 0;
    const f = n.map(v => { const x = v / (L * H); if (x >= 0.03) { total += x; return x; } return 0; });
    const sig = f.map(x => x / (total || 1));
    sigs[p.code] = sig;
    return sig;
  }

  function ecartSig(a, b) {
    let s = 0;
    for (let i = 0; i < a.length; i++) s += Math.abs(a[i] - b[i]);
    return s / 2;
  }

  async function preparerSignatures() {
    const file = pays.slice();
    const travailleur = async () => { while (file.length) await signature(file.pop()); };
    await Promise.all([travailleur(), travailleur(), travailleur(), travailleur()]);
  }

  /** Pays dont les proportions sont les mêmes que celles de p (réponses équivalentes). */
  function equivalents(p) {
    return pays.filter(x => x !== p && ecartSig(sigs[x.code], sigs[p.code]) < 0.05);
  }

  async function demarrer(id) {
    await charger();
    niveau = NIVEAUX.find(n => n.id === id);
    if (niveau.prop) await preparerSignatures();
    const base = niveau.pool === "celebres" ? celebres() : pays;
    partie = { pile: melange(base).slice(0, QUESTIONS), numero: 0, bonnes: 0 };
    return suivante();
  }

  function propositions(cible, pool) {
    const memeContinent = melange(pool.filter(p => p !== cible && p.continent === cible.continent));
    const autres = melange(pool.filter(p => p !== cible && p.continent !== cible.continent));
    const faux = memeContinent.slice(0, 3);
    for (const p of autres) { if (faux.length >= 3) break; faux.push(p); }
    return melange([cible, ...faux]);
  }

  function suivante() {
    const cible = partie.pile[partie.numero++];
    const pool = niveau.pool === "celebres" ? celebres() : pays;
    q = {
      cible, fini: false, bonne: false,
      choix: niveau.mode === "qcm" ? propositions(cible, pool) : null,
      teinte: niveau.teinte ? 70 + hasard(200) : 0, // rotation de teinte en degrés
      equiv: niveau.prop ? equivalents(cible) : [],
    };
    return q;
  }

  function repondre(p) {
    const ok = p === q.cible || q.equiv.includes(p);
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
    Commun.noter("drapeaux", niveau.id, e);
    return { etoiles: e, bonnes: partie.bonnes, sur: QUESTIONS };
  }

  return {
    NIVEAUX, QUESTIONS, PALETTE, charger, demarrer, suivante, repondre, suggestions, resume,
    signature, src, ecartSig,
    get q() { return q; }, get partie() { return partie; }, get niveau() { return niveau; },
    get pays() { return pays; }, get sigs() { return sigs; },
  };
})();

if (typeof module !== "undefined") module.exports = Drapeau; // pour les tests
