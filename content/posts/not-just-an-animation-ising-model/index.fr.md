---
title: "Ceci n'est pas juste une animation : le modèle d'Ising"
date: 2026-03-14
summary: "L'arrière-plan que vous venez de voir est une vraie simulation du modèle d'Ising exécutée dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "mécanique-statistique"]
---

Vous avez peut-être remarqué l'arrière-plan animé de la page d'où vous venez. Il peut sembler décoratif, mais c'est une **vraie simulation physique** exécutée en temps réel dans votre navigateur, compilée en WebAssembly à partir de Rust.

Pas de vidéo pré-rendue. Pas d'astuces CSS. Les mathématiques sont réellement calculées en ce moment même, sur votre appareil.

Ces simulations sont inspirées de la physique réelle mais optimisées pour l'impact visuel. Les paramètres sont choisis pour être beaux, pas pour reproduire une expérience spécifique. Pensez-y comme de l'**art computationnel ancré dans la vraie science**.

---

{{< simulation name="ising" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Le modèle d'Ising

Ce que vous avez vu est le **modèle d'Ising**, l'un des systèmes les plus étudiés de toute la physique. Il est trompeusement simple : une grille de spins, chacun pointant vers le haut ou vers le bas, interagissant uniquement avec leurs plus proches voisins. De cette simplicité émerge l'un des phénomènes les plus profonds de la nature : une **transition de phase**.

À haute température, les fluctuations thermiques dominent. Les spins basculent constamment et aléatoirement, produisant une soupe bruyante et désordonnée. À basse température, le coût énergétique des voisins désalignés l'emporte, et de grands **domaines** de spins alignés émergent et grandissent. Entre ces régimes se trouve la température critique, où le système est en équilibre entre ordre et désordre, les fluctuations se produisent à toutes les échelles de longueur et le système présente une invariance d'échelle.

La simulation utilise l'**algorithme de clusters de Swendsen-Wang**. Au lieu de retourner un spin à la fois (comme dans l'algorithme de Metropolis plus simple), Swendsen-Wang construit des clusters de spins alignés voisins en activant des liens entre eux avec une probabilité qui dépend de la température. Chaque cluster est ensuite retourné dans son ensemble avec une probabilité de 50%. Cela permet au système de faire de grandes mises à jour collectives en une seule itération, ce qui est particulièrement important près de la température critique, où les méthodes à spin unique souffrent de ralentissement critique.

Les éclats teal marquent des clusters entiers qui viennent de basculer, vous permettant de voir la dynamique collective en temps réel.

Ernst Ising résolut la version 1D en 1924 et ne trouva aucune transition de phase, le conduisant à conjecturer (à tort) qu'il n'y en avait pas non plus en dimensions supérieures. La solution exacte du modèle 2D par Lars Onsager en 1944 (montrant une transition de phase nette) est l'un des grands résultats de la physique mathématique.

### Sous le capot

La simulation exécute un Monte Carlo à clusters Swendsen-Wang sur un réseau carré 2D avec des conditions aux limites périodiques. Les clusters sont identifiés à l'aide d'une structure de données union-find. La sortie est une texture flottante à deux canaux : état du spin et glow (retournement récent du cluster). Le fragment shader rend les spin-up en bleu électrique, les spin-down en navy profond, avec un glow teal sur les clusters récemment retournés. Les bordures de cellules fournissent une structure visuelle.
