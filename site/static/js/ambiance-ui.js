/* Commit & Conquer — interface de l'ambiance : message de fête, météo locale
   (seulement si le visiteur l'accepte) et, sur le serveur de développement,
   le panneau de démonstration qui force le moment, la saison, la fête et la météo. */
(function () {
  "use strict";
  var script = document.currentScript;
  var lang = script.dataset.lang === "en" ? "en" : "fr";
  var amb = window.CC_AMBIANCE || {};
  var html = document.documentElement;

  var T = {
    fr: {
      fetes: {
        halloween: "🎃 Joyeux Halloween ! Même nos jeux ont peur.", avent: "🕯️ Bon temps de l'Avent !", noel: "🎄 Joyeux Noël !",
        "saint-nicolas": "🎁 Bonne Saint-Nicolas !", epiphanie: "👑 Bonne Épiphanie ! Qui aura la fève ?",
        rameaux: "🌿 Bon dimanche des Rameaux !", paques: "🥚 Joyeuses Pâques !", "semaine-sainte": "🔔 Les cloches sont parties pour Rome…",
        "mardi-gras": "🎭 Joyeux Mardi gras !", chandeleur: "🥞 Bonne Chandeleur !", toussaint: "🕯️ Toussaint",
        ascension: "☁️ Bonne fête de l'Ascension", pentecote: "🕊️ Bonne Pentecôte",
      },
      temps: { clair: "ciel dégagé", nuageux: "nuageux", pluie: "pluie", neige: "neige", orage: "orage", brouillard: "brouillard" },
      titre: "La météo de chez vous ?",
      explique: "Le site peut prendre la couleur du temps qu'il fait chez vous. Votre ville, ou votre position arrondie à une dizaine de kilomètres, est alors envoyée au service météo libre Open-Meteo. Rien d'autre, et rien n'est conservé ailleurs que dans votre navigateur.",
      geo: "📍 Me géolocaliser", ville: "Votre ville", chercher: "Chercher", non: "Non merci",
      actif: "Météo de {v} : {t}.", changer: "Changer de ville", arreter: "Ne plus utiliser la météo",
      cherche: "Recherche…", introuvable: "Ville introuvable.", erreur: "Météo indisponible pour le moment.", position: "votre position",
      refus: "Géolocalisation refusée : vous pouvez indiquer votre ville.",
      dev: "Ambiance", moment: "Moment", saison: "Saison", fete: "Fête", meteo: "Météo", auto: "Automatique", aucune: "Aucune",
      tout_auto: "Tout en automatique", calcule: "Affiché : ",
      moments: { matin: "Matin", jour: "Journée", soir: "Soir", nuit: "Nuit" },
      saisons: { printemps: "Printemps", ete: "Été", automne: "Automne", hiver: "Hiver" },
    },
    en: {
      fetes: {
        halloween: "🎃 Happy Halloween! Even our games are scared.", avent: "🕯️ Happy Advent!", noel: "🎄 Merry Christmas!",
        "saint-nicolas": "🎁 Happy Saint Nicholas Day!", epiphanie: "👑 Happy Epiphany! Who'll find the bean?",
        rameaux: "🌿 Happy Palm Sunday!", paques: "🥚 Happy Easter!", "semaine-sainte": "🔔 The bells have flown to Rome…",
        "mardi-gras": "🎭 Happy Mardi Gras!", chandeleur: "🥞 Happy Candlemas!", toussaint: "🕯️ All Saints' Day",
        ascension: "☁️ Happy Ascension Day", pentecote: "🕊️ Happy Pentecost",
      },
      temps: { clair: "clear sky", nuageux: "cloudy", pluie: "rain", neige: "snow", orage: "thunderstorm", brouillard: "fog" },
      titre: "Your local weather?",
      explique: "The site can take on the colours of the weather where you are. Your city, or your position rounded to about ten kilometres, is then sent to the free weather service Open-Meteo. Nothing else, and nothing is kept anywhere but in your browser.",
      geo: "📍 Locate me", ville: "Your city", chercher: "Search", non: "No thanks",
      actif: "Weather in {v}: {t}.", changer: "Change city", arreter: "Stop using the weather",
      cherche: "Searching…", introuvable: "City not found.", erreur: "Weather unavailable right now.", position: "your area",
      refus: "Location denied: you can type your city instead.",
      dev: "Ambiance", moment: "Time", saison: "Season", fete: "Feast", meteo: "Weather", auto: "Automatic", aucune: "None",
      tout_auto: "All automatic", calcule: "Showing: ",
      moments: { matin: "Morning", jour: "Daytime", soir: "Evening", nuit: "Night" },
      saisons: { printemps: "Spring", ete: "Summer", automne: "Autumn", hiver: "Winter" },
    },
  }[lang];

  function lire(cle) { try { return JSON.parse(localStorage.getItem(cle) || "null"); } catch (e) { return null; } }
  function ecrire(cle, v) { try { v === null ? localStorage.removeItem(cle) : localStorage.setItem(cle, JSON.stringify(v)); } catch (e) {} }
  function el(tag, attrs, enfants) {
    var e = document.createElement(tag);
    Object.keys(attrs || {}).forEach(function (k) { if (k === "texte") e.textContent = attrs[k]; else e.setAttribute(k, attrs[k]); });
    (enfants || []).forEach(function (c) { e.appendChild(c); });
    return e;
  }

  // ---- Message de fête ------------------------------------------------
  var bandeau = document.querySelector(".fete-bandeau");
  if (bandeau && amb.fete && T.fetes[amb.fete]) {
    bandeau.textContent = T.fetes[amb.fete];
    bandeau.hidden = false;
  }

  // ---- Météo ----------------------------------------------------------
  var ICONES = { clair: "☀️", nuageux: "⛅", pluie: "🌧️", neige: "❄️", orage: "⛈️", brouillard: "🌫️" };
  var bouton = document.querySelector(".meteo-btn");
  function icone() {
    if (!bouton) return;
    bouton.textContent = amb.meteo ? (amb.nuit && amb.meteo === "clair" ? "🌙" : ICONES[amb.meteo]) : (amb.nuit ? "🌙" : "🌤️");
  }
  /** Code météo WMO (Open-Meteo) → temps du site. */
  function traduire(code) {
    if (code <= 1) return "clair";
    if (code <= 3) return "nuageux";
    if (code === 45 || code === 48) return "brouillard";
    if ((code >= 71 && code <= 77) || code === 85 || code === 86) return "neige";
    if (code >= 95) return "orage";
    return "pluie";
  }
  function appliquer(temps) {
    ["clair", "nuageux", "pluie", "neige", "orage", "brouillard"].forEach(function (t) { html.classList.remove("meteo-" + t); });
    // En démonstration, une météo forcée garde la main.
    if (amb.force && amb.force.meteo && amb.force.meteo !== "auto") return;
    amb.meteo = temps;
    if (temps) html.classList.add("meteo-" + temps);
    icone();
    window.dispatchEvent(new CustomEvent("cc-ambiance", { detail: amb }));
  }
  function charger(lieu, rappel) {
    var url = "https://api.open-meteo.com/v1/forecast?latitude=" + lieu.lat + "&longitude=" + lieu.lon + "&current=weather_code&timezone=auto";
    fetch(url).then(function (r) { return r.json(); }).then(function (d) {
      var temps = traduire(d.current.weather_code);
      ecrire("cc-meteo", { temps: temps, t: Date.now(), ville: lieu.ville });
      amb.ville = lieu.ville;
      appliquer(temps);
      rappel && rappel(null);
    }).catch(function () { rappel && rappel(T.erreur); });
  }
  var lieu = lire("cc-meteo-lieu");
  var cache = lire("cc-meteo");
  if (lieu && (!cache || Date.now() - cache.t > 30 * 60 * 1000)) charger(lieu);
  icone();

  var pop = null;
  function fermer() { if (pop) { pop.remove(); pop = null; } }
  function ouvrir() {
    fermer();
    var etat = el("p", { class: "etat", "aria-live": "polite" });
    var corps;
    if (lieu) {
      var resume = cache && cache.temps ? T.actif.replace("{v}", lieu.ville).replace("{t}", T.temps[cache.temps]) : T.cherche;
      corps = [el("p", { texte: resume }), el("div", { class: "ligne" }, [
        bouton2(T.changer, function () { ecrire("cc-meteo-lieu", null); lieu = null; ouvrir(); }, true),
        bouton2(T.arreter, function () { ecrire("cc-meteo-lieu", null); ecrire("cc-meteo", null); lieu = null; appliquer(""); fermer(); }, true),
      ])];
    } else {
      var champ = el("input", { type: "text", placeholder: T.ville, autocomplete: "address-level2" });
      var chercher = function () {
        if (!champ.value.trim()) return;
        etat.textContent = T.cherche;
        fetch("https://geocoding-api.open-meteo.com/v1/search?count=1&language=" + lang + "&name=" + encodeURIComponent(champ.value.trim()))
          .then(function (r) { return r.json(); }).then(function (d) {
            var v = d.results && d.results[0];
            if (!v) { etat.textContent = T.introuvable; return; }
            choisir({ lat: v.latitude.toFixed(2), lon: v.longitude.toFixed(2), ville: v.name + (v.country_code ? " (" + v.country_code + ")" : "") }, etat);
          }).catch(function () { etat.textContent = T.erreur; });
      };
      champ.addEventListener("keydown", function (e) { if (e.key === "Enter") chercher(); });
      corps = [el("p", { texte: T.explique }),
        el("div", { class: "ligne" }, [bouton2(T.geo, function () {
          if (!navigator.geolocation) { etat.textContent = T.refus; return; }
          etat.textContent = T.cherche;
          navigator.geolocation.getCurrentPosition(function (p) {
            // Arrondi à 0,1° (une dizaine de kilomètres) : pas plus précis que nécessaire.
            choisir({ lat: p.coords.latitude.toFixed(1), lon: p.coords.longitude.toFixed(1), ville: T.position }, etat);
          }, function () { etat.textContent = T.refus; }, { maximumAge: 3600000, timeout: 15000 });
        })]),
        el("div", { class: "ligne" }, [champ, bouton2(T.chercher, chercher)]),
        el("div", { class: "ligne" }, [bouton2(T.non, fermer, true)])];
    }
    pop = el("div", { class: "meteo-pop", role: "dialog", "aria-label": T.titre }, [el("h3", { texte: T.titre })].concat(corps, [etat]));
    document.body.appendChild(pop);
  }
  function bouton2(texte, action, second) {
    var b = el("button", { type: "button", texte: texte });
    if (second) b.className = "second";
    b.addEventListener("click", action);
    return b;
  }
  function choisir(l, etat) {
    lieu = l;
    ecrire("cc-meteo-lieu", l);
    charger(l, function (err) {
      if (err) { etat.textContent = err; return; }
      cache = lire("cc-meteo");
      ouvrir();
    });
  }
  if (bouton) bouton.addEventListener("click", function () { pop ? fermer() : ouvrir(); });
  document.addEventListener("keydown", function (e) { if (e.key === "Escape") fermer(); });

  // ---- Panneau de démonstration (développement seulement) ---------------
  if (script.dataset.dev !== "1") return;
  var force = lire("cc-ambiance-force") || {};
  var choix = {
    moment: ["matin", "jour", "soir", "nuit"],
    saison: ["printemps", "ete", "automne", "hiver"],
    fete: ["aucune", "halloween", "avent", "noel", "saint-nicolas", "epiphanie", "rameaux", "paques", "semaine-sainte", "chandeleur"],
    meteo: ["aucune", "clair", "nuageux", "pluie", "neige", "orage", "brouillard"],
  };
  var libelle = function (cle, v) {
    if (v === "aucune") return T.aucune;
    if (cle === "moment") return T.moments[v];
    if (cle === "saison") return T.saisons[v];
    if (cle === "meteo") return T.temps[v];
    return (T.fetes[v] || v).replace(/[!…].*$/, "").trim();
  };
  var panneau = el("div", { class: "panneau", hidden: "" });
  Object.keys(choix).forEach(function (cle) {
    var sel = el("select", { "data-cle": cle });
    [["auto", T.auto]].concat(choix[cle].map(function (v) { return [v, libelle(cle, v)]; })).forEach(function (o) {
      var opt = el("option", { value: o[0], texte: o[1] });
      if ((force[cle] || "auto") === o[0]) opt.selected = true;
      sel.appendChild(opt);
    });
    sel.addEventListener("change", function () {
      force[cle] = sel.value;
      ecrire("cc-ambiance-force", force);
      location.reload();
    });
    panneau.appendChild(el("label", {}, [el("span", { texte: T[cle] }), sel]));
  });
  var reinit = bouton2(T.tout_auto, function () { ecrire("cc-ambiance-force", null); location.reload(); }, false);
  panneau.appendChild(reinit);
  var vrai = window.CC_FETE ? (function () {
    var d = new Date();
    return [T.moments[amb.moment], T.saisons[amb.saison], amb.fete ? libelle("fete", amb.fete) : "", amb.meteo ? T.temps[amb.meteo] : ""].filter(Boolean).join(" · ");
  })() : "";
  panneau.appendChild(el("p", { class: "actuel", texte: T.calcule + vrai }));
  var ouvre = el("button", { type: "button", texte: "🌦 " + T.dev });
  ouvre.addEventListener("click", function () { panneau.hidden = !panneau.hidden; });
  document.body.appendChild(el("div", { class: "dev-ambiance" }, [panneau, ouvre]));
})();
