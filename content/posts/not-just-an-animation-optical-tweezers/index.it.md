---
title: "Questa non è solo un'animazione: le pinzette ottiche"
date: 2026-03-11
summary: "Lo sfondo che hai appena visto è una vera simulazione di ordinamento con pinzette ottiche eseguita nel tuo browser tramite WebAssembly."
tags: ["fisica", "simulazione", "calcolo-quantistico"]
---

Forse hai notato lo sfondo animato sulla pagina da cui arrivi. Sembra decorativo, ma in realtà è una **vera simulazione fisica** che gira dal vivo nel tuo browser, compilata in WebAssembly a partire da Rust.

Nessun video pre-registrato, nessun trucco CSS. I calcoli avvengono davvero, proprio adesso, sul tuo dispositivo.

Queste simulazioni si ispirano alla fisica reale, ma le ho regolate per il piacere degli occhi. Ho scelto i parametri per la resa visiva, non per riprodurre un esperimento preciso. Puoi vederle come **arte computazionale radicata nella scienza vera**.

---

{{< simulation name="tweezer" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Ordinamento atomico con pinzette ottiche

Quello che hai visto è l'**ordinamento atomico** usato nei computer quantistici ad atomi neutri. Gli atomi vengono caricati a caso in una griglia di pinzette ottiche (fasci laser molto focalizzati che intrappolano gli atomi uno a uno), e poi una pinzetta mobile li riorganizza in una zona di destinazione compatta e senza difetti.

Negli esperimenti reali ogni trappola cattura un atomo con una probabilità di circa il 50 %. Una telecamera fotografa la griglia per capire quali trappole sono occupate. A quel punto un algoritmo veloce calcola il piano di riorganizzazione migliore, e una pinzetta motorizzata raccoglie gli atomi uno per uno (o in parallelo) e li sposta per colmare i vuoti. Tutto questo richiede pochi millisecondi e produce un registro di qubit perfetto, senza difetti, pronto per il calcolo quantistico.

La simulazione alterna due veri algoritmi di ordinamento:

**Algoritmo ungherese** (assegnazione ottimale): una sola pinzetta esegue il piano matematicamente ottimale. L'algoritmo di Kuhn-Munkres stabilisce a quale posizione di destinazione assegnare ciascun atomo di partenza in modo da ridurre al minimo lo spostamento totale. La pinzetta segue percorsi interstiziali (passa negli spazi tra una trappola e l'altra) per non disturbare gli altri atomi mentre trasporta. È l'approccio ottimale in teoria.

**Compressione** (pinzette in parallelo): più pinzette lavorano insieme, comprimendo prima tutte le colonne verso il centro e poi tutte le righe. A ogni passo gli atomi avanzano di esattamente un passo del reticolo, con tutte le pinzette all'unisono. Qui ci si avvicina a come funzionano davvero gli esperimenti: non è ottimale a livello globale, ma è veloce e si parallelizza in modo naturale.

I cerchi grigi sono le trappole ottiche fisse. I punti brillanti verde acqua sono gli atomi intrappolati. Il bagliore giallo è la pinzetta di ordinamento (o le pinzette, durante la compressione). Guarda come l'algoritmo ungherese traccia percorsi eleganti ed efficienti, mentre la compressione dà una passata coordinata davvero soddisfacente.

Questa tecnologia è al cuore di aziende come **QuEra**, **Pasqal** e **Atom Computing**, che stanno costruendo computer quantistici con centinaia o migliaia di qubit ad atomi neutri disposti da pinzette ottiche.

### Dietro le quinte

La simulazione porta avanti l'intera pipeline di ordinamento: caricamento casuale, calcolo della zona di destinazione, pianificazione dei movimenti specifica per ogni algoritmo (l'ungherese con Kuhn-Munkres in O(n^3), oppure la compressione per righe e colonne con passi paralleli di una sola casella) ed esecuzione animata con ricerca del percorso interstiziale. Il risultato è una texture float a due canali: il canale R codifica trappole e atomi (macchie gaussiane di ampiezza diversa), il canale G codifica la posizione delle pinzette. Il fragment shader sovrappone in modo corretto trappole (in grigio), pinzette (in giallo) e atomi (verde acqua).
