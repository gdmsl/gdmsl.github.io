---
title: "Ceci N'est Pas Juste une Animation: Modele d'Ising"
date: 2026-03-14
summary: "L'arriere-plan que vous venez de voir est une vraie simulation du modele d'Ising executee dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "mecanique-statistique"]
---

Vous avez peut-etre remarque l'arriere-plan anime de la page d'ou vous venez. Il peut sembler decoratif, mais c'est une **vraie simulation physique** executee en temps reel dans votre navigateur, compilee en WebAssembly a partir de Rust.

Pas de video pre-rendue. Pas d'astuces CSS. Les mathematiques sont reellement calculees en ce moment meme, sur votre appareil.

Ces simulations sont inspirees de la physique reelle mais optimisees pour l'impact visuel. Les parametres sont choisis pour etre beaux, pas pour reproduire une experience specifique. Pensez-y comme de l'**art computationnel ancre dans la vraie science**.

---

{{< simulation name="ising" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Le Modele d'Ising

Ce que vous avez vu est le **modele d'Ising**, l'un des systemes les plus etudies de toute la physique. Il est trompeusement simple : une grille de spins, chacun pointant vers le haut ou vers le bas, interagissant uniquement avec leurs plus proches voisins. De cette simplicite emerge l'un des phenomenes les plus profonds de la nature, une **transition de phase**.

A haute temperature, les fluctuations thermiques dominent. Les spins basculent constamment et aleatoirement, produisant une soupe bruyante et desordonnee. A basse temperature, le cout energetique des voisins desalignes l'emporte, et de grands **domaines** de spins alignes emergent et grandissent. Entre ces regimes se trouve la temperature critique, ou le systeme est en equilibre entre ordre et desordre, les fluctuations se produisent a toutes les echelles de longueur et le systeme exhibe une invariance d'echelle.

La simulation utilise l'**algorithme de clusters de Swendsen-Wang**. Au lieu de retourner un spin a la fois (comme dans l'algorithme de Metropolis plus simple), Swendsen-Wang construit des clusters de spins alignes voisins en activant des liens entre eux avec une probabilite qui depend de la temperature. Chaque cluster est ensuite retourne dans son ensemble avec une probabilite de 50%. Cela permet au systeme de faire de grandes mises a jour collectives en une seule iteration, ce qui est particulierement important pres de la temperature critique, ou les methodes a spin unique souffrent de ralentissement critique.

Les eclats teal marquent des clusters entiers qui viennent de basculer, vous permettant de voir la dynamique collective en temps reel.

Ernst Ising resolut la version 1D en 1924 et ne trouva aucune transition de phase, le conduisant a conjecturer (a tort) qu'il n'y en avait pas non plus en dimensions superieures. La solution exacte du modele 2D par Lars Onsager en 1944 (montrant une transition de phase nette) est l'un des grands resultats de la physique mathematique.

### Sous le capot

La simulation execute un Monte Carlo a clusters Swendsen-Wang sur un reseau carre 2D avec des conditions aux limites periodiques. Les clusters sont identifies a l'aide d'une structure de donnees union-find. La sortie est une texture flottante a deux canaux : etat du spin et glow (retournement recent du cluster). Le fragment shader rend les spin-up en bleu electrique, les spin-down en navy profond, avec un glow teal sur les clusters recemment retournes. Les bordures de cellules fournissent une structure visuelle.
