---
title: "Questa non è solo un'animazione: gli atomi di Rydberg"
date: 2026-03-12
summary: "Lo sfondo che hai appena visto è una vera simulazione di atomi di Rydberg che gira nel tuo browser grazie a WebAssembly."
tags: ["fisica", "simulazione", "calcolo-quantistico"]
---

Forse hai notato lo sfondo animato sulla pagina da cui arrivi. Sembra una semplice decorazione, ma in realtà è una **vera simulazione fisica** che gira dal vivo nel tuo browser, compilata da Rust a WebAssembly.

Niente video preregistrati, niente trucchi CSS. I calcoli avvengono davvero, proprio adesso, sul tuo dispositivo.

Queste simulazioni si ispirano a fenomeni fisici reali, ma le ho regolate per il piacere degli occhi. Ho scelto i parametri per la resa visiva, non per riprodurre un esperimento preciso. Considerale come **arte computazionale radicata nella scienza vera**.

---

{{< simulation name="rydberg" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Atomi di Rydberg

Quello che hai visto è un gas di **atomi di Rydberg** (atomi portati a numeri quantici principali altissimi) animato da una dinamica di eccitazione facilitata.

Gli atomi di Rydberg sono enormi su scala atomica. Un atomo di rubidio nello stato di Rydberg 70s ha un orbitale elettronico migliaia di volte più grande che nello stato fondamentale. Queste dimensioni fuori misura danno loro proprietà straordinarie: interagiscono tra loro tramite intense forze di **van der Waals** a lungo raggio, che decadono come 1/r^6.

Il fenomeno chiave è il **blocco di Rydberg**: appena un atomo viene eccitato, la sua interazione di van der Waals sposta i livelli di energia degli atomi vicini e impedisce anche a loro di eccitarsi. Si forma così una zona di esclusione (il raggio di blocco) attorno a ogni atomo di Rydberg. Se però il laser di eccitazione viene leggermente spostato verso il blu rispetto alla risonanza fondamentale-Rydberg, lo spostamento repulsivo dovuto a un atomo di Rydberg vicino può compensare quel disaccordo a una distanza ben precisa e riportare la transizione in risonanza. Nasce allora un **guscio di facilitazione**, dove l'eccitazione è invece favorita.

I punti blu sono gli atomi nello stato fondamentale, trattenuti da una trappola magneto-ottica (MOT). I lampi giallo acceso sono atomi portati allo stato di Rydberg. Guarda come le eccitazioni tendano a comparire a distanze ben precise dagli atomi di Rydberg già presenti: ecco il guscio di facilitazione. Gli atomi eccitati si respingono e alla fine ricadono nello stato fondamentale.

E non è solo una curiosità. Le schiere di atomi di Rydberg sono tra le piattaforme più promettenti per il **calcolo quantistico**. Aziende come QuEra o Pasqal, e gruppi di ricerca in tutto il mondo, dispongono gli atomi di Rydberg con grande precisione per costruire processori quantistici con centinaia di qubit.

### Dietro le quinte

La simulazione mette insieme una dinamica di eccitazione stocastica con l'algoritmo di Gillespie, un'integrazione meccanica velocity-Verlet e un termostato di Langevin per il raffreddamento del MOT. I tassi di eccitazione seguono un profilo lorentziano centrato sulla risonanza di facilitazione. In uscita si ottiene una texture float a due canali (il campo di densità degli atomi fondamentali e quello degli atomi eccitati), disegnati come macchie gaussiane dal fragment shader.
