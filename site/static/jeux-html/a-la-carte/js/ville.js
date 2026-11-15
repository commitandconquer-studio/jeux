// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Mode « Ville mystère ».
"use strict";

const Ville = (() => {
  const NIVEAUX = [
    { id: "facile", nom: "Facile", texte: "Préfectures, avec une flèche vers la ville cherchée", cible: "p", fleche: true },
    { id: "moyen", nom: "Moyen", texte: "Préfectures, sans flèche", cible: "p", fleche: false },
    { id: "difficile", nom: "Difficile", texte: "Sous-préfectures, avec une flèche", cible: "s", fleche: true },
    { id: "expert", nom: "Expert", texte: "Sous-préfectures, sans flèche", cible: "s", fleche: false },
  ];
  const VILLES_PAR_PARTIE = 3;

  let villes = [];      // toutes les villes proposées à la saisie
  let index = [];       // noms normalisés
  let niveau = null;
  let etat = null;      // la ville en cours
  let partie = null;    // la partie en cours

  async function charger() {
    if (villes.length) return;
    const r = await fetch("data/villes-france.json");
    villes = await r.json();
    index = villes.map(v => Commun.normaliser(v.n));
  }

  const hasard = n => Math.floor(Math.random() * n);

  /** Choisit trois villes de référence connues, éloignées de la cible et entre elles. */
  function choisirRefs(cible) {
    const connues = villes.filter(v => v !== cible && v.t === "p" && v.pop >= 40000);
    for (const [dmin, dmax, ecart] of [[120, 650, 120], [60, 900, 60], [0, 5000, 0]]) {
      const refs = [];
      const candidats = connues.filter(v => {
        const d = Commun.distance(v, cible);
        return d >= dmin && d <= dmax;
      });
      for (let essai = 0; essai < 200 && refs.length < 3 && candidats.length; essai++) {
        const c = candidats[hasard(candidats.length)];
        if (!refs.includes(c) && refs.every(r => Commun.distance(r, c) >= ecart)) refs.push(c);
      }
      if (refs.length === 3) return refs;
    }
    return connues.sort(() => Math.random() - .5).slice(0, 3);
  }

  function nouvelleVille() {
    const pool = villes.filter(v => v.t === niveau.cible && !partie.deja.has(v.c));
    const cible = pool[hasard(pool.length)];
    partie.deja.add(cible.c);
    etat = { cible, refs: choisirRefs(cible), essais: [], fini: false };
    return etat;
  }

  function demarrer(id) {
    niveau = NIVEAUX.find(n => n.id === id);
    partie = { deja: new Set(), points: [], numero: 0 };
    return suivante();
  }

  function suivante() {
    partie.numero++;
    return nouvelleVille();
  }

  /** Suggestions : noms commençant par la saisie d'abord, puis les noms qui la contiennent. */
  function suggestions(saisie, max = 6) {
    const q = Commun.normaliser(saisie);
    if (!q) return [];
    const debut = [], contient = [];
    index.forEach((n, i) => {
      if (etat.essais.some(e => e.ville === villes[i])) return;
      if (n.startsWith(q)) debut.push(villes[i]);
      else if (n.includes(q)) contient.push(villes[i]);
    });
    return debut.concat(contient).slice(0, max);
  }

  function etoilesPour(nbEssais) { return nbEssais <= 3 ? 3 : nbEssais <= 6 ? 2 : 1; }

  /** Propose une ville ; renvoie l'essai ajouté. */
  function proposer(ville) {
    const d = Commun.distance(ville, etat.cible);
    const gagne = ville === etat.cible;
    const essai = { ville, dist: d, cap: Commun.cap(ville, etat.cible), gagne };
    etat.essais.push(essai);
    if (gagne) { etat.fini = true; etat.etoiles = etoilesPour(etat.essais.length); partie.points.push(etat.etoiles); }
    return essai;
  }

  function abandonner() {
    etat.fini = true; etat.etoiles = 0; partie.points.push(0);
  }

  function resumePartie() {
    const total = partie.points.reduce((a, b) => a + b, 0);
    const moyenne = Math.floor(total / partie.points.length);
    Commun.noter("villes", niveau.id, moyenne);
    return { moyenne, points: partie.points.slice() };
  }

  return {
    NIVEAUX, VILLES_PAR_PARTIE, charger, demarrer, suivante, suggestions, proposer, abandonner, resumePartie,
    get etat() { return etat; }, get partie() { return partie; }, get niveau() { return niveau; },
    get villes() { return villes; },
  };
})();

if (typeof module !== "undefined") module.exports = Ville; // pour les tests
