# Ransomware (Academic Project)

**Authors:** Mike Baker, Tyler Bonnell, Leif Keidl  
**Course Project**

## Overview
This project explores the design and behavior of ransomware in a **controlled, academic, and virtualized environment**. The objective is to understand how ransomware operates and changes a live system.
This project **is not intended for real-world deployment**, and all testing is performed inside an isolated Virtual Machine specifically Seed Ubuntu (20.04).

## Project Goals
- Execute a program that encrypts the contents of the user’s **home directory** inside a Linux VM.
- Implement encryption using **Rust** for its memory safety and performance characteristics.
- Use the **OpenSSL Rust crate** to perform the cryptographic operations.
- Demonstrate concepts from class, including:
  - Key management
  - Encryption modes
  - Secure handling of sensitive operations
- Use GitHub for version control and collaboration between group members.

## Technical Approach
- The ransomware simulation is implemented in **Rust**, chosen for:
  - Low-level control similar to C
  - Memory safety through borrow-checking and strong type guarantees
- Encryption routines rely on the **OpenSSL** crate, which provides well-tested cryptographic primitives.
- The program targets **all directories within `$HOME`**, recursively walking and encrypting accessible files.
- All development and testing occur within a **Virtual Machine** to prevent accidental harm to the host system.

## Decryption Goal
A stretch goal of this project is to implement **full decryption of affected files** if sufficient development time remains.  
However, successful real-world ransomware frequently does **not** provide functional decryption even after payment; therefore, this feature is non-essential for the academic outcome.

## Safety Notes
- All testing must remain inside a **sandboxed VM environment**.
- The software produced in this project must **never** be executed on a production machine.
- This project is strictly for **educational and research purposes**.

## References
- Rust Language Documentation: https://doc.rust-lang.org/stable/
- OpenSSL Rust Crate Documentation: https://docs.rs/openssl/latest/openssl/

## User Manual

> **WARNING (VM ONLY):** This program **encrypts files under your Linux user’s `$HOME` directory**. Run it **only** inside an isolated class VM (Seed Ubuntu 20.04). **Take a VM snapshot first** so you can revert after testing.

### Prerequisites
- You are inside your **Seed Ubuntu 20.04 VM** (not your host machine).
- Rust tooling installed (`cargo`, `rustc`).
- Recommended: create a VM snapshot / restore point before running.

### Get the code

#### Clone (HTTPS)
```bash
git clone https://github.com/leifkeidl/ransomware.git YOUR_DIR_NAME
```

#### Clone (SSH)
```bash
git clone git@github.com:leifkeidl/ransomware.git YOUR_DIR_NAME
```

#### Enter the repository
```bash
cd YOUR_DIR_NAME
```

### Build and run

#### Option A: Build + run with Cargo (debug)
```bash
cargo run
```
- This builds and runs the program from `target/debug/`.

#### Option B: Build + run optimized binary
```bash
cargo build --release
chmod +x target/release/ransomware
./target/release/ransomware
```

### Result
- The program attempts to recursively process files under **`$HOME`**.
- Files become encrypted by AES-GCM (256-bit random keys)
- The program is resilient to errors and will safely continue if a file fails to encrypt.
- The user space directories get encrypted while system files remain unchanged.
