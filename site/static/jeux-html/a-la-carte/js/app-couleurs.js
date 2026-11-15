// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Écrans des modes « Les bonnes couleurs » et « Recolorier ».
"use strict";

(() => {
  const $ = id => document.getElementById(id);
  const A = window.AppAPI;
  const s = n => (n > 1 ? "s" : "");

  function fin(mode, M, rejouer) {
    const r = M.resume();
    $("fin-titre").textContent = "Partie terminée";
    $("fin-etoiles").innerHTML = Commun.etoilesHtml(r.etoiles);
    $("fin-texte").textContent = `${r.bonnes} bonne${s(r.bonnes)} réponse${s(r.bonnes)} sur ${r.sur}. ` +
      (r.etoiles >= 2 ? "Tu peux passer au niveau suivant." : "Fais 7 bonnes réponses pour 2 étoiles et débloquer la suite.");
    $("fin-suite").textContent = "Rejouer ce niveau";
    const id = M.niveau.id;
    A.suite(() => rejouer(id));
    A.majAccueil();
    A.montrer("fin");
  }

  /* ===================== Les bonnes couleurs ===================== */
  async function lancerCouleurs(id) {
    $("cou-titre").textContent = "Chargement…";
    A.montrer("cou");
    try { await BonnesCouleurs.demarrer(id); }
    catch (e) { alert("Les données n'ont pas pu être chargées."); A.afficherNiveaux("couleurs"); return; }
    await afficherCouleurs();
  }

  async function afficherCouleurs() {
    const { q, niveau, partie } = BonnesCouleurs;
    $("cou-titre").textContent = niveau.nom;
    $("cou-compteur").textContent = `${partie.numero} / ${BonnesCouleurs.QUESTIONS}`;
    $("cou-barre").style.width = ((partie.numero - 1) / BonnesCouleurs.QUESTIONS * 100) + "%";
    $("cou-pays").textContent = q.cible.nom;
    $("cou-retour").textContent = ""; $("cou-retour").className = "message";
    $("cou-suite").hidden = true;
    const choix = $("cou-choix");
    choix.innerHTML = "";
    await BonnesCouleurs.images();
    q.options.forEach((o, i) => {
      const b = document.createElement("button");
      b.innerHTML = `<img src="${o.url}" alt="Drapeau, proposition ${i + 1}">`;
      b.addEventListener("click", () => repondreCouleurs(i));
      choix.appendChild(b);
    });
  }

  function repondreCouleurs(i) {
    const { q } = BonnesCouleurs;
    if (q.fini) return;
    const ok = BonnesCouleurs.repondre(i);
    [...$("cou-choix").children].forEach((b, k) => {
      b.disabled = true;
      if (q.options[k].vrai) b.classList.add("bon"); else if (k === i) b.classList.add("faux");
    });
    $("cou-retour").textContent = ok ? "Bravo !" : "Raté : ce sont les couleurs du drapeau coché en vert.";
    $("cou-retour").className = "message" + (ok ? "" : " mauvais");
    const derniere = BonnesCouleurs.partie.numero >= BonnesCouleurs.QUESTIONS;
    $("cou-suite").textContent = derniere ? "Voir mon résultat" : "Suivant";
    $("cou-suite").hidden = false;
    $("cou-suite").focus({ preventScroll: true });
  }

  $("cou-suite").addEventListener("click", async () => {
    if (BonnesCouleurs.partie.numero >= BonnesCouleurs.QUESTIONS) { fin("couleurs", BonnesCouleurs, lancerCouleurs); return; }
    BonnesCouleurs.suivante();
    await afficherCouleurs();
  });

  /* ===================== Recolorier ===================== */
  let pastille = 0; // groupe de couleurs en cours de réglage

  async function lancerRec(id) {
    $("rec-titre").textContent = "Chargement…";
    A.montrer("rec");
    try { await Recolorier.demarrer(id); }
    catch (e) { alert("Les données n'ont pas pu être chargées."); A.afficherNiveaux("recolorier"); return; }
    await afficherRec();
  }

  async function majImage() { $("rec-img").src = await Recolorier.image(); }

  function dessinerPastilles() {
    const { q, niveau } = Recolorier;
    const conteneur = $("rec-pastilles");
    conteneur.innerHTML = "";
    q.groupes.forEach((g, i) => {
      const b = document.createElement("button");
      b.className = "pastille" + (i === pastille ? " choisie" : "");
      b.setAttribute("aria-label", `Couleur ${i + 1} du drapeau`);
      b.style.background = g.choix !== null ? niveau.palette[g.choix].hex : Teinte.decaler(g.rep, g.dh);
      b.addEventListener("click", () => { if (!q.fini) { pastille = i; dessinerPastilles(); } });
      conteneur.appendChild(b);
    });
  }

  async function afficherRec() {
    const { q, niveau, partie } = Recolorier;
    $("rec-titre").textContent = niveau.nom;
    $("rec-compteur").textContent = `${partie.numero} / ${Recolorier.QUESTIONS}`;
    $("rec-barre").style.width = ((partie.numero - 1) / Recolorier.QUESTIONS * 100) + "%";
    $("rec-pays").textContent = q.cible.nom;
    $("rec-retour").textContent = ""; $("rec-retour").className = "message";
    $("rec-suite").hidden = true; $("rec-valider").hidden = false;
    pastille = 0;
    const pal = $("rec-palette");
    pal.innerHTML = "";
    niveau.palette.forEach((c, k) => {
      const b = document.createElement("button");
      b.textContent = c.nom; b.style.background = c.hex;
      // texte lisible selon la luminosité du fond
      const [, , l] = Teinte.versTsl(c.hex);
      b.style.color = l > 0.55 ? "#0B2A63" : "#fff";
      b.addEventListener("click", async () => {
        if (q.fini) return;
        Recolorier.choisir(pastille, k);
        if (pastille < q.groupes.length - 1) pastille++;
        dessinerPastilles();
        await majImage();
      });
      pal.appendChild(b);
    });
    dessinerPastilles();
    await majImage();
  }

  $("rec-valider").addEventListener("click", async () => {
    const { q } = Recolorier;
    if (q.fini) return;
    const manque = q.groupes.findIndex(g => g.choix === null);
    if (manque >= 0) {
      pastille = manque; dessinerPastilles();
      $("rec-retour").textContent = "Choisis une couleur pour chaque pastille.";
      $("rec-retour").className = "message mauvais";
      return;
    }
    const detail = Recolorier.valider();
    [...$("rec-pastilles").children].forEach((b, i) => b.classList.add(detail[i] ? "ok" : "ko"));
    [...$("rec-palette").children].forEach(b => { b.disabled = true; });
    $("rec-img").src = await Recolorier.imageVraie();
    $("rec-retour").textContent = q.bonne ? "Bravo, toutes les couleurs sont bonnes !" : "Voici le vrai drapeau. Les pastilles ✗ étaient à corriger.";
    $("rec-retour").className = "message" + (q.bonne ? "" : " mauvais");
    const derniere = Recolorier.partie.numero >= Recolorier.QUESTIONS;
    $("rec-valider").hidden = true;
    $("rec-suite").textContent = derniere ? "Voir mon résultat" : "Suivant";
    $("rec-suite").hidden = false;
    $("rec-suite").focus({ preventScroll: true });
  });

  $("rec-suite").addEventListener("click", async () => {
    if (Recolorier.partie.numero >= Recolorier.QUESTIONS) { fin("recolorier", Recolorier, lancerRec); return; }
    Recolorier.suivante();
    await afficherRec();
  });

  /* ===================== Inscription ===================== */
  A.MODES.couleurs = {
    titre: "Les bonnes couleurs", niveaux: () => BonnesCouleurs.NIVEAUX, lancer: lancerCouleurs,
    accueil: "10 drapeaux par partie",
    aide: "On te donne un pays et quatre drapeaux dont les couleurs sont fausses, sauf un. Retrouve le bon ! Plus le niveau monte, plus les couleurs se ressemblent.",
  };
  A.MODES.recolorier = {
    titre: "Recolorier", niveaux: () => Recolorier.NIVEAUX, lancer: lancerRec,
    accueil: "10 drapeaux par partie",
    aide: "Le drapeau d'un pays est colorié n'importe comment. Redonne-lui ses vraies couleurs avec la palette.",
  };
  A.boutonMode("couleurs");
  A.boutonMode("recolorier");
  A.majAccueil();
})();
