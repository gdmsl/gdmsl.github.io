---
title: "MIMIQ: Computer Quantistico Virtuale"
date: 2022-06-01
summary: "Il computer quantistico virtuale piu potente al mondo. Simulazione veloce e accurata di circuiti quantistici con motori statevector e MPS, SDK Python e Julia, e deployment cloud gestito."
tags: ["quantum computing", "simulazione", "julia", "python"]
---

## Panoramica

[MIMIQ](https://qperfect.io/mimiq/) e il prodotto di punta di [QPerfect](https://qperfect.io), un computer quantistico virtuale che permette a ricercatori e ingegneri di programmare ed eseguire algoritmi quantistici con velocita, accuratezza e flessibilita senza pari. Permette di progettare, testare e validare circuiti quantistici prima di eseguirli su hardware reale, o di esplorare regimi che nessun hardware attuale puo raggiungere.

Come CTO, ho progettato MIMIQ dalle fondamenta, guidandone lo sviluppo da prototipo accademico a piattaforma di produzione al servizio di laboratori di ricerca e clienti enterprise.

## Tecnologia Core

MIMIQ integra due motori di simulazione complementari:

- **Motore Statevector**: Simulazione esatta ultraveloce di circuiti quantistici fino a ~32 qubit, ottimizzata con istruzioni CPU a basso livello (AVX/SIMD), pre-condizionamento avanzato dei circuiti e gestione efficiente della memoria.
- **Motore Matrix Product States (MPS)**: Simulazione su larga scala tramite tecniche di reti tensoriali, che abilita circuiti con **migliaia di qubit** per problemi a entanglement limitato. Questo motore rende MIMIQ unico nella capacita di gestire circuiti irraggiungibili per gli approcci statevector a forza bruta.

MIMIQ permette il calcolo veloce e numericamente esatto di circuiti quantistici arbitrari con **milioni di gate** e circuiti a entanglement limitato con **migliaia di qubit**.

## SDK MimiqCircuits

[MimiqCircuits](https://github.com/qperfect-io/) fornisce librerie open-source per circuiti quantistici in **Python** e **Julia**:

- Ampia libreria di gate e primitive per la composizione efficiente di circuiti
- Circuiti dinamici: misure mid-circuit, reset dei qubit, feedforward classico e logica condizionale
- Accesso completo alle proprieta dello stato quantistico: ampiezze, valori di aspettazione, misure di entanglement e fedelta del circuito
- Supporto completo OpenQASM v2 per importazione ed esportazione di circuiti
- Simulazione di circuiti quantistici ideali e rumorosi con modelli di rumore personalizzabili
- Integrazione nativa con il backend cloud MIMIQ

## Deployment

MIMIQ e disponibile come servizio cloud gestito o come deployment on-premises per cluster HPC. La piattaforma cloud fornisce accesso immediato tramite SDK Python e Julia, mentre l'opzione on-premises offre pieno controllo sulle risorse hardware e sulla sovranita dei dati.

## Benchmark

Testato sulla libreria MQT Bench su 28 algoritmi quantistici e oltre 70.000 circuiti (da 2 a 130+ qubit), MIMIQ raggiunge un'**accuratezza del 100% per la grande maggioranza dei circuiti**, piu di qualsiasi altra piattaforma quantistica testata.

## Link

- [Piattaforma MIMIQ](https://qperfect.io/mimiq/)
- [QPerfect](https://qperfect.io)
- [MimiqCircuits su GitHub](https://github.com/qperfect-io/)
