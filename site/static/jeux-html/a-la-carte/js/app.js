// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Écrans, saisie et affichage.
"use strict";

(() => {
  const $ = id => document.getElementById(id);

  const MODES = {
    villes: {
      titre: "Villes mystère", niveaux: () => Ville.niveaux("france"), lancer: (id, g) => lancerVille(id, "france", g),
      aide: "Trouve la ville cherchée avec la distance qui la sépare de trois villes. Chaque mauvaise ville te donne un indice de plus.",
    },
    capitales: {
      titre: "Capitales mystère", niveaux: () => Ville.niveaux("monde"), lancer: (id, g) => lancerVille(id, "monde", g),
      accueil: "3 capitales par partie",
      aide: "Trouve la capitale cherchée avec la distance qui la sépare de trois autres capitales. Chaque mauvaise capitale te donne un indice de plus.",
    },
    drapeaux: {
      titre: "Drapeaux", niveaux: () => Drapeau.NIVEAUX, lancer: (id, g) => lancerDrapeau(id, g),
      aide: "Dix drapeaux par partie, au calme, sans chrono. Fais 9 bonnes réponses sur 10 pour 3 étoiles.",
    },
    pays: {
      titre: "Pays", niveaux: () => Pays.NIVEAUX, lancer: (id, g) => lancerDrapeau(id, g),
      aide: "Dix silhouettes par partie, au calme, sans chrono. Fais 9 bonnes réponses sur 10 pour 3 étoiles.",
    },
  };
  const Mo = () => (modeCourant === "pays" ? Pays : Drapeau);
  let modeCourant = "villes";
  let actionSuite = () => {};

  let avecCarte = false;
  function montrer(id) {
    if (id === "fin") { $("fin-carte").hidden = !avecCarte; avecCarte = false; }
    document.body.dataset.mode = id === "accueil" ? "accueil" : modeCourant;
    document.querySelectorAll(".ecran").forEach(e => e.classList.toggle("actif", e.id === id));
    window.scrollTo(0, 0);
  }

  function majAccueil() {
    const n = Commun.defisFaits(), serie = Commun.serieJours();
    $("defis-resume").textContent = n || serie
      ? `Défis du jour : ${n} sur ${Object.keys(MODES).length} faits${serie > 1 ? ` · ${serie} jours de suite` : ""}`
      : "Un défi du jour t'attend dans chaque jeu !";
    for (const [mode, m] of Object.entries(MODES)) {
      const niv = m.niveaux();
      const n = niv.reduce((s, v) => s + Commun.etoilesDe(mode, v.id), 0);
      $("etoiles-" + mode).textContent = n ? `${n} / ${niv.length * 3} ★` : (m.accueil || (mode === "villes" ? "3 villes par partie" : mode === "pays" ? "10 silhouettes par partie" : "10 drapeaux par partie"));
    }
  }

  /* ===================== Défis du jour ===================== */

  function emojis(resultats) {
    return resultats.map(r => (typeof r === "boolean" ? (r ? "🟩" : "🟥") : r >= 3 ? "🟩" : r >= 1 ? "🟨" : "🟥")).join("");
  }

  async function partager(mode, res) {
    const [, mois, jour] = res.date.split("-");
    const texte = `À la carte ! — défi du jour ${jour}/${mois}\n${MODES[mode].titre} (${res.niveau}) : ${res.score}\n${emojis(res.resultats)}\n` +
      (location.protocol.startsWith("http") ? location.origin + location.pathname : "");
    if (navigator.share) { try { await navigator.share({ text: texte }); return; } catch (e) { /* annulé : on copie */ } }
    try { await navigator.clipboard.writeText(texte); Fun.bulle("content", "Résultat copié !", 1800); }
    catch (e) { window.prompt("Copie ce texte pour le partager :", texte); }
  }

  /** Affiche le résultat d'un défi du jour (à la fin de la partie, ou en revenant le voir). */
  function montrerDefi(mode, res, nouveau) {
    const serie = Commun.serieJours();
    $("fin-titre").textContent = "Défi du jour";
    $("fin-etoiles").innerHTML = `<span class="score-defi">${res.score}</span><span class="emojis-defi">${emojis(res.resultats)}</span>`;
    $("fin-texte").textContent = `${MODES[mode].titre}, niveau ${res.niveau}. ` + (serie > 1 ? `${serie} jours de suite !` : "Reviens demain pour un nouveau défi.");
    $("fin-carte").hidden = true;
    $("fin-suite").textContent = "Partager mon résultat";
    actionSuite = () => partager(mode, res);
    if (nouveau) Fun.finPartie(res.etoiles ?? 1);
    majAccueil();
    montrer("fin");
  }

  function defiTermine(mode, niveau, resultats, score, etoiles) {
    const res = { date: Commun.dateIso(), niveau, resultats, score, etoiles };
    Commun.defiNoter(mode, res);
    montrerDefi(mode, Commun.defiFait(mode), true);
  }

  function carteDefi(conteneur, mode, m, niveaux) {
    const idNiveau = Commun.niveauDuJour();
    const niv = niveaux.find(n => n.id === idNiveau) || niveaux[0];
    const fait = Commun.defiFait(mode);
    const b = document.createElement("button");
    b.className = "niveau defi";
    b.innerHTML = `<span class="corps"><b>Défi du jour</b><small>${fait
      ? `Fait : ${fait.score}. Touche pour voir et partager`
      : `Niveau ${niv.nom} aujourd'hui. Les mêmes questions pour tout le monde`}</small></span>` +
      `<span class="etoiles" aria-hidden="true">${fait ? "✓" : "🏆"}</span>`;
    b.addEventListener("click", () => (fait ? montrerDefi(mode, fait, false) : m.lancer(niv.id, Commun.graineDuJour(mode))));
    conteneur.appendChild(b);
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
    carteDefi(conteneur, mode, m, niveaux);
    niveaux.forEach((n, i) => {
      const b = document.createElement("button");
      b.className = "niveau";
      const prec = i === 0 ? 3 : Commun.etoilesDe(mode, niveaux[i - 1].id);
      const ouvert = prec >= 2;
      const e = Commun.etoilesDe(mode, n.id);
      b.disabled = !ouvert;
      b.innerHTML = `<span class="corps"><b>${n.nom}</b><small>${ouvert ? n.texte : "Fais 2 étoiles au niveau précédent"}</small></span>` +
        `<span class="etoiles" role="img" aria-label="${e} étoile${e > 1 ? "s" : ""} sur 3">${ouvert ? Commun.etoilesHtml(e) : "🔒"}</span>`;
      b.addEventListener("click", () => m.lancer(n.id));
      conteneur.appendChild(b);
    });
    montrer("niveaux");
  }

  /* ===================== Villes mystère ===================== */
  const champ = $("champ"), listeSugg = $("suggestions");
  let sugg = [], actif = -1;

  async function lancerVille(id, jeu = "france", graine = null) {
    try { await Ville.charger(jeu); }
    catch (e) { alert("Les données des villes n'ont pas pu être chargées."); return; }
    Fun.nouvellePartie();
    Ville.demarrer(id, graine);
    afficherVille();
    montrer("jeu");
    champ.focus({ preventScroll: true });
  }

  function afficherVille() {
    const { etat, niveau, partie } = Ville;
    $("jeu-titre").textContent = niveau.nom;
    const monde = partie.cle === "capitales";
    $("jeu-compteur").textContent = `${monde ? "Capitale" : "Ville"} ${partie.numero} / ${Ville.VILLES_PAR_PARTIE}`;
    $("jeu-consigne").textContent = monde ? "Où se trouve la capitale mystère ?" : "Où se trouve la ville mystère ?";
    champ.placeholder = monde ? "Tape le nom d'une capitale…" : "Tape le nom d'une ville…";
    $("jeu-barre").style.width = ((partie.numero - 1) / Ville.VILLES_PAR_PARTIE * 100) + "%";
    $("refs").innerHTML = etat.refs.map(r =>
      `<li class="ref"><span class="nom">${r.n}</span><span class="dist">${Commun.km(Commun.distance(r, etat.cible))}</span></li>`).join("");
    $("essais").innerHTML = "";
    $("message").textContent = "";
    champ.value = "";
    fermerSugg();
    $("abandon").hidden = false;
    majMiniCarte();
  }

  /** Carte en direct : les villes de départ, et un cercle de distance par ville proposée (la réponse reste cachée). */
  function majMiniCarte() {
    const { etat } = Ville;
    Carte.dessiner($("mini-carte"), { cible: etat.cible, refs: etat.refs, essais: etat.essais, secret: true, jeuCarte: Ville.jeu.carte })
      .then(() => { $("mini-carte").hidden = false; })
      .catch(() => { $("mini-carte").hidden = true; });
  }

  function ligneEssai(e) {
    const li = document.createElement("li");
    li.className = "essai" + (e.gagne ? " gagne" : "");
    const fl = !e.gagne && Ville.niveau.fleche
      ? `<svg class="boussole" viewBox="0 0 24 24" role="img" aria-label="direction : ${Commun.pointCardinal(e.cap)}" style="transform:rotate(${Math.round(e.cap)}deg)">` +
        `<circle cx="12" cy="12" r="11" fill="#fff" stroke="#0B3D91" stroke-width="1.5"/><path d="M12 3l3.6 9h-7.2z" fill="#FF9800"/><path d="M12 21l-3.6-9h7.2z" fill="#8AA0C8"/></svg>` : "";
    li.innerHTML = `<span><span class="nom">${e.ville.n}</span><small>${e.ville.d}</small></span>` +
      `<span class="dist">${e.gagne ? "Trouvé !" : Commun.km(e.dist)}${fl}</span>`;
    return li;
  }

  function jouerVille(ville) {
    const e = Ville.proposer(ville);
    $("essais").prepend(ligneEssai(e));
    if (e.gagne) Fun.reponse(true); else { Fun.jouerSon("faux"); Fun.bulle("triste", "Pas encore !", 1000); majMiniCarte(); }
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
    $("fin-suite").textContent = derniere ? "Voir mon résultat" : (partie.cle === "capitales" ? "Capitale suivante" : "Ville suivante");
    actionSuite = suiteVille;
    avecCarte = true;
    montrer("fin");
    Carte.dessiner($("fin-carte-svg"), { cible: etat.cible, refs: etat.refs, essais: etat.essais })
      .then(() => { $("fin-legende").innerHTML = Carte.legende(etat.cible, etat.refs, etat.essais); })
      .catch(() => { $("fin-carte").hidden = true; });
  }

  function suiteVille() {
    if (Ville.partie.points.length >= Ville.VILLES_PAR_PARTIE) {
      const r = Ville.resumePartie();
      if (Ville.partie.graine) {
        const total = r.points.reduce((a, b) => a + b, 0);
        defiTermine(Ville.partie.cle, Ville.niveau.id, r.points, `${total} ★ sur ${3 * r.points.length}`, Math.floor(total / r.points.length));
        return;
      }
      $("fin-titre").textContent = "Partie terminée";
      $("fin-etoiles").innerHTML = Commun.etoilesHtml(r.moyenne);
      Fun.finPartie(r.moyenne);
      $("fin-texte").textContent = r.moyenne >= 2
        ? "Bien joué ! Tu peux passer au niveau suivant."
        : "Encore un essai pour décrocher 2 étoiles et débloquer la suite.";
      $("fin-suite").textContent = "Rejouer ce niveau";
      $("fin-carte").hidden = true;
      const id = Ville.niveau.id, jeu = Ville.partie.cle === "capitales" ? "monde" : "france";
      actionSuite = () => lancerVille(id, jeu);
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

  $("abandon").addEventListener("click", () => { Ville.abandonner(); Fun.reponse(false); finVille(false); });

  /* ===================== Drapeaux ===================== */
  const dChamp = $("drap-champ");
  const saisieDrap = creerSaisie(dChamp, $("drap-sugg"), v => Mo().suggestions(v), v => repondreDrapeau(v),
    p => `<span>${p.nom}</span>`);
  dChamp.addEventListener("keydown", ev => {
    if (ev.key === "Enter" && !$("drap-sugg").children.length && dChamp.value.trim())
      $("drap-retour").textContent = "Je ne connais pas ce pays dans la liste.";
  });

  async function lancerDrapeau(id, graine = null) {
    $("drap-titre").textContent = "Chargement…";
    Fun.nouvellePartie();
    montrer("drap");
    try { await Mo().demarrer(id, graine); }
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
    Fun.reponse(ok);
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
      if (M.partie.graine) { defiTermine(modeCourant, M.niveau.id, r.resultats, `${r.bonnes} / ${r.sur}`, r.etoiles); return; }
      $("fin-titre").textContent = "Partie terminée";
      $("fin-etoiles").innerHTML = Commun.etoilesHtml(r.etoiles);
      Fun.finPartie(r.etoiles);
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
  document.querySelector(".mode-capitales").addEventListener("click", () => afficherNiveaux("capitales"));

  // Les autres modes (js/app-couleurs.js) s'inscrivent ici.
  window.AppAPI = {
    MODES, montrer, majAccueil, afficherNiveaux,
    boutonMode(mode) { document.querySelector(".mode-" + mode).addEventListener("click", () => afficherNiveaux(mode)); },
    suite(f) { actionSuite = f; },
    defiTermine, creerSaisie,
    carteFin() { avecCarte = true; },
  };

  const majSon = () => { $("son").textContent = "Son : " + (Fun.son ? "oui" : "non"); $("son").setAttribute("aria-pressed", String(Fun.son)); };
  $("son").addEventListener("click", () => { Fun.basculerSon(); majSon(); });
  majSon();

  majAccueil();
  if ("serviceWorker" in navigator) navigator.serviceWorker.register("sw.js").catch(() => {});
})();
