---
title: "QLU"
date: 2024-06-01
summary: "Fault-tolerant compilation and real-time error correction for neutral atom quantum computers. Bridging quantum software to hardware for the fault-tolerant era."
tags: ["quantum computing", "fault tolerance", "error correction", "neutral atoms"]
---

## Overview

[QLU](https://qperfect.io/qlu/) is [QPerfect](https://qperfect.io)'s fault-tolerant quantum middleware, a real-time system that compiles, optimizes, and error-corrects quantum circuits for execution on neutral atom quantum processors. It connects high-level quantum algorithms to the physical constraints of real hardware.

As CTO, I lead the architecture and development of QLU, building on QPerfect's expertise in quantum simulation and neutral atom physics.

## Core Capabilities

- **Fault-tolerant compiler**: Compiles logical quantum circuits into fault-tolerant physical circuits with quantum error correction, magic state distillation, and loss detection. Supports QLDPC codes and surface codes.
- **Hardware-specific optimization**: Tailors circuit execution for neutral atom QPUs with zoned architectures, native gate sets (RX, RZ, CZ), atom movement scheduling, and continuous atom reloading.
- **Real-time error decoder**: On-the-fly quantum error correction decoding for continuous fault-tolerant operation during circuit execution.
- **Secure cloud QDK**: Quantum Development Kit with secure cloud access for remote circuit design, compilation, and execution management.

## Hardware Co-design

Several aspects of QLU are inspired by a close collaboration with [aQCess](https://aQCess.eu), France's first public neutral-atom quantum computing platform at the University of Strasbourg.

## Synergy with MIMIQ

[MIMIQ](/projects/mimiq/) functions as a basis for QLU, since it already provides a formalism to represent and manipulate quantum operations.

## Links

- [QLU Platform](https://qperfect.io/qlu/)
- [QPerfect](https://qperfect.io)
