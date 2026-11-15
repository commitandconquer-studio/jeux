// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Carte de fin de ville mystère : villes, cercles de distance et flèches vers la réponse.
// Fonds de carte : Natural Earth (domaine public), voir tools/a-la-carte-cartes.py.
"use strict";

const Carte = (() => {
  const NS = "http://www.w3.org/2000/svg";
  const RAYON_TERRE = 6371;
  const fonds = {};

  async function fond(nom) {
    if (!fonds[nom]) fonds[nom] = await (await fetch(`data/carte-${nom}.json`)).json();
    return fonds[nom];
  }

  const rad = d => d * Math.PI / 180;
  /** Écart de longitude ramené entre −180 et 180. */
  const ecartLon = (a, b) => ((a - b + 540) % 360) - 180;

  /** Points d'un cercle de distance d (km) autour de c, en longitude/latitude (non repliées). */
  function cercle(c, d, n = 90) {
    const pts = [], delta = d / RAYON_TERRE, p1 = rad(c.lat), l1 = rad(c.lon);
    for (let i = 0; i <= n; i++) {
      const t = rad(i * 360 / n);
      const p2 = Math.asin(Math.sin(p1) * Math.cos(delta) + Math.cos(p1) * Math.sin(delta) * Math.cos(t));
      const l2 = l1 + Math.atan2(Math.sin(t) * Math.sin(delta) * Math.cos(p1), Math.cos(delta) - Math.sin(p1) * Math.sin(p2));
      pts.push({ lat: p2 / rad(1), dlon: ecartLon(l2 / rad(1), c.lon) });
    }
    return pts;
  }

  function el(nom, attrs = {}, parent) {
    const e = document.createElementNS(NS, nom);
    for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, v);
    if (parent) parent.appendChild(e);
    return e;
  }

  /** Choisit la carte : la France si tout y est, sinon le monde. */
  function choisirCarte(points, zone) {
    const [x0, y0, x1, y1] = zone;
    return points.every(p => p.lon > x0 && p.lon < x1 && p.lat > y0 && p.lat < y1) ? "france" : "monde";
  }

  /**
   * Dessine la carte dans `conteneur`.
   * cible : la ville cherchée ; refs : les trois villes de départ ; essais : les villes proposées.
   */
  async function dessiner(conteneur, { cible, refs, essais, secret = false, jeuCarte = "france" }) {
    const FRANCE = [-9, 40, 12, 52.5];
    const autres = refs.concat(essais.map(e => e.ville).filter(v => v !== cible && !refs.includes(v)));
    // En mode « secret » (pendant la partie), la ville cherchée n'est ni dessinée ni utilisée pour cadrer.
    const visibles = secret ? autres : [cible, ...autres];
    const centre = secret ? (refs[0] || cible) : cible;
    const nom = secret && jeuCarte === "monde" ? "monde" : choisirCarte(visibles, FRANCE);
    const donnees = await fond(nom);
    const monde = nom === "monde";

    // Longitudes ramenées autour de la cible (pour les cartes qui passent la ligne de changement de date).
    const L = v => centre.lon + (monde ? ecartLon(v.lon, centre.lon) : v.lon - centre.lon);
    const lats = visibles.map(v => v.lat);
    const latMoy = lats.reduce((a, b) => a + b, 0) / lats.length;
    const kx = Math.max(0.45, Math.cos(rad(monde ? latMoy : 46.5)));
    const px = v => L(v) * kx, py = v => -v.lat;

    // Cadre : tous les points, avec une marge et une taille minimale, en 4:3.
    const xs = visibles.map(px), ys = visibles.map(py);
    const minLarg = (monde ? (secret ? 70 : 30) : (secret ? 5 : 3)) * kx;
    let w = Math.max(Math.max(...xs) - Math.min(...xs), minLarg) * 1.3;
    let h = Math.max(Math.max(...ys) - Math.min(...ys), minLarg * 0.75) * 1.3;
    if (w / h > 4 / 3) h = w * 3 / 4; else w = h * 4 / 3;
    const cx = (Math.max(...xs) + Math.min(...xs)) / 2, cy = (Math.max(...ys) + Math.min(...ys)) / 2;
    let vx = cx - w / 2;
    // On élargit vers la droite si un nom de ville dépasse du cadre.
    const toutes = visibles;
    for (let i = 0; i < 4; i++) {
      const u0 = w / 100;
      const fin = Math.max(...toutes.map(v => px(v) + u0 * (2 + v.n.length * 1.75)));
      if (fin <= vx + w - u0) break;
      w = fin - vx + u0 * 2; h = w * 3 / 4;
    }
    const vy = cy - h / 2;

    const svg = el("svg", { viewBox: `${vx} ${vy} ${w} ${h}`, class: "carte-svg", role: "img",
      "aria-label": "Carte avec la ville cherchée, les trois villes de départ et des flèches vers la réponse" });
    el("rect", { x: vx - w, y: vy - h, width: w * 3, height: h * 3, fill: "#BFE0F7" }, svg);

    // Terre (répétée de part et d'autre pour la carte du monde).
    const defs = el("defs", {}, svg);
    el("path", { id: "terre", d: donnees.d, fill: "#FFF3D6", stroke: "#0B3D91", "stroke-width": "0.7",
      "stroke-opacity": ".55", "vector-effect": "non-scaling-stroke" }, defs);
    for (const dec of monde ? [-360, 0, 360] : [0]) {
      const g = el("g", { transform: `translate(${(dec) * kx} 0) scale(${kx} -1)` }, svg);
      el("use", { href: "#terre" }, g);
    }
    // Les longitudes de la terre sont en absolu ; on recale selon L() avec le même décalage que la cible.
    // (la cible est à lon réelle ; L(cible) = cible.lon, donc aucun décalage n'est nécessaire.)

    const u = w / 100;           // unité de dessin : 1 % de la largeur
    // Cercles de distance : autour des villes de départ (à la fin), ou des villes proposées (pendant la partie).
    const anneau = (v, d, attrs) => {
      let chemin = "", precedent = null;
      cercle(v, d).forEach(({ lat, dlon }) => {
        const x = (L(v) + dlon) * kx, y = -lat;
        chemin += (precedent === null || Math.abs(dlon - precedent) > 180 ? "M" : "L") + x.toFixed(3) + " " + y.toFixed(3);
        precedent = dlon;
      });
      el("path", { d: chemin, fill: "none", "vector-effect": "non-scaling-stroke", ...attrs }, svg);
    };
    if (secret) {
      const proposes = essais.filter(e => !e.gagne);
      proposes.forEach((e, i) => anneau(e.ville, e.dist, {
        stroke: "#E07B00", "stroke-width": "2", opacity: ".85", pathLength: "1",
        class: i === proposes.length - 1 ? "anneau-neuf" : "",
      }));
    } else {
      refs.forEach(r => anneau(r, Commun.distance(r, cible), { stroke: "#1E90FF", "stroke-width": "1.6", "stroke-dasharray": "6 5", opacity: ".8" }));
    }

    // Flèches vers la réponse.
    const couleurs = { ref: "#0B3D91", essai: "#E07B00" };
    const marqueur = (id, couleur) => {
      const m = el("marker", { id, markerUnits: "userSpaceOnUse", markerWidth: u * 3.2, markerHeight: u * 3.2,
        refX: u * 3, refY: u * 1.6, orient: "auto" }, defs);
      el("path", { d: `M0 0L${u * 3.2} ${u * 1.6}L0 ${u * 3.2}Z`, fill: couleur }, m);
    };
    marqueur("pointe-ref", couleurs.ref); marqueur("pointe-essai", couleurs.essai);
    const fleche = (v, type) => {
      const x1 = px(v), y1 = py(v), x2 = px(cible), y2 = py(cible);
      const dx = x2 - x1, dy = y2 - y1, n = Math.hypot(dx, dy) || 1;
      const arret = Math.min(u * 2.6, n * 0.4);                 // la pointe s'arrête avant la cible
      el("line", { x1, y1, x2: x2 - dx / n * arret, y2: y2 - dy / n * arret, stroke: couleurs[type],
        "stroke-width": u * 0.5, "marker-end": `url(#pointe-${type})` }, svg);
    };
    if (!secret) {
      refs.forEach(v => fleche(v, "ref"));
      essais.filter(e => !e.gagne).forEach(e => fleche(e.ville, "essai"));
    }

    // Villes et noms.
    const point = (v, couleur, r, nomVille, gras) => {
      el("circle", { cx: px(v), cy: py(v), r: u * r, fill: couleur, stroke: "#fff", "stroke-width": u * 0.35 }, svg);
      const t = el("text", { x: px(v) + u * (r + 0.8), y: py(v) + u * 1.1, "font-size": u * 3.1,
        "font-family": "Nunito, sans-serif", "font-weight": gras ? 900 : 800, fill: "#0B2A63",
        stroke: "#fff", "stroke-width": u * 0.9, "paint-order": "stroke", "stroke-linejoin": "round" }, svg);
      t.textContent = nomVille;
    };
    refs.forEach(v => point(v, couleurs.ref, 1.1, v.n, false));
    essais.filter(e => !e.gagne).forEach(e => point(e.ville, couleurs.essai, 1.1, e.ville.n, false));
    if (!secret) {
      point(cible, "#34C759", 1.7, cible.n, true);
      // Un avion en papier file vers la réponse.
      const a = refs[0] || (essais[0] && essais[0].ville) || cible, x1 = px(a), y1 = py(a), x2 = px(cible), y2 = py(cible);
      const avion = el("g", { class: "avion" }, svg);
      el("path", { d: `M${-u * 1.8} ${-u * 1.3}L${u * 2.2} 0L${-u * 1.8} ${u * 1.3}L${-u * 1.1} 0Z`, fill: "#fff", stroke: "#0B2A63", "stroke-width": u * 0.25, "stroke-linejoin": "round" }, avion);
      el("path", { d: `M${-u * 1.1} 0L${u * 2.2} 0`, stroke: "#0B2A63", "stroke-width": u * 0.2 }, avion);
      el("animateMotion", { dur: "3.2s", repeatCount: "indefinite", rotate: "auto", path: `M${x1} ${y1}L${x2} ${y2}`,
        keyPoints: "0;1;1", keyTimes: "0;0.85;1", calcMode: "linear" }, avion);
    }

    conteneur.innerHTML = "";
    conteneur.appendChild(svg);
    return nom;
  }

  /** Légende : les distances de chaque ville à la ville cherchée. */
  function legende(cible, refs, essais) {
    const ligne = (couleur, v, texte) =>
      `<li><i style="background:${couleur}"></i><span>${v.n}</span><b>${texte}</b></li>`;
    return refs.map(r => ligne("#0B3D91", r, Commun.km(Commun.distance(r, cible)))).join("") +
      essais.filter(e => !e.gagne).map(e => ligne("#E07B00", e.ville, Commun.km(e.dist))).join("") +
      ligne("#34C759", cible, "★");
  }

  return { dessiner, legende, cercle };
})();

if (typeof module !== "undefined") module.exports = Carte; // pour les tests
