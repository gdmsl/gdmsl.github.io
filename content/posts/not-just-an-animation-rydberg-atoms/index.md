---
title: "Not Just an Animation: Rydberg Atoms"
date: 2026-03-12
summary: "The background you just saw is a real Rydberg atom simulation running in your browser via WebAssembly."
tags: ["physics", "simulation", "quantum-computing"]
---

You may have noticed the animated background on the page you just came from. It might look decorative, but it is a **real physics simulation** running live in your browser, compiled to WebAssembly from Rust.

No pre-rendered video. No CSS tricks. The math is actually being computed right now, on your device.

These simulations are inspired by real physics but are tuned for visual appeal. The parameters are chosen to look good, not to match any specific experiment. Think of them as **computational art grounded in real science**.

---

{{< simulation name="rydberg" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Rydberg Atoms

What you saw is a gas of **Rydberg atoms**, atoms excited to extremely high principal quantum numbers, exhibiting facilitated excitation dynamics.

Rydberg atoms are enormous by atomic standards. A rubidium atom in the 70s Rydberg state has an electron orbital thousands of times larger than the ground state. This extreme size gives Rydberg atoms extraordinary properties: they interact with each other via strong, long-range **van der Waals forces** that fall off as 1/r^6.

The key phenomenon is the **Rydberg blockade**: when one atom is excited, its van der Waals interaction shifts the energy levels of nearby atoms, preventing them from being excited too. This creates an exclusion zone (the blockade radius) around each Rydberg atom. However, if the laser used for excitation is slightly blue-detuned from the ground-to-Rydberg resonance, the repulsive van der Waals shift from a nearby Rydberg atom can compensate the detuning at a specific distance, bringing the transition back into resonance. This creates a **facilitation shell** where excitation is enhanced.

The blue dots are ground-state atoms, confined by a magneto-optical trap (MOT). The bright yellow flashes are atoms excited to the Rydberg state. Watch how excitations tend to appear at specific distances from existing Rydberg atoms: that's the facilitation shell. The excited atoms repel each other and eventually decay back to the ground state.

This is not just a curiosity. Rydberg atom arrays are one of the leading platforms for **quantum computing**. Companies like QuEra, Pasqal, and research groups worldwide use precisely controlled arrays of Rydberg atoms to build quantum processors with hundreds of qubits.

### Under the hood

The simulation combines Gillespie-algorithm stochastic excitation dynamics with velocity-Verlet mechanical integration and a Langevin thermostat for MOT cooling. Excitation rates follow a Lorentzian profile centered at the facilitation resonance. The output is a two-channel float texture: ground-atom density field and excited-atom density field, rendered as Gaussian blobs via the fragment shader.
