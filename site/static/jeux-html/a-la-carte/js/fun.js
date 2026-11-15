// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Le côté fun : mascotte, séries de bonnes réponses, confettis, petits sons.
"use strict";

const Fun = (() => {
  const calme = window.matchMedia && matchMedia("(prefers-reduced-motion: reduce)").matches;
  let serie = 0, meilleure = 0;
  let son = true;
  try { son = localStorage.getItem("alacarte.son") !== "0"; } catch (e) { /* ignoré */ }

  /* ---------- Sons (synthétisés, aucun fichier) ---------- */
  let audio = null;
  function note(freq, debut, duree, type = "sine", volume = 0.12) {
    if (!son) return;
    try {
      audio = audio || new (window.AudioContext || window.webkitAudioContext)();
      const o = audio.createOscillator(), g = audio.createGain();
      o.type = type; o.frequency.value = freq;
      const t = audio.currentTime + debut;
      g.gain.setValueAtTime(0.0001, t);
      g.gain.exponentialRampToValueAtTime(volume, t + 0.02);
      g.gain.exponentialRampToValueAtTime(0.0001, t + duree);
      o.connect(g); g.connect(audio.destination);
      o.start(t); o.stop(t + duree + 0.05);
    } catch (e) { /* pas de son : tant pis */ }
  }
  const SONS = {
    bon: () => { note(660, 0, 0.12); note(880, 0.1, 0.18); },
    faux: () => { note(240, 0, 0.16, "triangle"); note(180, 0.12, 0.22, "triangle"); },
    serie: () => { note(660, 0, 0.1); note(880, 0.08, 0.1); note(1100, 0.16, 0.2); },
    etoile: n => note(520 + n * 160, 0, 0.18, "sine", 0.1),
    victoire: () => [523, 659, 784, 1047].forEach((f, i) => note(f, i * 0.11, 0.25)),
    clic: () => note(420, 0, 0.05, "sine", 0.06),
  };
  function jouerSon(nom, arg) { if (SONS[nom]) SONS[nom](arg); }
  function basculerSon() {
    son = !son;
    try { localStorage.setItem("alacarte.son", son ? "1" : "0"); } catch (e) { /* ignoré */ }
    if (son) jouerSon("bon");
    return son;
  }

  /* ---------- Mascotte et bulle ---------- */
  const $ = id => document.getElementById(id);
  let timerBulle = 0;
  function bulle(humeur, texte, duree = 1500) {
    const b = $("bulle");
    if (!b) return;
    b.className = "bulle montre " + humeur;
    $("bulle-texte").textContent = texte;
    clearTimeout(timerBulle);
    timerBulle = setTimeout(() => { b.className = "bulle"; }, duree);
  }
  const BRAVO = ["Bravo !", "Bien vu !", "Excellent !", "Tu es fort !", "Yes !", "Parfait !"];
  const RATE = ["Aïe…", "Presque !", "Pas grave !", "Ça viendra !", "Oups !"];
  const choix = t => t[Math.floor(Math.random() * t.length)];

  /* ---------- Série de bonnes réponses ---------- */
  function majSerie() {
    const s = $("serie");
    if (!s) return;
    s.hidden = serie < 2;
    $("serie-n").textContent = serie;
    s.classList.remove("saute"); void s.offsetWidth; s.classList.add("saute");
  }
  function nouvellePartie() { serie = 0; majSerie(); }

  /** À appeler après chaque réponse : ok = bonne réponse. */
  function reponse(ok) {
    if (ok) {
      serie++; meilleure = Math.max(meilleure, serie);
      if (serie >= 3 && serie % 3 === 0) {
        jouerSon("serie"); bulle("ravi", `${serie} de suite !`, 1700); confettis(24);
      } else { jouerSon("bon"); bulle("content", choix(BRAVO)); }
    } else {
      serie = 0; jouerSon("faux"); bulle("triste", choix(RATE));
    }
    majSerie();
  }

  /* ---------- Confettis ---------- */
  function confettis(n = 90) {
    if (calme) return;
    const cv = $("confettis");
    if (!cv) return;
    const ctx = cv.getContext("2d");
    cv.width = innerWidth; cv.height = innerHeight; cv.hidden = false;
    const couleurs = ["#1E90FF", "#34C759", "#FF9800", "#FFD54A", "#0B3D91", "#FF5C8A"];
    const ps = Array.from({ length: n }, () => ({
      x: cv.width / 2 + (Math.random() - .5) * 120, y: cv.height * 0.55,
      vx: (Math.random() - .5) * 11, vy: -Math.random() * 13 - 4,
      r: Math.random() * 6 + 4, a: Math.random() * 6, va: (Math.random() - .5) * .4,
      c: couleurs[Math.floor(Math.random() * couleurs.length)],
    }));
    let t = 0;
    (function image() {
      ctx.clearRect(0, 0, cv.width, cv.height);
      ps.forEach(p => {
        p.vy += 0.35; p.x += p.vx; p.y += p.vy; p.a += p.va;
        ctx.save(); ctx.translate(p.x, p.y); ctx.rotate(p.a);
        ctx.fillStyle = p.c; ctx.fillRect(-p.r / 2, -p.r / 4, p.r, p.r / 2); ctx.restore();
      });
      if (++t < 110) requestAnimationFrame(image); else { ctx.clearRect(0, 0, cv.width, cv.height); cv.hidden = true; }
    })();
  }

  /** Fin de partie : sons et confettis selon les étoiles. */
  function finPartie(etoiles) {
    if (etoiles >= 1) {
      for (let i = 0; i < etoiles; i++) setTimeout(() => jouerSon("etoile", i), 250 + i * 300);
      if (etoiles >= 2) setTimeout(() => { jouerSon("victoire"); confettis(etoiles === 3 ? 140 : 80); }, 250 + etoiles * 300);
      bulle(etoiles === 3 ? "ravi" : "content", etoiles === 3 ? "Trois étoiles !" : etoiles === 2 ? "Bien joué !" : "C'est un début !", 2200);
    } else bulle("triste", "On réessaie ?", 2000);
  }

  return { reponse, nouvellePartie, finPartie, confettis, bulle, jouerSon, basculerSon, get son() { return son; }, get serie() { return serie; } };
})();
