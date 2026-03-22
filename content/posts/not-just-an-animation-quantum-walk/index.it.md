---
title: "Questa Non E' Solo un'Animazione: Passeggiata Quantistica"
date: 2026-03-15
summary: "Lo sfondo che hai appena visto e' una vera simulazione di passeggiata quantistica eseguita nel tuo browser tramite WebAssembly."
tags: ["fisica", "simulazione", "quantistica"]
---

Potresti aver notato lo sfondo animato della pagina da cui arrivi. Potrebbe sembrare decorativo, ma e' una **vera simulazione fisica** eseguita in tempo reale nel tuo browser, compilata in WebAssembly a partire da Rust.

Nessun video pre-renderizzato. Nessun trucco CSS. La matematica viene effettivamente calcolata in questo momento, sul tuo dispositivo.

Queste simulazioni sono ispirate dalla fisica reale ma ottimizzate per l'impatto visivo. I parametri sono scelti per essere belli, non per riprodurre un esperimento specifico. Pensale come **arte computazionale radicata nella vera scienza**.

---

{{< simulation name="quantum-walk" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Passeggiata Quantistica

Lo sfondo della homepage simula una **passeggiata quantistica a tempo continuo** su un reticolo 2D.

A differenza di una passeggiata aleatoria classica, che si diffonde come una noiosa gaussiana, un camminatore quantistico esibisce **interferenza**. L'ampiezza di probabilita' si divide, si riflette ai bordi e crea pattern intricati che si espandono balisticamente piuttosto che diffusivamente.

In una passeggiata classica, la distanza tipica dall'origine cresce come la radice quadrata del tempo. In una passeggiata quantistica, cresce *linearmente* con il tempo, un'accelerazione quadratica. Non e' un caso: le passeggiate quantistiche sono uno dei mattoni fondamentali degli algoritmi quantistici, inclusa la ricerca di Grover e certi algoritmi su grafi.

Quello che vedi sullo schermo e' la distribuzione di probabilita' su una griglia 2D. La luminosita' di ogni cella rappresenta quanto e' probabile trovare il camminatore in quel punto. Le frange di interferenza (quelle creste ondulate) sono un fenomeno puramente quantistico senza analogo classico.

### Sotto il cofano

La simulazione risolve l'equazione di Schrodinger su un reticolo discreto usando un metodo split-operator. Ad ogni frame, il modulo WASM calcola uno step temporale e scrive il campo di probabilita' in una texture float a canale singolo. Uno shader WebGL2 mappa la probabilita' sulla rampa di colore blu con vignettatura e griglia sovrapposta.
