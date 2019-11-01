# Master's Thesis

Thesis work @ KU Leuven 2019-2020: [_Securing Smart Environments with Authentic Execution_](https://distrinet.cs.kuleuven.be/software/sancus/publications/scopelliti2020.pdf)

## Content

- `presentation_1`: first presentation made for the DistriNet research group at KU Leuven (December 2019)
- `presentation_2`: second presentation made for the DistriNet research group at KU Leuven (April 2020)
- `prototype`: source code and configurations of a prototype for a Smart Irrigation system using Authentic Execution
- `thesis-defense`: thesis defense presented at Politecnico di Torino (July 2020)

## Contributions

- [`authentic-execution`](https://github.com/gianlu33/authentic-execution)
  - Main repository. Contains documentation, utilities and a Makefile to install dependencies and run a demo
- [`reactive-tools`](https://github.com/gianlu33/reactive-tools)
  - Tool to deploy an Authentic Execution system. Forked from `sancus-tee/reactive-tools`
  - Contribution: code rewriting, fix Sancus support, documentation, SGX and native support
- [`reactive-tools-docker`](https://github.com/gianlu33/reactive-tools-docker)
  - Dockerfile and scripts to create a Docker image that contains all the dependencies, libraries and applications used in this work.

- [`rust-sgx-gen`](https://github.com/gianlu33/rust-sgx-gen)
  - A code generator for Rust applications to enable Authentic Execution for SGX. Used by `reactive-tools`
- [`rust-sgx-apps`](https://github.com/gianlu33/rust-sgx-apps)
  - Applications needed for the SGX Authentic Execution framework, e.g., the Event Manager.

- [`rust-sgx-libs`](https://github.com/gianlu33/rust-sgx-libs)
  - Rust libraries used in `rust-sgx-gen` and `rust-sgx-apps`. Contains a library for network communications and a library for crypto operations.
  
- [`sancus-support`](https://github.com/gianlu33/sancus-support)
  - Forked from `sancus-tee/sancus-support`
  - Contribution: fix a bug
  
- [`sancus-compiler`](https://github.com/gianlu33/sancus-compiler)
  - Forked from `sancus-tee/sancus-compiler`
  - Contribution: implement n:n relationships, fix bugs
  
- [`sancus-riot`](https://github.com/fritzalder/sancus-riot)
  - Sancus port of the RIOT OS. Branch: `reactive-app`
  - Contribution: Event Manager application for Sancus
  - **NOTE:** this repository at the moment is private. The ELF binary of the Event Manager for Sancus can be found [here](https://github.com/gianlu33/authentic-execution/tree/master/utils/sancus)
  
- [`reactive-net`](https://github.com/gianlu33/reactive-net)
  - Python module that implements functions for network communication within the framework. Analogous to the Rust library cited above
  
- [`reactive-uart2ip`](https://github.com/gianlu33/reactive-uart2ip)
  - Python application that bridges the UART interface of a Sancus board to a TCP/IP socket


## Other libraries / projects used

- [`spongent-rs`](https://github.com/gianlu33/spongent-rs)
  - A Rust library that implements the SPONGENT crypto library. Forked from `stenverbois/spongent-rs`
  
- [`rust-sgx-remote-attestation`](https://github.com/ndokmai/rust-sgx-remote-attestation)
  - An implementation of Remote Attestation for SGX enclaves built with Fortanix EDP
