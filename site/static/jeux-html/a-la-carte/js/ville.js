// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Mode « Ville mystère » : villes de France (préfectures, sous-préfectures) ou capitales du monde.
"use strict";

const Ville = (() => {
  const JEUX = {
    france: {
      fichier: "data/villes-france.json", cle: "villes", carte: "france",
      etoiles: [3, 6],                         // essais maximum pour 3 et 2 étoiles
      fenetres: [[120, 650, 120], [60, 900, 60], [0, 5000, 0]],   // distances des références : min, max, écart entre elles
      niveaux: [
        { id: "facile", nom: "Facile", texte: "Préfectures, avec une flèche vers la ville cherchée", cible: "p", fleche: true },
        { id: "moyen", nom: "Moyen", texte: "Préfectures, sans flèche", cible: "p", fleche: false },
        { id: "difficile", nom: "Difficile", texte: "Sous-préfectures, avec une flèche", cible: "s", fleche: true },
        { id: "expert", nom: "Expert", texte: "Sous-préfectures, sans flèche", cible: "s", fleche: false },
      ],
    },
    monde: {
      fichier: "data/capitales.json", cle: "capitales", carte: "monde",
      etoiles: [4, 8],
      fenetres: [[1500, 9000, 1500], [800, 13000, 800], [0, 25000, 0]],
      niveaux: [
        { id: "facile", nom: "Facile", texte: "Capitales célèbres, avec une flèche", celebres: true, fleche: true },
        { id: "moyen", nom: "Moyen", texte: "Toutes les capitales, avec une flèche", celebres: false, fleche: true },
        { id: "difficile", nom: "Difficile", texte: "Capitales célèbres, sans flèche", celebres: true, fleche: false },
        { id: "expert", nom: "Expert", texte: "Toutes les capitales, sans flèche", celebres: false, fleche: false },
      ],
    },
  };
  const NIVEAUX = JEUX.france.niveaux;
  const VILLES_PAR_PARTIE = 3;

  const cache = {};     // jeu -> { villes, index }
  let jeu = JEUX.france, villes = [], index = [];
  let niveau = null, etat = null, partie = null;

  async function charger(nom = "france") {
    jeu = JEUX[nom];
    if (!cache[nom]) {
      const v = await (await fetch(jeu.fichier)).json();
      cache[nom] = { villes: v, index: v.map(x => Commun.normaliser(x.n)) };
    }
    ({ villes, index } = cache[nom]);
  }

  const hasard = n => Math.floor(Math.random() * n);
  const parNotoriete = liste => liste.slice().sort((a, b) => b.pop - a.pop);

  /** Villes connues, pouvant servir de références. */
  function connues(cible) {
    const pool = jeu === JEUX.france
      ? villes.filter(v => v.t === "p" && v.pop >= 40000)
      : parNotoriete(villes).slice(0, 100);
    return pool.filter(v => v !== cible);
  }

  /** Choisit trois villes de référence connues, éloignées de la cible et entre elles. */
  function choisirRefs(cible) {
    const pool = connues(cible);
    for (const [dmin, dmax, ecart] of jeu.fenetres) {
      const refs = [];
      const candidats = pool.filter(v => {
        const d = Commun.distance(v, cible);
        return d >= dmin && d <= dmax;
      });
      for (let essai = 0; essai < 300 && refs.length < 3 && candidats.length; essai++) {
        const c = candidats[hasard(candidats.length)];
        if (!refs.includes(c) && refs.every(r => Commun.distance(r, c) >= ecart)) refs.push(c);
      }
      if (refs.length === 3) return refs;
    }
    return pool.sort(() => Math.random() - .5).slice(0, 3);
  }

  function reserve() {
    if (jeu === JEUX.france) return villes.filter(v => v.t === niveau.cible);
    return niveau.celebres ? parNotoriete(villes).slice(0, 70) : villes;
  }

  function nouvelleVille() {
    const pool = reserve().filter(v => !partie.deja.has(v.c));
    const cible = pool[hasard(pool.length)];
    partie.deja.add(cible.c);
    etat = { cible, refs: choisirRefs(cible), essais: [], fini: false };
    return etat;
  }

  function demarrer(id) {
    niveau = jeu.niveaux.find(n => n.id === id);
    partie = { deja: new Set(), points: [], numero: 0, cle: jeu.cle };
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

  function etoilesPour(nbEssais) { return nbEssais <= jeu.etoiles[0] ? 3 : nbEssais <= jeu.etoiles[1] ? 2 : 1; }

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
    Commun.noter(partie.cle, niveau.id, moyenne);
    return { moyenne, points: partie.points.slice() };
  }

  return {
    NIVEAUX, VILLES_PAR_PARTIE, charger, demarrer, suivante, suggestions, proposer, abandonner, resumePartie,
    niveaux: nom => JEUX[nom].niveaux,
    get etat() { return etat; }, get partie() { return partie; }, get niveau() { return niveau; },
    get villes() { return villes; }, get jeu() { return jeu; },
  };
})();

if (typeof module !== "undefined") module.exports = Ville; // pour les tests
