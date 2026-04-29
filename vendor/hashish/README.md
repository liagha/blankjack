# hashish

> 🚬 A fork of `hashbrown` with a chill name. Same fast hash maps, open to experiments.

`hashish` is a high-performance hash map and hash set implementation in Rust, based on the amazing work done in [`hashbrown`](https://github.com/rust-lang/hashbrown). It’s fast, flexible, and battle-tested — just with a slightly more *relaxed* name.

Think of it as the same hash table core that powers Rust’s standard `HashMap`, but now with room to tinker, explore new ideas, or just enjoy the vibes.

#### To help with reading it correctly, think of it as hash-ish (pronounced hash-ihsh). The name reflects both the concept of hashing (as in hash maps and sets) and a bit of a chill vibe — a fun, laid-back approach to a powerful data structure.

---

## 🚀 Features

- ✅ Drop-in replacement for `hashbrown`
- 🧪 Experimental playground for hash table tweaks and optimizations
- 📦 Supports raw entry API, custom allocators, and more
- 💨 Powered by SwissTable-style probing and SIMD

---

## 📦 Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hashish = "0.0.1"