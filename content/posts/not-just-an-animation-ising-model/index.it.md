---
title: "Questa non è solo un'animazione: il modello di Ising"
date: 2026-03-14
summary: "Lo sfondo che hai appena visto è una vera simulazione del modello di Ising eseguita nel tuo browser tramite WebAssembly."
tags: ["fisica", "simulazione", "meccanica-statistica"]
---

Potresti aver notato lo sfondo animato della pagina da cui arrivi. Potrebbe sembrare decorativo, ma è una **vera simulazione fisica** eseguita in tempo reale nel tuo browser, compilata in WebAssembly a partire da Rust.

Nessun video pre-renderizzato. Nessun trucco CSS. La matematica viene calcolata in questo momento, sul tuo dispositivo.

Queste simulazioni sono ispirate dalla fisica reale ma ottimizzate per l'impatto visivo. I parametri sono scelti per essere belli, non per riprodurre un esperimento specifico. Pensale come **arte computazionale radicata nella vera scienza**.

---

{{< simulation name="ising" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Il modello di Ising

Quello che hai visto è il **modello di Ising**, uno dei sistemi più studiati in tutta la fisica. È ingannevolmente semplice: una griglia di spin, ciascuno rivolto verso l'alto o verso il basso, che interagiscono solo con i primi vicini. Da questa semplicità emerge uno dei fenomeni più profondi della natura: una **transizione di fase**.

Ad alta temperatura, le fluttuazioni termiche dominano. Gli spin si capovolgono costantemente e casualmente, producendo una zuppa rumorosa e disordinata. A bassa temperatura, il costo energetico dei vicini disallineati prevale, e grandi **domini** di spin allineati emergono e crescono. Tra questi regimi si trova la temperatura critica, dove il sistema è in bilico tra ordine e disordine, le fluttuazioni avvengono a tutte le scale di lunghezza e il sistema mostra invarianza di scala.

La simulazione usa l'**algoritmo a cluster di Swendsen-Wang**. Invece di capovolgere uno spin alla volta (come nel più semplice algoritmo di Metropolis), Swendsen-Wang costruisce cluster di spin allineati vicini attivando legami tra di essi con una probabilità che dipende dalla temperatura. Ogni cluster viene poi capovolto nel suo insieme con probabilità del 50%. Questo permette al sistema di fare grandi aggiornamenti collettivi in una singola iterazione, il che è particolarmente importante vicino alla temperatura critica, dove i metodi a singolo spin soffrono di rallentamento critico.

I lampi teal segnalano interi cluster appena capovolti, permettendoti di vedere la dinamica collettiva in tempo reale.

Ernst Ising risolse la versione 1D nel 1924 e non trovò nessuna transizione di fase, portandolo a congetturare (erroneamente) che non ce ne fosse neanche in dimensioni superiori. La soluzione esatta del modello 2D di Lars Onsager nel 1944 (che mostra una transizione di fase netta) è uno dei grandi risultati della fisica matematica.

### Sotto il cofano

La simulazione esegue un Monte Carlo a cluster Swendsen-Wang su un reticolo quadrato 2D con condizioni al contorno periodiche. I cluster sono identificati tramite una struttura dati union-find. L'output è una texture float a due canali: stato dello spin e glow (capovolgimento recente del cluster). Il fragment shader renderizza spin-up come blu elettrico, spin-down come navy profondo, con glow teal sui cluster recentemente capovolti. I bordi delle celle forniscono struttura visiva.
