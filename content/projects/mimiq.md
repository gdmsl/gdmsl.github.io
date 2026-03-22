---
title: "MIMIQ"
date: 2022-06-01
summary: "The world's most powerful virtual quantum computer. Fast, accurate quantum circuit simulation with statevector and MPS engines, Python and Julia SDKs, and managed cloud deployment."
tags: ["quantum computing", "simulation", "julia", "python"]
---

## Overview

[MIMIQ](https://qperfect.io/mimiq/) is the flagship product of [QPerfect](https://qperfect.io), a virtual quantum computer that lets researchers and engineers program and run quantum algorithms with high speed, accuracy, and flexibility. It allows users to design, test, and validate quantum circuits before running them on real hardware, or to explore regimes that no current hardware can reach.

As CTO, I architected MIMIQ from the ground up, leading its development from an academic prototype into a production platform serving research labs and enterprise customers.

## Core Technology

MIMIQ integrates two complementary simulation engines:

- **Statevector engine**: Fast exact simulation of quantum circuits up to ~32 qubits, powered by low-level CPU optimizations (AVX/SIMD), advanced circuit pre-conditioning, and memory-efficient state management.
- **Matrix Product States (MPS) engine**: Large-scale simulation using tensor network techniques, enabling circuits with **thousands of qubits** for problems with bounded entanglement. This makes MIMIQ capable of handling circuits that are out of reach for brute-force statevector approaches.

MIMIQ enables fast and numerically exact calculation of arbitrary quantum circuits with **millions of gates** and entanglement-bounded circuits with **thousands of qubits**.

## MimiqCircuits SDK

[MimiqCircuits](https://github.com/qperfect-io/) provides open-source quantum circuit libraries in both **Python** and **Julia**:

- Extensive library of gates and circuit primitives for efficient circuit composition
- Dynamic circuits: midcircuit measurements, qubit reset, classical feedforward, and conditional logic
- Full access to quantum state properties: amplitudes, expectation values, entanglement measures, and circuit fidelity
- Full OpenQASM v2 support for circuit import and export
- Simulation of both ideal and noisy quantum circuits with customizable noise models
- Integration with the MIMIQ cloud backend

## Deployment

MIMIQ is available as a managed cloud service or as an on-premises deployment for HPC clusters. The cloud platform provides instant access through Python and Julia SDKs, while the on-premises option gives full control over hardware resources and data sovereignty.

## Benchmarks

Benchmarked against the MQT Bench library across 28 quantum algorithms and over 70,000 circuits (2 to 130+ qubits), MIMIQ achieves **100% accuracy for the vast majority of circuits**, more than any other quantum platform tested.

## Links

- [MIMIQ Platform](https://qperfect.io/mimiq/)
- [QPerfect](https://qperfect.io)
- [MimiqCircuits on GitHub](https://github.com/qperfect-io/)
