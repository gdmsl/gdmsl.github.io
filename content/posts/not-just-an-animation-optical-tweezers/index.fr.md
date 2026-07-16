---
title: "Ceci n'est pas juste une animation : les pinces optiques"
date: 2026-03-11
summary: "L'arrière-plan que vous venez de voir est une vraie simulation de tri par pinces optiques exécutée dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "calcul-quantique"]
---

Vous avez peut-être remarqué l'arrière-plan animé sur la page d'où vous venez. Il a l'air décoratif, mais c'est en réalité une **vraie simulation physique** qui tourne en direct dans votre navigateur, compilée en WebAssembly depuis du Rust.

Aucune vidéo pré-enregistrée, aucune astuce CSS. Les calculs se font vraiment, en ce moment même, sur votre appareil.

Ces simulations s'inspirent de la physique réelle, mais je les ai réglées pour le plaisir des yeux. J'ai choisi les paramètres pour le rendu, pas pour coller à une expérience précise. Voyez-les comme de l'**art computationnel ancré dans la science**.

---

{{< simulation name="tweezer" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Tri atomique par pinces optiques

Ce que vous avez vu, c'est le **tri atomique** employé dans les ordinateurs quantiques à atomes neutres. Des atomes sont chargés au hasard dans un réseau de pinces optiques (des faisceaux laser très focalisés qui piègent les atomes un à un), puis une pince mobile les réarrange dans une zone cible compacte et sans défaut.

Dans les expériences réelles, chaque piège capture un atome avec une probabilité d'environ 50 %. Une caméra photographie le réseau pour repérer les pièges occupés. Un algorithme rapide calcule alors le meilleur plan de réarrangement, et une pince motorisée récupère les atomes un à un (ou en parallèle) pour les déplacer et combler les trous. Le tout prend quelques millisecondes et donne un registre de qubits parfait, sans défaut, prêt pour le calcul quantique.

La simulation alterne entre deux vrais algorithmes de tri :

**Algorithme hongrois** (affectation optimale) : une seule pince exécute le plan mathématiquement optimal. L'algorithme de Kuhn-Munkres détermine quelle position cible attribuer à chaque atome de départ de façon à minimiser le déplacement total. La pince emprunte des chemins interstitiels (elle passe dans les espaces entre les pièges) pour ne pas déranger les autres atomes pendant le transport. C'est l'approche optimale en théorie.

**Compression** (pinces en parallèle) : plusieurs pinces travaillent en même temps, en resserrant d'abord toutes les colonnes vers le centre, puis toutes les lignes. À chaque étape, les atomes avancent d'exactement un pas de réseau, toutes les pinces à l'unisson. On se rapproche ici du fonctionnement réel des expériences : ce n'est pas optimal à l'échelle globale, mais c'est rapide et naturellement parallélisable.

Les cercles gris sont les pièges optiques fixes. Les points teal brillants sont les atomes piégés. La lueur jaune, c'est la pince de tri (ou les pinces, pendant la compression). Regardez comme l'algorithme hongrois trace des chemins élégants et efficaces, tandis que la compression donne un balayage coordonné très satisfaisant.

Cette technologie est au cœur d'entreprises comme **QuEra**, **Pasqal** et **Atom Computing**, qui construisent des ordinateurs quantiques comptant des centaines, voire des milliers de qubits à atomes neutres disposés par pinces optiques.

### Dans les coulisses

La simulation déroule toute la chaîne de tri : chargement aléatoire, calcul de la zone cible, planification des déplacements propre à chaque algorithme (le hongrois avec Kuhn-Munkres en O(n^3), ou la compression par lignes et colonnes avec des pas parallèles d'une seule case), et exécution animée avec recherche de chemin interstitielle. Le résultat est une texture flottante à deux canaux : le canal R code les pièges et les atomes (des taches gaussiennes d'amplitudes différentes), le canal G code la position des pinces. Le fragment shader superpose correctement les pièges (en gris), les pinces (en jaune) et les atomes (teal).
