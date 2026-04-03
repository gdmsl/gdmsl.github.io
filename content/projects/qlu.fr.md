---
title: "QLU: Quantum Logic Unit"
date: 2024-06-01
summary: "Compilation fault-tolerant et correction d'erreurs en temps réel pour ordinateurs quantiques à atomes neutres. Le lien entre logiciel quantique et matériel pour l'ère fault-tolerant."
tags: ["informatique quantique", "fault tolerance", "correction d'erreurs", "atomes neutres"]
---

## Aperçu

[QLU](https://qperfect.io/qlu/) est le middleware quantique fault-tolerant de [QPerfect](https://qperfect.io), un système en temps réel qui compile, optimise et corrige les erreurs des circuits quantiques pour l'exécution sur des processeurs quantiques à atomes neutres. Il comble le fossé entre les algorithmes quantiques de haut niveau et les contraintes physiques du matériel réel.

En tant que CTO, je dirige l'architecture et le développement de QLU, en m'appuyant sur l'expertise approfondie de QPerfect en simulation quantique et en physique des atomes neutres.

## Fonctionnalités principales

- **Compilateur fault-tolerant** : Compile les circuits quantiques logiques en circuits physiques fault-tolerant avec correction d'erreurs quantiques, distillation d'états magiques et détection de pertes. Support des codes QLDPC et des codes de surface.
- **Optimisation spécifique au matériel** : Adapte l'exécution des circuits pour les QPU à atomes neutres avec architectures zonées, jeux de portes natifs (RX, RZ, CZ), ordonnancement du mouvement des atomes et rechargement continu des atomes.
- **Décodeur d'erreurs en temps réel** : Décodage de la correction d'erreurs quantiques en temps réel pour des opérations fault-tolerant continues pendant l'exécution du circuit.
- **QDK cloud sécurisé** : Quantum Development Kit avec accès cloud sécurisé pour la conception, la compilation et la gestion de l'exécution des circuits à distance.

## Co-design matériel

Plusieurs aspects de QLU sont inspirés par une étroite collaboration avec [aQCess](https://aQCess.eu), la première plateforme publique de calcul quantique à atomes neutres en France, à l'Université de Strasbourg.

## Synergie avec MIMIQ

[MIMIQ](/projects/mimiq/) sert de base à QLU, car il fournit déjà un formalisme pour représenter et manipuler les opérations quantiques.

## Liens

- [Plateforme QLU](https://qperfect.io/qlu/)
- [QPerfect](https://qperfect.io)
