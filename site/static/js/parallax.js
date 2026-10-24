// Commit & Conquer — parallaxe au défilement : chaque élément [data-vitesse]
// glisse à sa propre vitesse par rapport à la page (0 = immobile avec elle).
(function () {
  if (matchMedia("(prefers-reduced-motion: reduce)").matches) return;
  var plans = Array.prototype.slice.call(document.querySelectorAll("[data-vitesse]"));
  if (!plans.length) return;
  var prevu = false;
  function placer() {
    prevu = false;
    var h = innerHeight;
    plans.forEach(function (el) {
      var r = el.parentElement.getBoundingClientRect();
      if (r.bottom < -200 || r.top > h + 200) return;
      var d = (r.top + r.height / 2 - h / 2) * parseFloat(el.dataset.vitesse);
      el.style.transform = "translate3d(0," + d.toFixed(1) + "px,0)";
    });
  }
  function demander() { if (!prevu) { prevu = true; requestAnimationFrame(placer); } }
  addEventListener("scroll", demander, { passive: true });
  addEventListener("resize", demander);
  placer();
})();
