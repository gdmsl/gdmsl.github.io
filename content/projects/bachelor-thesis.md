---
title: "Bachelor Thesis: Monte Carlo Algorithms for Frustrated Systems"
date: 2013-07-01
summary: "Swendsen-Wang cluster algorithms applied to frustrated spin systems: the 2D Ising model and O(N) nonlinear sigma model."
tags: ["monte carlo", "statistical physics", "ising model", "lattice"]
---

## Overview

**Un algoritmo di Monte Carlo per la simulazione di un sistema frustrato**
(A Monte Carlo Algorithm for the Simulation of a Frustrated System)
Universita degli Studi di Pisa, Dipartimento di Fisica "E. Fermi", 2013.
Advisor: Dott. Giancarlo Cella.

This thesis investigates the performance of cluster Monte Carlo algorithms, specifically the Swendsen-Wang algorithm, when applied to frustrated spin systems, where the usual efficiency gains from cluster updates can break down.

## Key Topics

- **Swendsen-Wang algorithm**: A cluster-based Monte Carlo method that dramatically reduces critical slowing down in unfrustrated spin systems by flipping entire clusters of correlated spins in a single update.
- **Frustration and critical slowing down**: In frustrated systems, where competing interactions prevent all bonds from being simultaneously satisfied, the cluster decomposition becomes less effective. The thesis analyzes how frustration reintroduces critical slowing down.
- **2D Ising model**: Benchmark simulations on the standard 2D Ising model to validate the implementation and measure dynamical critical exponents.
- **O(N) nonlinear sigma model**: Extension of the Swendsen-Wang approach to continuous spin models with Symanzik-improved lattice actions, exploring generalized update rules.

## Links

- [Universita degli Studi di Pisa](https://www.unipi.it)
