---
title: "MIMIQ: Ordinateur Quantique Virtuel"
date: 2022-06-01
summary: "L'ordinateur quantique virtuel le plus puissant au monde. Simulation rapide et precise de circuits quantiques avec moteurs statevector et MPS, SDK Python et Julia, et deploiement cloud gere."
tags: ["informatique quantique", "simulation", "julia", "python"]
---

## Apercu

[MIMIQ](https://qperfect.io/mimiq/) est le produit phare de [QPerfect](https://qperfect.io), un ordinateur quantique virtuel qui permet aux chercheurs et ingenieurs de programmer et executer des algorithmes quantiques avec une vitesse, une precision et une flexibilite inegalees. Il permet de concevoir, tester et valider des circuits quantiques avant de les executer sur du materiel reel, ou d'explorer des regimes qu'aucun materiel actuel ne peut atteindre.

En tant que CTO, j'ai concu l'architecture de MIMIQ depuis les fondations, dirigeant son developpement d'un prototype academique vers une plateforme de production servant des laboratoires de recherche et des clients entreprise.

## Technologie Centrale

MIMIQ integre deux moteurs de simulation complementaires :

- **Moteur Statevector**: Simulation exacte ultra-rapide de circuits quantiques jusqu'a ~32 qubits, optimisee avec des instructions CPU bas niveau (AVX/SIMD), un pre-conditionnement avance des circuits et une gestion memoire efficace.
- **Moteur Matrix Product States (MPS)**: Simulation a grande echelle utilisant des techniques de reseaux de tenseurs, permettant des circuits avec **des milliers de qubits** pour les problemes a intrication limitee. Ce moteur rend MIMIQ unique dans sa capacite a traiter des circuits hors de portee des approches statevector par force brute.

MIMIQ permet le calcul rapide et numeriquement exact de circuits quantiques arbitraires avec **des millions de portes** et de circuits a intrication limitee avec **des milliers de qubits**.

## SDK MimiqCircuits

[MimiqCircuits](https://github.com/qperfect-io/) fournit des bibliotheques open-source de circuits quantiques en **Python** et **Julia** :

- Vaste bibliotheque de portes et primitives pour la composition efficace de circuits
- Circuits dynamiques : mesures mid-circuit, reinitialisation de qubits, feedforward classique et logique conditionnelle
- Acces complet aux proprietes de l'etat quantique : amplitudes, valeurs moyennes, mesures d'intrication et fidelite du circuit
- Support complet OpenQASM v2 pour l'importation et l'exportation de circuits
- Simulation de circuits quantiques ideaux et bruites avec modeles de bruit personnalisables
- Integration native avec le backend cloud MIMIQ

## Deploiement

MIMIQ est disponible en tant que service cloud gere ou en deploiement on-premises pour clusters HPC. La plateforme cloud fournit un acces instantane via les SDK Python et Julia, tandis que l'option on-premises offre un controle total sur les ressources materielles et la souverainete des donnees.

## Benchmarks

Teste sur la bibliotheque MQT Bench a travers 28 algorithmes quantiques et plus de 70 000 circuits (de 2 a 130+ qubits), MIMIQ atteint une **precision de 100% pour la grande majorite des circuits**, plus que toute autre plateforme quantique testee.

## Liens

- [Plateforme MIMIQ](https://qperfect.io/mimiq/)
- [QPerfect](https://qperfect.io)
- [MimiqCircuits sur GitHub](https://github.com/qperfect-io/)
