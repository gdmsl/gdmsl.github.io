---
title: "Ceci n'est pas juste une animation : la marche quantique"
date: 2026-03-15
summary: "L'arrière-plan que vous venez de voir est une vraie simulation de marche quantique qui tourne dans votre navigateur grâce à WebAssembly."
tags: ["physique", "simulation", "quantique"]
---

Vous avez peut-être remarqué l'arrière-plan animé sur la page d'où vous venez. Il a beau ressembler à une simple décoration, c'est en réalité une **vraie simulation de physique** qui tourne en direct dans votre navigateur, compilée de Rust vers WebAssembly.

Aucune vidéo préenregistrée, aucune astuce CSS. Les calculs se font vraiment en ce moment même, sur votre appareil.

Ces simulations s'inspirent de phénomènes physiques réels, mais elles sont réglées pour le plaisir des yeux. J'ai choisi les paramètres pour le rendu visuel, pas pour coller à une expérience précise. Voyez-les comme de l'**art computationnel enraciné dans de la vraie science**.

---

{{< simulation name="quantum-walk" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Marche quantique

L'arrière-plan de la page d'accueil simule une **marche quantique à temps continu** sur un réseau à deux dimensions.

Contrairement à une marche aléatoire classique, qui s'étale en une bête tache gaussienne, un marcheur quantique donne lieu à des **interférences**. L'amplitude de probabilité se divise, se réfléchit sur les bords et dessine des motifs complexes qui se propagent de façon balistique plutôt que diffusive.

Dans une marche classique, la distance à l'origine augmente en moyenne comme la racine carrée du temps. Dans une marche quantique, elle augmente *linéairement* avec le temps : un gain quadratique. Ce n'est pas un hasard. Les marches quantiques sont l'une des briques de base des algorithmes quantiques, dont la recherche de Grover et certains algorithmes sur les graphes.

Ce que vous voyez à l'écran, c'est la distribution de probabilité sur une grille 2D. La luminosité de chaque case indique la probabilité d'y trouver le marcheur. Les franges d'interférence (ces crêtes qui ondulent) sont un phénomène purement quantique, sans équivalent classique.

### Sous le capot

La simulation résout l'équation de Schrödinger sur un réseau discret par une méthode split-operator. À chaque image, le module WASM calcule un pas de temps et écrit le champ de probabilité dans une texture flottante à un seul canal. Un shader WebGL2 convertit ensuite la probabilité en dégradé de bleu, avec vignettage et grille par-dessus.
