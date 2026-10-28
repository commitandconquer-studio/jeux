// Commit & Conquer — vitrine des jeux (diaporama plein écran), carrousel des mondes
// et agrandissement des images au clic.
(function () {
  var reduit = window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  // Vitrine : un jeu à la fois, vignettes en bas, défilement automatique (arrêté au survol).
  document.querySelectorAll("[data-vitrine]").forEach(function (v) {
    var slides = v.querySelectorAll(".vitrine-slide");
    var vignettes = v.querySelectorAll(".vitrine-vignette");
    var n = slides.length, cur = 0, minuteur = null;
    if (n < 2) return;
    function aller(i) {
      cur = (i + n) % n;
      slides.forEach(function (s, k) { s.classList.toggle("on", k === cur); });
      vignettes.forEach(function (b, k) { b.classList.toggle("on", k === cur); });
    }
    function armer() {
      clearInterval(minuteur);
      if (!reduit) minuteur = setInterval(function () { aller(cur + 1); }, 7000);
    }
    vignettes.forEach(function (b) {
      b.addEventListener("click", function () { aller(+b.dataset.i); armer(); });
    });
    v.querySelector("[data-vitrine-prec]").addEventListener("click", function () { aller(cur - 1); armer(); });
    v.querySelector("[data-vitrine-suiv]").addEventListener("click", function () { aller(cur + 1); armer(); });
    v.addEventListener("mouseenter", function () { clearInterval(minuteur); });
    v.addEventListener("mouseleave", armer);
    v.addEventListener("focusin", function () { clearInterval(minuteur); });
    armer();
  });

  // Carrousel des mondes : flèches, points, défilement tactile.
  document.querySelectorAll("[data-carrousel]").forEach(function (c) {
    var rail = c.querySelector(".mondes-rail");
    var items = rail.querySelectorAll(".monde");
    if (items.length < 2) return;
    function pas() { return items[1].offsetLeft - items[0].offsetLeft; }
    function fleche(sens, txt) {
      var b = document.createElement("button");
      b.type = "button"; b.className = "carrousel-fleche " + sens; b.textContent = txt; b.setAttribute("aria-label", txt);
      b.addEventListener("click", function () { rail.scrollBy({ left: (sens === "prec" ? -1 : 1) * pas(), behavior: "smooth" }); });
      c.appendChild(b);
    }
    fleche("prec", "‹"); fleche("suiv", "›");
    var points = document.createElement("div");
    points.className = "carrousel-points";
    items.forEach(function (it, k) {
      var p = document.createElement("button");
      p.type = "button"; p.setAttribute("aria-label", String(k + 1));
      p.addEventListener("click", function () {
        rail.scrollTo({ left: it.offsetLeft - (rail.clientWidth - it.offsetWidth) / 2, behavior: "smooth" });
      });
      points.appendChild(p);
    });
    c.appendChild(points);
    function maj() {
      var centre = rail.scrollLeft + rail.clientWidth / 2, best = 0, d = 1e9;
      items.forEach(function (it, k) {
        var dd = Math.abs(it.offsetLeft + it.offsetWidth / 2 - centre);
        if (dd < d) { d = dd; best = k; }
      });
      points.querySelectorAll("button").forEach(function (p, k) { p.classList.toggle("on", k === best); });
    }
    rail.addEventListener("scroll", function () { requestAnimationFrame(maj); }, { passive: true });
    window.addEventListener("resize", maj);
    maj();
  });

  // Agrandissement : un clic sur une capture ou un monde l'ouvre en grand.
  var imgs = Array.prototype.slice.call(document.querySelectorAll("img[data-zoom]"));
  if (!imgs.length) return;
  var box = null, grande, legende, idx = 0;
  function legendeDe(i) {
    var f = i.closest("figure"), c = f && f.querySelector("figcaption");
    if (!c) return i.alt;
    var parts = Array.prototype.map.call(c.children, function (x) { return x.textContent.trim(); });
    return parts.length ? parts.join(" — ") : c.textContent.replace(/\s+/g, " ").trim();
  }
  function montrer(k) {
    idx = (k + imgs.length) % imgs.length;
    grande.src = imgs[idx].currentSrc || imgs[idx].src;
    grande.alt = imgs[idx].alt;
    legende.textContent = legendeDe(imgs[idx]);
  }
  function fermer() {
    if (!box) return;
    box.hidden = true; document.body.classList.remove("zoom-ouvert");
    imgs[idx].focus({ preventScroll: true });
  }
  function ouvrir(k) {
    if (!box) {
      box = document.createElement("div");
      box.className = "zoom"; box.setAttribute("role", "dialog"); box.setAttribute("aria-modal", "true");
      box.innerHTML = '<button type="button" class="zoom-x" aria-label="×">×</button>' +
        '<button type="button" class="zoom-fl prec" aria-label="‹">‹</button>' +
        '<figure><img alt=""><figcaption></figcaption></figure>' +
        '<button type="button" class="zoom-fl suiv" aria-label="›">›</button>';
      document.body.appendChild(box);
      grande = box.querySelector("img"); legende = box.querySelector("figcaption");
      box.querySelector(".zoom-x").addEventListener("click", fermer);
      box.querySelector(".prec").addEventListener("click", function (e) { e.stopPropagation(); montrer(idx - 1); });
      box.querySelector(".suiv").addEventListener("click", function (e) { e.stopPropagation(); montrer(idx + 1); });
      box.addEventListener("click", function (e) { if (e.target === box || e.target.tagName === "FIGURE") fermer(); });
      document.addEventListener("keydown", function (e) {
        if (box.hidden) return;
        if (e.key === "Escape") fermer();
        else if (e.key === "ArrowLeft") montrer(idx - 1);
        else if (e.key === "ArrowRight") montrer(idx + 1);
      });
    }
    montrer(k); box.hidden = false; document.body.classList.add("zoom-ouvert");
    box.querySelector(".zoom-x").focus();
  }
  imgs.forEach(function (i, k) {
    i.addEventListener("click", function () { ouvrir(k); });
    i.addEventListener("keydown", function (e) { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); ouvrir(k); } });
  });
})();
