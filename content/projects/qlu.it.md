---
title: "QLU: Quantum Logic Unit"
date: 2024-06-01
summary: "Compilazione fault-tolerant e correzione degli errori in tempo reale per i computer quantistici ad atomi neutri. Il ponte tra il software quantistico e l'hardware per l'era fault-tolerant."
tags: ["quantum computing", "fault tolerance", "correzione errori", "atomi neutri"]
---

## Panoramica

[QLU](https://qperfect.io/qlu/) è il middleware quantistico fault-tolerant di [QPerfect](https://qperfect.io): un sistema in tempo reale che compila, ottimizza e corregge gli errori dei circuiti quantistici in vista dell'esecuzione su processori quantistici ad atomi neutri. Fa da ponte tra gli algoritmi quantistici ad alto livello e i vincoli fisici dell'hardware reale.

Come CTO guido l'architettura e lo sviluppo di QLU, facendo leva sull'esperienza di QPerfect nella simulazione quantistica e nella fisica degli atomi neutri.

## Funzionalità principali

- **Compilatore fault-tolerant**: compila i circuiti quantistici logici in circuiti fisici fault-tolerant, con correzione degli errori quantistici, distillazione di stati magici e rilevamento delle perdite. Supporta i codici QLDPC e i codici di superficie.
- **Ottimizzazione su misura per l'hardware**: adatta l'esecuzione dei circuiti alle QPU ad atomi neutri con architettura a zone: set di gate nativi (RX, RZ, CZ), pianificazione dei movimenti degli atomi e ricarica continua degli atomi.
- **Decodificatore di errori in tempo reale**: decodifica al volo la correzione degli errori quantistici, per un funzionamento fault-tolerant continuo durante l'esecuzione del circuito.
- **QDK cloud sicuro**: un Quantum Development Kit con accesso cloud sicuro per progettare, compilare e gestire da remoto l'esecuzione dei circuiti.

## Co-design hardware

Diverse scelte progettuali di QLU nascono da una stretta collaborazione con [aQCess](https://aQCess.eu), la prima piattaforma pubblica di calcolo quantistico ad atomi neutri in Francia, presso l'Università di Strasburgo.

## Sinergia con MIMIQ

[MIMIQ](/projects/mimiq/) fa da base a QLU: offre già un formalismo per rappresentare e manipolare le operazioni quantistiche.

## Link

- [Piattaforma QLU](https://qperfect.io/qlu/)
- [QPerfect](https://qperfect.io)
