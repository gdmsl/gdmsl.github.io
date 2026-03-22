---
title: "QLU: Quantum Logic Unit"
date: 2024-06-01
summary: "Compilation fault-tolerant et correction d'erreurs en temps reel pour ordinateurs quantiques a atomes neutres. Le lien entre logiciel quantique et materiel pour l'ere fault-tolerant."
tags: ["informatique quantique", "fault tolerance", "correction d'erreurs", "atomes neutres"]
---

## Apercu

[QLU](https://qperfect.io/qlu/) est le middleware quantique fault-tolerant de [QPerfect](https://qperfect.io), un systeme en temps reel qui compile, optimise et corrige les erreurs des circuits quantiques pour l'execution sur des processeurs quantiques a atomes neutres. Il comble le fosse entre les algorithmes quantiques de haut niveau et les contraintes physiques du materiel reel.

En tant que CTO, je dirige l'architecture et le developpement de QLU, en m'appuyant sur l'expertise approfondie de QPerfect en simulation quantique et en physique des atomes neutres.

## Fonctionnalites Principales

- **Compilateur fault-tolerant**: Compile les circuits quantiques logiques en circuits physiques fault-tolerant avec correction d'erreurs quantiques, distillation d'etats magiques et detection de pertes. Support des codes QLDPC et des codes de surface.
- **Optimisation specifique au materiel**: Adapte l'execution des circuits pour les QPU a atomes neutres avec architectures zonees, jeux de portes natifs (RX, RZ, CZ), ordonnancement du mouvement des atomes et rechargement continu des atomes.
- **Decodeur d'erreurs en temps reel**: Decodage de la correction d'erreurs quantiques en temps reel pour des operations fault-tolerant continues pendant l'execution du circuit.
- **QDK cloud securise**: Quantum Development Kit avec acces cloud securise pour la conception, la compilation et la gestion de l'execution des circuits a distance.

## Co-design Materiel

Plusieurs aspects de QLU sont inspires par une etroite collaboration avec [aQCess](https://aQCess.eu), la premiere plateforme publique de calcul quantique a atomes neutres en France, a l'Universite de Strasbourg.

## Synergie avec MIMIQ

[MIMIQ](/projects/mimiq/) sert de base a QLU, car il fournit deja un formalisme pour representer et manipuler les operations quantiques.

## Liens

- [Plateforme QLU](https://qperfect.io/qlu/)
- [QPerfect](https://qperfect.io)
