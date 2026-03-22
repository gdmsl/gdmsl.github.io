---
title: "Ceci N'est Pas Juste une Animation: Marche Quantique"
date: 2026-03-15
summary: "L'arriere-plan que vous venez de voir est une vraie simulation de marche quantique executee dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "quantique"]
---

Vous avez peut-etre remarque l'arriere-plan anime de la page d'ou vous venez. Il peut sembler decoratif, mais c'est une **vraie simulation physique** executee en temps reel dans votre navigateur, compilee en WebAssembly a partir de Rust.

Pas de video pre-rendue. Pas d'astuces CSS. Les mathematiques sont reellement calculees en ce moment meme, sur votre appareil.

Ces simulations sont inspirees de la physique reelle mais optimisees pour l'impact visuel. Les parametres sont choisis pour etre beaux, pas pour reproduire une experience specifique. Pensez-y comme de l'**art computationnel ancre dans la vraie science**.

---

{{< simulation name="quantum-walk" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Marche Quantique

L'arriere-plan de la page d'accueil simule une **marche quantique a temps continu** sur un reseau 2D.

Contrairement a une marche aleatoire classique, qui s'etale en une ennuyeuse gaussienne, un marcheur quantique exhibe des **interferences**. L'amplitude de probabilite se divise, se reflechit aux bords et cree des motifs complexes qui se propagent balistiquement plutot que diffusivement.

Dans une marche classique, la distance typique a l'origine croit comme la racine carree du temps. Dans une marche quantique, elle croit *lineairement* avec le temps, une acceleration quadratique. Ce n'est pas une coincidence : les marches quantiques sont l'un des blocs de construction des algorithmes quantiques, y compris la recherche de Grover et certains algorithmes sur graphes.

Ce que vous voyez a l'ecran est la distribution de probabilite sur une grille 2D. La luminosite de chaque cellule represente la probabilite de trouver le marcheur en ce point. Les franges d'interference (ces cretes ondulees) sont un phenomene purement quantique sans analogue classique.

### Sous le capot

La simulation resout l'equation de Schrodinger sur un reseau discret en utilisant une methode split-operator. A chaque frame, le module WASM calcule un pas de temps et ecrit le champ de probabilite dans une texture flottante mono-canal. Un shader WebGL2 mappe la probabilite sur la rampe de couleur bleue avec vignettage et grille superposee.
