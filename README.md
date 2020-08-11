### A random rust project for maze generation to get acquainted with the language

---

I implemented a genetic algorithm to create rooms dynamically for fun.  I am eventually
going to try this out with WASM.

---

Some bugs that require refactoring include:
- The validator for rooms needs to be improved, otherwise some bad rooms appear sometimes due to the fact that the match doesn't cover all cases