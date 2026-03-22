---
title: "Ceci N'est Pas Juste une Animation: Atomes de Rydberg"
date: 2026-03-12
summary: "L'arriere-plan que vous venez de voir est une vraie simulation d'atomes de Rydberg executee dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "calcul-quantique"]
---

Vous avez peut-etre remarque l'arriere-plan anime de la page d'ou vous venez. Il peut sembler decoratif, mais c'est une **vraie simulation physique** executee en temps reel dans votre navigateur, compilee en WebAssembly a partir de Rust.

Pas de video pre-rendue. Pas d'astuces CSS. Les mathematiques sont reellement calculees en ce moment meme, sur votre appareil.

Ces simulations sont inspirees de la physique reelle mais optimisees pour l'impact visuel. Les parametres sont choisis pour etre beaux, pas pour reproduire une experience specifique. Pensez-y comme de l'**art computationnel ancre dans la vraie science**.

---

{{< simulation name="rydberg" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Atomes de Rydberg

Ce que vous avez vu est un gaz d'**atomes de Rydberg** (des atomes excites a des nombres quantiques principaux extremement eleves) exhibant des dynamiques d'excitation facilitee.

Les atomes de Rydberg sont enormes selon les standards atomiques. Un atome de rubidium dans l'etat de Rydberg 70s a un orbital electronique des milliers de fois plus grand que l'etat fondamental. Cette taille extreme confere aux atomes de Rydberg des proprietes extraordinaires : ils interagissent entre eux via de fortes forces de **van der Waals** a longue portee qui decroissent en 1/r^6.

Le phenomene cle est le **blocage de Rydberg** : quand un atome est excite, son interaction de van der Waals deplace les niveaux d'energie des atomes voisins, les empechant d'etre excites aussi. Cela cree une zone d'exclusion (le rayon de blocage) autour de chaque atome de Rydberg. Cependant, si le laser utilise pour l'excitation est legerement desaccorde vers le bleu par rapport a la resonance fondamental-Rydberg, le decalage repulsif de van der Waals d'un atome de Rydberg voisin peut compenser le desaccord a une distance specifique, ramenant la transition en resonance. Cela cree une **coquille de facilitation** ou l'excitation est amplifiee.

Les points bleus sont les atomes dans l'etat fondamental, confines par un piege magneto-optique (MOT). Les eclats jaunes brillants sont des atomes excites a l'etat de Rydberg. Observez comment les excitations tendent a apparaitre a des distances specifiques des atomes de Rydberg existants, c'est la coquille de facilitation. Les atomes excites se repoussent et finissent par retomber a l'etat fondamental.

Ce n'est pas qu'une curiosite. Les reseaux d'atomes de Rydberg sont l'une des plateformes leaders pour le **calcul quantique**. Des entreprises comme QuEra, Pasqal et des groupes de recherche du monde entier utilisent des reseaux precisement controles d'atomes de Rydberg pour construire des processeurs quantiques avec des centaines de qubits.

### Sous le capot

La simulation combine des dynamiques d'excitation stochastiques avec l'algorithme de Gillespie, une integration mecanique velocity-Verlet et un thermostat de Langevin pour le refroidissement MOT. Les taux d'excitation suivent un profil lorentzien centre sur la resonance de facilitation. La sortie est une texture flottante a deux canaux : champ de densite des atomes fondamentaux et champ de densite des atomes excites, rendus comme des blobs gaussiens via le fragment shader.
