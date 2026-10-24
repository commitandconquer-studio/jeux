// Commit & Conquer — scène 3D de l'accueil : l'emblème « CC » flotte au-dessus
// d'une île low poly, avec des îlots et des nuages au loin. Au défilement,
// chaque plan bouge à sa vitesse (parallaxe).
import { THREE, alea, entre, ile, emblème, sapin, rocher, nuage, decorer, monter } from "./lowpoly.js";

const hero = document.querySelector(".hero");
const s = monter(hero, {
  cadrer({ camera, large, aspect, etat }) {
    // Grand écran : l'île à droite du texte (image décentrée, vue de face).
    camera.filmOffset = large ? -Math.min(9, 4 + aspect * 2.2) : 0;
    etat.cible.set(0, large ? 3.0 : -2.5, 0);
    camera.position.set(0, 8, large ? 40 : 60);
  },
  animer(t) {
    île.position.y = Math.sin(t * 0.7) * 0.25;
    île.rotation.y = -0.35 + Math.sin(t * 0.18) * 0.12;
    cc.position.y = yCC + Math.sin(t * 0.9 + 1) * 0.3;
    cc.rotation.y = 0.5 + Math.sin(t * 0.4) * 0.08;
    cc.userData.onduler(t);
    fête?.(t);
    îlots.forEach((g, i) => { g.position.y = g.userData.y0 + Math.sin(t * 0.6 + i * 2) * 0.4; });
    nuages.forEach((n) => {
      n.position.x += n.userData.v * 0.016;
      if (n.position.x > 30) n.position.x = -30;
    });
  },
});

let île, cc, yCC, îlots, nuages, fête;
if (s) {
  const loin = s.couche(-4), milieu = s.couche(3), devant = s.couche(9);

  île = new THREE.Group();
  milieu.add(île);
  const R = 5.2;
  const sol = ile(R);
  île.add(sol);
  const y = sol.userData.dessus;
  [[-3.9, -0.6, 0.85], [-3.1, 2.3, 0.7], [-4.3, 1.1, 1.0], [3.9, -1.6, 0.65], [3.3, 2.6, 0.8], [4.4, 0.6, 0.6], [-1.2, -3.9, 0.55], [1.6, 3.9, 0.6]]
    .forEach(([x, z, h]) => { const a = sapin(h); a.position.set(x, y - 0.05, z); a.rotation.y = alea() * 3; île.add(a); });
  [[-1.0, 1.2, 0.8], [1.4, -0.6, 0.6], [-4.3, -2.0, 0.5], [4.3, 2.1, 0.6], [0.2, 3.2, 0.45]]
    .forEach(([x, z, t]) => { const r = rocher(t); r.position.set(x, y + 0.1, z); île.add(r); });

  fête = decorer(île, y, R, devant);

  // L'emblème flotte au-dessus de l'île, de face et un peu penché vers nous.
  cc = emblème();
  const taille = 0.82;
  cc.scale.setScalar(taille);
  yCC = y + 3.4 * taille + 3.4;
  cc.position.set(0, yCC, 0);
  cc.rotation.set(0.32, 0.5, 0);
  devant.add(cc);

  îlots = [[19, 9, -30, 2.0], [15, -3, -22, 1.6], [-22, 13, -40, 1.8]].map(([x, yy, z, r]) => {
    const g = new THREE.Group();
    const petit = ile(r, 9);
    g.add(petit);
    const a = sapin(0.7); a.position.y = petit.userData.dessus; g.add(a);
    g.position.set(x, yy, z);
    g.userData.y0 = yy;
    loin.add(g);
    return g;
  });
  nuages = Array.from({ length: 7 }, () => {
    const n = nuage();
    n.position.set(entre(-30, 30), entre(6, 13), entre(-34, -12));
    n.scale.setScalar(entre(0.7, 1.3));
    n.userData.v = entre(0.25, 0.6);
    loin.add(n);
    return n;
  });
  s.demarrer();
}
