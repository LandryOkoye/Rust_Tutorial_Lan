# 🦀 Rust Tutorial Notes

A personal learning repository documenting my progress and lessons learned while mastering Rust programming.

##  Table of Contents

- [Learning Goals](#learning-goals)
- [Progress Overview](#progress-overview)
- [Core Concepts](#core-concepts)
- [Daily Lessons](#daily-lessons)
- [Resources](#resources)
- [Quick Reference](#quick-reference)

---

##  Learning Goals

- [ ] Understand Rust fundamentals (ownership, borrowing, lifetimes)
- [ ] Master pattern matching and error handling
- [ ] Learn async/await and concurrency
- [ ] Build practical projects
- [ ] Understand the type system and generics
- [ ] Master the module system and crates

---

##  Progress Overview

| Week | Focus Area | Status | Notes |
|------|-----------|--------|-------|
| Week 1 | Basics & Setup | ⏳ | Getting familiar with Rust syntax |
| Week 2 | Ownership & Borrowing | ⏳ | Foundation concepts |
| Week 3 | Structs & Methods | ⏳ | Organizing data |
| Week 4 | Error Handling | ⏳ | Working with Results and Options |
| Week 5+ | Advanced Topics | ⏳ | TBD |

---

##  Core Concepts

### 1. **Ownership & Borrowing**

### 2. **Pattern Matching**

### 3. **Error Handling (Result & Option)**

### 4. **Lifetimes**

### 5. **Traits & Generics**

---

##  Daily Lessons

### **Date: [2026-02-11]**
- **Topic**: ***Getting Started***
- **What I Learned**: *Today i installed Rust, created my fist rust project with cargo(Rust package manager) and ran my first Hello World!! program in Rust.*
- **Code Examples**: 
```rust
fn main(){
    println!("Hello World!! This is my first Rust program as a Rustaceun ")
}
```

### **Date: [2026-03-20]**
- **Topic**: ***Variables***
- **What I Learned**: 
    - Unlike variables in other programming languages, Variables in Rust are immutable by defualt until you annotate the variable with the 'mut' keywor to make it mutable.
    - **Shadowing**: In Rust, shadowing refers to the practice of declaring a new variable with the same name as a previous variable. The new variable "shadows" the original, making it inaccessible within the current scope while the new one is active. 

- **Code Examples**: 
```rust
// Todo
// Imma add the code here but i dont know when mate.
```
### **Date: [2026-03-29]**
- **Topic**: ***Data Types***
- **What I Learned**: 
    - Read more about data types [text](https://doc.rust-lang.org/book/ch03-02-data-types.html)   

- **Code Examples**: 
```rust
// SCALAR TYPES

// Signed Int
let number: i32 = 30;
let number: i64 = -30;
let number = 30i32; // You can either anotate or suffix the integer type to the value it self.

// Un-signed int(Intgers that cannot have a sign '-' to them)
let number: u32 = 42;

// Floats are represented with 'f' e.g, f32, f64, f8, f16, depending on the bit size.
let floats: f64 = 12.4;

// Booleans True/False
// just as you know.... true/false
let bool = true;

// Char 
let char = 'L'

// COMPOUND TYPES

// Tupples: can contain values of dif types
let obj = ("landry", 18, 9.9); // just adding values to the tuples.
let objt: (string, i32, f32); // Defining the tuples types

// Arrays
let arr: [i32, 5] = [32, 44, 5, 03, 22]
```

Read more:  [text](https://doc.rust-lang.org/book/ch03-02-data-types.html)
