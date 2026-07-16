---
title: "Ceci n'est pas juste une animation : le modèle d'Ising"
date: 2026-03-14
summary: "L'arrière-plan que vous venez de voir est une vraie simulation du modèle d'Ising exécutée dans votre navigateur via WebAssembly."
tags: ["physique", "simulation", "mécanique-statistique"]
---

Vous avez peut-être remarqué l'arrière-plan animé sur la page d'où vous venez. Il a l'air décoratif, mais c'est en réalité une **vraie simulation physique** qui tourne en direct dans votre navigateur, compilée en WebAssembly depuis du Rust.

Aucune vidéo pré-enregistrée, aucune astuce CSS. Les calculs se font vraiment, en ce moment même, sur votre appareil.

Ces simulations s'inspirent de la physique réelle, mais je les ai réglées pour le plaisir des yeux. J'ai choisi les paramètres pour le rendu, pas pour coller à une expérience précise. Voyez-les comme de l'**art computationnel ancré dans la science**.

---

{{< simulation name="ising" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Le modèle d'Ising

Ce que vous avez vu, c'est le **modèle d'Ising**, l'un des systèmes les plus étudiés de toute la physique. Il paraît simple : une grille de spins, chacun orienté vers le haut ou vers le bas, qui n'interagissent qu'avec leurs plus proches voisins. Et de cette simplicité émerge un phénomène parmi les plus profonds de la nature : une **transition de phase**.

À haute température, les fluctuations thermiques l'emportent. Les spins basculent sans cesse et au hasard, dans un désordre bruyant. À basse température, c'est le coût énergétique des voisins désalignés qui prend le dessus : de grands **domaines** de spins alignés apparaissent et s'étendent. Entre les deux se trouve la température critique, où le système se tient à la frontière de l'ordre et du désordre : les fluctuations ont lieu à toutes les échelles et le système devient invariant d'échelle.

La simulation utilise l'**algorithme de clusters de Swendsen-Wang**. Plutôt que de retourner un spin à la fois (comme le fait l'algorithme de Metropolis, plus simple), Swendsen-Wang regroupe les spins alignés voisins en activant des liens entre eux avec une probabilité qui dépend de la température. Chaque cluster est ensuite retourné d'un bloc, une fois sur deux. Le système peut ainsi effectuer de grandes mises à jour collectives en une seule passe, ce qui compte particulièrement près de la température critique, là où les méthodes qui ne touchent qu'un spin à la fois se heurtent au ralentissement critique.

Les éclats teal signalent les clusters qui viennent de basculer d'un coup : on suit ainsi la dynamique collective en temps réel.

Ernst Ising a résolu la version 1D en 1924 sans trouver de transition de phase, ce qui l'a conduit à supposer (à tort) qu'il n'y en avait pas non plus en dimension supérieure. La résolution exacte du modèle 2D par Lars Onsager en 1944, qui met en évidence une transition de phase nette, reste l'un des grands résultats de la physique mathématique.

### Dans les coulisses

La simulation fait tourner un Monte Carlo à clusters de Swendsen-Wang sur un réseau carré 2D avec conditions aux limites périodiques. Les clusters sont identifiés grâce à une structure de données union-find. Le résultat est une texture flottante à deux canaux : l'état du spin et le glow (les clusters retournés récemment). Le fragment shader affiche les spins vers le haut en bleu électrique, les spins vers le bas en bleu marine profond, et ajoute un glow teal sur les clusters qui viennent de basculer. Les bordures des cellules donnent un peu de structure visuelle.
