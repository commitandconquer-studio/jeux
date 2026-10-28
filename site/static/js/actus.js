// Commit & Conquer — widget d'actualités : survoler (ou cibler au clavier) un article
// montre son image à gauche.
(function () {
  document.querySelectorAll(".actus").forEach(function (w) {
    var lien = w.querySelector("[data-actu-image]");
    if (!lien) return;
    var img = lien.querySelector("img");
    var legende = lien.querySelector("[data-actu-legende]");
    function montrer(a) {
      w.querySelectorAll(".actu").forEach(function (x) { x.classList.toggle("on", x === a); });
      img.src = a.dataset.image;
      legende.textContent = a.dataset.titre;
      lien.href = a.href;
    }
    w.querySelectorAll(".actu").forEach(function (a) {
      a.addEventListener("mouseenter", function () { montrer(a); });
      a.addEventListener("focus", function () { montrer(a); });
    });
  });
})();
