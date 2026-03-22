---
title: "Not Just an Animation: Ising Model"
date: 2026-03-14
summary: "The background you just saw is a real Ising model simulation running in your browser via WebAssembly."
tags: ["physics", "simulation", "statistical-mechanics"]
---

You may have noticed the animated background on the page you just came from. It might look decorative, but it is a **real physics simulation** running live in your browser, compiled to WebAssembly from Rust.

No pre-rendered video. No CSS tricks. The math is actually being computed right now, on your device.

These simulations are inspired by real physics but are tuned for visual appeal. The parameters are chosen to look good, not to match any specific experiment. Think of them as **computational art grounded in real science**.

---

{{< simulation name="ising" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## The Ising Model

What you saw is the **Ising model**, one of the most studied systems in all of physics. It's simple: a grid of spins, each pointing up or down, interacting only with their nearest neighbors. From this simplicity emerges a **phase transition**.

At high temperature, thermal fluctuations dominate. Spins flip constantly and randomly, producing a noisy, disordered state. At low temperature, the energy cost of misaligned neighbors wins out, and large **domains** of aligned spins emerge and grow. Between these regimes lies the critical temperature, where the system sits at the edge of order and disorder: fluctuations happen at all length scales and the system exhibits scale invariance.

The simulation uses the **Swendsen-Wang cluster algorithm**. Instead of flipping one spin at a time (as in the simpler Metropolis algorithm), Swendsen-Wang builds clusters of aligned neighboring spins by activating bonds between them with a probability that depends on temperature. Each cluster is then flipped as a whole with 50% probability. This allows the system to make large collective updates in a single sweep, which is especially important near the critical temperature where single-spin methods suffer from critical slowing down.

The teal flashes mark entire clusters that just flipped, letting you see the collective dynamics in real time.

Ernst Ising solved the 1D version in 1924 and found no phase transition, leading him to (incorrectly) conjecture there was none in higher dimensions either. Lars Onsager's exact solution of the 2D model in 1944, showing a sharp phase transition, is one of the great results of mathematical physics.

### Under the hood

The simulation runs a Swendsen-Wang cluster Monte Carlo on a 2D square lattice with periodic boundary conditions. Clusters are identified using a union-find data structure. The output is a two-channel float texture: spin state and glow (recent cluster flip). The fragment shader renders spin-up as electric blue, spin-down as deep navy, with teal glow on recently flipped clusters. Cell borders provide visual structure.
