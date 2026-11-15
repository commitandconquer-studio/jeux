// À la carte ! — tests des calculs. Lancer : node site/static/jeux-html/a-la-carte/tests/test.js
"use strict";
const assert = require("assert");
const C = require("../js/commun.js");

const paris = { lat: 48.8566, lon: 2.3522 }, lyon = { lat: 45.764, lon: 4.8357 }, macon = { lat: 46.3265, lon: 4.8083 };

// distance : Paris–Lyon ≈ 392 km
const d = C.distance(paris, lyon);
assert(d > 385 && d < 400, "Paris–Lyon " + d);
assert.strictEqual(C.distance(paris, paris), 0);
assert(Math.abs(C.distance(paris, lyon) - C.distance(lyon, paris)) < 1e-9);

// cap : Lyon vers Paris = nord-ouest ; Lyon vers Mâcon = nord
assert.strictEqual(C.pointCardinal(C.cap(lyon, paris)), "nord-ouest");
assert.strictEqual(C.pointCardinal(C.cap(lyon, macon)), "nord");
assert.strictEqual(C.pointCardinal(C.cap({ lat: 45, lon: 5 }, { lat: 45, lon: 6 })), "est");
assert.strictEqual(C.pointCardinal(C.cap(paris, { lat: 48.8566, lon: 1 })), "ouest");
assert.strictEqual(C.fleche(0), "↑");
assert.strictEqual(C.fleche(359), "↑");
assert.strictEqual(C.fleche(180), "↓");

// saisie tolérante
assert.strictEqual(C.normaliser("  Mâcon "), "macon");
assert.strictEqual(C.normaliser("Saint-Étienne"), "saint etienne");
assert.strictEqual(C.normaliser("St Étienne"), "saint etienne");
assert.strictEqual(C.normaliser("Sainte-Menehould"), "sainte menehould");
assert.strictEqual(C.normaliser("L'Haÿ-les-Roses"), "l hay les roses");
assert.strictEqual(C.normaliser("Bœuf"), "boeuf");

assert.strictEqual(C.km(0.4), "0,4 km");
assert(C.km(392.3).startsWith("392"));

// étoiles : 1 à 3 selon le nombre d'essais (Ville.etoilesPour est interne : on teste via une partie)
const V = require("../js/ville.js");
assert.strictEqual(V.NIVEAUX.length, 4);
const D = require("../js/drapeau.js");
assert.strictEqual(D.NIVEAUX.length, 4);
assert.strictEqual(D.ecartSig([1, 0], [0, 1]), 1);
assert.strictEqual(D.ecartSig([0.5, 0.5], [0.5, 0.5]), 0);
assert(Math.abs(D.ecartSig([0.4, 0.6], [0.5, 0.5]) - 0.1) < 1e-9);

// données : chaque pays a un drapeau, un nom et un continent
const fs = require("fs"), path = require("path");
const dossier = path.join(__dirname, "../data");
const pays = JSON.parse(fs.readFileSync(path.join(dossier, "pays.json"), "utf8"));
assert(pays.length >= 190);
for (const p of pays) {
  assert(p.nom && p.continent, p.code);
  assert(fs.existsSync(path.join(dossier, "drapeaux", p.code + ".svg")), "drapeau " + p.code);
}
assert.strictEqual(new Set(pays.map(p => p.code)).size, pays.length, "codes uniques");
const villes = JSON.parse(fs.readFileSync(path.join(dossier, "villes-france.json"), "utf8"));
assert.strictEqual(villes.filter(v => v.t === "p").length, 101);
assert(villes.filter(v => v.t === "s").length > 200);
for (const v of villes) assert(v.lat > -22 && v.lat < 52 && v.lon > -63 && v.lon < 56, v.n);
const P = require("../js/pays.js");
assert.strictEqual(P.NIVEAUX.length, 4);
const formes = JSON.parse(fs.readFileSync(path.join(dossier, "formes.json"), "utf8"));
const jouables = pays.filter(p => formes[p.code] && formes[p.code].s >= 0.45);
assert(jouables.length >= 150, "silhouettes jouables : " + jouables.length);
for (const p of jouables) assert(/^M[\d. LZM-]+Z$/.test(formes[p.code].d), "tracé " + p.code);
console.log("Tous les tests passent.");
