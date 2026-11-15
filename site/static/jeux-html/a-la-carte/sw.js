// À la carte ! — service worker : tout le jeu fonctionne hors ligne.
// Le scope est limité au dossier du jeu (chemins relatifs). Changer VERSION à chaque mise à jour.
const VERSION = "alacarte-9";
const FICHIERS = [
  "./", "index.html", "css/style.css", "css/fun.css", "css/mise-en-page.css", "js/commun.js", "js/fun.js", "js/ville.js", "js/drapeau.js", "js/pays.js", "js/couleurs.js", "js/carte.js", "js/decouverte.js", "js/app-decouverte.js", "js/app-couleurs.js", "js/app.js",
  "data/villes-france.json", "data/pays.json", "data/formes.json", "data/couleurs.json", "data/capitales.json", "data/faits.json", "data/carte-france.json", "data/carte-monde.json", "manifest.webmanifest",
  "fonts/Baloo2.ttf", "fonts/Nunito.ttf",
  "img/logo.png", "img/icone-192.png", "img/icone-512.png",
];

self.addEventListener("install", e => {
  e.waitUntil(caches.open(VERSION).then(async c => {
    await c.addAll(FICHIERS);
    // Les drapeaux : la liste vient de pays.json.
    const pays = await (await fetch("data/pays.json")).json();
    await c.addAll(pays.map(p => `data/drapeaux/${p.code}.svg`));
  }).then(() => self.skipWaiting()));
});
self.addEventListener("activate", e => {
  e.waitUntil(caches.keys()
    .then(cles => Promise.all(cles.filter(c => c !== VERSION).map(c => caches.delete(c))))
    .then(() => self.clients.claim()));
});
self.addEventListener("fetch", e => {
  if (e.request.method !== "GET") return;
  e.respondWith(caches.match(e.request).then(r => r || fetch(e.request)));
});
