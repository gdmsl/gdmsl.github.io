---
title: "Ceci n'est pas juste une animation : les atomes de Rydberg"
date: 2026-03-12
summary: "L'arrière-plan que vous venez de voir est une vraie simulation d'atomes de Rydberg qui tourne dans votre navigateur grâce à WebAssembly."
tags: ["physique", "simulation", "calcul-quantique"]
---

Vous avez peut-être remarqué l'arrière-plan animé sur la page d'où vous venez. Il a beau ressembler à une simple décoration, c'est en réalité une **vraie simulation de physique** qui tourne en direct dans votre navigateur, compilée de Rust vers WebAssembly.

Aucune vidéo préenregistrée, aucune astuce CSS. Les calculs se font vraiment en ce moment même, sur votre appareil.

Ces simulations s'inspirent de phénomènes physiques réels, mais elles sont réglées pour le plaisir des yeux. J'ai choisi les paramètres pour le rendu visuel, pas pour coller à une expérience précise. Voyez-les comme de l'**art computationnel enraciné dans de la vraie science**.

---

{{< simulation name="rydberg" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Atomes de Rydberg

Ce que vous avez vu est un gaz d'**atomes de Rydberg** (des atomes portés à des nombres quantiques principaux extrêmement élevés) animé par une dynamique d'excitation facilitée.

Les atomes de Rydberg sont énormes à l'échelle atomique. Un atome de rubidium dans l'état de Rydberg 70s possède une orbitale électronique des milliers de fois plus grande que dans l'état fondamental. Cette taille démesurée leur donne des propriétés hors du commun : ils interagissent entre eux par de fortes forces de **van der Waals** à longue portée, qui décroissent en 1/r^6.

Le phénomène clé, c'est le **blocage de Rydberg** : dès qu'un atome est excité, son interaction de van der Waals décale les niveaux d'énergie des atomes voisins et les empêche de s'exciter à leur tour. Il se forme ainsi une zone d'exclusion (le rayon de blocage) autour de chaque atome de Rydberg. Mais si l'on désaccorde légèrement le laser d'excitation vers le bleu par rapport à la résonance fondamental-Rydberg, le décalage répulsif dû à un atome de Rydberg voisin peut compenser ce désaccord à une distance bien précise et ramener la transition à résonance. On obtient alors une **coquille de facilitation**, où l'excitation est au contraire favorisée.

Les points bleus sont les atomes dans l'état fondamental, retenus par un piège magnéto-optique (MOT). Les éclats jaune vif sont des atomes portés à l'état de Rydberg. Regardez comme les excitations ont tendance à surgir à des distances bien précises des atomes de Rydberg déjà présents : voilà la coquille de facilitation. Les atomes excités se repoussent, puis finissent par retomber dans l'état fondamental.

Et ce n'est pas qu'une curiosité. Les réseaux d'atomes de Rydberg comptent parmi les plateformes les plus prometteuses pour le **calcul quantique**. Des entreprises comme QuEra ou Pasqal, et des équipes de recherche partout dans le monde, s'en servent : elles agencent des atomes de Rydberg avec une grande précision pour bâtir des processeurs quantiques dotés de centaines de qubits.

### Sous le capot

La simulation associe une dynamique d'excitation stochastique par algorithme de Gillespie, une intégration mécanique velocity-Verlet et un thermostat de Langevin pour le refroidissement du MOT. Les taux d'excitation suivent un profil lorentzien centré sur la résonance de facilitation. En sortie, on obtient une texture flottante à deux canaux (le champ de densité des atomes fondamentaux et celui des atomes excités), affichés sous forme de taches gaussiennes par le fragment shader.
