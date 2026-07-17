---
title: "BIM! BUM! BAM!"
date: 2026-07-17
summary: "Un piccolo strumento che ho costruito perché mia figlia potesse pestare la tastiera in pace, e due parole su quanto sia diventato facile farsi gli strumenti da soli."
tags: ["rust", "ai", "strumenti"]
featureAlt: "Uno schermo pieno di grandi lettere colorate sparse su uno sfondo verde acqua scuro, una tela di bimbumbam creata da mia figlia."
---

Ogni volta che a casa mi siedo davanti al computer per lavorare, mia figlia corre subito da me, mi sale in braccio e sbatte le mani sulla tastiera come fa il papà. Ma il portatile con i miei documenti di lavoro aperti non è un giocattolo, e chiudere il portatile o bloccare lo schermo non fa che rovinare il divertimento a tutti e due tra urla e pianti.

Per questo motivo ho creato un piccolo e divertente programma chiamato **bimbumbam**. Basta una scorciatoia e il portatile diventa suo: una specie di parco giochi a tutto schermo, dove ogni tasto si trasforma in una lettera che rimbalza, in un fuoco d'artificio, in una nota musicale. Una volta stancata, mi basta tenere premuti tre tasti insieme per tre secondi e torno al lavoro.

Solo dopo qualche giorno dalla prima versione ho scoperto di non essere stato il primo ad averci pensato: c'è almeno un sito che fa una cosa simile, che tra l'altro ho visto pubblicizzato in un reel su Instagram ([tinykeys.net](http://tinykeys.net/homepage)). Ma una pagina in un browser, a me, non basta: le lettere che rimbalzano te le fa vedere, d'accordo, però non blocca tutto il resto. bimbumbam sì. Si prende il window manager e intercetta quasi tutte le scorciatoie globali, così che, finché è aperto, non lascia passare nessun comando né combinazione di tasti, per quanto strani o improbabili: non può cambiare workspace, chiudere una finestra, mandare per sbaglio la mail lasciata a metà sull'altro schermo o far partire un messaggio a caso su Microsoft Teams. Le lascio il portatile e mi rilasso per davvero.

È un programmino sciocco, quasi banale, ma mi ci sono affezionato.

Quello che mi ha stupito è quanto poco mi sia costato: bimbumbam è troppo di nicchia perché qualcuno lo faccia o lo venda. Qualche anno fa, per farlo da solo, avrei buttato via diverse serate solo per capire come mettere insieme le varie parti: dialogare con Wayland tramite i protocolli layer-shell e shortcuts-inhibit, leggere la tastiera con `xkbcommon`, disegnare sulla GPU con `wgpu`, generare il suono con `rodio`. Ognuno di questi è un pozzo senza fondo a sé. Con un'AI di fianco, invece, è bastata una serata del fine settimana, e non ne è uscito un accrocchio usa e getta: ha i test, l'integrazione continua, un pacchetto per Nix/NixOS e gira su una mezza dozzina di compositor/window manager per Wayland (io uso `niri`).

Sono più che sicuro che non saprei ricostruirlo da zero in una serata; forse nemmeno in due. Ma non è quello il punto. Piuttosto, è un'ottima base per imparare: cambio un pezzetto, chiedo all'AI perché o come funziona, e ogni volta riesco a capire qualcosa in più. È così, in fondo, che ho finito per prendere confidenza con `wgpu`.

Alla fine, non è stato tutto tempo buttato dietro a un programma-giocattolo. Lo stesso rendering che ho imparato qui è finito dritto negli strumenti di ricerca e sviluppo interni di QPerfect, che sviluppo per il nostro framework di compilazione [QLU]({{< relref "projects/qlu" >}}); librerie e programmi che mostrano il movimento degli atomi neutri: trappole, atomi, pinzette ottiche, o intere zone. Una sera passata a far rimbalzare lettere per mia figlia si è trasformata, col tempo, in competenze che oggi mi servono per scrivere software vero.

L'AI ha scritto buona parte del codice, ma il problema, l'idea e la soluzione sono venuti da me, e lo stesso vale per il gusto. Lo stile, i colori; il fatto che la combinazione di uscita vada ricordata ogni tanto agli utenti dalla memoria corta; o il fatto che serva una modalità tranquilla, senza lampeggi, per quando tutta quella luce diventa troppa; o addirittura che le note vadano accordate in modo che anche una pestata furiosa sulla tastiera suoni comunque come musica: queste scelte le ho fatte tutte io.

Ed è questo, alla fine, quello che mi resta: costruire strumenti è ormai quasi gratis; prendersi cura dei dettagli no, ed è sempre stata lì la parte più bella e interessante del programmare.

bimbumbam è su [GitHub](https://github.com/gdmsl/bimbumbam), con licenza MIT. L'immagine qui sopra l'ha fatta mia figlia.
