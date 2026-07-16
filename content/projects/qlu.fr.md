---
title: "QLU: Quantum Logic Unit"
date: 2024-06-01
summary: "Compilation fault-tolerant et correction d'erreurs en temps réel pour les ordinateurs quantiques à atomes neutres. Le trait d'union entre le logiciel quantique et le matériel pour l'ère fault-tolerant."
tags: ["informatique quantique", "fault tolerance", "correction d'erreurs", "atomes neutres"]
---

## Aperçu

[QLU](https://qperfect.io/qlu/) est le middleware quantique fault-tolerant de [QPerfect](https://qperfect.io) : un système temps réel qui compile, optimise et corrige les erreurs des circuits quantiques en vue de leur exécution sur des processeurs quantiques à atomes neutres. Il fait le lien entre les algorithmes quantiques de haut niveau et les contraintes physiques du matériel réel.

En tant que CTO, je pilote l'architecture et le développement de QLU, en m'appuyant sur le savoir-faire de QPerfect en simulation quantique et en physique des atomes neutres.

## Fonctionnalités principales

- **Compilateur fault-tolerant** : compile les circuits quantiques logiques en circuits physiques fault-tolerant, avec correction d'erreurs quantiques, distillation d'états magiques et détection de pertes. Prend en charge les codes QLDPC et les codes de surface.
- **Optimisation adaptée au matériel** : ajuste l'exécution des circuits aux QPU à atomes neutres à architecture zonée : jeux de portes natifs (RX, RZ, CZ), ordonnancement des déplacements d'atomes et rechargement continu des atomes.
- **Décodeur d'erreurs en temps réel** : décode la correction d'erreurs quantiques à la volée, pour un fonctionnement fault-tolerant continu pendant l'exécution du circuit.
- **QDK cloud sécurisé** : un Quantum Development Kit avec accès cloud sécurisé pour concevoir, compiler et piloter à distance l'exécution des circuits.

## Co-design matériel

Plusieurs choix de conception de QLU découlent d'une collaboration étroite avec [aQCess](https://aQCess.eu), la première plateforme publique de calcul quantique à atomes neutres en France, à l'Université de Strasbourg.

## Synergie avec MIMIQ

[MIMIQ](/projects/mimiq/) sert de socle à QLU : il offre déjà un formalisme pour représenter et manipuler les opérations quantiques.

## Liens

- [Plateforme QLU](https://qperfect.io/qlu/)
- [QPerfect](https://qperfect.io)
