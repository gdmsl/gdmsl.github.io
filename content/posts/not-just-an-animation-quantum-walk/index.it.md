---
title: "Questa non è solo un'animazione: la passeggiata quantistica"
date: 2026-03-15
summary: "Lo sfondo che hai appena visto è una vera simulazione di passeggiata quantistica che gira nel tuo browser grazie a WebAssembly."
tags: ["fisica", "simulazione", "quantistica"]
---

Forse hai notato lo sfondo animato sulla pagina da cui arrivi. Sembra una semplice decorazione, ma in realtà è una **vera simulazione fisica** che gira dal vivo nel tuo browser, compilata da Rust a WebAssembly.

Niente video preregistrati, niente trucchi CSS. I calcoli avvengono davvero, proprio adesso, sul tuo dispositivo.

Queste simulazioni si ispirano a fenomeni fisici reali, ma le ho regolate per il piacere degli occhi. Ho scelto i parametri per la resa visiva, non per riprodurre un esperimento preciso. Considerale come **arte computazionale radicata nella scienza vera**.

---

{{< simulation name="quantum-walk" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Passeggiata quantistica

Lo sfondo della homepage simula una **passeggiata quantistica a tempo continuo** su un reticolo bidimensionale.

A differenza di una passeggiata aleatoria classica, che si allarga in una banale macchia gaussiana, un camminatore quantistico dà luogo a **interferenze**. L'ampiezza di probabilità si divide, si riflette sui bordi e disegna motivi complessi che si propagano in modo balistico anziché diffusivo.

In una passeggiata classica, la distanza dall'origine cresce in media come la radice quadrata del tempo. In una passeggiata quantistica cresce *linearmente* con il tempo: un guadagno quadratico. E non è un caso: le passeggiate quantistiche sono uno dei mattoni fondamentali degli algoritmi quantistici, tra cui la ricerca di Grover e alcuni algoritmi sui grafi.

Quello che vedi sullo schermo è la distribuzione di probabilità su una griglia 2D. La luminosità di ogni cella indica quanto è probabile trovarci il camminatore. Le frange di interferenza (quelle creste che ondeggiano) sono un fenomeno puramente quantistico, senza equivalente classico.

### Dietro le quinte

La simulazione risolve l'equazione di Schrödinger su un reticolo discreto con un metodo split-operator. A ogni frame il modulo WASM calcola un passo temporale e scrive il campo di probabilità in una texture float a canale singolo. Uno shader WebGL2 traduce poi la probabilità in un gradiente di blu, con vignettatura e griglia in sovrimpressione.
