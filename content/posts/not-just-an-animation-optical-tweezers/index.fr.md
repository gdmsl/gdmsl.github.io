---
title: "Ceci N'est Pas Juste une Animation: Pinces Optiques"
date: 2026-03-11
summary: "L'arriere-plan que vous venez de voir est une vraie simulation de tri par pinces optiques executee dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "calcul-quantique"]
---

Vous avez peut-etre remarque l'arriere-plan anime de la page d'ou vous venez. Il peut sembler decoratif, mais c'est une **vraie simulation physique** executee en temps reel dans votre navigateur, compilee en WebAssembly a partir de Rust.

Pas de video pre-rendue. Pas d'astuces CSS. Les mathematiques sont reellement calculees en ce moment meme, sur votre appareil.

Ces simulations sont inspirees de la physique reelle mais optimisees pour l'impact visuel. Les parametres sont choisis pour etre beaux, pas pour reproduire une experience specifique. Pensez-y comme de l'**art computationnel ancre dans la vraie science**.

---

{{< simulation name="tweezer" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Tri Atomique par Pinces Optiques

Ce que vous avez vu est le processus de **tri atomique** utilise dans les ordinateurs quantiques a atomes neutres. Des atomes sont charges aleatoirement dans un reseau de pinces optiques (des faisceaux laser focalises qui piegent des atomes individuels), et une pince de tri mobile les rearrange dans une region cible compacte et sans defauts.

Dans les experiences reelles, chaque piege capture un atome avec une probabilite d'environ 50%. Une camera image le reseau pour voir quels pieges sont charges. Ensuite, un algorithme rapide calcule le plan de rearrangement optimal, et une pince motorisee preleve les atomes un par un (ou en parallele), les transportant pour combler les lacunes. L'ensemble du processus prend quelques millisecondes et produit un registre de qubits parfait, sans defauts, pret pour le calcul quantique.

La simulation alterne entre deux vrais algorithmes de tri :

**Algorithme hongrois** (assignation optimale) : Une seule pince execute le plan mathematiquement optimal. L'algorithme de Kuhn-Munkres trouve l'assignation des atomes sources aux positions cibles qui minimise le deplacement total. La pince suit des chemins interstitiels (voyageant a travers les espaces entre les sites de piege) pour eviter de perturber les autres atomes pendant le transport. C'est l'approche theoriquement optimale.

**Compression** (pinces paralleles) : Plusieurs pinces operent simultanement, comprimant d'abord toutes les colonnes vers le centre, puis toutes les lignes. Chaque etape deplace les atomes d'exactement un espacement de piege, toutes les pinces marchant au pas. C'est plus proche de la facon dont les experiences reelles fonctionnent, ce n'est pas globalement optimal, mais c'est rapide et naturellement parallelisable.

Les cercles gris sont les pieges optiques statiques. Les points teal brillants sont les atomes pieges. La lueur jaune est la pince de tri (ou les pinces, pendant la compression). Observez comment l'algorithme hongrois produit des chemins elegants et efficaces, tandis que la compression cree un balayage coordonne et satisfaisant.

Cette technologie est au coeur d'entreprises comme **QuEra**, **Pasqal** et **Atom Computing**, qui construisent des ordinateurs quantiques avec des centaines a des milliers de qubits a atomes neutres arranges par des pinces optiques.

### Sous le capot

La simulation execute l'ensemble du pipeline de tri : chargement aleatoire, calcul de la region cible, planification des mouvements specifique a l'algorithme (hongrois avec Kuhn-Munkres O(n^3), ou compression par lignes/colonnes avec des etapes paralleles a un seul saut), et execution animee avec pathfinding interstitiel. La sortie est une texture flottante a deux canaux : le canal R encode les pieges et les atomes (blobs gaussiens a differentes amplitudes), le canal G encode les positions des pinces. Le fragment shader compose les pieges (gris), les pinces (jaune) et les atomes (teal) avec une superposition correcte.
