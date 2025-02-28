# Multi-Threading and Inter-Process Communication (IPC) Project

## Overview
This project demonstrates **multi-threading, synchronization, and iter-process communication(IPC)** concepts in an **Operating Systems** context. It consiste of two main components:

1. **Multi-Threaded Banking Simulation** (Project A)
- Implements thread management, mutex locks for resource protection, deadlock handling, and condition variables for synchronization.

2. **Inter-Process Communication (IPC) via Pipes** (Project B)
- Demonstrates data exhange between process using **Unix pipes**

The project is developed in **Rust** and runs on **Ubuntu** within a **Parallels Desktop** environment.

---

## **Building and Runing the Project**

1. **Clone the repository**
'''sh
git clone git@github.com:spollar89/Project-1-Multi-Threading-and-IPC.git 
cd Project-1-Multi-Threading-and-IPC

2. **Install Dependencies**
Ensure you have Rust and Cargo installed. If not, install them using: 

curl --proto '=https' --tlsv1.2 =sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

Then, update dependencies:
cargo update

3. **Build the Project**
Run the following command to compile the project:
cargo build --release

4. **Run Multi-Threaded Banking Simulation**
Execute the banking simulation using:
cargo run --bin banking_simulation

5. **Run IPC Demonstration**
Execute the IPC program using:
cargo run --bin ipc_demo


## **Dependecies & Installation**

Ensure the following dependencies are installed:

**Rust Toolchain**

install Rust: rust-lang.org

Verify installation:

rustc --version
cargo --version

**Ubuntu-Specific Packages**

Install necessary Linux utilities: 

sudo apt update && sudo apt install build-essential

**Project-Specific Dependencies**

There are managed via Cargo.toml. Ensure they are installed:

cargo install --path .


Projec Structure
