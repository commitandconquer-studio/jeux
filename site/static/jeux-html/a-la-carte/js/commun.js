// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Fonctions communes : calculs géographiques, saisie tolérante, sauvegarde locale.
"use strict";

const Commun = (() => {
  const R_TERRE = 6371; // km

  /** Distance à vol d'oiseau (haversine), en kilomètres. */
  function distance(a, b) {
    const rad = Math.PI / 180;
    const dLat = (b.lat - a.lat) * rad, dLon = (b.lon - a.lon) * rad;
    const h = Math.sin(dLat / 2) ** 2 +
      Math.cos(a.lat * rad) * Math.cos(b.lat * rad) * Math.sin(dLon / 2) ** 2;
    return 2 * R_TERRE * Math.asin(Math.min(1, Math.sqrt(h)));
  }

  /** Cap initial de a vers b, en degrés (0 = nord, 90 = est). */
  function cap(a, b) {
    const rad = Math.PI / 180;
    const dLon = (b.lon - a.lon) * rad;
    const y = Math.sin(dLon) * Math.cos(b.lat * rad);
    const x = Math.cos(a.lat * rad) * Math.sin(b.lat * rad) -
      Math.sin(a.lat * rad) * Math.cos(b.lat * rad) * Math.cos(dLon);
    return (Math.atan2(y, x) / rad + 360) % 360;
  }

  const FLECHES = ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"];
  const POINTS = ["nord", "nord-est", "est", "sud-est", "sud", "sud-ouest", "ouest", "nord-ouest"];
  function fleche(c) { return FLECHES[Math.round(c / 45) % 8]; }
  function pointCardinal(c) { return POINTS[Math.round(c / 45) % 8]; }

  /** Minuscules, sans accents, tirets et apostrophes comme des espaces, « st » = « saint ». */
  function normaliser(s) {
    s = s.normalize("NFD").replace(/[̀-ͯ]/g, "").toLowerCase()
      .replace(/œ/g, "oe").replace(/æ/g, "ae")
      .replace(/[-'’.,]/g, " ").replace(/\s+/g, " ").trim();
    return s.replace(/\bsts?\b/g, m => (m === "st" ? "saint" : "sainte"));
  }

  function km(d) {
    return d < 10 ? d.toFixed(1).replace(".", ",") + " km" : Math.round(d).toLocaleString("fr-FR") + " km";
  }

  /* Sauvegarde locale : tout est facultatif, le jeu marche sans. */
  const CLE = "alacarte.v1";
  function lire() {
    try { return JSON.parse(localStorage.getItem(CLE)) || {}; } catch (e) { return {}; }
  }
  function ecrire(o) {
    try { localStorage.setItem(CLE, JSON.stringify(o)); } catch (e) { /* ignoré */ }
  }
  function etoilesDe(mode, niveau) {
    const s = lire();
    return (s[mode] && s[mode][niveau]) || 0;
  }
  function noter(mode, niveau, etoiles) {
    const s = lire();
    s[mode] = s[mode] || {};
    if (etoiles > (s[mode][niveau] || 0)) s[mode][niveau] = etoiles;
    ecrire(s);
  }
  function etoilesHtml(n, max = 3) {
    return Array.from({ length: max }, (_, i) => `<span class="e${i < n ? "" : " vide"}" style="--i:${i}">★</span>`).join("");
  }

  /* ---------- Défis du jour : hasard à graine, date, résultats ---------- */
  /** Générateur pseudo-aléatoire (mulberry32) à partir d'un texte : mêmes questions pour tout le monde. */
  function prng(texte) {
    let h = 1779033703 ^ texte.length;
    for (let i = 0; i < texte.length; i++) { h = Math.imul(h ^ texte.charCodeAt(i), 3432918353); h = (h << 13) | (h >>> 19); }
    let a = h >>> 0;
    return () => {
      a = (a + 0x6D2B79F5) >>> 0;
      let t = Math.imul(a ^ (a >>> 15), 1 | a);
      t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
      return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
    };
  }
  /** Date locale au format AAAA-MM-JJ. */
  function dateIso(d = new Date()) {
    const p = n => String(n).padStart(2, "0");
    return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}`;
  }
  /** Niveau du défi : plus facile en début de semaine, expert le dimanche. */
  const NIVEAU_DU_JOUR = ["expert", "facile", "moyen", "moyen", "difficile", "moyen", "difficile"]; // dimanche = 0
  function niveauDuJour(date = dateIso()) {
    const [a, m, j] = date.split("-").map(Number);
    return NIVEAU_DU_JOUR[new Date(a, m - 1, j).getDay()];
  }
  const graineDuJour = (mode, date = dateIso()) => `defi:${date}:${mode}`;

  function defiFait(mode, date = dateIso()) {
    const s = lire();
    return (s.defis && s.defis[date] && s.defis[date][mode]) || null;
  }
  function defiNoter(mode, resultat, date = dateIso()) {
    const s = lire();
    s.defis = s.defis || {};
    s.defis[date] = s.defis[date] || {};
    if (!s.defis[date][mode]) s.defis[date][mode] = resultat;   // un seul essai par jour et par mode
    ecrire(s);
  }
  /** Nombre de jours de suite avec au moins un défi fait (aujourd'hui ou jusqu'à hier). */
  function serieJours(date = dateIso()) {
    const s = lire(), jours = s.defis || {};
    const [a, m, j] = date.split("-").map(Number);
    const d = new Date(a, m - 1, j);
    if (!jours[dateIso(d)] || !Object.keys(jours[dateIso(d)]).length) d.setDate(d.getDate() - 1);
    let n = 0;
    while (jours[dateIso(d)] && Object.keys(jours[dateIso(d)]).length) { n++; d.setDate(d.getDate() - 1); }
    return n;
  }
  const defisFaits = (date = dateIso()) => Object.keys((lire().defis || {})[date] || {}).length;

  return { distance, cap, fleche, pointCardinal, normaliser, km, etoilesDe, noter, etoilesHtml,
    prng, dateIso, niveauDuJour, graineDuJour, defiFait, defiNoter, serieJours, defisFaits };
})();

if (typeof module !== "undefined") module.exports = Commun; // pour les tests
