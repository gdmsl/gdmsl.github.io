---
title: "Questa Non E' Solo un'Animazione: Atomi di Rydberg"
date: 2026-03-12
summary: "Lo sfondo che hai appena visto e' una vera simulazione di atomi di Rydberg eseguita nel tuo browser tramite WebAssembly."
tags: ["fisica", "simulazione", "calcolo-quantistico"]
---

Potresti aver notato lo sfondo animato della pagina da cui arrivi. Potrebbe sembrare decorativo, ma e' una **vera simulazione fisica** eseguita in tempo reale nel tuo browser, compilata in WebAssembly a partire da Rust.

Nessun video pre-renderizzato. Nessun trucco CSS. La matematica viene effettivamente calcolata in questo momento, sul tuo dispositivo.

Queste simulazioni sono ispirate dalla fisica reale ma ottimizzate per l'impatto visivo. I parametri sono scelti per essere belli, non per riprodurre un esperimento specifico. Pensale come **arte computazionale radicata nella vera scienza**.

---

{{< simulation name="rydberg" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Atomi di Rydberg

Quello che hai visto e' un gas di **atomi di Rydberg** (atomi eccitati a numeri quantici principali estremamente elevati) che esibiscono dinamiche di eccitazione facilitata.

Gli atomi di Rydberg sono enormi per gli standard atomici. Un atomo di rubidio nello stato di Rydberg 70s ha un orbitale elettronico migliaia di volte piu' grande dello stato fondamentale. Questa dimensione estrema conferisce agli atomi di Rydberg proprieta' straordinarie: interagiscono tra loro tramite forti forze di **van der Waals** a lungo raggio che decadono come 1/r^6.

Il fenomeno chiave e' il **blocco di Rydberg**: quando un atomo e' eccitato, la sua interazione di van der Waals sposta i livelli energetici degli atomi vicini, impedendo loro di essere eccitati. Questo crea una zona di esclusione (il raggio di blocco) attorno a ogni atomo di Rydberg. Tuttavia, se il laser usato per l'eccitazione e' leggermente spostato verso il blu rispetto alla risonanza fondamentale-Rydberg, lo shift repulsivo di van der Waals di un atomo di Rydberg vicino puo compensare il detuning a una distanza specifica, riportando la transizione in risonanza. Questo crea un **guscio di facilitazione** dove l'eccitazione e' potenziata.

I punti blu sono atomi nello stato fondamentale, confinati da una trappola magneto-ottica (MOT). I lampi gialli brillanti sono atomi eccitati allo stato di Rydberg. Osserva come le eccitazioni tendano ad apparire a distanze specifiche dagli atomi di Rydberg esistenti, quello e' il guscio di facilitazione. Gli atomi eccitati si respingono e alla fine decadono di nuovo allo stato fondamentale.

Non e' solo una curiosita'. Gli array di atomi di Rydberg sono una delle piattaforme leader per il **calcolo quantistico**. Aziende come QuEra, Pasqal e gruppi di ricerca in tutto il mondo usano array precisamente controllati di atomi di Rydberg per costruire processori quantistici con centinaia di qubit.

### Sotto il cofano

La simulazione combina dinamiche di eccitazione stocastica con l'algoritmo di Gillespie, integrazione meccanica velocity-Verlet e un termostato di Langevin per il raffreddamento MOT. I tassi di eccitazione seguono un profilo lorentziano centrato sulla risonanza di facilitazione. L'output e' una texture float a due canali: campo di densita' degli atomi fondamentali e campo di densita' degli atomi eccitati, renderizzati come blob gaussiani tramite il fragment shader.
