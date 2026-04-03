---
title: "Ceci n'est pas juste une animation : les pinces optiques"
date: 2026-03-11
summary: "L'arrière-plan que vous venez de voir est une vraie simulation de tri par pinces optiques exécutée dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "calcul-quantique"]
---

Vous avez peut-être remarqué l'arrière-plan animé de la page d'où vous venez. Il peut sembler décoratif, mais c'est une **vraie simulation physique** exécutée en temps réel dans votre navigateur, compilée en WebAssembly à partir de Rust.

Pas de vidéo pré-rendue. Pas d'astuces CSS. Les mathématiques sont réellement calculées en ce moment même, sur votre appareil.

Ces simulations sont inspirées de la physique réelle mais optimisées pour l'impact visuel. Les paramètres sont choisis pour être beaux, pas pour reproduire une expérience spécifique. Pensez-y comme de l'**art computationnel ancré dans la vraie science**.

---

{{< simulation name="tweezer" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Tri atomique par pinces optiques

Ce que vous avez vu est le processus de **tri atomique** utilisé dans les ordinateurs quantiques à atomes neutres. Des atomes sont chargés aléatoirement dans un réseau de pinces optiques (des faisceaux laser focalisés qui piègent des atomes individuels), et une pince de tri mobile les réarrange dans une région cible compacte et sans défauts.

Dans les expériences réelles, chaque piège capture un atome avec une probabilité d'environ 50%. Une caméra image le réseau pour voir quels pièges sont chargés. Ensuite, un algorithme rapide calcule le plan de réarrangement optimal, et une pince motorisée prélève les atomes un par un (ou en parallèle), les transportant pour combler les lacunes. L'ensemble du processus prend quelques millisecondes et produit un registre de qubits parfait, sans défauts, prêt pour le calcul quantique.

La simulation alterne entre deux vrais algorithmes de tri :

**Algorithme hongrois** (assignation optimale) : Une seule pince exécute le plan mathématiquement optimal. L'algorithme de Kuhn-Munkres trouve l'assignation des atomes sources aux positions cibles qui minimise le déplacement total. La pince suit des chemins interstitiels (voyageant à travers les espaces entre les sites de piège) pour éviter de perturber les autres atomes pendant le transport. C'est l'approche théoriquement optimale.

**Compression** (pinces parallèles) : Plusieurs pinces opèrent simultanément, comprimant d'abord toutes les colonnes vers le centre, puis toutes les lignes. Chaque étape déplace les atomes d'exactement un espacement de piège, toutes les pinces marchant au pas. C'est plus proche de la façon dont les expériences réelles fonctionnent : ce n'est pas globalement optimal, mais c'est rapide et naturellement parallélisable.

Les cercles gris sont les pièges optiques statiques. Les points teal brillants sont les atomes piégés. La lueur jaune est la pince de tri (ou les pinces, pendant la compression). Observez comment l'algorithme hongrois produit des chemins élégants et efficaces, tandis que la compression crée un balayage coordonné et satisfaisant.

Cette technologie est au cœur d'entreprises comme **QuEra**, **Pasqal** et **Atom Computing**, qui construisent des ordinateurs quantiques avec des centaines à des milliers de qubits à atomes neutres arrangés par des pinces optiques.

### Sous le capot

La simulation exécute l'ensemble du pipeline de tri : chargement aléatoire, calcul de la région cible, planification des mouvements spécifique à l'algorithme (hongrois avec Kuhn-Munkres O(n^3), ou compression par lignes/colonnes avec des étapes parallèles à un seul saut), et exécution animée avec pathfinding interstitiel. La sortie est une texture flottante à deux canaux : le canal R encode les pièges et les atomes (blobs gaussiens à différentes amplitudes), le canal G encode les positions des pinces. Le fragment shader compose les pièges (gris), les pinces (jaune) et les atomes (teal) avec une superposition correcte.
