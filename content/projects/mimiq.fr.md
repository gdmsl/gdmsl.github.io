---
title: "MIMIQ : ordinateur quantique virtuel"
date: 2022-06-01
summary: "L'ordinateur quantique virtuel le plus puissant au monde. Simulation rapide et précise de circuits quantiques avec moteurs statevector et MPS, SDK Python et Julia, et déploiement cloud géré."
tags: ["informatique quantique", "simulation", "julia", "python"]
---

## Aperçu

[MIMIQ](https://qperfect.io/mimiq/) est le produit phare de [QPerfect](https://qperfect.io), un ordinateur quantique virtuel qui permet aux chercheurs et ingénieurs de programmer et exécuter des algorithmes quantiques avec une vitesse, une précision et une flexibilité inégalées. Il permet de concevoir, tester et valider des circuits quantiques avant de les exécuter sur du matériel réel, ou d'explorer des régimes qu'aucun matériel actuel ne peut atteindre.

En tant que CTO, j'ai conçu l'architecture de MIMIQ depuis les fondations, dirigeant son développement d'un prototype académique vers une plateforme de production servant des laboratoires de recherche et des clients entreprise.

## Technologie centrale

MIMIQ intègre deux moteurs de simulation complémentaires :

- **Moteur Statevector** : Simulation exacte ultra-rapide de circuits quantiques jusqu'à ~32 qubits, optimisée avec des instructions CPU bas niveau (AVX/SIMD), un pré-conditionnement avancé des circuits et une gestion mémoire efficace.
- **Moteur Matrix Product States (MPS)** : Simulation à grande échelle utilisant des techniques de réseaux de tenseurs, permettant des circuits avec **des milliers de qubits** pour les problèmes à intrication limitée. Ce moteur rend MIMIQ unique dans sa capacité à traiter des circuits hors de portée des approches statevector par force brute.

MIMIQ permet le calcul rapide et numériquement exact de circuits quantiques arbitraires avec **des millions de portes** et de circuits à intrication limitée avec **des milliers de qubits**.

## SDK MimiqCircuits

[MimiqCircuits](https://github.com/qperfect-io/) fournit des bibliothèques open-source de circuits quantiques en **Python** et **Julia** :

- Vaste bibliothèque de portes et primitives pour la composition efficace de circuits
- Circuits dynamiques : mesures mid-circuit, réinitialisation de qubits, feedforward classique et logique conditionnelle
- Accès complet aux propriétés de l'état quantique : amplitudes, valeurs moyennes, mesures d'intrication et fidélité du circuit
- Support complet OpenQASM v2 pour l'importation et l'exportation de circuits
- Simulation de circuits quantiques idéaux et bruités avec modèles de bruit personnalisables
- Intégration native avec le backend cloud MIMIQ

## Déploiement

MIMIQ est disponible en tant que service cloud géré ou en déploiement on-premises pour clusters HPC. La plateforme cloud fournit un accès instantané via les SDK Python et Julia, tandis que l'option on-premises offre un contrôle total sur les ressources matérielles et la souveraineté des données.

## Benchmarks

Testé sur la bibliothèque MQT Bench à travers 28 algorithmes quantiques et plus de 70 000 circuits (de 2 à 130+ qubits), MIMIQ atteint une **précision de 100% pour la grande majorité des circuits**, plus que toute autre plateforme quantique testée.

## Liens

- [Plateforme MIMIQ](https://qperfect.io/mimiq/)
- [QPerfect](https://qperfect.io)
- [MimiqCircuits sur GitHub](https://github.com/qperfect-io/)
