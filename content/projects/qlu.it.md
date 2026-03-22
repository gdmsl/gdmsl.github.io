---
title: "QLU: Quantum Logic Unit"
date: 2024-06-01
summary: "Compilazione fault-tolerant e correzione degli errori in tempo reale per computer quantistici ad atomi neutri. Il ponte tra software quantistico e hardware per l'era fault-tolerant."
tags: ["quantum computing", "fault tolerance", "correzione errori", "atomi neutri"]
---

## Panoramica

[QLU](https://qperfect.io/qlu/) e il middleware quantistico fault-tolerant di [QPerfect](https://qperfect.io), un sistema in tempo reale che compila, ottimizza e corregge gli errori dei circuiti quantistici per l'esecuzione su processori quantistici ad atomi neutri. Colma il divario tra algoritmi quantistici ad alto livello e i vincoli fisici dell'hardware reale.

Come CTO, guido l'architettura e lo sviluppo di QLU, costruendo sulla profonda competenza di QPerfect nella simulazione quantistica e nella fisica degli atomi neutri.

## Funzionalita Principali

- **Compilatore fault-tolerant**: Compila circuiti quantistici logici in circuiti fisici fault-tolerant con correzione degli errori quantistici, distillazione di stati magici e rilevamento delle perdite. Supporto per codici QLDPC e codici di superficie.
- **Ottimizzazione hardware-specifica**: Adatta l'esecuzione dei circuiti per QPU ad atomi neutri con architetture a zone, set di gate nativi (RX, RZ, CZ), scheduling del movimento degli atomi e ricaricamento continuo degli atomi.
- **Decodificatore di errori in tempo reale**: Decodifica della correzione degli errori quantistici in tempo reale per operazioni fault-tolerant continue durante l'esecuzione del circuito.
- **QDK cloud sicuro**: Quantum Development Kit con accesso cloud sicuro per la progettazione, compilazione e gestione dell'esecuzione dei circuiti da remoto.

## Co-design Hardware

Diversi aspetti di QLU sono ispirati da una stretta collaborazione con [aQCess](https://aQCess.eu), la prima piattaforma pubblica di calcolo quantistico ad atomi neutri in Francia, presso l'Universita di Strasburgo.

## Sinergia con MIMIQ

[MIMIQ](/projects/mimiq/) funge da base per QLU, poiche fornisce gia un formalismo per rappresentare e manipolare le operazioni quantistiche.

## Link

- [Piattaforma QLU](https://qperfect.io/qlu/)
- [QPerfect](https://qperfect.io)
