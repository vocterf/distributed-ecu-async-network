# Distributed Automotive ECU Network (Asynchronous Rust Learning Sandbox)

Welcome to my dedicated repository for mastering asynchronous programming in Rust applied to automotive systems engineering. 

After successfully completing my synchronous Software-In-The-Loop (SIL) braking core project, this sandbox marks a major architectural evolution: transitioning from single-process loops to a **fully distributed, multi-target ECU network simulation** using Cargo Workspaces.

## Core Objectives
The goal of this repository is to design and implement a decentralized vehicle sub-network that strictly mirrors real-world automotive architectures. 

Instead of simulating nodes inside a single binary, this project splits functionality into independent, isolated hardware/software targets communicating concurrently over a CAN bus network governed by a central network matrix (`network.dbc`).

- **OS-Level Async (Telematics Gateway):** Mastering the `async/await` execution model, concurrent task scheduling, and non-blocking I/O using the `Tokio` runtime and `socketcan` to ingest and process network traffic on the host machine.
- **Bare-Metal Async (ECU Nodes):** Implementing hardware-level concurrency on microcontrollers. Currently starting with bare-metal register manipulation (`no_std`), with a direct roadmap toward utilizing the `Embassy` framework for async peripheral driving.

## Network Architecture (Cargo Workspace)

The repository is organized as a multi-target Cargo Workspace to completely isolate different CPU architectures and compilation targets:

```text
distributed-ecu-network/
├── Cargo.toml         # Top-level Workspace configuration
├── network.dbc        # Global CAN network matrix defining frame structures
├── README.md          # Project documentation
│
├── telematics-gateway/# Host PC Node (x86_64 Standard std)
│   └── src/main.rs    # Async Tokio hub; ingests CAN frames, processes data
│
└── ecu-rpm-node/      # Embedded Node (ESP32 Xtensa no_std)
    └── src/main.rs    # Bare-metal microcontroller; reads sensors, broadcasts CAN
```

## Roadmap

- [x] Setup the entire environment and make sure it compiles.
- [ ] Learn the principles of `Embassy` framework.
- [ ] Start the actual making process of this project,
- [ ] Implement CI pipeline, use all the rigoristic flags (clippy and missing_docs).
- [ ] Implement `Embassy` into the project.