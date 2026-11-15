// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Écrans, saisie et affichage.
"use strict";

(() => {
  const $ = id => document.getElementById(id);

  const MODES = {
    villes: {
      titre: "Villes mystère", niveaux: () => Ville.NIVEAUX,
      aide: "Trouve la ville cherchée avec la distance qui la sépare de trois villes. Chaque mauvaise ville te donne un indice de plus.",
    },
    drapeaux: {
      titre: "Drapeaux", niveaux: () => Drapeau.NIVEAUX,
      aide: "Dix drapeaux par partie, au calme, sans chrono. Fais 9 bonnes réponses sur 10 pour 3 étoiles.",
    },
    pays: {
      titre: "Pays", niveaux: () => Pays.NIVEAUX,
      aide: "Dix silhouettes par partie, au calme, sans chrono. Fais 9 bonnes réponses sur 10 pour 3 étoiles.",
    },
  };
  const Mo = () => (modeCourant === "pays" ? Pays : Drapeau);
  let modeCourant = "villes";
  let actionSuite = () => {};

  function montrer(id) {
    document.querySelectorAll(".ecran").forEach(e => e.classList.toggle("actif", e.id === id));
    window.scrollTo(0, 0);
  }

  function majAccueil() {
    for (const [mode, m] of Object.entries(MODES)) {
      const niv = m.niveaux();
      const n = niv.reduce((s, v) => s + Commun.etoilesDe(mode, v.id), 0);
      $("etoiles-" + mode).textContent = n ? `${n} / ${niv.length * 3} ★` : (mode === "villes" ? "3 villes par partie" : mode === "pays" ? "10 silhouettes par partie" : "10 drapeaux par partie");
    }
  }

  /* Niveaux */
  function afficherNiveaux(mode = modeCourant) {
    modeCourant = mode;
    const m = MODES[mode];
    $("niveaux-titre").textContent = m.titre;
    $("niveaux-aide").textContent = m.aide;
    const conteneur = $("liste-niveaux");
    conteneur.innerHTML = "";
    const niveaux = m.niveaux();
    niveaux.forEach((n, i) => {
      const b = document.createElement("button");
      b.className = "niveau";
      const prec = i === 0 ? 3 : Commun.etoilesDe(mode, niveaux[i - 1].id);
      const ouvert = prec >= 2;
      const e = Commun.etoilesDe(mode, n.id);
      b.disabled = !ouvert;
      b.innerHTML = `<span class="corps"><b>${n.nom}</b><small>${ouvert ? n.texte : "Fais 2 étoiles au niveau précédent"}</small></span>` +
        `<span class="etoiles" role="img" aria-label="${e} étoile${e > 1 ? "s" : ""} sur 3">${ouvert ? Commun.etoilesHtml(e) : "🔒"}</span>`;
      b.addEventListener("click", () => (mode === "villes" ? lancerVille : lancerDrapeau)(n.id));
      conteneur.appendChild(b);
    });
    montrer("niveaux");
  }

  /* ===================== Villes mystère ===================== */
  const champ = $("champ"), listeSugg = $("suggestions");
  let sugg = [], actif = -1;

  async function lancerVille(id) {
    try { await Ville.charger(); }
    catch (e) { alert("Les données des villes n'ont pas pu être chargées."); return; }
    Ville.demarrer(id);
    afficherVille();
    montrer("jeu");
    champ.focus({ preventScroll: true });
  }

  function afficherVille() {
    const { etat, niveau, partie } = Ville;
    $("jeu-titre").textContent = niveau.nom;
    $("jeu-compteur").textContent = `Ville ${partie.numero} / ${Ville.VILLES_PAR_PARTIE}`;
    $("jeu-barre").style.width = ((partie.numero - 1) / Ville.VILLES_PAR_PARTIE * 100) + "%";
    $("refs").innerHTML = etat.refs.map(r =>
      `<li class="ref"><span class="nom">${r.n}</span><span class="dist">${Commun.km(Commun.distance(r, etat.cible))}</span></li>`).join("");
    $("essais").innerHTML = "";
    $("message").textContent = "";
    champ.value = "";
    fermerSugg();
    $("abandon").hidden = false;
  }

  function ligneEssai(e) {
    const li = document.createElement("li");
    li.className = "essai" + (e.gagne ? " gagne" : "");
    const fl = !e.gagne && Ville.niveau.fleche
      ? `<span class="fleche" aria-label="direction : ${Commun.pointCardinal(e.cap)}">${Commun.fleche(e.cap)}</span>` : "";
    li.innerHTML = `<span><span class="nom">${e.ville.n}</span><small>${e.ville.d}</small></span>` +
      `<span class="dist">${e.gagne ? "Trouvé !" : Commun.km(e.dist)}${fl}</span>`;
    return li;
  }

  function jouerVille(ville) {
    const e = Ville.proposer(ville);
    $("essais").prepend(ligneEssai(e));
    champ.value = "";
    fermerSugg();
    $("message").textContent = "";
    if (e.gagne) setTimeout(() => finVille(true), 700);
    else champ.focus({ preventScroll: true });
  }

  function finVille(trouve) {
    const { etat, partie } = Ville;
    const derniere = partie.numero >= Ville.VILLES_PAR_PARTIE;
    $("fin-titre").textContent = trouve ? "Bravo !" : "C'était…";
    $("fin-etoiles").innerHTML = Commun.etoilesHtml(etat.etoiles);
    $("fin-texte").textContent = trouve
      ? `${etat.cible.n} (${etat.cible.d}), trouvée en ${etat.essais.length} essai${etat.essais.length > 1 ? "s" : ""}.`
      : `${etat.cible.n} (${etat.cible.d}).`;
    $("fin-suite").textContent = derniere ? "Voir mon résultat" : "Ville suivante";
    actionSuite = suiteVille;
    montrer("fin");
  }

  function suiteVille() {
    if (Ville.partie.points.length >= Ville.VILLES_PAR_PARTIE) {
      const r = Ville.resumePartie();
      $("fin-titre").textContent = "Partie terminée";
      $("fin-etoiles").innerHTML = Commun.etoilesHtml(r.moyenne);
      $("fin-texte").textContent = r.moyenne >= 2
        ? "Bien joué ! Tu peux passer au niveau suivant."
        : "Encore un essai pour décrocher 2 étoiles et débloquer la suite.";
      $("fin-suite").textContent = "Rejouer ce niveau";
      const id = Ville.niveau.id;
      actionSuite = () => lancerVille(id);
      majAccueil();
      return;
    }
    Ville.suivante();
    afficherVille();
    montrer("jeu");
    champ.focus({ preventScroll: true });
  }

  /* Liste de suggestions, commune aux deux modes à saisie */
  function creerSaisie(input, ul, fournir, valider, afficher) {
    let s = [], a = -1;
    const fermer = () => { ul.hidden = true; ul.innerHTML = ""; s = []; a = -1; input.setAttribute("aria-expanded", "false"); };
    const maj = () => {
      s = fournir(input.value);
      a = s.length ? 0 : -1;
      ul.innerHTML = "";
      s.forEach((v, i) => {
        const li = document.createElement("li");
        li.setAttribute("role", "option");
        li.className = i === a ? "actif" : "";
        li.innerHTML = afficher(v);
        li.addEventListener("mousedown", ev => { ev.preventDefault(); valider(v); });
        ul.appendChild(li);
      });
      ul.hidden = !s.length;
      input.setAttribute("aria-expanded", String(s.length > 0));
    };
    const bouger = d => {
      if (!s.length) return;
      a = (a + d + s.length) % s.length;
      [...ul.children].forEach((li, i) => li.classList.toggle("actif", i === a));
    };
    input.addEventListener("input", maj);
    input.addEventListener("blur", () => setTimeout(fermer, 150));
    input.addEventListener("keydown", ev => {
      if (ev.key === "ArrowDown") { ev.preventDefault(); bouger(1); }
      else if (ev.key === "ArrowUp") { ev.preventDefault(); bouger(-1); }
      else if (ev.key === "Escape") fermer();
      else if (ev.key === "Enter") {
        ev.preventDefault();
        if (a >= 0) valider(s[a]);
        else if (input.value.trim()) return "inconnu";
      }
    });
    return { fermer };
  }

  function fermerSugg() { saisieVille.fermer(); }
  const saisieVille = creerSaisie(champ, listeSugg, v => Ville.suggestions(v), jouerVille,
    v => `<span>${v.n}</span><small>${v.d}</small>`);
  champ.addEventListener("keydown", ev => {
    if (ev.key === "Enter" && !listeSugg.children.length && champ.value.trim())
      $("message").textContent = "Je ne connais pas cette ville dans la liste.";
  });

  $("abandon").addEventListener("click", () => { Ville.abandonner(); finVille(false); });

  /* ===================== Drapeaux ===================== */
  const dChamp = $("drap-champ");
  const saisieDrap = creerSaisie(dChamp, $("drap-sugg"), v => Mo().suggestions(v), v => repondreDrapeau(v),
    p => `<span>${p.nom}</span>`);
  dChamp.addEventListener("keydown", ev => {
    if (ev.key === "Enter" && !$("drap-sugg").children.length && dChamp.value.trim())
      $("drap-retour").textContent = "Je ne connais pas ce pays dans la liste.";
  });

  async function lancerDrapeau(id) {
    $("drap-titre").textContent = "Chargement…";
    montrer("drap");
    try { await Mo().demarrer(id); }
    catch (e) { alert("Les données des drapeaux n'ont pas pu être chargées."); afficherNiveaux(modeCourant); return; }
    await afficherDrapeau();
  }

  async function afficherDrapeau() {
    const M = Mo();
    const { q, niveau, partie } = M;
    const formes = modeCourant === "pays";
    $("drap-titre").textContent = niveau.nom;
    $("drap-compteur").textContent = `${partie.numero} / ${M.QUESTIONS}`;
    $("drap-barre").style.width = ((partie.numero - 1) / M.QUESTIONS * 100) + "%";
    $("drap-retour").textContent = ""; $("drap-retour").className = "message";
    $("drap-suite").hidden = true;
    const img = $("drap-img"), prop = $("drap-prop");
    const forme = $("drap-forme");
    img.hidden = formes || !!niveau.prop; prop.hidden = formes || !niveau.prop; forme.hidden = !formes;
    img.style.filter = q.teinte ? `hue-rotate(${q.teinte}deg)` : "";
    $("drap-consigne").textContent = formes
      ? (niveau.miroir ? "La silhouette est tournée ou retournée. Quel est ce pays ?"
        : niveau.rotation ? "La silhouette est tournée. Quel est ce pays ?" : "Quel est ce pays ?")
      : niveau.prop ? "Quel pays a ces proportions de couleurs ?"
      : niveau.teinte ? "Les couleurs ont changé. De quel pays est ce drapeau ?" : "De quel pays est ce drapeau ?";
    if (formes) {
      forme.classList.remove("revele");
      forme.innerHTML = Pays.svg(q.cible);
      forme.firstChild.style.transform = `rotate(${q.angle}deg)` + (q.miroir ? " scaleX(-1)" : "");
    } else if (niveau.prop) {
      const sig = await Drapeau.signature(q.cible);
      prop.innerHTML = sig.map((f, i) => ({ f, c: Drapeau.PALETTE[i] })).filter(x => x.f > 0)
        .sort((a, b) => b.f - a.f)
        .map(x => `<span style="flex:${x.f};background:rgb(${x.c.rgb.join(",")})" title="${x.c.nom}"></span>`).join("");
      prop.setAttribute("aria-label", "Proportions : " + sig.map((f, i) => f > 0 ? `${Math.round(f * 100)} % ${Drapeau.PALETTE[i].nom}` : "").filter(Boolean).join(", "));
    } else {
      img.src = Drapeau.src(q.cible);
    }
    const choix = $("drap-choix");
    choix.innerHTML = "";
    $("drap-saisie").hidden = niveau.mode !== "saisie";
    choix.hidden = niveau.mode !== "qcm";
    if (niveau.mode === "qcm") {
      q.choix.forEach(p => {
        const b = document.createElement("button");
        b.textContent = p.nom; b.dataset.code = p.code;
        b.addEventListener("click", () => repondreDrapeau(p));
        choix.appendChild(b);
      });
    } else {
      dChamp.value = ""; saisieDrap.fermer();
      dChamp.focus({ preventScroll: true });
    }
  }

  function repondreDrapeau(p) {
    const M = Mo();
    const { q, niveau } = M;
    if (q.fini) return;
    const ok = M.repondre(p);
    $("drap-img").style.filter = "";                // on rend les vraies couleurs
    if (modeCourant === "pays") {                   // on remet la silhouette à l'endroit
      $("drap-forme").firstChild.style.transform = "";
      $("drap-forme").classList.add("revele");
    } else if (niveau.prop) { $("drap-img").src = Drapeau.src(q.cible); $("drap-img").hidden = false; $("drap-prop").hidden = true; }
    if (niveau.mode === "qcm") {
      document.querySelectorAll("#drap-choix button").forEach(b => {
        b.disabled = true;
        if (b.dataset.code === q.cible.code) b.classList.add("bon");
        else if (b.dataset.code === p.code) b.classList.add("faux");
      });
    } else { $("drap-saisie").hidden = true; saisieDrap.fermer(); }
    const autres = q.equiv.length ? ` (aussi possible : ${q.equiv.map(x => x.nom).join(", ")})` : "";
    const r = $("drap-retour");
    r.textContent = ok ? (p === q.cible ? "Bravo !" : `Bravo ! C'était ${q.cible.nom}, mais ${p.nom} a les mêmes proportions.`)
      : `C'était : ${q.cible.nom}.${niveau.prop ? autres : ""}`;
    r.className = "message" + (ok ? "" : " mauvais");
    const derniere = M.partie.numero >= M.QUESTIONS;
    const suite = $("drap-suite");
    suite.textContent = derniere ? "Voir mon résultat" : "Suivant";
    suite.hidden = false;
    suite.focus({ preventScroll: true });
  }

  $("drap-suite").addEventListener("click", async () => {
    const M = Mo();
    if (M.partie.numero >= M.QUESTIONS) {
      const r = M.resume();
      $("fin-titre").textContent = "Partie terminée";
      $("fin-etoiles").innerHTML = Commun.etoilesHtml(r.etoiles);
      $("fin-texte").textContent = `${r.bonnes} bonne${r.bonnes > 1 ? "s" : ""} réponse${r.bonnes > 1 ? "s" : ""} sur ${r.sur}. ` +
        (r.etoiles >= 2 ? "Tu peux passer au niveau suivant." : "Fais 7 bonnes réponses pour 2 étoiles et débloquer la suite.");
      $("fin-suite").textContent = "Rejouer ce niveau";
      const id = M.niveau.id;
      actionSuite = () => lancerDrapeau(id);
      majAccueil();
      montrer("fin");
      return;
    }
    M.suivante();
    await afficherDrapeau();
  });

  /* ===================== Navigation ===================== */
  $("fin-suite").addEventListener("click", () => actionSuite());
  $("fin-menu").addEventListener("click", () => afficherNiveaux());

  document.querySelectorAll("[data-retour]").forEach(b =>
    b.addEventListener("click", () => {
      if (b.dataset.retour === "niveaux") afficherNiveaux(); else montrer(b.dataset.retour);
    }));
  document.querySelector(".mode-villes").addEventListener("click", () => afficherNiveaux("villes"));
  document.querySelector(".mode-drapeaux").addEventListener("click", () => afficherNiveaux("drapeaux"));
  document.querySelector(".mode-pays").addEventListener("click", () => afficherNiveaux("pays"));

  majAccueil();
  if ("serviceWorker" in navigator) navigator.serviceWorker.register("sw.js").catch(() => {});
})();
