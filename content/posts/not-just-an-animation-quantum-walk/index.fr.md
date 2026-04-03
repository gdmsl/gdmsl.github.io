---
title: "Ceci n'est pas juste une animation : la marche quantique"
date: 2026-03-15
summary: "L'arrière-plan que vous venez de voir est une vraie simulation de marche quantique exécutée dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "quantique"]
---

Vous avez peut-être remarqué l'arrière-plan animé de la page d'où vous venez. Il peut sembler décoratif, mais c'est une **vraie simulation physique** exécutée en temps réel dans votre navigateur, compilée en WebAssembly à partir de Rust.

Pas de vidéo pré-rendue. Pas d'astuces CSS. Les mathématiques sont réellement calculées en ce moment même, sur votre appareil.

Ces simulations sont inspirées de la physique réelle mais optimisées pour l'impact visuel. Les paramètres sont choisis pour être beaux, pas pour reproduire une expérience spécifique. Pensez-y comme de l'**art computationnel ancré dans la vraie science**.

---

{{< simulation name="quantum-walk" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Marche quantique

L'arrière-plan de la page d'accueil simule une **marche quantique à temps continu** sur un réseau 2D.

Contrairement à une marche aléatoire classique, qui s'étale en une ennuyeuse gaussienne, un marcheur quantique présente des **interférences**. L'amplitude de probabilité se divise, se réfléchit aux bords et crée des motifs complexes qui se propagent balistiquement plutôt que diffusivement.

Dans une marche classique, la distance typique à l'origine croît comme la racine carrée du temps. Dans une marche quantique, elle croît *linéairement* avec le temps : une accélération quadratique. Ce n'est pas une coïncidence : les marches quantiques sont l'un des blocs de construction des algorithmes quantiques, y compris la recherche de Grover et certains algorithmes sur graphes.

Ce que vous voyez à l'écran est la distribution de probabilité sur une grille 2D. La luminosité de chaque cellule représente la probabilité de trouver le marcheur en ce point. Les franges d'interférence (ces crêtes ondulées) sont un phénomène purement quantique sans analogue classique.

### Sous le capot

La simulation résout l'équation de Schrödinger sur un réseau discret en utilisant une méthode split-operator. À chaque frame, le module WASM calcule un pas de temps et écrit le champ de probabilité dans une texture flottante mono-canal. Un shader WebGL2 mappe la probabilité sur la rampe de couleur bleue avec vignettage et grille superposée.
