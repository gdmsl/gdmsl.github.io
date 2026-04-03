---
title: "Ceci n'est pas juste une animation : les atomes de Rydberg"
date: 2026-03-12
summary: "L'arrière-plan que vous venez de voir est une vraie simulation d'atomes de Rydberg exécutée dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "calcul-quantique"]
---

Vous avez peut-être remarqué l'arrière-plan animé de la page d'où vous venez. Il peut sembler décoratif, mais c'est une **vraie simulation physique** exécutée en temps réel dans votre navigateur, compilée en WebAssembly à partir de Rust.

Pas de vidéo pré-rendue. Pas d'astuces CSS. Les mathématiques sont réellement calculées en ce moment même, sur votre appareil.

Ces simulations sont inspirées de la physique réelle mais optimisées pour l'impact visuel. Les paramètres sont choisis pour être beaux, pas pour reproduire une expérience spécifique. Pensez-y comme de l'**art computationnel ancré dans la vraie science**.

---

{{< simulation name="rydberg" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Atomes de Rydberg

Ce que vous avez vu est un gaz d'**atomes de Rydberg** (des atomes excités à des nombres quantiques principaux extrêmement élevés) présentant des dynamiques d'excitation facilitée.

Les atomes de Rydberg sont énormes selon les standards atomiques. Un atome de rubidium dans l'état de Rydberg 70s a un orbital électronique des milliers de fois plus grand que l'état fondamental. Cette taille extrême confère aux atomes de Rydberg des propriétés extraordinaires : ils interagissent entre eux via de fortes forces de **van der Waals** à longue portée qui décroissent en 1/r^6.

Le phénomène clé est le **blocage de Rydberg** : quand un atome est excité, son interaction de van der Waals déplace les niveaux d'énergie des atomes voisins, les empêchant d'être excités aussi. Cela crée une zone d'exclusion (le rayon de blocage) autour de chaque atome de Rydberg. Cependant, si le laser utilisé pour l'excitation est légèrement désaccordé vers le bleu par rapport à la résonance fondamental-Rydberg, le décalage répulsif de van der Waals d'un atome de Rydberg voisin peut compenser le désaccord à une distance spécifique, ramenant la transition en résonance. Cela crée une **coquille de facilitation** où l'excitation est amplifiée.

Les points bleus sont les atomes dans l'état fondamental, confinés par un piège magnéto-optique (MOT). Les éclats jaunes brillants sont des atomes excités à l'état de Rydberg. Observez comment les excitations tendent à apparaître à des distances spécifiques des atomes de Rydberg existants : c'est la coquille de facilitation. Les atomes excités se repoussent et finissent par retomber à l'état fondamental.

Ce n'est pas qu'une curiosité. Les réseaux d'atomes de Rydberg sont l'une des plateformes leaders pour le **calcul quantique**. Des entreprises comme QuEra, Pasqal et des groupes de recherche du monde entier utilisent des réseaux précisément contrôlés d'atomes de Rydberg pour construire des processeurs quantiques avec des centaines de qubits.

### Sous le capot

La simulation combine des dynamiques d'excitation stochastiques avec l'algorithme de Gillespie, une intégration mécanique velocity-Verlet et un thermostat de Langevin pour le refroidissement MOT. Les taux d'excitation suivent un profil lorentzien centré sur la résonance de facilitation. La sortie est une texture flottante à deux canaux : champ de densité des atomes fondamentaux et champ de densité des atomes excités, rendus comme des blobs gaussiens via le fragment shader.
