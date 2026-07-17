---
title: "bimbumbam"
date: 2026-07-17
summary: "A tiny tool I built so my daughter could bash my keyboard in peace, and a short note on how easy small tools have become."
tags: ["rust", "ai", "tools"]
featureAlt: "A screen full of big, colourful letters scattered on a dark teal background, a bimbumbam canvas made by my daughter."
---

When I sit down to work, my daughter wants to join in: she reaches for the keyboard because that is what she sees me do, and I love that. But a laptop with my work on it is not a toy, and closing the lid or locking the screen only ends the fun for both of us.

So I built her a small thing called **bimbumbam**. One shortcut, and the laptop becomes hers: a fullscreen playground where every key turns into a big bouncing letter, a firework, a little musical note. To come back, I hold a three-key chord for three seconds; that is all.

I only found out later that I am not the first to think of this: there is at least one website that does something similar, which I happened to see advertised on an Instagram reel ([tinykeys.net](http://tinykeys.net/homepage)). But a page in a browser does not help me. It shows the bouncing letters, of course, yet it cannot stop the rest of the machine from reacting. bimbumbam does. It takes over the window manager and grabs almost every global shortcut, so that, while it is running, nothing she presses can reach my work: she cannot switch a workspace, close a window, send the half-written email sitting on my second screen, or drop a random message into Microsoft Teams. I can hand her the laptop and truly relax, which was the whole reason for building it.

It is a silly little program, and I am quite fond of it.

What surprised me was how cheap it turned out to be; bimbumbam is far too niche for anyone to build or sell. A few years ago, doing it on my own would have meant several evenings spent simply understanding the pieces: talking to Wayland through the layer-shell and shortcuts-inhibit protocols, reading the keyboard with `xkbcommon`, drawing on the GPU with `wgpu`, producing sound with `rodio`. Each one is a rabbit hole of its own. With an AI beside me, a single weekend was enough, and the result did not come out as a throwaway hack: it has tests, continuous integration, a Nix package, and it runs on half a dozen Wayland compositors.

I will not pretend that I could rebuild it from scratch in a single evening, or maybe not even in two; but I do not think this is really the point. It is an excellent thing to learn from: I change one small piece, I ask the AI why it works, and each time I understand a little more than before. This is, in fact, how I finally became comfortable with `wgpu`.

And that part was not wasted on a toy: the same rendering I picked up here went straight into the internal R&D tools that I build as part of our [QLU]({{< relref "projects/qlu" >}}) stack at work, the ones that show the movements of neutral atoms: traps, atoms, tweezers, entire zones. An evening spent making letters bounce for my daughter has, over time, turned into skills that I now use to write real software.

The AI wrote a large part of the code, but it did not have the idea, and it did not have the taste. That the exit chord should not fire on a flat-handed slap; that there should be a calm mode without any flashing, for the times when all that light becomes too much; that the notes should be tuned so that even furious mashing still sounds like music: these were my decisions.

This, in the end, is what stayed with me: building tools has become almost free, while caring about the small things has not, and that is still the good part.

bimbumbam is on [GitHub](https://github.com/gdmsl/bimbumbam), under an MIT licence. The picture above is one of my daughter's.
