/* Commit & Conquer — ambiance du site : le moment de la journée, la saison,
   les fêtes et (si le visiteur l'a accepté) la météo de chez lui.
   Les règles sont celles des jeux (game_common::calendar) : un même
   calendrier pour tout le studio.

   Chargé tout en haut de la page (sans attendre) pour poser les classes
   moment-*, saison-*, meteo-* et fete-* sur <html> avant l'affichage.
   window.CC_AMBIANCE donne le résultat aux scènes 3D.
   En développement, `cc-ambiance-force` (localStorage) force des valeurs. */
(function () {
  "use strict";

  function jours(a, m, j) { return Math.floor(Date.UTC(a, m - 1, j) / 86400000); }

  /** Dimanche de Pâques (algorithme grégorien « anonyme »). */
  function paques(a) {
    var A = a % 19, b = Math.floor(a / 100), c = a % 100, d = Math.floor(b / 4), e = b % 4;
    var f = Math.floor((b + 8) / 25), g = Math.floor((b - f + 1) / 3);
    var h = (19 * A + b - d - g + 15) % 30, i = Math.floor(c / 4), k = c % 4;
    var l = (32 + 2 * e + 2 * i - h - k) % 7, m = Math.floor((A + 11 * h + 22 * l) / 451);
    var mois = Math.floor((h + l - 7 * m + 114) / 31), jour = ((h + l - 7 * m + 114) % 31) + 1;
    return jours(a, mois, jour);
  }

  /** Fête du jour : la même liste que dans les jeux. */
  function fete(a, m, j) {
    var n = jours(a, m, j), ecart = n - paques(a);
    if (ecart === -47) return "mardi-gras";
    if (ecart === -7) return "rameaux";
    if (ecart >= -3 && ecart <= -1) return "semaine-sainte";
    if (ecart === 0 || ecart === 1) return "paques";
    if (ecart === 39) return "ascension";
    if (ecart === 49 || ecart === 50) return "pentecote";
    if (m === 1 && j === 6) return "epiphanie";
    if (m === 2 && j === 2) return "chandeleur";
    if (m === 10 && j === 31) return "halloween";
    if (m === 11 && (j === 1 || j === 2)) return "toussaint";
    if (m === 12 && j === 6) return "saint-nicolas";
    if (m === 12 && j >= 24 && j <= 26) return "noel";
    // Avent : du 4ᵉ dimanche avant Noël au 23 décembre.
    var noel = jours(a, 12, 25), jourNoel = (new Date(Date.UTC(a, 11, 25)).getUTCDay() + 6) % 7;
    var premier = noel - (jourNoel + 1) - 21;
    if (n >= premier && n < noel - 1) return "avent";
    return "";
  }

  function moment(h) {
    if (h >= 6 && h < 10) return "matin";
    if (h >= 10 && h < 18) return "jour";
    if (h >= 18 && h < 21) return "soir";
    return "nuit";
  }

  function saison(m, j) {
    var x = m * 100 + j;
    if (x >= 320 && x < 621) return "printemps";
    if (x >= 621 && x < 922) return "ete";
    if (x >= 922 && x < 1221) return "automne";
    return "hiver";
  }

  function lire(cle) { try { return JSON.parse(localStorage.getItem(cle) || "null"); } catch (e) { return null; } }

  var maintenant = new Date();
  var a = maintenant.getFullYear(), mo = maintenant.getMonth() + 1, jo = maintenant.getDate();
  var meteo = lire("cc-meteo");
  // Une météo de plus de trois heures n'est plus fiable.
  var temps = meteo && Date.now() - meteo.t < 3 * 3600 * 1000 ? meteo.temps : "";
  var amb = {
    moment: moment(maintenant.getHours()),
    saison: saison(mo, jo),
    fete: fete(a, mo, jo),
    meteo: temps,
    ville: meteo && meteo.ville || "",
  };
  var force = lire("cc-ambiance-force") || {};
  Object.keys(force).forEach(function (k) { if (force[k] !== "auto") amb[k] = force[k] === "aucune" ? "" : force[k]; });
  amb.force = force;
  amb.nuit = amb.moment === "nuit";
  window.CC_AMBIANCE = amb;
  window.CC_FETE = fete;

  var html = document.documentElement;
  html.classList.add("moment-" + amb.moment, "saison-" + amb.saison);
  if (amb.meteo) html.classList.add("meteo-" + amb.meteo);
  if (amb.fete) html.classList.add("fete-" + amb.fete);
})();
