---
title: "Questa non è solo un'animazione: le pinzette ottiche"
date: 2026-03-11
summary: "Lo sfondo che hai appena visto è una vera simulazione di ordinamento con pinzette ottiche eseguita nel tuo browser tramite WebAssembly."
tags: ["fisica", "simulazione", "calcolo-quantistico"]
---

Potresti aver notato lo sfondo animato della pagina da cui arrivi. Potrebbe sembrare decorativo, ma è una **vera simulazione fisica** eseguita in tempo reale nel tuo browser, compilata in WebAssembly a partire da Rust.

Nessun video pre-renderizzato. Nessun trucco CSS. La matematica viene calcolata in questo momento, sul tuo dispositivo.

Queste simulazioni sono ispirate dalla fisica reale ma ottimizzate per l'impatto visivo. I parametri sono scelti per essere belli, non per riprodurre un esperimento specifico. Pensale come **arte computazionale radicata nella vera scienza**.

---

{{< simulation name="tweezer" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Ordinamento atomico con pinzette ottiche

Quello che hai visto è il processo di **ordinamento atomico** usato nei computer quantistici ad atomi neutri. Gli atomi vengono caricati casualmente in un array di pinzette ottiche (fasci laser focalizzati che intrappolano singoli atomi), e una pinzetta mobile di ordinamento li riorganizza in una regione target compatta e priva di difetti.

Negli esperimenti reali, ogni trappola cattura un atomo con probabilità di circa il 50%. Una telecamera cattura un'immagine dell'array per vedere quali trappole sono cariche. Poi un algoritmo veloce calcola il piano di riorganizzazione ottimale, e una pinzetta motorizzata preleva gli atomi uno alla volta (o in parallelo), trasportandoli per riempire le lacune. L'intero processo richiede millisecondi e produce un registro di qubit perfetto, privo di difetti, pronto per il calcolo quantistico.

La simulazione alterna due algoritmi di ordinamento reali:

**Algoritmo ungherese** (assegnamento ottimale): Una singola pinzetta esegue il piano matematicamente ottimale. L'algoritmo di Kuhn-Munkres trova l'assegnamento degli atomi sorgente alle posizioni target che minimizza lo spostamento totale. La pinzetta segue percorsi interstiziali (viaggiando attraverso gli spazi tra i siti delle trappole) per evitare di disturbare altri atomi durante il trasporto. È l'approccio teoricamente ottimale.

**Compressione** (pinzette parallele): Più pinzette operano simultaneamente, prima comprimendo tutte le colonne verso il centro, poi comprimendo tutte le righe. Ogni passo muove gli atomi di esattamente uno spaziamento di trappola, con tutte le pinzette sincronizzate. È più vicino a come funzionano gli esperimenti reali: non è globalmente ottimale, ma è veloce e naturalmente parallelizzabile.

I cerchi grigi sono le trappole ottiche statiche. I punti teal brillanti sono gli atomi intrappolati. Il bagliore giallo è la pinzetta di ordinamento (o le pinzette, durante la compressione). Osserva come l'algoritmo ungherese produce percorsi eleganti ed efficienti, mentre la compressione crea uno sweep coordinato e soddisfacente.

Questa tecnologia è al cuore di aziende come **QuEra**, **Pasqal** e **Atom Computing**, che stanno costruendo computer quantistici con centinaia o migliaia di qubit ad atomi neutri organizzati da pinzette ottiche.

### Sotto il cofano

La simulazione esegue l'intera pipeline di ordinamento: caricamento casuale, calcolo della regione target, pianificazione dei movimenti specifica per algoritmo (ungherese con Kuhn-Munkres O(n^3), o compressione per righe/colonne con passi paralleli a singolo hop), ed esecuzione animata con pathfinding interstiziale. L'output è una texture float a due canali: il canale R codifica trappole e atomi (blob gaussiani a diverse ampiezze), il canale G codifica le posizioni delle pinzette. Il fragment shader compone trappole (grigio), pinzette (giallo) e atomi (teal) con stratificazione corretta.
