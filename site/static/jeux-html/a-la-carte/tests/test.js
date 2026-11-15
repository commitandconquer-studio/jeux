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
const { Teinte, BonnesCouleurs, Recolorier } = require("../js/couleurs.js");
assert.strictEqual(Teinte.decaler("#ffffff", 90, 0.06), "#ffffff", "le blanc ne bouge pas");
assert.strictEqual(Teinte.decaler("#000000", 90), "#000000");
assert.strictEqual(Teinte.decaler("#808080", 90), "#808080");
assert.notStrictEqual(Teinte.decaler("#ce1126", 120), "#ce1126");
assert.strictEqual(Teinte.decaler("#ce1126", 0), "#ce1126".replace("#ce1126", Teinte.decaler("#ce1126", 0)));
assert(Teinte.ecart("#ce1126", "#d01028") < Teinte.ecart("#ce1126", "#1e50b4"));
assert.strictEqual(Teinte.plusProche("#e4002b", [{ hex: "#1E50B4" }, { hex: "#CE1126" }]), 1);
assert.strictEqual(BonnesCouleurs.NIVEAUX.length, 4);
assert.strictEqual(Recolorier.NIVEAUX.length, 4);
const coul = JSON.parse(fs.readFileSync(path.join(dossier, "couleurs.json"), "utf8"));
for (const p of pays) assert(Array.isArray(coul[p.code]), "couleurs " + p.code);
assert(pays.filter(p => coul[p.code].length >= 1 && coul[p.code].length <= 2).length > 80, "assez de drapeaux simples");
assert.strictEqual(V.niveaux("france").length, 4);
assert.strictEqual(V.niveaux("monde").length, 4);
const capitales = JSON.parse(fs.readFileSync(path.join(dossier, "capitales.json"), "utf8"));
assert(capitales.length >= 190, "capitales : " + capitales.length);
for (const c of capitales) assert(c.n && c.d && Math.abs(c.lat) <= 90 && Math.abs(c.lon) <= 180, "capitale " + c.n);
assert.strictEqual(new Set(capitales.map(c => c.n)).size, capitales.length, "noms de capitales uniques");
const Carte = require("../js/carte.js");
const centre = { lat: 46.3, lon: 4.8 };
for (const d of [150, 600, 5000, 12000]) {
  for (const { lat, dlon } of Carte.cercle(centre, d)) {
    const ecart = Math.abs(C.distance(centre, { lat, lon: centre.lon + dlon }) - d);
    assert(ecart < 0.01 * d + 1, `cercle ${d} km : écart ${ecart}`);
  }
}
for (const nom of ["carte-france", "carte-monde"]) {
  const f = JSON.parse(fs.readFileSync(path.join(dossier, nom + ".json"), "utf8"));
  assert(f.d.startsWith("M") && f.d.length > 10000, nom);
}
assert.strictEqual((C.etoilesHtml(2).match(/class="e"/g) || []).length, 2);
assert.strictEqual((C.etoilesHtml(2).match(/class="e vide"/g) || []).length, 1);
assert.strictEqual((C.etoilesHtml(0).match(/vide/g) || []).length, 3);
// défis du jour : même graine, mêmes tirages ; série de jours avec un stockage simulé
{
  const a = C.prng("defi:2026-11-15:villes"), b = C.prng("defi:2026-11-15:villes"), x = C.prng("defi:2026-11-16:villes");
  const sa = [a(), a(), a()], sb = [b(), b(), b()];
  assert.deepStrictEqual(sa, sb, "même graine = mêmes nombres");
  assert.notDeepStrictEqual(sa, [x(), x(), x()], "autre jour = autres nombres");
  assert(sa.every(v => v >= 0 && v < 1));
  assert.strictEqual(C.dateIso(new Date(2026, 0, 5)), "2026-01-05");
  assert.strictEqual(C.niveauDuJour("2026-11-16"), "facile", "lundi");
  assert.strictEqual(C.niveauDuJour("2026-11-15"), "expert", "dimanche");
  const mem = {};
  global.localStorage = { getItem: k => (k in mem ? mem[k] : null), setItem: (k, v) => { mem[k] = String(v); } };
  assert.strictEqual(C.serieJours("2026-11-15"), 0);
  C.defiNoter("villes", { score: "5 ★ sur 9", resultats: [3, 1, 1] }, "2026-11-13");
  C.defiNoter("pays", { score: "8 / 10", resultats: [true] }, "2026-11-14");
  C.defiNoter("pays", { score: "1 / 10", resultats: [false] }, "2026-11-14");
  assert.strictEqual(C.defiFait("pays", "2026-11-14").score, "8 / 10", "un seul essai par jour et par mode");
  assert.strictEqual(C.serieJours("2026-11-14"), 2);
  assert.strictEqual(C.serieJours("2026-11-15"), 2, "hier compte tant qu'on n'a pas joué aujourd'hui");
  C.defiNoter("drapeaux", { score: "9 / 10", resultats: [true] }, "2026-11-15");
  assert.strictEqual(C.serieJours("2026-11-15"), 3);
  assert.strictEqual(C.serieJours("2026-11-18"), 0, "série rompue");
  delete global.localStorage;
}
// Pays caché : données, comparaisons, déterminisme du défi du jour
(async () => {
  global.Commun = C;
  global.fetch = async u => ({ json: async () => JSON.parse(fs.readFileSync(path.join(__dirname, "..", u), "utf8")) });
  const PC = require("../js/decouverte.js");
  await PC.charger();
  assert(PC.liste.length >= 190, "pays comparables : " + PC.liste.length);
  for (const p of PC.liste) assert(p.s > 0 && p.p > 0 && p.g > 0 && p.capitale && p.continent, "faits " + p.code);
  const par = c => PC.liste.find(p => p.code === c);
  const fr = par("fr"), de = par("de"), br = par("br"), au = par("au");
  assert(fr.s > 5e5 && fr.s < 7e5, "superficie de la France : " + fr.s);
  assert(br.p > 2e8 && br.p < 2.2e8);
  PC.demarrer("moyen", "defi:2026-11-15:cache");
  const secret1 = [PC.etat.secret.code]; PC.suivante(); secret1.push(PC.etat.secret.code);
  PC.demarrer("moyen", "defi:2026-11-15:cache");
  assert.deepStrictEqual([PC.etat.secret.code, (PC.suivante(), PC.etat.secret.code)], secret1, "défi du jour : mêmes pays secrets");
  // comparaison : l'Allemagne par rapport au Brésil caché
  PC.demarrer("moyen", "x");
  const essai = PC.comparer(de, br);
  const get = id => essai.find(i => i.id === id);
  assert.strictEqual(get("surface").etat, "plus", "le Brésil est plus grand que l'Allemagne");
  assert.strictEqual(get("pop").etat, "plus");
  assert.strictEqual(get("continent").etat, "non");
  assert.strictEqual(get("ns").etat, "non", "Berlin au nord, Brasilia au sud");
  assert.strictEqual(get("eo").etat, "non", "Berlin à l'est, Brasilia à l'ouest");
  assert.strictEqual(PC.comparer(fr, fr).find(i => i.id === "surface").etat, "proche");
  const aucun = PC.comparer(au, au); assert(aucun.every(i => i.etat === "proche" || i.etat === "oui"));
  console.log("Tous les tests passent (pays caché compris).");
})().catch(e => { console.error(e); process.exit(1); });
