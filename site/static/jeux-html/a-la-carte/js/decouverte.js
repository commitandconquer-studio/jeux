// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Mode « Pays caché » : on tape des pays, le jeu compare avec le pays caché
// (superficie, population, PIB, hémisphères, continent, distance et direction).
// Chiffres : Natural Earth (domaine public), voir tools/a-la-carte-faits.py.
"use strict";

const PaysCache = (() => {
  const NIVEAUX = [
    { id: "facile", nom: "Facile", texte: "Pays célèbres, tous les indices", celebres: true, boussole: true, distance: true, pibHab: true, etoiles: [5, 8] },
    { id: "moyen", nom: "Moyen", texte: "Tous les pays, tous les indices", celebres: false, boussole: true, distance: true, pibHab: true, etoiles: [6, 10] },
    { id: "difficile", nom: "Difficile", texte: "Sans la boussole ni le PIB par habitant", celebres: false, boussole: false, distance: true, pibHab: false, etoiles: [7, 12] },
    { id: "expert", nom: "Expert", texte: "Sans la distance : seulement les comparaisons", celebres: false, boussole: false, distance: false, pibHab: false, etoiles: [8, 14] },
  ];
  const PAR_PARTIE = 3;
  const CONTINENTS = { Asia: "Asie", Europe: "Europe", Africa: "Afrique", "North America": "Amérique du Nord", "South America": "Amérique du Sud", Oceania: "Océanie" };
  const SURFACE_MIN_CACHE = 500;   // sous ce seuil, les chiffres sont trop approximatifs pour en faire un pays caché

  let liste = [], index = [];
  let niveau = null, partie = null, etat = null;
  let alea = Math.random;
  const hasard = n => Math.floor(alea() * n);

  async function charger() {
    if (liste.length) return;
    const [p, f, c] = await Promise.all([fetch("data/pays.json"), fetch("data/faits.json"), fetch("data/capitales.json")]);
    const pays = await p.json(), faits = await f.json(), capitales = await c.json();
    const cap = {};
    capitales.forEach(x => { cap[x.c] = x; });
    liste = pays.filter(x => faits[x.code] && faits[x.code].g > 0 && cap[x.code]).map(x => ({
      code: x.code, nom: x.nom, continent: CONTINENTS[x.continent] || x.continent, sl: x.sl,
      s: faits[x.code].s, p: faits[x.code].p, g: faits[x.code].g * 1e6,        // PIB en dollars
      gh: faits[x.code].p ? faits[x.code].g * 1e6 / faits[x.code].p : 0,       // PIB par habitant
      capitale: cap[x.code].n, lat: cap[x.code].lat, lon: cap[x.code].lon,
    }));
    index = liste.map(x => Commun.normaliser(x.nom));
  }

  /* ---------- Mise en forme des chiffres ---------- */
  const sig = (v, n = 2) => Number(v.toPrecision(n));
  function nombre(v) {
    if (v >= 1e9) return sig(v / 1e9).toLocaleString("fr-FR") + " milliards";
    if (v >= 1e6) return sig(v / 1e6).toLocaleString("fr-FR") + " millions";
    if (v >= 1e4) return sig(v / 1e3).toLocaleString("fr-FR") + " mille";
    return Math.round(v).toLocaleString("fr-FR");
  }
  const surface = v => (v >= 1e4 ? sig(v, 3) : sig(v, 2)).toLocaleString("fr-FR") + " km²";
  const habitants = v => nombre(v) + " hab.";
  const dollars = v => nombre(v) + " $";
  const parHab = v => Math.round(sig(v, 2)).toLocaleString("fr-FR") + " $/hab.";

  /** Un indice numérique : le pays caché est-il plus grand, plus petit, ou à peu près pareil ? */
  function numerique(id, nom, essai, secret, cle, fmt, mots) {
    const r = secret[cle] / essai[cle];
    const etatNum = r >= 0.85 && r <= 1.18 ? "proche" : r > 1 ? "plus" : "moins";
    return { id, nom, val: fmt(essai[cle]), etat: etatNum, mot: mots[etatNum] };
  }

  function comparer(essai, secret) {
    const idx = [
      numerique("surface", "Superficie", essai, secret, "s", surface, { plus: "plus grand", moins: "plus petit", proche: "à peu près pareil" }),
      numerique("pop", "Population", essai, secret, "p", habitants, { plus: "plus peuplé", moins: "moins peuplé", proche: "à peu près pareil" }),
      numerique("pib", "PIB", essai, secret, "g", dollars, { plus: "plus riche", moins: "moins riche", proche: "à peu près pareil" }),
    ];
    if (niveau.pibHab) idx.push(numerique("pibhab", "PIB par habitant", essai, secret, "gh", parHab, { plus: "plus aisé", moins: "moins aisé", proche: "à peu près pareil" }));
    const meme = (id, nom, v, ok, oui, non) => ({ id, nom, val: v, etat: ok ? "oui" : "non", mot: ok ? oui : non });
    idx.push(meme("continent", "Continent", essai.continent, essai.continent === secret.continent, "même continent", "autre continent"));
    idx.push(meme("ns", "Hémisphère", essai.lat >= 0 ? "Nord" : "Sud", (essai.lat >= 0) === (secret.lat >= 0), "même hémisphère", "autre hémisphère"));
    idx.push(meme("eo", "Hémisphère", essai.lon >= 0 ? "Est" : "Ouest", (essai.lon >= 0) === (secret.lon >= 0), "même hémisphère", "autre hémisphère"));
    return idx;
  }

  function reserve() {
    if (niveau.celebres) return liste.slice().sort((a, b) => b.sl - a.sl).slice(0, 70).filter(x => x.s >= SURFACE_MIN_CACHE);
    return liste.filter(x => x.s >= SURFACE_MIN_CACHE);
  }

  function demarrer(id, graine = null) {
    niveau = NIVEAUX.find(n => n.id === id);
    partie = { deja: new Set(), points: [], numero: 0, cle: "cache", graine };
    return suivante();
  }

  function suivante() {
    partie.numero++;
    alea = partie.graine ? Commun.prng(partie.graine + ":" + partie.numero) : Math.random;
    const pool = reserve().filter(x => !partie.deja.has(x.code));
    const secret = pool[hasard(pool.length)];
    partie.deja.add(secret.code);
    etat = { secret, essais: [], fini: false };
    return etat;
  }

  function suggestions(saisie, max = 6) {
    const s = Commun.normaliser(saisie);
    if (!s) return [];
    const debut = [], contient = [];
    index.forEach((n, i) => {
      if (etat.essais.some(e => e.pays === liste[i])) return;
      if (n.startsWith(s)) debut.push(liste[i]);
      else if (n.includes(s)) contient.push(liste[i]);
    });
    return debut.concat(contient).slice(0, max);
  }

  function etoilesPour(nb) { return nb <= niveau.etoiles[0] ? 3 : nb <= niveau.etoiles[1] ? 2 : 1; }

  function proposer(pays) {
    const secret = etat.secret, gagne = pays === secret;
    const a = { lat: pays.lat, lon: pays.lon }, b = { lat: secret.lat, lon: secret.lon };
    const essai = { pays, gagne, indices: gagne ? [] : comparer(pays, secret), dist: Commun.distance(a, b), cap: Commun.cap(a, b) };
    etat.essais.push(essai);
    if (gagne) { etat.fini = true; etat.etoiles = etoilesPour(etat.essais.length); partie.points.push(etat.etoiles); }
    return essai;
  }

  function abandonner() { etat.fini = true; etat.etoiles = 0; partie.points.push(0); }

  function resume() {
    const total = partie.points.reduce((a, b) => a + b, 0);
    const moyenne = Math.floor(total / partie.points.length);
    if (!partie.graine) Commun.noter("cache", niveau.id, moyenne);    // un défi du jour ne débloque rien
    return { moyenne, points: partie.points.slice() };
  }

  return {
    NIVEAUX, PAR_PARTIE, charger, demarrer, suivante, suggestions, proposer, abandonner, resume, comparer,
    get etat() { return etat; }, get partie() { return partie; }, get niveau() { return niveau; }, get liste() { return liste; },
  };
})();

if (typeof module !== "undefined") module.exports = PaysCache; // pour les tests
