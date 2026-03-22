---
title: "Master Thesis: Many-Body Physics of Strongly Interacting Rydberg Atoms"
date: 2015-10-01
summary: "Kinetic Monte Carlo simulations of Rydberg excitation dynamics, blockade, and facilitation in ultracold atomic gases."
tags: ["quantum physics", "rydberg atoms", "monte carlo", "cold atoms"]
---

## Overview

**Many-Body Physics of Strongly Interacting Rydberg Atoms**
Universita degli Studi di Pisa, Dipartimento di Fisica, 2015.
Advisor: Prof. Oliver Morsch.

This thesis explores the out-of-equilibrium dynamics of strongly interacting Rydberg atom ensembles, where long-range van der Waals interactions lead to collective phenomena such as the Rydberg blockade and facilitated excitation.

## Key Topics

- **Rydberg blockade**: When one atom is excited to a Rydberg state, the strong C_6/r^6 van der Waals interaction shifts neighboring atoms out of resonance, preventing their excitation within a blockade radius.
- **Facilitation dynamics**: With a finite detuning, excitation is suppressed for isolated atoms but enhanced at a specific facilitation shell where the vdW shift compensates the detuning, creating spatially correlated excitation avalanches.
- **Rate equations and KMC**: The excitation dynamics are modeled via rate equations with Lorentzian line profiles, solved using kinetic Monte Carlo (Gillespie algorithm) methods.
- **Mechanical dynamics**: Excited atoms experience vdW repulsion and move in space, coupling the internal (excitation) and external (motional) degrees of freedom.

## The Simulation

The Rydberg atom simulation on the [Projects](/projects/) page is directly inspired by this thesis and the [johannes](https://github.com/gdmsl) C++ library developed for it. It implements the same physics: rate-equation excitation with facilitation, van der Waals forces, and velocity Verlet integration for Rb-87 atoms in the 70s Rydberg state.

## Links

- [Universita degli Studi di Pisa](https://www.unipi.it)
