---
title: "MIMIQ: computer quantistico virtuale"
date: 2022-06-01
summary: "Il computer quantistico virtuale più potente al mondo. Simulazione veloce e accurata di circuiti quantistici con motori statevector e MPS, SDK Python e Julia, e deployment cloud gestito."
tags: ["quantum computing", "simulazione", "julia", "python"]
---

## Panoramica

[MIMIQ](https://qperfect.io/mimiq/) è il prodotto di punta di [QPerfect](https://qperfect.io): un computer quantistico virtuale che permette a ricercatori e ingegneri di programmare ed eseguire algoritmi quantistici con velocità, accuratezza e flessibilità. Con MIMIQ si possono progettare, testare e validare i propri circuiti quantistici prima di eseguirli su hardware reale, oppure esplorare regimi che nessun hardware attuale riesce a raggiungere.

Come CTO ho progettato l'architettura di MIMIQ da zero e ne ho guidato lo sviluppo, trasformando un prototipo nato dalla ricerca in una piattaforma di produzione usata da laboratori di ricerca e clienti industriali.

## Tecnologia

MIMIQ combina due motori di simulazione complementari:

- **Motore Statevector**: simulazione esatta e ultraveloce di circuiti quantistici fino a ~32 qubit, grazie a ottimizzazioni CPU a basso livello (AVX/SIMD), a un precondizionamento avanzato dei circuiti e a una gestione parsimoniosa della memoria.
- **Motore Matrix Product States (MPS)**: simulazione su larga scala con reti tensoriali, capace di gestire circuiti da **migliaia di qubit** quando l'entanglement resta limitato. In questo modo MIMIQ affronta circuiti fuori dalla portata degli approcci statevector a forza bruta.

MIMIQ calcola in modo veloce e numericamente esatto circuiti quantistici arbitrari con **milioni di gate**, e circuiti a entanglement limitato con **migliaia di qubit**.

## SDK MimiqCircuits

[MimiqCircuits](https://github.com/qperfect-io/) offre librerie open source per circuiti quantistici, sia in **Python** sia in **Julia**:

- un'ampia libreria di gate e primitive per comporre circuiti in modo efficiente;
- circuiti dinamici: misure mid-circuit, reset dei qubit, feedforward classico e logica condizionale;
- accesso completo alle proprietà dello stato quantistico: ampiezze, valori di aspettazione, misure di entanglement e fedeltà del circuito;
- pieno supporto di OpenQASM v2 per importare ed esportare circuiti;
- simulazione di circuiti quantistici, sia ideali sia rumorosi, con modelli di rumore personalizzabili;
- integrazione con il backend cloud MIMIQ.

## Deployment

MIMIQ è disponibile come servizio cloud gestito oppure come installazione on-premises su cluster HPC. Il cloud dà accesso immediato tramite gli SDK Python e Julia, mentre l'opzione on-premises garantisce pieno controllo sulle risorse hardware e sulla sovranità dei dati.

## Benchmark

Valutato con la libreria MQT Bench su 28 algoritmi quantistici e oltre 70.000 circuiti (da 2 a oltre 130 qubit), MIMIQ raggiunge un'**accuratezza del 100% sulla grande maggioranza dei circuiti**, più di qualsiasi altra piattaforma quantistica testata.

## Link

- [Piattaforma MIMIQ](https://qperfect.io/mimiq/)
- [QPerfect](https://qperfect.io)
- [MimiqCircuits su GitHub](https://github.com/qperfect-io/)
