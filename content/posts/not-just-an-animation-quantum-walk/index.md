---
title: "Not Just an Animation: Quantum Walk"
date: 2026-03-15
summary: "The background you just saw is a real quantum walk simulation running in your browser via WebAssembly."
tags: ["physics", "simulation", "quantum"]
---

You may have noticed the animated background on the page you just came from. It might look decorative, but it is a **real physics simulation** running live in your browser, compiled to WebAssembly from Rust.

No pre-rendered video. No CSS tricks. The math is actually being computed right now, on your device.

These simulations are inspired by real physics but are tuned for visual appeal. The parameters are chosen to look good, not to match any specific experiment. Think of them as **computational art grounded in real science**.

---

{{< simulation name="quantum-walk" height="500px" config='{"overlayGrid":true,"maxAlpha":1.0}' >}}

## Quantum Walk

The homepage background simulates a **continuous-time quantum walk** on a 2D lattice.

Unlike a classical random walk, which spreads out as a Gaussian blob, a quantum walker exhibits **interference**. The probability amplitude splits, reflects off boundaries, and creates intricate patterns that spread ballistically rather than diffusively.

In a classical random walk, the typical distance from the origin grows as the square root of time. In a quantum walk, it grows *linearly* with time, a quadratic speedup. This is not a coincidence: quantum walks are one of the building blocks of quantum algorithms, including Grover's search and certain graph algorithms.

What you see on screen is the probability distribution over a 2D grid. Each cell's brightness represents how likely the walker is to be found there. The interference fringes (those wave-like ridges) are a purely quantum phenomenon with no classical analogue.

### Under the hood

The simulation solves the Schrodinger equation on a discrete lattice using a split-operator method. Each frame, the WASM module computes one time step and writes the probability field into a single-channel float texture. A WebGL2 fragment shader maps the probability to the blue color ramp with vignette and grid overlay.
