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
    return "★".repeat(n) + `<span class="vide">${"★".repeat(max - n)}</span>`;
  }

  return { distance, cap, fleche, pointCardinal, normaliser, km, etoilesDe, noter, etoilesHtml };
})();

if (typeof module !== "undefined") module.exports = Commun; // pour les tests
