# Unsafe Pointer Manipulation in Rust

This repository demonstrates a potential issue with unsafe pointer manipulation in Rust. The code in `bug.rs` shows how modifying a vector's contents through a raw pointer can lead to undefined behavior if not handled carefully. The corrected version is provided in `bugSolution.rs`.

**Issue:**
Directly manipulating memory using raw pointers can easily introduce subtle bugs. The `bug.rs` example showcases this danger. Because Rust's ownership system is bypassed using unsafe code, unexpected behavior, crashes, or data corruption can arise.

**Solution:**
The `bugSolution.rs` file provides a safer alternative that avoids raw pointers altogether, leveraging Rust's memory management and borrow checker to maintain data integrity.