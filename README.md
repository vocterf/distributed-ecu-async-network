# Distributed ECU Async Network (Asynchronous Rust Learning Sandbox)

Welcome to my dedicated learning repository for mastering asynchronous programming in Rust. 

After successfully completing my synchronous Software-In-The-Loop (SIL) braking core project, this sandbox marks the next logical step in my systems engineering journey: transitioning from single-threaded, sequential loops to concurrent, distributed network simulations.

##  Core Objectives
The ultimate goal of this repository is to learn how to design a decentralized vehicle sub-network. I plan to build a multi-node simulation where independent processes (simulating various electronic control units or sensors) communicate concurrently over a virtual CAN bus (`vcan0`).

* **Current Focus (OS-Level Concurrency):** Learning the fundamentals of the `async/await` execution model, task scheduling, and non-blocking timers using the **Tokio** runtime.
* **Future Milestone (Bare-Metal Async):** Porting these asynchronous concepts down to low-level microcontrollers without an operating system using the **Embassy** framework.

##  Learning Roadmap
- [x] Repository initialization & environment setup under WSL2.
- [ ] Master Tokio basics (spawning asynchronous background tasks, handling loops).
- [ ] Implement non-blocking virtual CAN bus (`vcan0`) frame ingestion.
- [ ] Design a multi-node ecosystem using Cargo Workspaces to simulate separate ECUs.
- [ ] Transition into embedded hardware abstraction layers with Embassy.

##  Current Status
This repository is a live document of my progress. I am currently focused on building basic asynchronous loops and understanding how runtime executors manage multiple tasks simultaneously without blocking the main execution thread.