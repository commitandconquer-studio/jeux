// À la carte ! — Commit & Conquer, licence GPL-3.0-or-later.
// Écran du mode « Pays caché ».
"use strict";

(() => {
  const $ = id => document.getElementById(id);
  const A = window.AppAPI;
  const champ = $("cache-champ");
  const FLECHE = { plus: "▲", moins: "▼", proche: "≈", oui: "✓", non: "✗" };

  async function lancer(id, graine = null) {
    try { await PaysCache.charger(); }
    catch (e) { alert("Les données n'ont pas pu être chargées."); A.afficherNiveaux("cache"); return; }
    Fun.nouvellePartie();
    PaysCache.demarrer(id, graine);
    afficher();
    A.montrer("cache");
    champ.focus({ preventScroll: true });
  }

  function afficher() {
    const { partie, niveau } = PaysCache;
    $("cache-titre").textContent = niveau.nom;
    $("cache-compteur").textContent = `Pays ${partie.numero} / ${PaysCache.PAR_PARTIE}`;
    $("cache-barre").style.width = ((partie.numero - 1) / PaysCache.PAR_PARTIE * 100) + "%";
    $("cache-essais").innerHTML = "";
    $("cache-message").textContent = "";
    champ.value = "";
    saisie.fermer();
  }

  /** Une carte par pays proposé : drapeau, nom, puis les indices. */
  function carteEssai(e) {
    const { niveau } = PaysCache;
    const li = document.createElement("li");
    li.className = "devine" + (e.gagne ? " gagne" : "");
    const entete = `<div class="devine-tete"><img src="data/drapeaux/${e.pays.code}.svg" alt="" width="40" height="30"><b>${e.pays.nom}</b>` +
      `<small>${e.pays.capitale}</small></div>`;
    if (e.gagne) { li.innerHTML = entete + `<p class="devine-ok">Trouvé !</p>`; return li; }
    const puces = e.indices.map(i =>
      `<li class="ind ${i.etat}"><span class="ind-nom">${i.nom}</span><span class="ind-val">${i.val}</span>` +
      `<span class="ind-mot"><span class="ind-signe" aria-hidden="true">${FLECHE[i.etat]}</span> ${i.mot}</span></li>`);
    if (niveau.distance) {
      const boussole = niveau.boussole
        ? `<svg class="boussole" viewBox="0 0 24 24" role="img" aria-label="direction : ${Commun.pointCardinal(e.cap)}" style="transform:rotate(${Math.round(e.cap)}deg)">` +
          `<circle cx="12" cy="12" r="11" fill="#fff" stroke="#0B3D91" stroke-width="1.5"/><path d="M12 3l3.6 9h-7.2z" fill="#FF9800"/><path d="M12 21l-3.6-9h7.2z" fill="#8AA0C8"/></svg>` : "";
      puces.push(`<li class="ind dist"><span class="ind-nom">Distance</span><span class="ind-val">${Commun.km(e.dist)}</span><span class="ind-mot">entre les capitales ${boussole}</span></li>`);
    }
    li.innerHTML = entete + `<ul class="indices">${puces.join("")}</ul>`;
    return li;
  }

  function jouer(pays) {
    const e = PaysCache.proposer(pays);
    $("cache-essais").prepend(carteEssai(e));
    champ.value = ""; saisie.fermer(); $("cache-message").textContent = "";
    if (e.gagne) { Fun.reponse(true); setTimeout(() => finPays(true), 800); }
    else { Fun.jouerSon("faux"); Fun.bulle("triste", "Pas encore !", 900); champ.focus({ preventScroll: true }); }
  }

  const saisie = A.creerSaisie(champ, $("cache-sugg"), v => PaysCache.suggestions(v), jouer, p => `<span>${p.nom}</span><small>${p.capitale}</small>`);
  champ.addEventListener("keydown", ev => {
    if (ev.key === "Enter" && !$("cache-sugg").children.length && champ.value.trim())
      $("cache-message").textContent = "Je ne connais pas ce pays dans la liste.";
  });
  $("cache-abandon").addEventListener("click", () => { PaysCache.abandonner(); Fun.reponse(false); finPays(false); });

  function finPays(trouve) {
    const { etat, partie } = PaysCache;
    const derniere = partie.numero >= PaysCache.PAR_PARTIE;
    const s = etat.secret;
    $("fin-titre").textContent = trouve ? "Bravo !" : "C'était…";
    $("fin-etoiles").innerHTML = Commun.etoilesHtml(etat.etoiles);
    $("fin-texte").textContent = trouve
      ? `${s.nom} (capitale : ${s.capitale}), trouvé en ${etat.essais.length} essai${etat.essais.length > 1 ? "s" : ""}.`
      : `${s.nom} (capitale : ${s.capitale}).`;
    $("fin-suite").textContent = derniere ? "Voir mon résultat" : "Pays suivant";
    A.suite(suite);
    A.carteFin();
    A.montrer("fin");
    // La carte montre les capitales : les pays proposés pointent vers le pays caché.
    const point = p => ({ n: p.nom, lat: p.lat, lon: p.lon });
    const essais = etat.essais.map(e => ({ ville: point(e.pays), dist: e.dist, gagne: e.gagne }));
    Carte.dessiner($("fin-carte-svg"), { cible: point(s), refs: [], essais })
      .then(() => { $("fin-legende").innerHTML = Carte.legende(point(s), [], essais); })
      .catch(() => { $("fin-carte").hidden = true; });
  }

  function suite() {
    const { partie } = PaysCache;
    if (partie.points.length >= PaysCache.PAR_PARTIE) {
      const r = PaysCache.resume();
      if (partie.graine) {
        const total = r.points.reduce((a, b) => a + b, 0);
        A.defiTermine("cache", PaysCache.niveau.id, r.points, `${total} ★ sur ${3 * r.points.length}`, r.moyenne);
        return;
      }
      $("fin-titre").textContent = "Partie terminée";
      $("fin-etoiles").innerHTML = Commun.etoilesHtml(r.moyenne);
      Fun.finPartie(r.moyenne);
      $("fin-texte").textContent = r.moyenne >= 2 ? "Bien joué ! Tu peux passer au niveau suivant." : "Encore un essai pour décrocher 2 étoiles et débloquer la suite.";
      $("fin-suite").textContent = "Rejouer ce niveau";
      $("fin-carte").hidden = true;
      const id = PaysCache.niveau.id;
      A.suite(() => lancer(id));
      A.majAccueil();
      return;
    }
    PaysCache.suivante();
    afficher();
    A.montrer("cache");
    champ.focus({ preventScroll: true });
  }

  A.MODES.cache = {
    titre: "Pays caché", niveaux: () => PaysCache.NIVEAUX, lancer, accueil: "3 pays par partie",
    aide: "Un pays est caché. Tape un pays : je te dis si le pays caché est plus grand, plus peuplé, plus riche, dans le même hémisphère ou le même continent, et à quelle distance il se trouve. Trouve-le en peu d'essais !",
  };
  A.boutonMode("cache");
  A.majAccueil();
})();
