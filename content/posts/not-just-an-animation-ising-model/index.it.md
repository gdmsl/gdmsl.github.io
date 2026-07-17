---
title: "Questa non è solo un'animazione: il modello di Ising"
date: 2026-03-14
summary: "Lo sfondo che hai appena visto è una vera simulazione del modello di Ising che gira nel tuo browser grazie a WebAssembly."
tags: ["fisica", "simulazione", "meccanica-statistica"]
---

Forse hai notato lo sfondo animato sulla pagina da cui arrivi. Sembra una semplice decorazione, ma in realtà è una **vera simulazione fisica** che gira dal vivo nel tuo browser, compilata da Rust a WebAssembly.

Niente video preregistrati, niente trucchi CSS. I calcoli avvengono davvero, proprio adesso, sul tuo dispositivo.

Queste simulazioni si ispirano a fenomeni fisici reali, ma le ho regolate per il piacere degli occhi. Ho scelto i parametri per la resa visiva, non per riprodurre un esperimento preciso. Considerale come **arte computazionale radicata nella scienza vera**.

---

{{< simulation name="ising" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Il modello di Ising

Quello che hai visto è il **modello di Ising**, uno dei sistemi più studiati di tutta la fisica. Sembra semplice: una griglia di spin, ciascuno rivolto verso l'alto o verso il basso, che interagiscono solo con i primi vicini. Eppure da questa semplicità emerge uno dei fenomeni più profondi della natura: una **transizione di fase**.

Alle alte temperature prevalgono le fluttuazioni termiche. Gli spin si capovolgono in continuazione e a caso, in un disordine rumoroso. Alle basse temperature ha invece la meglio il costo energetico dei vicini disallineati: nascono e si allargano ampi **domini** di spin allineati. In mezzo c'è la temperatura critica, dove il sistema resta sul confine tra ordine e disordine: le fluttuazioni si presentano a tutte le scale e il sistema diventa invariante di scala.

La simulazione usa l'**algoritmo a cluster di Swendsen-Wang**. Invece di capovolgere uno spin alla volta (come fa l'algoritmo di Metropolis, più semplice), Swendsen-Wang raggruppa gli spin allineati vicini attivando dei legami tra loro con una probabilità che dipende dalla temperatura. Ogni cluster viene poi capovolto in blocco, una volta su due. Così il sistema può compiere grandi aggiornamenti collettivi in un solo passaggio, cosa che conta soprattutto vicino alla temperatura critica, dove i metodi che toccano un solo spin alla volta si scontrano con il rallentamento critico.

I lampi verde acqua segnalano i cluster appena capovolti in blocco: così puoi seguire la dinamica collettiva in tempo reale.

Ernst Ising risolse la versione 1D nel 1924 senza trovare alcuna transizione di fase, e questo lo portò a supporre (a torto) che non ce ne fosse nemmeno in dimensioni superiori. La soluzione esatta del modello 2D trovata da Lars Onsager nel 1944, che mostra una netta transizione di fase, resta uno dei grandi risultati della fisica matematica.

### Dietro le quinte

La simulazione esegue un Monte Carlo a cluster di Swendsen-Wang su un reticolo quadrato 2D con condizioni al contorno periodiche. I cluster vengono individuati con una struttura dati union-find. Il risultato è una texture float a due canali: lo stato dello spin e il glow (i cluster capovolti di recente). Il fragment shader disegna gli spin verso l'alto in blu elettrico, quelli verso il basso in blu notte, e aggiunge un glow verde acqua sui cluster appena capovolti. I bordi delle celle danno un po' di struttura visiva.
