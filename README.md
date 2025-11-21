# Ransomware (Academic Project)

**Authors:** Mike Baker, Tyler Bonnell, Leif Keidl  
**Course Project**

## Overview
This project explores the design and behavior of ransomware in a **controlled, academic, and virtualized environment**. The objective is to understand how ransomware operates, how encryption affects file availability, and how defensive strategies may mitigate such attacks.  
This project **is not intended for real-world deployment**, and all testing is performed inside an isolated Virtual Machine.

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

