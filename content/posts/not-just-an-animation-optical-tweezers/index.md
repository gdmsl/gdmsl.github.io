---
title: "Not Just an Animation: Optical Tweezers"
date: 2026-03-11
summary: "The background you just saw is a real optical tweezer sorting simulation running in your browser via WebAssembly."
tags: ["physics", "simulation", "quantum-computing"]
---

You may have noticed the animated background on the page you just came from. It might look decorative, but it is a **real physics simulation** running live in your browser, compiled to WebAssembly from Rust.

No pre-rendered video. No CSS tricks. The math is actually being computed right now, on your device.

These simulations are inspired by real physics but are tuned for visual appeal. The parameters are chosen to look good, not to match any specific experiment. Think of them as **computational art grounded in real science**.

---

{{< simulation name="tweezer" height="500px" config='{"overlayGrid":false,"maxAlpha":1.0}' >}}

## Optical Tweezer Atom Sorting

What you saw is the **atom sorting** process used in neutral-atom quantum computers. Atoms are randomly loaded into an array of optical tweezers (tightly focused laser beams that trap individual atoms), and a movable tweezer rearranges them into a compact, defect-free target region.

In real experiments, each trap captures an atom with roughly 50% probability. A camera images the array to see which traps are loaded. Then a fast algorithm computes the optimal rearrangement plan, and a motorized tweezer picks up atoms one by one (or in parallel), transporting them to fill the gaps. The whole process takes milliseconds and produces a perfect, defect-free qubit register ready for quantum computation.

The simulation alternates between two real sorting algorithms:

**Hungarian algorithm** (optimal assignment): A single tweezer executes the mathematically optimal plan. The Kuhn-Munkres algorithm finds the assignment of source atoms to target positions that minimizes total displacement. The tweezer follows interstitial paths (traveling through the gaps between trap sites) to avoid disturbing other atoms while carrying. This is the theoretically optimal approach.

**Compression** (parallel tweezers): Multiple tweezers operate simultaneously, first compressing all columns toward the center, then compressing all rows. Each step moves atoms by exactly one trap spacing, with all tweezers in lockstep. This is closer to how real experiments work: it's not globally optimal, but it's fast and naturally parallelizable.

The grey circles are the static optical traps. The bright teal dots are trapped atoms. The yellow glow is the sorting tweezer (or tweezers, during compression). Watch how the Hungarian algorithm produces elegant, efficient paths, while compression creates a satisfying coordinated sweep.

This technology is at the heart of companies like **QuEra**, **Pasqal**, and **Atom Computing**, which are building quantum computers with hundreds to thousands of neutral-atom qubits arranged by optical tweezers.

### Under the hood

The simulation runs the full sorting pipeline: random loading, target region computation, algorithm-specific move planning (Hungarian with O(n^3) Kuhn-Munkres, or row/column compression with parallel single-hop steps), and animated execution with interstitial pathfinding. The output is a two-channel float texture: R channel encodes traps and atoms (Gaussian blobs at different amplitudes), G channel encodes tweezer positions. The fragment shader composites traps (grey), tweezers (yellow), and atoms (teal) with proper layering.
