---
title: "MIMIQ : ordinateur quantique virtuel"
date: 2022-06-01
summary: "L'ordinateur quantique virtuel le plus puissant au monde. Simulation rapide et précise de circuits quantiques avec moteurs statevector et MPS, SDK Python et Julia, et déploiement cloud géré."
tags: ["informatique quantique", "simulation", "julia", "python"]
---

## Aperçu

[MIMIQ](https://qperfect.io/mimiq/) est le produit phare de [QPerfect](https://qperfect.io) : un ordinateur quantique virtuel qui permet aux chercheurs et aux ingénieurs de programmer et d'exécuter des algorithmes quantiques avec rapidité, précision et souplesse. On peut concevoir, tester et valider ses circuits quantiques avant de les lancer sur du matériel réel, ou explorer des régimes qu'aucun matériel actuel n'atteint.

En tant que CTO, j'ai conçu l'architecture de MIMIQ de A à Z et mené son développement, faisant passer un prototype issu de la recherche à une plateforme de production utilisée par des laboratoires de recherche et des clients industriels.

## Technologie

MIMIQ combine deux moteurs de simulation complémentaires :

- **Moteur Statevector** : simulation exacte ultra-rapide de circuits quantiques jusqu'à ~32 qubits, grâce à des optimisations CPU bas niveau (AVX/SIMD), un préconditionnement avancé des circuits et une gestion économe de la mémoire.
- **Moteur Matrix Product States (MPS)** : simulation à grande échelle par réseaux de tenseurs, capable de gérer des circuits de **plusieurs milliers de qubits** lorsque l'intrication reste limitée. MIMIQ traite ainsi des circuits hors de portée des approches statevector par force brute.

MIMIQ calcule de façon rapide et numériquement exacte des circuits quantiques quelconques comptant **des millions de portes**, et des circuits à intrication limitée comptant **des milliers de qubits**.

## SDK MimiqCircuits

[MimiqCircuits](https://github.com/qperfect-io/) propose des bibliothèques open source de circuits quantiques, en **Python** comme en **Julia** :

- une vaste bibliothèque de portes et de primitives pour composer des circuits efficacement ;
- des circuits dynamiques : mesures mid-circuit, réinitialisation de qubits, feedforward classique et logique conditionnelle ;
- un accès complet aux propriétés de l'état quantique : amplitudes, valeurs moyennes, mesures d'intrication et fidélité du circuit ;
- la prise en charge complète d'OpenQASM v2 pour importer et exporter des circuits ;
- la simulation de circuits quantiques, idéaux comme bruités, avec des modèles de bruit personnalisables ;
- l'intégration avec le backend cloud MIMIQ.

## Déploiement

MIMIQ est disponible sous forme de service cloud géré ou en installation on-premises sur clusters HPC. Le cloud donne un accès immédiat via les SDK Python et Julia ; l'option on-premises, elle, offre la maîtrise totale des ressources matérielles et la souveraineté des données.

## Benchmarks

Évalué avec la bibliothèque MQT Bench sur 28 algorithmes quantiques et plus de 70 000 circuits (de 2 à plus de 130 qubits), MIMIQ atteint une **précision de 100 % sur la grande majorité des circuits**, plus que toute autre plateforme quantique testée.

## Liens

- [Plateforme MIMIQ](https://qperfect.io/mimiq/)
- [QPerfect](https://qperfect.io)
- [MimiqCircuits sur GitHub](https://github.com/qperfect-io/)
